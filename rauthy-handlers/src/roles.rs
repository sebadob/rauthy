use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use actix_web_grants::proc_macro::has_roles;
use rauthy_common::error_response::ErrorResponse;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::principal::Principal;
use rauthy_models::entity::roles::Role;
use rauthy_models::entity::sessions::Session;
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
#[has_roles("rauthy_admin")]
pub async fn get_roles(
    data: web::Data<AppState>,
    principal: web::ReqData<Option<Principal>>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::from_req(principal)?;
    principal.validate_rauthy_admin()?;

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
#[has_roles("rauthy_admin")]
pub async fn post_role(
    data: web::Data<AppState>,
    role_req: actix_web_validator::Json<NewRoleRequest>,
    req: HttpRequest,
    principal: web::ReqData<Option<Principal>>,
    session_req: web::ReqData<Option<Session>>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::from_req(principal)?;
    principal.validate_rauthy_admin()?;
    if session_req.is_some() {
        Session::extract_validate_csrf(session_req, &req)?;
    }

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
#[has_roles("rauthy_admin")]
pub async fn put_role(
    data: web::Data<AppState>,
    id: web::Path<String>,
    role_req: actix_web_validator::Json<NewRoleRequest>,
    req: HttpRequest,
    principal: web::ReqData<Option<Principal>>,
    session_req: web::ReqData<Option<Session>>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::from_req(principal)?;
    principal.validate_rauthy_admin()?;
    if session_req.is_some() {
        Session::extract_validate_csrf(session_req, &req)?;
    }

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
#[has_roles("rauthy_admin")]
pub async fn delete_role(
    data: web::Data<AppState>,
    id: web::Path<String>,
    req: HttpRequest,
    principal: web::ReqData<Option<Principal>>,
    session_req: web::ReqData<Option<Session>>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::from_req(principal)?;
    principal.validate_rauthy_admin()?;
    if session_req.is_some() {
        Session::extract_validate_csrf(session_req, &req)?;
    }

    Role::delete(&data, id.as_str())
        .await
        .map(|_| HttpResponse::Ok().finish())
}
