use crate::ReqPrincipal;
use actix_web::{delete, get, post, put, web, HttpResponse};
use rauthy_error::ErrorResponse;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_models::entity::roles::Role;
use rauthy_models::request::NewRoleRequest;

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
pub async fn get_roles(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Roles, AccessRights::Read)?;

    Role::find_all(&data)
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
    request_body = NewRoleRequest,
    responses(
        (status = 200, description = "Ok", body = Role),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/roles")]
pub async fn post_role(
    data: web::Data<AppState>,
    role_req: actix_web_validator::Json<NewRoleRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Roles, AccessRights::Create)?;

    Role::create(&data, role_req.into_inner())
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
    request_body = NewRoleRequest,
    responses(
        (status = 200, description = "Ok", body = Role),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/roles/{id}")]
pub async fn put_role(
    data: web::Data<AppState>,
    id: web::Path<String>,
    role_req: actix_web_validator::Json<NewRoleRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Roles, AccessRights::Update)?;

    Role::update(&data, id.into_inner(), role_req.role.to_owned())
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
    data: web::Data<AppState>,
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Roles, AccessRights::Delete)?;

    Role::delete(&data, id.as_str())
        .await
        .map(|_| HttpResponse::Ok().finish())
}
