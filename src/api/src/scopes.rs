use crate::ReqPrincipal;
use actix_web::web::Json;
use actix_web::{HttpResponse, delete, get, post, put, web};
use rauthy_api_types::scopes::{ScopeRequest, ScopeResponse};
use rauthy_data::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_data::entity::scopes::Scope;
use rauthy_error::ErrorResponse;
use validator::Validate;

/// Returns all existing scopes
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/scopes",
    tag = "scopes",
    responses(
        (status = 200, description = "Ok", body = [Scope]),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/scopes")]
pub async fn get_scopes(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Scopes, AccessRights::Read)?;

    Scope::find_all().await.map(|scp| {
        let res = scp
            .into_iter()
            .map(ScopeResponse::from)
            .collect::<Vec<ScopeResponse>>();
        HttpResponse::Ok().json(res)
    })
}

/// Adds a new scope to the database
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/scopes",
    tag = "scopes",
    request_body = ScopeRequest,
    responses(
        (status = 200, description = "Ok", body = Scope),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/scopes")]
pub async fn post_scope(
    principal: ReqPrincipal,
    Json(payload): Json<ScopeRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Scopes, AccessRights::Create)?;
    payload.validate()?;

    Scope::create(payload)
        .await
        .map(|s| HttpResponse::Ok().json(s))
}

/// Modifies a scopes name
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/scopes/{id}",
    tag = "scopes",
    request_body = ScopeRequest,
    responses(
        (status = 200, description = "Ok", body = Scope),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/scopes/{id}")]
pub async fn put_scope(
    path: web::Path<String>,
    principal: ReqPrincipal,
    Json(payload): Json<ScopeRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Scopes, AccessRights::Update)?;
    payload.validate()?;

    Scope::update(path.as_str(), payload)
        .await
        .map(|s| HttpResponse::Ok().json(ScopeResponse::from(s)))
}

/// Deletes a scope
///
/// It will be deleted from all currently assigned users too and this operation cannot be reverted.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/scopes/{id}",
    tag = "scopes",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[delete("/scopes/{id}")]
pub async fn delete_scope(
    path: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Scopes, AccessRights::Delete)?;

    Scope::delete(path.as_str())
        .await
        .map(|_| HttpResponse::Ok().finish())
}
