use crate::ReqPrincipal;
use actix_web::web::Json;
use actix_web::{HttpResponse, delete, get, post, web};
use chrono::Utc;
use rauthy_api_types::blacklist::{BlacklistResponse, BlacklistedIp, IpBlacklistRequest};
use rauthy_error::ErrorResponse;
use rauthy_models::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_models::entity::ip_blacklist::IpBlacklist;
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
pub async fn get_blacklist(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Blacklist, AccessRights::Read)?;

    let ips = IpBlacklist::get_all()
        .await?
        .into_iter()
        .map(|(ip, blacklist)| BlacklistedIp {
            ip,
            exp: blacklist.exp.timestamp(),
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
    principal: ReqPrincipal,
    Json(payload): Json<IpBlacklistRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Blacklist, AccessRights::Create)?;
    payload.validate()?;

    let now = Utc::now().timestamp();
    if payload.exp <= now {
        return Ok(HttpResponse::Ok().finish());
    }

    let ttl = payload.exp - now;
    IpBlacklist::put(payload.ip.to_string(), ttl).await?;

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
    principal: ReqPrincipal,
    ip: web::Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Blacklist, AccessRights::Delete)?;

    IpBlacklist::delete(ip.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}
