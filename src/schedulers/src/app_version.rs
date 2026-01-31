use rauthy_common::constants::RAUTHY_VERSION;
use rauthy_data::database::DB;
use rauthy_data::entity::app_version::LatestAppVersion;
use rauthy_data::events::event::Event;
use rauthy_data::rauthy_config::RauthyConfig;
use semver::Version;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info, warn};

/// Checks for newly available Rauthy app versions
pub async fn app_version_check() {
    if RauthyConfig::get().vars.events.disable_app_version_check {
        warn!("The automatic Rauthy version checker is disabled");
        return;
    }

    let mut last_version_notification = None;

    tokio::time::sleep(Duration::from_secs(10)).await;

    let mut interval = tokio::time::interval(Duration::from_secs(3595 * 8));
    loop {
        interval.tick().await;
        check_app_version(&mut last_version_notification).await;

        // For some reason, the interval could `.tick()` multiple times,
        // if it finished too quickly.
        time::sleep(Duration::from_secs(3)).await;
    }
}

async fn check_app_version(last_version_notification: &mut Option<Version>) {
    if !DB::hql().is_leader_cache().await {
        debug!("Running HA mode without being the leader - skipping app_version_check scheduler");
        return;
    }

    debug!("Running app_version_check scheduler");

    match LatestAppVersion::lookup().await {
        Ok((latest_version, url)) => {
            if let Err(err) = LatestAppVersion::upsert(latest_version.clone(), url.clone()).await {
                error!(?err, "Inserting LatestAppVersion into database");
            }

            let this_version = Version::parse(RAUTHY_VERSION).unwrap();

            if latest_version > this_version && latest_version.pre.is_empty() {
                if last_version_notification.as_ref() == Some(&latest_version) {
                    debug!("Notified about version {latest_version} already - skipping it");
                } else {
                    info!("A new Rauthy App Version is available: {latest_version}");

                    RauthyConfig::get()
                        .tx_events
                        .send_async(Event::new_rauthy_version(url))
                        .await
                        .unwrap();

                    *last_version_notification = Some(latest_version);
                }
            } else {
                debug!("No new Rauthy App Version available");
            }
        }

        Err(err) => {
            error!(?err, "LatestAppVersion::lookup()");
        }
    };
}
