use actix_web::{
    HttpRequest, HttpResponse, get,
    http::header::{self, HeaderValue, LOCATION},
    post,
    web::{self, Json, Query},
};
use rauthy_api_types::{
    atproto,
    auth_providers::{ProviderCallbackRequest, ProviderLookupResponse},
};
use rauthy_common::constants::ATPROTO_ENABLE;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::{
    app_state::AppState,
    entity::{atproto::AtprotoCallback, auth_providers::AuthProviderCallback, theme::ThemeCssFull},
    html::HtmlCached,
};
use validator::Validate as _;

use crate::{ReqPrincipal, map_auth_step};

#[utoipa::path(
    get,
    path = "/atproto/client_metadata",
    tag = "atproto",
    responses(
        (status = 200, description = "OK"),
    ),
)]
#[get("/atproto/client_metadata")]
pub async fn get_client_metadata(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    is_atproto_enabled()?;

    Ok(HttpResponse::Ok()
        .insert_header((
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_str("*").unwrap(),
        ))
        .json(&data.atproto.client_metadata))
}

#[utoipa::path(
    post,
    path = "/atproto/login",
    tag = "atproto",
    responses(
        (status = 202, description = "Accepted"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/atproto/login")]
pub async fn post_login(
    data: web::Data<AppState>,
    payload: Json<atproto::LoginRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    is_atproto_enabled()?;
    principal.validate_session_auth_or_init()?;

    let payload = payload.into_inner();
    let (cookie, xsrf_token, location) =
        <AuthProviderCallback as AtprotoCallback>::login_start(&data, payload).await?;

    Ok(HttpResponse::Accepted()
        .insert_header((LOCATION, location))
        .cookie(cookie)
        .body(xsrf_token))
}

#[get("/atproto/callback")]
pub async fn get_atproto_callback_html(req: HttpRequest) -> Result<HttpResponse, ErrorResponse> {
    HtmlCached::AtprotoCallback
        .handle(req, ThemeCssFull::find_theme_ts_rauthy().await?, true)
        .await
}

/// Callback for an upstream atproto login
///
/// **Permissions**
/// - `session-init`
/// - `session-auth`
#[utoipa::path(
    post,
    path = "/atproto/callback",
    tag = "atproto",
    request_body = atproto::CallbackRequest,
    responses(
        (status = 202, description = "Correct credentials, adds Location header"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/atproto/callback")]
#[tracing::instrument(
    name = "post_atproto_callback",
    skip_all, fields(callback_id = payload.state)
)]
pub async fn post_atproto_callback(
    data: web::Data<AppState>,
    req: HttpRequest,
    Json(payload): Json<atproto::CallbackRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    post_atproto_callback_handle(data, req, payload, principal).await
}

// extracted to make it usable in `post_dev_only_endpoints()`
#[inline(always)]
pub async fn post_atproto_callback_handle(
    data: web::Data<AppState>,
    req: HttpRequest,
    payload: atproto::CallbackRequest,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    is_atproto_enabled()?;
    principal.validate_session_auth_or_init()?;
    payload.validate()?;

    let session = principal.get_session()?;
    let (auth_step, cookie) =
        AtprotoCallback::login_finish(&data, &req, &payload, session.clone()).await?;

    let mut resp = map_auth_step(auth_step, &req).await?;
    resp.add_cookie(&cookie).map_err(|err| {
        ErrorResponse::new(
            ErrorResponseType::Internal,
            format!("Error adding cookie after map_auth_step: {}", err),
        )
    })?;

    Ok(resp)
}

#[inline(always)]
fn is_atproto_enabled() -> Result<(), ErrorResponse> {
    if *ATPROTO_ENABLE {
        Ok(())
    } else {
        Err(ErrorResponse::new(
            ErrorResponseType::Internal,
            "The atproto client is disabled on this instance",
        ))
    }
}
