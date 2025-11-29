use rauthy_common::utils::get_rand_between;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::failed_backchannel_logout::FailedBackchannelLogout;
use rauthy_data::entity::jwk::{JwkKeyPair, JwkKeyPairAlg};
use rauthy_data::events::event::Event;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::ErrorResponse;
use rauthy_service::oidc::logout;
use std::str::FromStr;
use std::time::Duration;
use tokio::task::JoinSet;
use tokio::time;
use tracing::{debug, error, info, warn};

pub async fn backchannel_logout_retry() {
    let retry_count = RauthyConfig::get().vars.backchannel_logout.retry_count;
    let mut clients: Vec<Client> = Vec::with_capacity(1);
    let mut kps: Vec<JwkKeyPair> = Vec::with_capacity(1);

    loop {
        // We want to randomize the sleep because this scheduler should run on all cluster members.
        // This increases the chance opf success in case of a network segmentation.
        let millis = get_rand_between(60_000, 90_000);
        time::sleep(Duration::from_millis(millis)).await;
        debug!("Running scheduler backchannel_logout_retry");

        clients.clear();
        kps.clear();
        if let Err(err) = execute_logout_retries(&mut clients, &mut kps, retry_count).await {
            error!("Error during backchannel_logout_retry: {}", err.message);
        }
    }
}

async fn execute_logout_retries(
    clients: &mut Vec<Client>,
    kps: &mut Vec<JwkKeyPair>,
    retry_count: u16,
) -> Result<(), ErrorResponse> {
    let failures = FailedBackchannelLogout::find_all().await?;
    if failures.is_empty() {
        return Ok(());
    }

    let mut tasks = JoinSet::new();

    for failure in failures {
        if failure.retry_count >= retry_count as i32 {
            warn!("Retry count exceeded for backchannel logout {failure:?}");

            Event::backchannel_logout_failed(
                &failure.client_id,
                &failure.sub,
                failure.retry_count as i64,
            )
            .send()
            .await?;

            failure.delete().await?;
            continue;
        }

        let sub = if failure.sub.is_empty() {
            None
        } else {
            Some(failure.sub.clone())
        };
        let sid = if failure.sid.is_empty() {
            None
        } else {
            Some(failure.sid.clone())
        };

        let client = match clients.iter().find(|c| c.id == failure.client_id) {
            None => {
                let c = Client::find(failure.client_id.clone()).await?;
                clients.push(c.clone());
                c
            }
            Some(c) => c.clone(),
        };
        if client.backchannel_logout_uri.is_none() {
            info!(
                "Backchannel Logout URI for failed logout has been removed in the meantime - \
                deleting the failure"
            );
            failure.delete().await?;
            continue;
        }

        let mut kp = kps.iter().find(|kp| kp.typ.as_str() == client.id_token_alg);
        if kp.is_none() {
            let alg = JwkKeyPairAlg::from_str(client.id_token_alg.as_str())?;
            kps.push(JwkKeyPair::find_latest(alg).await?);
            kp = kps.last();
        }
        debug_assert!(kp.is_some());

        match logout::send_backchannel_logout(
            client.id.clone(),
            client.backchannel_logout_uri.unwrap_or_default(),
            sub,
            sid,
            kp.unwrap(),
            &mut tasks,
        )
        .await
        {
            Ok(_) => {
                info!(
                    "Success retrying backchannel logout for {}",
                    failure.client_id
                );
                failure.delete().await?;
            }
            Err(err) => {
                error!(?err, "executing Backchannel Logout");
            }
        }
    }

    Ok(())
}
