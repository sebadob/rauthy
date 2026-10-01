use chrono::{DateTime, Utc};
use rauthy_data::database::DB;
use rauthy_data::fido_mds::authenticator::MdsAuthenticator;
use rauthy_data::fido_mds::dataset::MdsDataset;
use rauthy_data::fido_mds::metadata::MdsMetadata;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::tls;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info};

pub async fn fido_mds_updater() {
    {
        let config = &RauthyConfig::get().vars.webauthn;
        if !config.optimistic_attestation
            && config.force_passkey_cert_level.is_none()
            && config.force_passkey_protection.is_none()
            && config.force_passkey_attachment.is_none()
        {
            info!("No FIDO device attestation configured. Exiting update scheduler.");
            return;
        }
    }
    time::sleep(Duration::from_secs(3)).await;

    loop {
        let meta = MdsMetadata::find().await.unwrap_or_default();
        let now = Utc::now().timestamp();
        let next = DateTime::from_timestamp(meta.next_update, 0).unwrap_or_default();
        if meta.next_update > now {
            info!("Found MDS metadata. Next update available: {next}");
            time::sleep(Duration::from_secs((meta.next_update - now) as u64 + 3600)).await;
        } else {
            info!("Found outdated MDS metadata. Next update was available: {next}");
        }

        if !DB::hql().is_leader_cache().await {
            debug!(
                "Running HA mode without being the leader - skipping fido_mds_updater scheduler"
            );
            continue;
        }
        debug!("Running fido_mds_updater scheduler: {meta:?}");

        if let Err(err) = exec().await {
            error!("Error running fido_mds_updater: {err:?}");
        };

        // Avoid busy loop in error cases. We want to retry without hammering the servers.
        time::sleep(Duration::from_secs(600)).await;
    }
}

async fn exec() -> Result<(), ErrorResponse> {
    let meta = MdsMetadata::find().await?;
    if meta.next_update > Utc::now().timestamp() {
        return Err(ErrorResponse::new(
            ErrorResponseType::Internal,
            "broken next_update logic in fido_mds_updater exec() trigger",
        ));
    }

    // They don't support an ETAG via HEAD upfront, so we need to rely on `next_update` from our
    // saved metadata.
    // TODO make configurable?
    let source = "https://mds.fidoalliance.org/";

    // reqwest follows redirects by default, and the alliance may redirect. Downgrading to plain
    // `http` on a hop would give up exactly the transport protection the caller checked for, so
    // every hop has to stay on `https` too.
    let client = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::custom(|attempt| {
            if attempt.url().scheme() == "https" {
                attempt.follow()
            } else {
                attempt.error("refusing to follow a redirect off https")
            }
        }))
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(10))
        .tls_version_min(tls::Version::TLS_1_2)
        .build()?;

    let mut last_err = String::new();
    for attempt in 1..=3 {
        if attempt > 1 {
            let backoff = Duration::from_secs(5u64.pow(attempt));
            info!(
                "Retrying Fido MDS update in {}s: {last_err}",
                backoff.as_secs()
            );
            time::sleep(backoff).await;
        }

        let res = client.get(source).send().await?;
        let status = res.status();

        // the server has a pretty strong rate-limit
        if status.as_u16() == 429 {
            info!("Fido MDS update server rate-limit hit. Retrying in 1 hour.");
            time::sleep(Duration::from_secs(3660)).await;
            continue;
        }

        let body = res.text().await?;
        if status.is_success() && !body.is_empty() {
            let dataset = body.parse::<MdsDataset>()?;
            dataset.upsert().await?;
            MdsAuthenticator::clear_cache().await?;
            info!(
                "Fido MDS dataset updated successfully. blob_no: {}, next_update: {}",
                dataset.blob_no, dataset.next_update_ts
            );

            return Ok(());
        }

        last_err = format!(
            "the metadata service answered {status} with {:?}",
            body.trim().chars().take(120).collect::<String>()
        );
    }

    Err(ErrorResponse::new(ErrorResponseType::Connection, last_err))
}
