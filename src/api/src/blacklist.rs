use crate::ReqPrincipal;
use actix_web::web::Json;
use actix_web::{delete, get, post, web, HttpResponse};
use chrono::DateTime;
use rauthy_api_types::blacklist::{BlacklistResponse, BlacklistedIp, IpBlacklistRequest};
use rauthy_error::ErrorResponse;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_models::events::event::Event;
use rauthy_models::events::ip_blacklist_handler::IpBlacklistReq;
use tokio::sync::oneshot;
use validator::Validate;

/// Returns all blacklisted IP's
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/blacklist",
    tag = "blacklist",
    responses(
        (status = 200, description = "Ok", body = BlacklistResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/blacklist")]
pub async fn get_blacklist(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Blacklist, AccessRights::Read)?;

    let (tx, rx) = oneshot::channel();
    data.tx_ip_blacklist
        .send_async(IpBlacklistReq::GetBlacklistedIps(tx))
        .await
        .unwrap();

    let ips = rx
        .await
        .unwrap()
        .into_iter()
        .map(|(ip, exp)| BlacklistedIp {
            ip,
            exp: exp.timestamp(),
        })
        .collect();

    Ok(HttpResponse::Ok().json(BlacklistResponse { ips }))
}

/// Manually blacklist an IP
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/blacklist",
    tag = "blacklist",
    request_body = IpBlacklistRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/blacklist")]
pub async fn post_blacklist(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
    Json(payload): Json<IpBlacklistRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Blacklist, AccessRights::Create)?;
    payload.validate()?;

    data.tx_events
        .send_async(Event::ip_blacklisted(
            DateTime::from_timestamp(payload.exp, 0).unwrap_or_default(),
            payload.ip.to_string(),
        ))
        .await
        .unwrap();

    Ok(HttpResponse::Ok().finish())
}

/// Manually delete a blacklisted IP
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/blacklist/{ip}",
    tag = "blacklist",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[delete("/blacklist/{ip}")]
pub async fn delete_blacklist(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
    ip: web::Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Blacklist, AccessRights::Delete)?;

    data.tx_events
        .send_async(Event::ip_blacklist_removed(ip.into_inner()))
        .await
        .unwrap();

    Ok(HttpResponse::Ok().finish())
}
