use ::time::OffsetDateTime;
use actix_web::web;
use chrono::Utc;
use rauthy_common::constants::{CACHE_NAME_12HR, DB_TYPE, IDX_JWK_KID, OFFLINE_TOKEN_LT};
use rauthy_common::DbType;
use rauthy_models::app_state::{AppState, DbPool};
use rauthy_models::email::send_pwd_reset_info;
use rauthy_models::entity::jwk::Jwk;
use rauthy_models::entity::refresh_tokens::RefreshToken;
use rauthy_models::entity::sessions::Session;
use rauthy_models::entity::users::User;
use rauthy_models::migration::backup_db;
use redhac::{cache_del, QuorumHealthState, QuorumState};
use std::collections::HashSet;
use std::env;
use std::ops::{Add, Sub};
use std::str::FromStr;
use std::time::Duration;
use tokio::sync::watch::Receiver;
use tokio::time;
use tracing::{debug, error, info};

pub async fn scheduler_main(data: web::Data<AppState>) {
    info!("Starting schedulers");

    let rx_health = data.caches.ha_cache_config.rx_health_state.clone();

    tokio::spawn(db_backup(data.db.clone()));
    tokio::spawn(events_cleanup(data.db.clone(), rx_health.clone()));
    tokio::spawn(magic_link_cleanup(data.db.clone(), rx_health.clone()));
    tokio::spawn(refresh_tokens_cleanup(data.db.clone(), rx_health.clone()));
    tokio::spawn(sessions_cleanup(data.db.clone(), rx_health.clone()));
    tokio::spawn(jwks_cleanup(data.clone(), rx_health.clone()));
    tokio::spawn(password_expiry_checker(data.clone(), rx_health.clone()));
    tokio::spawn(user_expiry_checker(data, rx_health));
}

// Creates a backup of the data store
pub async fn db_backup(db: DbPool) {
    if *DB_TYPE == DbType::Postgres {
        debug!("Using Postgres as the main database - automatic backups disabled");
        return;
    }

    let mut cron_task = env::var("BACKUP_TASK").unwrap_or_else(|_| "0 0 4 * * * *".to_string());

    // sec min hour day_of_month month day_of_week year
    let schedule = match cron::Schedule::from_str(&cron_task) {
        Ok(sched) => sched,
        Err(err) => {
            error!("Error creating a cron scheduler with the given BACKUP_TASK input: {} - using default \"0 0 4 * * * *\": {}", cron_task, err);
            cron_task = "0 0 4 * * * *".to_string();
            cron::Schedule::from_str(&cron_task).unwrap()
        }
    };

    info!("Database backups are scheduled for: {}", cron_task);

    loop {
        sleep_schedule_next(&schedule).await;
        debug!("Running db_backup scheduler");

        // VACUUM main INTO 'data/rauthy.db.backup-'

        if let Err(err) = backup_db(&db).await {
            error!("{}", err.message);
        }
    }
}

// Cleans up all Events that exceed the configured EVENT_CLEANUP_DAYS
pub async fn events_cleanup(db: DbPool, rx_health: Receiver<Option<QuorumHealthState>>) {
    let mut interval = time::interval(Duration::from_secs(3600));

    let cleanup_days = env::var("EVENT_CLEANUP_DAYS")
        .unwrap_or_else(|_| "31".to_string())
        .parse::<u32>()
        .expect("Cannot parse EVENT_CLEANUP_DAYS to u32") as i64;

    loop {
        interval.tick().await;

        // will return None in a non-HA deployment
        if let Some(is_ha_leader) = is_ha_leader(&rx_health) {
            if !is_ha_leader {
                debug!(
                    "Running HA mode without being the leader - skipping events_cleanup scheduler"
                );
                continue;
            }
        }

        debug!("Running events_cleanup scheduler");

        let threshold = Utc::now()
            .sub(chrono::Duration::days(cleanup_days))
            .timestamp_millis();
        let res = sqlx::query!("DELETE FROM events WHERE timestamp < $1", threshold)
            .execute(&db)
            .await;
        match res {
            Ok(r) => {
                debug!("Cleaned up {} expired events", r.rows_affected());
            }
            Err(err) => error!("Events cleanup error: {:?}", err),
        }
    }
}

// Cleans up old / expired magic links and deletes users, that have never used their
// 'set first ever password' magic link to keep the database clean in case of an open user registration.
// Runs every 6 hours.
pub async fn magic_link_cleanup(db: DbPool, rx_health: Receiver<Option<QuorumHealthState>>) {
    let mut interval = time::interval(Duration::from_secs(3600 * 6));

    loop {
        interval.tick().await;

        // will return None in a non-HA deployment
        if let Some(is_ha_leader) = is_ha_leader(&rx_health) {
            if !is_ha_leader {
                debug!("Running HA mode without being the leader - skipping magic_link_cleanup scheduler");
                continue;
            }
        }

        debug!("Running magic_link_cleanup scheduler");

        // allow 300 seconds of clock skew before cleaning up magic links
        let exp = OffsetDateTime::now_utc().unix_timestamp() - 300;

        // Check for expired and unused magic links that are bound to a user which has no password
        // at all. These users should be deleted since they never cared about the (very important)
        // password E-Mail.
        let res = sqlx::query(
            r#"delete from users where
            id in (select distinct user_id from magic_links where exp < 1683003398 and used = false)
            and password is null"#,
        )
        .bind(exp)
        .execute(&db)
        .await;
        match res {
            Ok(r) => {
                debug!(
                    "Cleaned up {} users which did not use their initial password reset magic link",
                    r.rows_affected()
                );
            }
            Err(err) => error!("Magic link / orphan users cleanup error: {:?}", err),
        }

        // now we can just delete all expired magic links
        let res = sqlx::query("delete from magic_links where exp < $1")
            .bind(exp)
            .execute(&db)
            .await;
        match res {
            Ok(r) => {
                debug!(
                    "Cleaned up {} expired and used magic links",
                    r.rows_affected()
                );
            }
            Err(err) => error!("Magic link cleanup error: {:?}", err),
        }
    }
}

// Checks soon expiring passwords and notifies the user accordingly.
// Runs once every night at 04:30.
// TODO modify somehow to prevent multiple E-Mails in a HA deployment
pub async fn password_expiry_checker(
    data: web::Data<AppState>,
    rx_health: Receiver<Option<QuorumHealthState>>,
) {
    // sec min hour day_of_month month day_of_week year
    let schedule = cron::Schedule::from_str("0 30 4 * * * *").unwrap();

    loop {
        sleep_schedule_next(&schedule).await;

        // will return None in a non-HA deployment
        if let Some(is_ha_leader) = is_ha_leader(&rx_health) {
            if !is_ha_leader {
                debug!("Running HA mode without being the leader - skipping password_expiry_checker scheduler");
                continue;
            }
        }

        debug!("Running password_expiry_checker scheduler");

        // warns, if the duration until the expiry is between 9 and 10 days, to only warn once
        let now = OffsetDateTime::now_utc();
        let lower = now.add(::time::Duration::days(9)).unix_timestamp();
        let upper = now.add(::time::Duration::days(10)).unix_timestamp();

        match User::find_all(&data).await {
            Ok(users) => {
                let users_to_notify = users
                    .into_iter()
                    .filter(|user| {
                        if let Some(exp) = user.password_expires {
                            exp <= upper && exp > lower
                        } else {
                            false
                        }
                    })
                    .collect::<Vec<User>>();

                for user in users_to_notify {
                    send_pwd_reset_info(&data, &user).await;
                    debug!("User {} notified about password expiry", user.email);
                }
            }

            Err(err) => {
                error!("password_expiry_checker error: {}", err.message);
            }
        };
    }
}

// Checks for expired users
pub async fn user_expiry_checker(
    data: web::Data<AppState>,
    rx_health: Receiver<Option<QuorumHealthState>>,
) {
    let secs = env::var("SCHED_USER_EXP_MINS")
        .unwrap_or_else(|_| "60".to_string())
        .parse::<u64>()
        .expect("Cannot parse 'SCHED_USER_EXP_MINS' to u64");
    let mut interval = time::interval(Duration::from_secs(secs * 60));
    let cleanup_after_secs = env::var("SCHED_USER_EXP_DELETE_MINS")
        .map(|s| {
            s.parse::<u64>()
                .expect("Cannot parse 'SCHED_USER_EXP_DELETE_MINS' to u64")
                * 60
        })
        .ok();
    if cleanup_after_secs.is_none() {
        info!("Auto cleanup for expired users disabled");
    }

    loop {
        interval.tick().await;

        // will return None in a non-HA deployment
        if let Some(is_ha_leader) = is_ha_leader(&rx_health) {
            if !is_ha_leader {
                debug!("Running HA mode without being the leader - skipping user_expiry_checker scheduler");
                continue;
            }
        }

        debug!("Running user_expiry_checker scheduler");

        match User::find_expired(&data).await {
            Ok(users) => {
                let now = OffsetDateTime::now_utc().unix_timestamp();
                // could possibly be optimized (if necessary) by collecting all IDs and use a
                // non-prepared statement
                for user in users {
                    debug!("Found expired user {}: {}", user.id, user.email);

                    let exp_ts = if let Some(ts) = user.user_expires {
                        if now < ts {
                            error!("Got not yet expired user in user_expiry_checker - this should never happen");
                            continue;
                        }
                        ts
                    } else {
                        error!("Got non-expiring user in user_expiry_checker - this should never happen");
                        continue;
                    };

                    // invalidate all sessions
                    if let Err(err) = Session::invalidate_for_user(&data, &user.id).await {
                        error!(
                            "Error invalidating sessions for user {}: {:?}",
                            user.id, err
                        );
                    }

                    // invalidate all refresh tokens
                    if let Err(err) = RefreshToken::invalidate_for_user(&data, &user.id).await {
                        error!(
                            "Error invalidating refresh tokens for user {}: {:?}",
                            user.id, err
                        );
                    }

                    // possibly auto-cleanup expired user
                    if let Some(secs) = cleanup_after_secs {
                        let expired_since_secs = (exp_ts - now).unsigned_abs();
                        if expired_since_secs > secs {
                            info!(
                                "Auto cleanup for user {} after being expired for {} minutes",
                                user.id,
                                expired_since_secs / 60
                            );
                            if let Err(err) = user.delete(&data).await {
                                error!(
                                    "Error during auto cleanup - deleting user {}: {:?}",
                                    user.id, err
                                );
                            }
                        }
                    }
                }
            }

            Err(err) => {
                error!("user_expiry_checker error: {}", err.message);
            }
        };
    }
}

// Cleans up old / expired / already used Refresh Tokens
pub async fn refresh_tokens_cleanup(db: DbPool, rx_health: Receiver<Option<QuorumHealthState>>) {
    let mut interval = time::interval(Duration::from_secs(3600 * 3));

    loop {
        interval.tick().await;

        // will return None in a non-HA deployment
        if let Some(is_ha_leader) = is_ha_leader(&rx_health) {
            if !is_ha_leader {
                debug!("Running HA mode without being the leader - skipping refresh_tokens_cleanup scheduler");
                continue;
            }
        }

        debug!("Running refresh_tokens_cleanup scheduler");

        let now = OffsetDateTime::now_utc().unix_timestamp();
        let res = sqlx::query("delete from refresh_tokens where exp < $1")
            .bind(now)
            .execute(&db)
            .await;

        match res {
            Ok(_) => {}
            Err(err) => error!("Refresh Token Cleanup Error: {:?}", err),
        }
    }
}

// Cleans up old / expired Sessions
pub async fn sessions_cleanup(db: DbPool, rx_health: Receiver<Option<QuorumHealthState>>) {
    let mut interval = time::interval(Duration::from_secs(3595 * 2));

    loop {
        interval.tick().await;

        // will return None in a non-HA deployment
        if let Some(is_ha_leader) = is_ha_leader(&rx_health) {
            if !is_ha_leader {
                debug!("Running HA mode without being the leader - skipping sessions_cleanup scheduler");
                continue;
            }
        }

        debug!("Running sessions_cleanup scheduler");

        let thres = OffsetDateTime::now_utc()
            .sub(::time::Duration::hours(24))
            .unix_timestamp();

        let res = sqlx::query("delete from sessions where exp < $1")
            .bind(thres)
            .execute(&db)
            .await;

        match res {
            Ok(_) => {}
            Err(err) => error!("Session Cleanup Error: {:?}", err),
        }
    }
}

// Cleans up old / expired JWKSs
pub async fn jwks_cleanup(
    data: web::Data<AppState>,
    rx_health: Receiver<Option<QuorumHealthState>>,
) {
    let mut interval = time::interval(Duration::from_secs(3600 * 24));

    loop {
        interval.tick().await;

        // will return None in a non-HA deployment
        if let Some(is_ha_leader) = is_ha_leader(&rx_health) {
            if !is_ha_leader {
                debug!(
                    "Running HA mode without being the leader - skipping jwks_cleanup scheduler"
                );
                continue;
            }
        }

        debug!("Running jwks_cleanup scheduler");

        let cleanup_threshold = OffsetDateTime::now_utc()
            .sub(::time::Duration::seconds(*OFFLINE_TOKEN_LT))
            .unix_timestamp();

        // find all existing jwks
        let res = sqlx::query_as::<_, Jwk>("select * from jwks order by created_at asc")
            .fetch_all(&data.db)
            .await;

        let jwks_all = match res {
            Ok(jwks) => jwks,
            Err(err) => {
                error!(
                    "Error while running the jwks_cleanup - cannot access the DATABASE_URL: {}",
                    err
                );
                continue;
            }
        };

        // TODO after rdbms migration has been done, a nice query can do this more easily

        // At this point, the latest / current one will always be the first for each key type.
        // Skip it and check the created timestamps for all following to be older than the max time
        // they could be needed (offline tokens) and delete them.
        let mut found = HashSet::with_capacity(4);
        let mut to_delete: HashSet<String> = HashSet::default();
        for jwk in jwks_all {
            let signature = jwk.signature.to_string();
            if found.contains(&signature) {
                // We already found the first JWK for the current key type -> check created timestamp
                if jwk.created_at < cleanup_threshold {
                    to_delete.insert(jwk.kid);
                }
            } else {
                found.insert(signature);
            }
        }

        // finally, delete all expired JWKs
        let count = to_delete.len();
        for kid in to_delete {
            if let Err(err) = sqlx::query("delete from jwks where kid = $1")
                .bind(&kid)
                .execute(&data.db)
                .await
            {
                error!("Cannot clean up JWK {} in jwks_cleanup: {}", kid, err);
                continue;
            }

            let idx = format!("{}{}", IDX_JWK_KID, kid);
            let _ = cache_del(
                CACHE_NAME_12HR.to_string(),
                idx,
                &data.caches.ha_cache_config,
            )
            .await;
        }
        info!("Cleaned up old JWKs: {}", count);
    }
}

// sleeps until the next scheduled event
async fn sleep_schedule_next(schedule: &cron::Schedule) {
    // this 10 sec sleep is done to prevent an overlap with the calculation in some cases
    time::sleep(Duration::from_secs(10)).await;

    let next = schedule.upcoming(chrono::Local).next().unwrap();
    let until = next.signed_duration_since(chrono::Local::now());

    // we are adding a future date here --> safe to cast from i64 to u64
    time::sleep(Duration::from_secs(until.num_seconds() as u64)).await;
}

fn is_ha_leader(rx_health: &Receiver<Option<QuorumHealthState>>) -> Option<bool> {
    let health_state = rx_health.borrow();
    health_state
        .as_ref()
        .map(|state| state.state == QuorumState::Leader)
}
