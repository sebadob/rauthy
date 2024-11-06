use crate::ReqPrincipal;
use actix_web::{delete, get, post, put, web, HttpResponse};
use actix_web_validator::Json;
use mime_guess::mime::TEXT_PLAIN_UTF_8;
use rauthy_api_types::api_keys::{ApiKeyRequest, ApiKeyResponse, ApiKeysResponse};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::api_keys::ApiKeyEntity;

/// Returns all API Keys
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/api_keys",
    tag = "api_keys",
    responses(
        (status = 200, description = "Ok", body = ApiKeysResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/api_keys")]
pub async fn get_api_keys(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let entities = ApiKeyEntity::find_all(&data).await?;
    let mut keys = Vec::with_capacity(entities.len());
    for entity in entities {
        let key = entity.into_api_key()?;
        keys.push(ApiKeyResponse::from(key));
    }

    Ok(HttpResponse::Ok().json(ApiKeysResponse { keys }))
}

/// Create a new API Key
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/api_keys",
    tag = "api_keys",
    request_body = ApiKeyRequest,
    responses(
        (status = 200, description = "Ok", body = String),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/api_keys")]
pub async fn post_api_key(
    principal: ReqPrincipal,
    payload: Json<ApiKeyRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let req = payload.into_inner();
    let access = req.access.into_iter().map(|a| a.into()).collect();
    let secret = ApiKeyEntity::create(req.name, req.exp, access).await?;

    Ok(HttpResponse::Ok()
        .content_type(TEXT_PLAIN_UTF_8)
        .body(secret))
}

/// Update an API Key
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/api_keys/{name}",
    tag = "api_keys",
    request_body = ApiKeyRequest,
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/api_keys/{name}")]
pub async fn put_api_key(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
    name: web::Path<String>,
    payload: Json<ApiKeyRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let name = name.into_inner();
    let req = payload.into_inner();
    if req.name != name {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "JSON payload does not match the Name from the path".to_string(),
        ));
    }

    let access = req.access.into_iter().map(|a| a.into()).collect();
    ApiKeyEntity::update(&data, &name, req.exp, access).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Delete an API Key
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/api_keys/{name}",
    tag = "api_keys",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[delete("/api_keys/{name}")]
pub async fn delete_api_key(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
    name: web::Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let name = name.into_inner();
    ApiKeyEntity::delete(&data, &name).await?;

    Ok(HttpResponse::Ok().finish())
}

/// Endpoint for testing an API Key and getting its Access Rights
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/api_keys/{name}/test",
    tag = "api_keys",
    responses(
        (status = 200, description = "Ok", body = ApiKeyResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/api_keys/{name}/test")]
pub async fn get_api_key_test(
    principal: ReqPrincipal,
    name: web::Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    let p = principal.into_inner();
    if let Some(api_key) = p.api_key {
        let name = name.into_inner();
        if name != api_key.name {
            Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Wrong API Key given".to_string(),
            ))
        } else {
            Ok(HttpResponse::Ok().json(ApiKeyResponse::from(api_key)))
        }
    } else {
        Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "No API Key given".to_string(),
        ))
    }
}

/// Generate a new secret for the given API Key
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/api_keys/{name}/secret",
    tag = "api_keys",
    responses(
        (status = 200, description = "Ok", body = String),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/api_keys/{name}/secret")]
pub async fn put_api_key_secret(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
    name: web::Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let name = name.into_inner();
    let secret = ApiKeyEntity::generate_secret(&data, &name).await?;

    Ok(HttpResponse::Ok()
        .content_type(TEXT_PLAIN_UTF_8)
        .body(secret))
}
