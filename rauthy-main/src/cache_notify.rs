use actix_web::web;
use rauthy_common::constants::CACHE_NAME_MFA_APP_REQ;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::mfa_auth_code::MfaAuthCode;
use rauthy_models::mfa::listen_ws::WsListenRouteReq;
use rauthy_models::mfa::{get_mfa_login_req, put_mfa_login_req};
use rauthy_models::response::{MfaAckResponse, MfaLoginRequestState};
use redhac::{CacheMethod, CacheNotify};
use tokio::sync::{mpsc, oneshot};
use tracing::{debug, error};

pub async fn handle_notify(data: web::Data<AppState>, mut rx: mpsc::Receiver<CacheNotify>) {
    while let Some(msg) = rx.recv().await {
        match msg.method {
            CacheMethod::Put => {
                debug!(
                    "Remote push to the cache for '{}/{}'",
                    msg.cache_name, msg.entry
                );

                if msg.cache_name == CACHE_NAME_MFA_APP_REQ {
                    tokio::spawn(handle_mfa_app_req(msg.entry, data.clone()));
                }
            }

            CacheMethod::Del => {
                debug!(
                    "Remote delete from the cache for '{}/{}'",
                    msg.cache_name, msg.entry
                );
            }

            _ => {}
        }
    }
}

async fn handle_mfa_app_req(entry: String, data: web::Data<AppState>) {
    // unwrap is safe - can only be Some, if we get here
    let mut login_req = get_mfa_login_req(&data, &entry).await.unwrap();
    let mut msg = None;

    match login_req.state {
        MfaLoginRequestState::Accepted => {
            let auth_code = MfaAuthCode::find(&data, login_req.email.clone())
                .await
                .expect("'get_mfa_auth_code' in 'handle_mfa_app_req'");
            if auth_code.is_none() {
                error!("'auth_code' is None in 'handle_mfa_app_req' when it should not");
                return;
            }

            msg = Some(format!(
                "message ACK {}",
                serde_json::to_string(&MfaAckResponse {
                    req_id: &login_req.req_id,
                    loc: &auth_code.unwrap().header_loc,
                })
                .unwrap()
            ));
        }

        MfaLoginRequestState::Rejected => {
            msg = Some("message REJECTED".to_string());
        }
        _ => {}
    }

    if msg.is_some() {
        let (tx, rx) = oneshot::channel();
        data.caches
            .tx_ws_listen
            .send(WsListenRouteReq::TrySend {
                app_id: login_req.mfa_app_id.clone(),
                msg: msg.unwrap(),
                tx_ack: tx,
            })
            .await
            .unwrap();

        let success = rx.await.unwrap();
        debug!(
            "Answer in 'handle_mfa_app_req' from 'ws_listen_router': {}",
            success
        );
        if success {
            login_req.state = MfaLoginRequestState::Sent;
            let _ = put_mfa_login_req(&data, &login_req).await;
        }
    }
}
