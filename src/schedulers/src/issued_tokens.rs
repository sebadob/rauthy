use rauthy_common::utils::get_rand_between;
use rauthy_data::database::DB;
use rauthy_data::entity::issued_tokens::IssuedToken;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error};

pub async fn cleanup_issued_tokens() {
    let mut interval = tokio::time::interval(Duration::from_secs(get_rand_between(3500, 3700)));

    loop {
        interval.tick().await;

        if !DB::hql().is_leader_cache().await {
            debug!(
                "Running HA mode without being the leader - skipping cleanup_issued_tokens scheduler"
            );
            continue;
        }

        debug!("Running cleanup_issued_tokens scheduler");

        if let Err(err) = IssuedToken::cleanup_expired().await {
            error!("Error during cleanup_issued_tokens: {:?}", err);
        }

        // For some reason, the interval could `.tick()` multiple times,
        // if it finished too quickly.
        time::sleep(Duration::from_secs(3)).await;
    }
}
