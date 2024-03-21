use crate::ReqPrincipal;
use actix_web::{delete, get, post, put, web, HttpResponse};
use actix_web_validator::Json;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::auth_provider::AuthProvider;
use rauthy_models::request::{ProviderLookupRequest, ProviderRequest};
use rauthy_models::response::ProviderResponse;

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
pub async fn post_provider_lookup(
    // data: web::Data<AppState>,
    payload: Json<ProviderLookupRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let payload = payload.into_inner();
    let resp = AuthProvider::lookup_config(
        &payload.issuer,
        payload.danger_allow_http.unwrap_or(false),
        payload.danger_allow_insecure.unwrap_or(false),
    )
    .await?;

    Ok(HttpResponse::Ok().json(resp))
}

/// PUT update an upstream auth provider
///
/// **Permissions**
/// - `rauthy_admin`
#[utoipa::path(
    put,
    path = "/providers",
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
    path = "/providers",
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

    todo!()
    // let provider = AuthProvider::create(&data, payload.into_inner()).await?;
    // Ok(HttpResponse::Ok().json(ProviderResponse::from(provider)))
}
