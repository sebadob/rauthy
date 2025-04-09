use chrono::Utc;
use hiqlite::{Param, params};
use rauthy_common::is_hiqlite;
use rauthy_models::database::DB;
use std::ops::Sub;
use std::time::Duration;
use tracing::{debug, error};

/// Cleans up fully expired devices. These need to do a full re-authentication anyway.
/// All devices that are expired for at least 1 day will be removed.
pub async fn devices_cleanup() {
    let mut interval = tokio::time::interval(Duration::from_secs(24 * 3600));

    loop {
        interval.tick().await;

        if !DB::hql().is_leader_cache().await {
            debug!("Running HA mode without being the leader - skipping devices_cleanup scheduler");
            continue;
        }

        debug!("Running devices_cleanup scheduler");

        let threshold = Utc::now().sub(chrono::Duration::days(1)).timestamp();
        let sql = r#"
DELETE FROM devices
WHERE access_exp < $1
AND (refresh_exp IS NULL OR refresh_exp < $1)"#;

        if is_hiqlite() {
            let res = DB::hql().execute(sql, params!(threshold)).await;
            match res {
                Ok(rows_affected) => {
                    debug!("Cleaned up {} expired devices", rows_affected);
                }
                Err(err) => {
                    error!("devices_cleanup error: {:?}", err)
                }
            }
        } else {
            let res = DB::pg_execute(sql, &[&threshold]).await;
            match res {
                Ok(rows_affected) => {
                    debug!("Cleaned up {} expired devices", rows_affected);
                }
                Err(err) => {
                    error!("devices_cleanup error: {:?}", err)
                }
            }
        };
    }
}
