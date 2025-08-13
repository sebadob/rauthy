use crate::ReqPrincipal;
use actix_web::web::Json;
use actix_web::{HttpResponse, delete, get, post, put, web};
use rauthy_api_types::roles::RoleRequest;
use rauthy_data::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_data::entity::roles::Role;
use rauthy_error::ErrorResponse;
use validator::Validate;

/// Returns all existing roles
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/roles",
    tag = "roles",
    responses(
        (status = 200, description = "Ok", body = [Role]),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/roles")]
pub async fn get_roles(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Roles, AccessRights::Read)?;

    Role::find_all()
        .await
        .map(|rls| HttpResponse::Ok().json(rls))
}

/// Adds a new role to the database
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/roles",
    tag = "roles",
    request_body = RoleRequest,
    responses(
        (status = 200, description = "Ok", body = Role),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/roles")]
pub async fn post_role(
    Json(payload): Json<RoleRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Roles, AccessRights::Create)?;
    payload.validate()?;

    Role::create(payload)
        .await
        .map(|r| HttpResponse::Ok().json(r))
}

/// Modifies a roles name
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/roles/{id}",
    tag = "roles",
    request_body = RoleRequest,
    responses(
        (status = 200, description = "Ok", body = Role),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/roles/{id}")]
pub async fn put_role(
    id: web::Path<String>,
    Json(payload): Json<RoleRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Roles, AccessRights::Update)?;
    payload.validate()?;

    Role::update(id.into_inner(), payload.role)
        .await
        .map(|r| HttpResponse::Ok().json(r))
}

/// Deletes a role
///
/// It will be deleted from all currently assigned users too and this operation cannot be reverted.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/roles/{id}",
    tag = "roles",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[delete("/roles/{id}")]
pub async fn delete_role(
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Roles, AccessRights::Delete)?;

    Role::delete(id.as_str())
        .await
        .map(|_| HttpResponse::Ok().finish())
}
