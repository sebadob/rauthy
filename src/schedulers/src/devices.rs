use crate::is_ha_leader;
use chrono::Utc;
use rauthy_models::app_state::DbPool;
use redhac::QuorumHealthState;
use std::ops::Sub;
use std::time::Duration;
use tokio::sync::watch::Receiver;
use tracing::{debug, error};

/// Cleans up fully expired devices. These need to do a full re-authentication anyway.
/// All devices that are expired for at least 1 day will be removed.
pub async fn devices_cleanup(db: DbPool, rx_health: Receiver<Option<QuorumHealthState>>) {
    let mut interval = tokio::time::interval(Duration::from_secs(24 * 3600));

    loop {
        interval.tick().await;

        // will return None in a non-HA deployment
        if let Some(is_ha_leader) = is_ha_leader(&rx_health) {
            if !is_ha_leader {
                debug!(
                    "Running HA mode without being the leader - skipping devices_cleanup scheduler"
                );
                continue;
            }
        }

        debug!("Running devices_cleanup scheduler");

        let threshold = Utc::now().sub(chrono::Duration::days(1)).timestamp();
        let res = sqlx::query!(
            r#"DELETE FROM devices
            WHERE access_exp < $1
            AND (refresh_exp is null OR refresh_exp < $1)"#,
            threshold
        )
        .execute(&db)
        .await;
        match res {
            Ok(r) => {
                debug!("Cleaned up {} expired devices", r.rows_affected());
            }
            Err(err) => error!("devices_cleanup error: {:?}", err),
        }
    }
}
