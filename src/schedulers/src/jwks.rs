use crate::sleep_schedule_next;
use actix_web::web;
use rauthy_common::constants::IDX_JWK_KID;
use rauthy_models::app_state::AppState;
use rauthy_models::cache::{Cache, DB};
use rauthy_models::entity::jwk::{Jwk, JWKS};
use std::collections::HashSet;
use std::ops::Sub;
use std::str::FromStr;
use std::time::Duration;
use time::OffsetDateTime;
use tracing::{debug, error, info};

/// Auto-Rotates JWKS
pub async fn jwks_auto_rotate(data: web::Data<AppState>) {
    // sec min hour day_of_month month day_of_week year
    let schedule = cron::Schedule::from_str("0 30 3 1 * * *").unwrap();

    loop {
        sleep_schedule_next(&schedule).await;

        if !DB::client().is_leader_cache().await {
            debug!("Running HA mode without being the leader - skipping jwks_cleanup scheduler");
            continue;
        }

        if let Err(err) = JWKS::rotate(&data).await {
            error!("Error during JWKS auto-rotation: {}", err.message);
        }
    }
}

/// Cleans up old / expired JWKSs
pub async fn jwks_cleanup(data: web::Data<AppState>) {
    let mut interval = tokio::time::interval(Duration::from_secs(3600 * 24));

    loop {
        interval.tick().await;

        if !DB::client().is_leader_cache().await {
            debug!("Running HA mode without being the leader - skipping jwks_cleanup scheduler");
            continue;
        }

        debug!("Running jwks_cleanup scheduler");

        // clean up all JWKs older than 90 days
        let cleanup_threshold = OffsetDateTime::now_utc()
            .sub(::time::Duration::seconds(3600 * 24 * 90))
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
            if let Err(err) = DB::client().delete(Cache::App, idx).await {
                error!("Error deleting JWK from cache: {}", err);
            }
        }
        info!("Cleaned up old JWKs: {}", count);
    }
}
