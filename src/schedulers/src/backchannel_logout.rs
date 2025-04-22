use actix_web::web;
use rauthy_common::utils::get_rand_between;
use rauthy_error::ErrorResponse;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::failed_backchannel_logout::FailedBackchannelLogout;
use rauthy_models::entity::jwk::{JwkKeyPair, JwkKeyPairAlg};
use rauthy_models::events::event::Event;
use rauthy_service::oidc::logout;
use std::env;
use std::str::FromStr;
use std::time::Duration;
use tokio::task::JoinSet;
use tokio::time;
use tracing::{error, info, warn};

pub async fn backchannel_logout_retry(data: web::Data<AppState>) {
    let retry_count = env::var("BACKCHANNEL_LOGOUT_RETRY_COUNT")
        .as_deref()
        .unwrap_or("100")
        .parse::<u16>()
        .expect("Cannot parse BACKCHANNEL_LOGOUT_RETRY_COUNT as u16");

    loop {
        // We want to randomize the sleep because this scheduler should run on all cluster members.
        // This increases the chance opf success in case of a network segmentation.
        let millis = get_rand_between(60_000, 90_000);
        time::sleep(Duration::from_millis(millis)).await;

        if let Err(err) = execute_logout_retries(&data, retry_count).await {
            error!("Error during backchannel_logout_retry: {}", err.message);
        }
    }
}

async fn execute_logout_retries(
    data: &web::Data<AppState>,
    retry_count: u16,
) -> Result<(), ErrorResponse> {
    let failures = FailedBackchannelLogout::find_all().await?;
    if failures.is_empty() {
        return Ok(());
    }

    let mut clients: Vec<Client> = Vec::with_capacity(1);
    let mut kps: Vec<JwkKeyPair> = Vec::with_capacity(1);
    let mut tasks = JoinSet::new();

    for failure in failures {
        if failure.retry_count >= retry_count as i32 {
            warn!("Retry count exceeded for backchannel logout {:?}", failure);

            Event::backchannel_logout_failed(
                &failure.client_id,
                &failure.sub,
                failure.retry_count as i64,
            )
            .send(&data.tx_events)
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
                "Backchannel Logout URI for failed logout has been removed in the meantime - deleting the failure"
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
            data.issuer.clone(),
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
                error!("Error executing Backchannel Logout: {}", err);
            }
        }
    }

    Ok(())
}
