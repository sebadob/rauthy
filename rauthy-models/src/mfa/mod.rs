use crate::app_state::AppState;
use crate::response::MfaLoginRequest;
use actix_web::web;
use rauthy_common::constants::{CACHE_NAME_MFA_APP_REQ, IDX_MFA_LOGIN_REQ};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use redhac::{cache_get, cache_get_from, cache_get_value, cache_put};

/// Handles the WebSocket Connections to the Authenticator App
pub mod app_reg_ws;
/// Handles the WebSocket Connections for all registered and listening Authenticator Apps
pub mod listen_ws;

pub async fn put_mfa_login_req(
    data: &web::Data<AppState>,
    req: &MfaLoginRequest,
) -> Result<(), ErrorResponse> {
    let idx = format!("{}{}", IDX_MFA_LOGIN_REQ, req.mfa_app_id);
    cache_put(
        CACHE_NAME_MFA_APP_REQ.to_string(),
        idx,
        &data.caches.ha_cache_config,
        &req,
    )
    .await?;
    Ok(())
}

pub async fn get_mfa_login_req(
    data: &web::Data<AppState>,
    mfa_app_id: &str,
) -> Result<MfaLoginRequest, ErrorResponse> {
    let idx = format!("{}{}", IDX_MFA_LOGIN_REQ, mfa_app_id);
    match cache_get!(
        MfaLoginRequest,
        CACHE_NAME_MFA_APP_REQ.to_string(),
        idx,
        &data.caches.ha_cache_config,
        true
    )
    .await?
    {
        Some(res) => Ok(res),
        None => Err(ErrorResponse::new(
            ErrorResponseType::NotFound,
            String::from("MFA request not found"),
        )),
    }
}
