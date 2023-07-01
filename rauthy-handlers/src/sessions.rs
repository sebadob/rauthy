use actix_web::{delete, get, web, HttpRequest, HttpResponse};
use actix_web_grants::proc_macro::has_roles;
use rauthy_common::error_response::ErrorResponse;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::principal::Principal;
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
#[has_roles("rauthy_admin")]
pub async fn get_sessions(
    data: web::Data<AppState>,
    principal: web::ReqData<Option<Principal>>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::get_from_req(principal.into_inner())?;
    principal.validate_rauthy_admin()?;

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
#[has_roles("rauthy_admin")]
pub async fn delete_sessions(
    data: web::Data<AppState>,
    req: HttpRequest,
    principal: web::ReqData<Option<Principal>>,
    session_req: web::ReqData<Option<Session>>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::get_from_req(principal.into_inner())?;
    principal.validate_rauthy_admin()?;
    if session_req.is_some() {
        Session::extract_validate_csrf(session_req, &req)?;
    }

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
#[has_roles("rauthy_admin")]
pub async fn delete_sessions_for_user(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: HttpRequest,
    principal: web::ReqData<Option<Principal>>,
    session_req: web::ReqData<Option<Session>>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::get_from_req(principal.into_inner())?;
    principal.validate_rauthy_admin()?;
    if session_req.is_some() {
        Session::extract_validate_csrf(session_req, &req)?;
    }

    let uid = path.into_inner();
    Session::invalidate_for_user(&data, &uid).await?;
    RefreshToken::invalidate_for_user(&data, &uid).await?;

    Ok(HttpResponse::Ok().finish())
}
