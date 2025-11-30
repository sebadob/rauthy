use crate::sleep_schedule_next;
use rauthy_data::database::DB;
use rauthy_data::entity::pam::authorized_keys::AuthorizedKey;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::ErrorResponse;
use std::str::FromStr;
use tracing::{debug, error};

pub async fn cleanup_authorized_keys() {
    if !RauthyConfig::get()
        .vars
        .pam
        .authorized_keys
        .blacklist_used_keys
    {
        debug!("SSH Key blacklisting disabled - exiting cleanup scheduler");
        return;
    }

    // sec min hour day_of_month month day_of_week year
    let schedule = cron::Schedule::from_str("0 45 4 * * * *").unwrap();

    loop {
        sleep_schedule_next(&schedule).await;

        if !DB::hql().is_leader_cache().await {
            debug!(
                "Running HA mode without being the leader - skipping cleanup_authorized_keys scheduler"
            );
            continue;
        }

        debug!("Running cleanup_authorized_keys scheduler");

        if let Err(err) = exec().await {
            error!("Error during cleanup_authorized_keys: {:?}", err);
        }
    }

    async fn exec() -> Result<(), ErrorResponse> {
        AuthorizedKey::cleanup_blacklist().await?;

        let expired = AuthorizedKey::find_expired(30).await?;
        for key in expired {
            // delete one by one to blacklist them properly
            key.delete().await?;
        }

        Ok(())
    }
}
