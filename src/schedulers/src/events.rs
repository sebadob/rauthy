use crate::is_ha_leader;
use chrono::Utc;
use rauthy_models::app_state::DbPool;
use redhac::QuorumHealthState;
use std::env;
use std::ops::Sub;
use std::time::Duration;
use tokio::sync::watch::Receiver;
use tracing::{debug, error};

/// Cleans up all Events that exceed the configured EVENT_CLEANUP_DAYS
pub async fn events_cleanup(db: DbPool, rx_health: Receiver<Option<QuorumHealthState>>) {
    let mut interval = tokio::time::interval(Duration::from_secs(3600));

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
