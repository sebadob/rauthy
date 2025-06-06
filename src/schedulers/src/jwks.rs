use crate::sleep_schedule_next;
use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::constants::IDX_JWK_KID;
use rauthy_common::is_hiqlite;
use rauthy_models::database::{Cache, DB};
use rauthy_models::entity::jwk::{JWKS, Jwk};
use rauthy_models::rauthy_config::RauthyConfig;
use std::collections::HashSet;
use std::ops::Sub;
use std::str::FromStr;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info};

/// Auto-Rotates JWKS
pub async fn jwks_auto_rotate() {
    // sec min hour day_of_month month day_of_week year
    let schedule = {
        cron::Schedule::from_str(
            RauthyConfig::get()
                .vars
                .lifetimes
                .jwk_autorotate_cron
                .as_ref(),
        )
        .unwrap()
    };

    loop {
        sleep_schedule_next(&schedule).await;

        if !DB::hql().is_leader_cache().await {
            debug!("Running HA mode without being the leader - skipping jwks_cleanup scheduler");
            continue;
        }

        if let Err(err) = JWKS::rotate().await {
            error!("Error during JWKS auto-rotation: {}", err.message);
        }
    }
}

/// Cleans up old / expired JWKSs
pub async fn jwks_cleanup() {
    let mut interval = tokio::time::interval(Duration::from_secs(3600 * 24));

    loop {
        interval.tick().await;

        if !DB::hql().is_leader_cache().await {
            debug!("Running HA mode without being the leader - skipping jwks_cleanup scheduler");
            continue;
        }

        debug!("Running jwks_cleanup scheduler");

        // clean up all JWKs older than 90 days
        let cleanup_threshold = Utc::now().sub(chrono::Duration::days(90)).timestamp();

        let sql = "SELECT * FROM jwks ORDER BY created_at asc";
        let res: Result<Vec<Jwk>, String> = if is_hiqlite() {
            DB::hql()
                .query_as(sql, params!())
                .await
                .map_err(|err| err.to_string())
        } else {
            DB::pg_query(sql, &[], 8)
                .await
                .map_err(|err| err.message.to_string())
        };

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
        let sql = "DELETE FROM jwks WHERE kid = $1";
        for kid in to_delete {
            if is_hiqlite() {
                if let Err(err) = DB::hql().execute(sql, params!()).await {
                    error!("Cannot clean up JWK {} in jwks_cleanup: {}", kid, err);
                    continue;
                }
            } else if let Err(err) = DB::pg_execute(sql, &[]).await {
                error!("Cannot clean up JWK {} in jwks_cleanup: {}", kid, err);
                continue;
            }

            let idx = format!("{}{}", IDX_JWK_KID, kid);
            if let Err(err) = DB::hql().delete(Cache::App, idx).await {
                error!("Error deleting JWK from cache: {}", err);
            }
        }
        info!("Cleaned up old JWKs: {}", count);

        // For some reason, the interval could `.tick()` multiple times,
        // if it finished too quickly.
        time::sleep(Duration::from_secs(3)).await;
    }
}
