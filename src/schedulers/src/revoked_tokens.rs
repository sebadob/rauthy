use rauthy_data::database::DB;
use rauthy_data::entity::revoked_tokens::RevokedToken;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error};

pub async fn cleanup_revoked_tokens() {
    let mut interval = tokio::time::interval(Duration::from_secs(3 * 3600));

    loop {
        interval.tick().await;

        if !DB::hql().is_leader_cache().await {
            debug!(
                "Running HA mode without being the leader - skipping cleanup_revoked_tokens scheduler"
            );
            continue;
        }

        debug!("Running cleanup_revoked_tokens scheduler");

        if let Err(err) = RevokedToken::cleanup_expired().await {
            error!("Error during cleanup_revoked_tokens: {:?}", err);
        }

        // For some reason, the interval could `.tick()` multiple times,
        // if it finished too quickly.
        time::sleep(Duration::from_secs(3)).await;
    }
}
