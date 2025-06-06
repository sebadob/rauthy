use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_models::database::DB;
use rauthy_models::rauthy_config::RauthyConfig;
use std::ops::Sub;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error};

/// Cleans up all Events that exceed the configured EVENT_CLEANUP_DAYS
pub async fn events_cleanup() {
    let mut interval = tokio::time::interval(Duration::from_secs(3600));

    loop {
        interval.tick().await;

        if !DB::hql().is_leader_cache().await {
            debug!("Running HA mode without being the leader - skipping events_cleanup scheduler");
            continue;
        }

        debug!("Running events_cleanup scheduler");

        let cleanup_days = RauthyConfig::get().vars.events.cleanup_days as i64;
        let threshold = Utc::now()
            .sub(chrono::Duration::days(cleanup_days))
            .timestamp_millis();

        let sql = "DELETE FROM events WHERE timestamp < $1";
        if is_hiqlite() {
            let res = DB::hql().execute(sql, params!(threshold)).await;
            match res {
                Ok(rows_affected) => {
                    debug!("Cleaned up {} expired events", rows_affected);
                }
                Err(err) => error!("Events cleanup error: {:?}", err),
            }
        } else {
            let res = DB::pg_execute(sql, &[&threshold]).await;
            match res {
                Ok(rows_affected) => {
                    debug!("Cleaned up {} expired events", rows_affected);
                }
                Err(err) => error!("Events cleanup error: {:?}", err),
            }
        };

        // For some reason, the interval could `.tick()` multiple times,
        // if it finished too quickly.
        time::sleep(Duration::from_secs(3)).await;
    }
}
