use crate::is_ha_leader;
use rauthy_models::app_state::DbPool;
use redhac::QuorumHealthState;
use std::ops::Sub;
use std::time::Duration;
use time::OffsetDateTime;
use tokio::sync::watch::Receiver;
use tracing::{debug, error};

// Cleans up old / expired Sessions
pub async fn sessions_cleanup(db: DbPool, rx_health: Receiver<Option<QuorumHealthState>>) {
    let mut interval = tokio::time::interval(Duration::from_secs(3595 * 2));

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
