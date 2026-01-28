use crate::ReqPrincipal;
use actix_web::http::header;
use actix_web::http::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_METHODS, HeaderValue, LOCATION, ORIGIN,
};
use actix_web::web::{Json, Path};
use actix_web::{HttpRequest, HttpResponse, get, post, web};
use chrono::Utc;
use rauthy_api_types::tos::{
    ToSAcceptRequest, ToSLatestResponse, ToSRequest, ToSResponse, ToSUserAcceptResponse,
};
use rauthy_common::constants::HEADER_HTML;
use rauthy_common::markdown::render_sanitized_markdown;
use rauthy_common::sanitize_html::sanitize_html;
use rauthy_common::utils::real_ip_from_req;
use rauthy_data::entity::auth_codes::{AuthCode, AuthCodeToSAwait};
use rauthy_data::entity::theme::ThemeCssFull;
use rauthy_data::entity::tos::ToS;
use rauthy_data::entity::tos_user_accept::ToSUserAccept;
use rauthy_data::entity::users::User;
use rauthy_data::ipgeo::get_location;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use tracing::warn;

/// Returns all ToS
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/tos",
    tag = "tos",
    responses(
        (status = 200, description = "Ok", body = [ToSResponse]),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/tos")]
pub async fn get_tos(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let resp = ToS::find_all()
        .await?
        .into_iter()
        .map(ToSResponse::from)
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(resp))
}

/// Create and publish new ToS
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/tos",
    tag = "tos",
    responses(
        (status = 201, description = "Created"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/tos")]
pub async fn post_tos(
    principal: ReqPrincipal,
    payload: Json<ToSRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let payload = payload.into_inner();
    let user = User::find(principal.user_id()?.to_string()).await?;

    let content = if payload.is_html {
        sanitize_html(&payload.content)
    } else {
        // Note: No need to skip sanitization in this case. The UI will only insert HTML
        // data if this flag is explicitly set.
        payload.content
    };

    // Note: We do not use an FK here on purpose. We still want to be able to know the email even
    // if this user gets deleted at some point in the future.
    ToS::create(user.email, payload.is_html, payload.opt_until, content).await?;

    Ok(HttpResponse::Created().finish())
}

/// Returns the latest ToS
#[utoipa::path(
    get,
    path = "/tos/latest",
    tag = "tos",
    responses(
        (status = 200, description = "Ok", body = ToSLatestResponse),
        (status = 204, description = "NoContent"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/tos/latest")]
pub async fn get_tos_latest(
    accept: web::Header<header::Accept>,
) -> Result<HttpResponse, ErrorResponse> {
    if let Some(tos) = ToS::find_latest().await? {
        warn!("Accept: {:?}", accept);
        let wants_html = accept
            .into_inner()
            .iter()
            .any(|p| p.item.essence_str() == "text/html");

        if wants_html {
            let theme_ts = ThemeCssFull::find_theme_ts_rauthy().await?;
            let div = render_sanitized_markdown(&tos.content);
            let html = format!(
                r#"<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="robots" content="noindex, nofollow" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Rauthy ToS</title>
    <link rel="icon" href="/auth/v1/assets/rauthy_light.svg" />
    <link rel="stylesheet" href="/auth/v1/theme/rauthy/{theme_ts}" />
</head>
<body>{div}</body>
</html>
            "#
            );
            Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(html))
        } else {
            Ok(HttpResponse::Ok().json(ToSLatestResponse::from(tos)))
        }
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}

/// GET user accept status for all existing ToS
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/tos/user/{id}",
    tag = "tos",
    responses(
        (status = 200, description = "Ok", body = [ToSUserAcceptResponse]),
        (status = 204, description = "NoContent"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/tos/user/{id}")]
pub async fn get_tos_user_status(
    principal: ReqPrincipal,
    uid: Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let res = ToSUserAccept::find_all(uid.into_inner())
        .await?
        .into_iter()
        .map(ToSUserAcceptResponse::from)
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(res))
}

/// Accept an updated ToS for existing accounts
///
/// **Permissions**
/// - session in init or auth state
#[utoipa::path(
    post,
    path = "/tos/accept",
    tag = "tos",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/tos/accept")]
pub async fn post_tos_accept(
    principal: ReqPrincipal,
    req: HttpRequest,
    payload: Json<ToSAcceptRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    handle_tos_accept_deny(principal, req, payload.into_inner(), true).await
}

/// Deny update ToS as long as they are within the transition time window. Will error afterward.
///
/// **Permissions**
/// - session in init or auth state
#[utoipa::path(
    post,
    path = "/tos/deny",
    tag = "tos",
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/tos/deny")]
pub async fn post_tos_deny(
    principal: ReqPrincipal,
    req: HttpRequest,
    payload: Json<ToSAcceptRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    handle_tos_accept_deny(principal, req, payload.into_inner(), false).await
}

async fn handle_tos_accept_deny(
    principal: ReqPrincipal,
    req: HttpRequest,
    payload: ToSAcceptRequest,
    is_accept: bool,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth_or_init()?;

    let Some(code_await) = AuthCodeToSAwait::find(&payload.accept_code).await? else {
        return Err(ErrorResponse::new(
            ErrorResponseType::NotFound,
            "Invalid ToS accept code",
        ));
    };

    let Some(mut auth_code) = AuthCode::find(code_await.auth_code.clone()).await? else {
        return Err(ErrorResponse::new(
            ErrorResponseType::NotFound,
            "AuthCode does not exist anymore",
        ));
    };

    let user = User::find(auth_code.user_id.clone()).await?;
    user.check_enabled()?;
    user.check_expired()?;

    let tos = ToS::find(payload.tos_ts).await?;

    if is_accept {
        let ip = real_ip_from_req(&req)?;
        let loc = get_location(&req, ip)?;
        ToSUserAccept::create(user.id, tos.ts, ip, loc).await?;
    } else if let Some(ts) = tos.opt_until {
        if ts <= Utc::now().timestamp() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "ToS transition time is over",
            ));
        }
    } else {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "ToS accept is mandatory",
        ));
    }

    auth_code.reset_exp(code_await.auth_code_lifetime).await?;
    code_await.delete().await?;

    let mut builder = if code_await.needs_user_update {
        let mut builder = HttpResponse::ResetContent();
        builder.insert_header((LOCATION, code_await.header_loc));
        builder
    } else {
        HttpResponse::Accepted()
    };

    if let Some(value) = code_await.header_origin {
        builder.insert_header((ORIGIN, HeaderValue::from_str(&value)?));
        builder.insert_header((
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("POST"),
        ));
        builder.insert_header((
            ACCESS_CONTROL_ALLOW_CREDENTIALS,
            HeaderValue::from_static("true"),
        ));
    }

    Ok(builder.finish())
}
