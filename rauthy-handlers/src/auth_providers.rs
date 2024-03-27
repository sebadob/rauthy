use crate::ReqPrincipal;
use actix_web::http::header::{LOCATION, SET_COOKIE};
use actix_web::http::StatusCode;
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use actix_web_validator::Json;
use rauthy_common::constants::{COOKIE_UPSTREAM_CALLBACK, HEADER_HTML};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::auth_provider::{AuthProvider, AuthProviderCallback};
use rauthy_models::entity::colors::ColorEntity;
use rauthy_models::request::{
    ProviderCallbackRequest, ProviderLoginRequest, ProviderLookupRequest, ProviderRequest,
};
use rauthy_models::response::ProviderResponse;
use rauthy_models::templates::{AdminUsersHtml, ProviderCallbackHtml};
use tracing::error;

/// GET all upstream auth providers
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    get,
    path = "/providers",
    tag = "providers",
    responses(
        (status = 200, description = "OK", body = ProviderResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/providers")]
pub async fn get_providers(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let providers = AuthProvider::find_all(&data).await?;
    let mut resp = Vec::with_capacity(providers.len());
    for provider in providers {
        resp.push(ProviderResponse::try_from(provider)?);
    }

    Ok(HttpResponse::Ok().json(resp))
}

/// POST create a new upstream auth provider
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    post,
    path = "/providers",
    tag = "providers",
    responses(
        (status = 200, description = "OK", body = ProviderResponse),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/providers")]
pub async fn post_provider(
    data: web::Data<AppState>,
    payload: Json<ProviderRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    if !payload.use_pkce && payload.client_secret.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Must at least be a confidential client or use PKCE".to_string(),
        ));
    }

    let provider = AuthProvider::create(&data, payload.into_inner()).await?;
    Ok(HttpResponse::Ok().json(ProviderResponse::try_from(provider)?))
}

/// POST possible upstream auth provider config lookup
///
/// This will try to autoconfigure and build and upstream auth provider by the given issuer URL.
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    post,
    path = "/providers/lookup",
    tag = "providers",
    responses(
        (status = 200, description = "OK", body = ProviderLookupResponse),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/providers/lookup")]
#[tracing::instrument(
    name = "post_provider_lookup",
    skip_all, fields(issuer = payload.issuer)
)]
pub async fn post_provider_lookup(
    // data: web::Data<AppState>,
    payload: Json<ProviderLookupRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let payload = payload.into_inner();
    let resp = AuthProvider::lookup_config(
        &payload.issuer,
        payload.danger_allow_insecure.unwrap_or(false),
        payload.root_pem.as_deref(),
    )
    .await?;

    Ok(HttpResponse::Ok().json(resp))
}

/// Start the login flow for an upstream auth provider
///
/// **Permissions**
/// - `session-init`
/// - `session-auth`
#[utoipa::path(
    post,
    path = "/providers/login",
    tag = "providers",
    responses(
        (status = 202, description = "Accepted"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/providers/login")]
pub async fn post_provider_login(
    data: web::Data<AppState>,
    // req: HttpRequest,
    payload: Json<ProviderLoginRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth_or_init()?;

    let payload = payload.into_inner();
    // TODO can we get rid of allowed origins here? Do we even need it?
    let (cookie, xsrf_token, location, allowed_origins) =
        AuthProviderCallback::login_start(&data, payload).await?;

    Ok(HttpResponse::Accepted()
        .insert_header((LOCATION, location))
        .cookie(cookie)
        .body(xsrf_token))
}

#[get("/providers/callback")]
pub async fn get_provider_callback_html(
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy(&data).await?;
    let body = ProviderCallbackHtml::build(&colors);

    Ok(HttpResponse::Ok().insert_header(HEADER_HTML).body(body))
}

/// Callback for an upstream auth provider login
///
/// **Permissions**
/// - `session-init`
/// - `session-auth`
#[utoipa::path(
    post,
    path = "/providers/callback",
    tag = "providers",
    responses(
        (status = 200, description = "OK", body = ProviderLookupResponse),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/providers/callback")]
#[tracing::instrument(
    name = "post_provider_callback",
    skip_all, fields(callback_id = payload.state)
)]
pub async fn post_provider_callback(
    data: web::Data<AppState>,
    req: HttpRequest,
    payload: Json<ProviderCallbackRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth_or_init()?;

    let cookie = req.cookie(COOKIE_UPSTREAM_CALLBACK).ok_or_else(|| {
        ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "Missing encrypted callback cookie".to_string(),
        )
    })?;
    let payload = payload.into_inner();
    let res = AuthProviderCallback::login_finish(&data, cookie, &payload).await?;

    todo!()
    // Ok(HttpResponse::Accepted()
    //     .insert_header((LOCATION, location))
    //     .cookie(cookie)
    //     .body(xsrf_token))
}

/// PUT update an upstream auth provider
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    put,
    path = "/providers/{id}",
    tag = "providers",
    responses(
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[put("/providers/{id}")]
pub async fn put_provider(
    data: web::Data<AppState>,
    id: web::Path<String>,
    payload: Json<ProviderRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    if !payload.use_pkce && payload.client_secret.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Must at least be a confidential client or use PKCE".to_string(),
        ));
    }

    AuthProvider::update(&data, id.into_inner(), payload.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}

/// DELETE update an upstream auth provider
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    delete,
    path = "/providers/{id}",
    tag = "providers",
    responses(
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[delete("/providers/{id}")]
pub async fn delete_provider(
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    AuthProvider::delete(&data, &id.into_inner()).await?;
    Ok(HttpResponse::Ok().finish())
}
