use actix_web::web;
use rauthy_models::app_state::AppState;
use rauthy_models::cache::DB;
use rauthy_models::entity::refresh_tokens::RefreshToken;
use rauthy_models::entity::sessions::Session;
use rauthy_models::entity::users::User;
use std::env;
use std::time::Duration;
use time::OffsetDateTime;
use tracing::{debug, error, info};

// Checks for expired users
pub async fn user_expiry_checker(data: web::Data<AppState>) {
    let secs = env::var("SCHED_USER_EXP_MINS")
        .unwrap_or_else(|_| "60".to_string())
        .parse::<u64>()
        .expect("Cannot parse 'SCHED_USER_EXP_MINS' to u64");
    let mut interval = tokio::time::interval(Duration::from_secs(secs * 60));
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

        if !DB::client().is_leader_cache().await {
            debug!(
                "Running HA mode without being the leader - skipping user_expiry_checker scheduler"
            );
            continue;
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
