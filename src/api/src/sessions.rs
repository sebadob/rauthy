use crate::ReqPrincipal;
use actix_web::web::Query;
use actix_web::{HttpResponse, delete, get, web};
use rauthy_api_types::generic::PaginationParams;
use rauthy_api_types::sessions::SessionResponse;
use rauthy_data::entity;
use rauthy_data::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_data::entity::continuation_token::ContinuationToken;
use rauthy_data::entity::issued_tokens::IssuedToken;
use rauthy_data::entity::refresh_tokens::RefreshToken;
use rauthy_data::entity::sessions::Session;
use rauthy_data::entity::users::User;
use rauthy_data::events::event::Event;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::ErrorResponse;
use rauthy_service::oidc::logout;
use std::cmp::max;
use tokio::task;
use tracing::error;
use validator::Validate;

/// Returns all existing sessions
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/sessions",
    tag = "sessions",
    params(PaginationParams),
    responses(
        (status = 200, description = "Ok", body = [SessionResponse]),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[get("/sessions")]
pub async fn get_sessions(
    principal: ReqPrincipal,
    Query(params): Query<PaginationParams>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Sessions, AccessRights::Read)?;
    params.validate()?;

    let state = entity::sessions::SessionState::from(
        params
            .session_state
            .unwrap_or(rauthy_api_types::sessions::SessionState::Auth),
    );

    // sessions will be dynamically paginated based on the same setting as users
    let user_count = User::count().await?;
    let ssp_threshold = RauthyConfig::get().vars.server.ssp_threshold;
    if user_count >= ssp_threshold as i64 {
        let page_size = max(params.page_size.unwrap_or(20), ssp_threshold) as i64;
        let offset = params.offset.unwrap_or(0) as i64;
        let backwards = params.backwards.unwrap_or(false);
        let continuation_token = if let Some(token) = &params.continuation_token {
            Some(ContinuationToken::try_from(token.as_str())?)
        } else {
            None
        };

        let (users, continuation_token) =
            Session::find_paginated(continuation_token, page_size, offset, backwards, state)
                .await?;
        let x_page_count = (user_count as f64 / page_size as f64).ceil() as u32;

        if let Some(token) = continuation_token {
            Ok(HttpResponse::PartialContent()
                // .insert_header(("x-user-count", user_count))
                .insert_header(("x-page-count", x_page_count))
                .insert_header(("x-page-size", page_size as u32))
                .insert_header(token.into_header_pair())
                .json(users))
        } else {
            Ok(HttpResponse::PartialContent()
                // .insert_header(("x-user-count", user_count))
                .insert_header(("x-page-count", x_page_count))
                .insert_header(("x-page-size", page_size as u32))
                .json(users))
        }
    } else {
        let sessions = Session::find_all(state).await?;

        let mut resp = Vec::with_capacity(sessions.len());
        for s in &sessions {
            resp.push(SessionResponse {
                id: &s.id,
                user_id: s.user_id.as_deref(),
                is_mfa: s.is_mfa,
                state: rauthy_api_types::sessions::SessionState::from(
                    s.state()
                        .unwrap_or(rauthy_data::entity::sessions::SessionState::Unknown),
                ),
                exp: s.exp,
                last_seen: s.last_seen,
                remote_ip: s.remote_ip.as_deref(),
            })
        }

        Ok(HttpResponse::Ok().json(resp))
    }
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
pub async fn delete_sessions(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Sessions, AccessRights::Delete)?;

    Session::invalidate_all().await?;
    RefreshToken::invalidate_all().await?;
    IssuedToken::revoke_all().await?;

    // This task should run async in the background, as it could take quite a long time to finish.
    task::spawn(async move {
        if let Err(err) = logout::execute_backchannel_logout_for_everything().await {
            // TODO we should throw an error or new event in this case maybe, because
            //  invalidations for everything usually come with a good reason.
            error!(
                "Error during backchannel logout for the whole application: {:?}",
                err
            );
        }
    });

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
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Sessions, AccessRights::Delete)?;

    let uid = path.into_inner();
    let user = User::find(uid).await?;

    Session::invalidate_for_user(&user.id).await?;
    RefreshToken::invalidate_for_user(&user.id).await?;
    IssuedToken::revoke_for_user(&user.id).await?;
    logout::execute_backchannel_logout(None, Some(user.id)).await?;

    Event::force_logout(user.email).send().await?;

    Ok(HttpResponse::Ok().finish())
}

/// Delete a single session by ID
///
///**Important:** Since JWT Tokens are stateless, it cannot invalidate already existing tokens.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/sessions/id/{session_id}",
    tag = "sessions",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized"),
        (status = 403, description = "Forbidden"),
    ),
)]
#[delete("/sessions/id/{session_id}")]
pub async fn delete_session_by_id(
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Sessions, AccessRights::Delete)?;

    let sid = path.into_inner();
    let session = Session::find(sid.clone()).await?;
    let uid = session.user_id.clone();

    session.delete().await?;
    RefreshToken::delete_by_sid(sid.clone()).await?;
    IssuedToken::revoke_for_session(&sid).await?;
    logout::execute_backchannel_logout(Some(sid), uid).await?;

    Ok(HttpResponse::Ok().finish())
}
