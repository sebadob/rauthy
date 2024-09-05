use actix_web::web;
use rauthy_common::constants::RAUTHY_VERSION;
use rauthy_models::app_state::AppState;
use rauthy_models::cache::DB;
use rauthy_models::entity::app_version::LatestAppVersion;
use rauthy_models::events::event::Event;
use semver::Version;
use std::env;
use std::time::Duration;
use tracing::{debug, error, info, warn};

/// Checks for newly available Rauthy app versions
pub async fn app_version_check(data: web::Data<AppState>) {
    let disable = env::var("DISABLE_APP_VERSION_CHECK")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .expect("Cannot parse DISABLE_APP_VERSION_CHECK to bool");
    if disable {
        warn!("The automatic Rauthy version checker is disabled");
        return;
    }

    let mut last_version_notification = None;

    // do a first check shortly after startup to not wait hours on a fresh install
    // tokio::time::sleep(Duration::from_secs(3)).await;
    tokio::time::sleep(Duration::from_secs(120)).await;
    check_app_version(&data, &mut last_version_notification).await;

    let mut interval = tokio::time::interval(Duration::from_secs(3595 * 8));
    loop {
        interval.tick().await;
        check_app_version(&data, &mut last_version_notification).await;
    }
}

async fn check_app_version(
    data: &web::Data<AppState>,
    last_version_notification: &mut Option<Version>,
) {
    if !DB::client().is_leader_cache().await {
        debug!("Running HA mode without being the leader - skipping app_version_check scheduler");
        return;
    }

    debug!("Running app_version_check scheduler");

    match LatestAppVersion::lookup().await {
        Ok((latest_version, url)) => {
            if let Err(err) =
                LatestAppVersion::upsert(data, latest_version.clone(), url.clone()).await
            {
                error!("Inserting LatestAppVersion into database: {:?}", err);
            }

            let this_version = Version::parse(RAUTHY_VERSION).unwrap();

            if latest_version > this_version && latest_version.pre.is_empty() {
                if last_version_notification.as_ref() == Some(&latest_version) {
                    debug!(
                        "Notified about version {} already - skipping it",
                        latest_version
                    );
                } else {
                    info!("A new Rauthy App Version is available: {}", latest_version);

                    if let Err(err) =
                        LatestAppVersion::upsert(data, latest_version.clone(), url.clone()).await
                    {
                        error!("Saving LatestAppVersion into DB: {:?}", err);
                    }

                    data.tx_events
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
            error!("LatestAppVersion::lookup(): {:?}", err);
        }
    };
}
