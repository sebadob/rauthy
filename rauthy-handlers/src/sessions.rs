use crate::ReqPrincipal;
use actix_web::{delete, get, web, HttpResponse};
use rauthy_common::error_response::ErrorResponse;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_models::entity::refresh_tokens::RefreshToken;
use rauthy_models::entity::sessions::Session;
use rauthy_models::response::SessionResponse;

/// Returns all existing sessions
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/sessions",
    tag = "sessions",
    responses(
        (status = 200, description = "Ok", body = [SessionResponse]),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[get("/sessions")]
pub async fn get_sessions(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Sessions, AccessRights::Read)?;

    // TODO benchmark, which of these 2 implementations is faster in the end
    // let sessions = Session::find_all()
    //     .await?
    //     .drain(..)
    //     .map(SessionResponse::from)
    //     .collect::<Vec<SessionResponse>>();
    let sessions = Session::find_all(&data).await?;
    let resp = sessions
        .iter()
        .map(|s| SessionResponse {
            id: &s.id,
            user_id: s.user_id.as_deref(),
            is_mfa: s.is_mfa,
            state: &s.state,
            exp: s.exp,
            last_seen: s.last_seen,
            remote_ip: s.remote_ip.as_deref(),
        })
        .collect::<Vec<SessionResponse>>();
    Ok(HttpResponse::Ok().json(resp))
}

/// Invalidates all existing sessions and therefore logs out every single user.
///
/// **Important:** Since JWT Tokens are stateless, it cannot invalidate already existing tokens.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/sessions",
    tag = "sessions",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[delete("/sessions")]
pub async fn delete_sessions(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Sessions, AccessRights::Delete)?;

    Session::invalidate_all(&data).await?;
    RefreshToken::invalidate_all(&data).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Invalidates all existing sessions for the given `user_id`.
///
///**Important:** Since JWT Tokens are stateless, it cannot invalidate already existing tokens.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/sessions/{user_id}",
    tag = "sessions",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[delete("/sessions/{user_id}")]
pub async fn delete_sessions_for_user(
    data: web::Data<AppState>,
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Sessions, AccessRights::Delete)?;

    let uid = path.into_inner();
    Session::invalidate_for_user(&data, &uid).await?;
    RefreshToken::invalidate_for_user(&data, &uid).await?;

    Ok(HttpResponse::Ok().finish())
}
