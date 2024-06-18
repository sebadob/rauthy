use crate::is_ha_leader;
use rauthy_models::app_state::DbPool;
use redhac::QuorumHealthState;
use std::time::Duration;
use time::OffsetDateTime;
use tokio::sync::watch::Receiver;
use tracing::{debug, error};

// Cleans up old / expired / already used Refresh Tokens
pub async fn refresh_tokens_cleanup(db: DbPool, rx_health: Receiver<Option<QuorumHealthState>>) {
    let mut interval = tokio::time::interval(Duration::from_secs(3600 * 3));

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
