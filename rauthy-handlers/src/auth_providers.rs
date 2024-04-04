use crate::{map_auth_step, ReqPrincipal};
use actix_web::body::BoxBody;
use actix_web::http::header::DispositionType::FormData;
use actix_web::http::header::{CONTENT_TYPE, LOCATION};
use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use actix_web_lab::__reexports::futures_util::StreamExt;
use actix_web_validator::Json;
use image::imageops::FilterType;
use rauthy_common::constants::{HEADER_HTML, HEADER_JSON};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::auth_provider::{
    AuthProvider, AuthProviderCallback, AuthProviderTemplate,
};
use rauthy_models::entity::auth_provider_logo::{AuthProviderLogo, AuthProviderLogoRes};
use rauthy_models::entity::colors::ColorEntity;
use rauthy_models::language::Language;
use rauthy_models::request::{
    ProviderCallbackRequest, ProviderLoginRequest, ProviderLookupRequest, ProviderRequest,
};
use rauthy_models::response::ProviderResponse;
use rauthy_models::templates::ProviderCallbackHtml;
use tracing::{debug, error};

/// GET all upstream auth providers
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    post,
    path = "/providers",
    tag = "providers",
    responses(
        (status = 200, description = "OK", body = ProviderResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/providers")]
pub async fn post_providers(
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
    path = "/providers/create",
    tag = "providers",
    responses(
        (status = 200, description = "OK", body = ProviderResponse),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/providers/create")]
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
    let resp = AuthProvider::lookup_config(&payload).await?;

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
    let (cookie, xsrf_token, location) = AuthProviderCallback::login_start(&data, payload).await?;

    Ok(HttpResponse::Accepted()
        .insert_header((LOCATION, location))
        .cookie(cookie)
        .body(xsrf_token))
}

#[get("/providers/callback")]
pub async fn get_provider_callback_html(
    data: web::Data<AppState>,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    let colors = ColorEntity::find_rauthy(&data).await?;
    let lang = Language::try_from(&req).unwrap_or_default();
    let body = ProviderCallbackHtml::build(&colors, &lang);

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

    let payload = payload.into_inner();
    let session = principal.get_session()?;
    let (auth_step, cookie) =
        AuthProviderCallback::login_finish(&data, &req, &payload, session.clone()).await?;

    let (mut resp, _) = map_auth_step(auth_step, &req)
        .await
        .map_err(|(err, _)| err)?;
    resp.add_cookie(&cookie).map_err(|err| {
        ErrorResponse::new(
            ErrorResponseType::Internal,
            format!("Error adding cookie after map_auth_step: {}", err),
        )
    })?;
    Ok(resp)
}

/// GET all upstream auth providers as templated minimal JSON
///
/// This returns the same version of the auth providers as used in the templated `/authorize`
/// page which is inserted during SSR.
#[utoipa::path(
    get,
    path = "/providers/minimal",
    tag = "providers",
    responses(
        (status = 200, description = "OK", body = ProviderResponse),
    ),
)]
#[get("/providers/minimal")]
pub async fn get_providers_minimal(
    data: web::Data<AppState>,
) -> Result<HttpResponse, ErrorResponse> {
    // unauthorized - does not leak any sensitive information other than shown in the
    // default login page anyway

    let json_tpl = AuthProviderTemplate::get_all_json_template(&data)
        .await?
        .unwrap_or_else(|| String::default());

    Ok(HttpResponse::Ok().insert_header(HEADER_JSON).body(json_tpl))
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

/// PUT upload an image / icon for an auth provider
///
/// The image can only be max 10MB in size and will be minified automatically.
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    get,
    path = "/providers/{id}/img",
    tag = "providers",
    responses(
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[get("/providers/{id}/img")]
pub async fn get_provider_img(
    data: web::Data<AppState>,
    id: web::Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    let id = id.into_inner();
    let logo = AuthProviderLogo::find_cached(&data, &id).await?;

    Ok(HttpResponse::Ok()
        .insert_header((CONTENT_TYPE, logo.content_type))
        .body(logo.data))
}

/// PUT upload an image / icon for an auth provider
///
/// The image can only be max 10MB in size and will be minified automatically.
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    put,
    path = "/providers/{id}/img",
    tag = "providers",
    responses(
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[put("/providers/{id}/img")]
pub async fn put_provider_img(
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
    mut payload: actix_multipart::Multipart,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    // we only accept a single field from the Multipart upload -> no looping here
    let mut buf: Vec<u8> = Vec::with_capacity(128 * 1024);
    let mut content_type = None;
    if let Some(part) = payload.next().await {
        let mut field = part?;

        match field.content_type() {
            Some(mime) => {
                debug!("content_type: {:?}", mime);
                content_type = Some(mime.clone());
            }
            None => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "content_type is missing".to_string(),
                ));
            }
        }

        while let Some(chunk) = field.next().await {
            let bytes = chunk?;
            buf.extend(bytes);
        }
    }

    // content_type unwrap cannot panic -> checked above
    AuthProviderLogo::upsert(&data, id.into_inner(), buf, content_type.unwrap()).await?;

    Ok(HttpResponse::Ok().finish())
}
