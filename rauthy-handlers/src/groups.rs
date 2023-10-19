use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use actix_web_grants::proc_macro::has_roles;
use rauthy_common::error_response::ErrorResponse;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::groups::Group;
use rauthy_models::entity::principal::Principal;
use rauthy_models::entity::sessions::Session;
use rauthy_models::request::NewGroupRequest;

/// Returns all existing *groups*
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/groups",
    tag = "groups",
    responses(
        (status = 200, description = "Ok", body = [Group]),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/groups")]
#[has_roles("rauthy_admin")]
pub async fn get_groups(
    data: web::Data<AppState>,
    principal: web::ReqData<Option<Principal>>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::from_req(principal)?;
    principal.validate_rauthy_admin()?;

    Group::find_all(&data)
        .await
        .map(|rls| HttpResponse::Ok().json(rls))
}

/// Adds a new group to the database
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/groups",
    tag = "groups",
    request_body = NewGroupRequest,
    responses(
        (status = 200, description = "Ok", body = Group),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/groups")]
#[has_roles("rauthy_admin")]
pub async fn post_group(
    data: web::Data<AppState>,
    group_req: actix_web_validator::Json<NewGroupRequest>,
    req: HttpRequest,
    principal: web::ReqData<Option<Principal>>,
    session_req: web::ReqData<Option<Session>>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::from_req(principal)?;
    principal.validate_rauthy_admin()?;
    if session_req.is_some() {
        Session::extract_validate_csrf(session_req, &req)?;
    }

    Group::create(&data, group_req.into_inner())
        .await
        .map(|r| HttpResponse::Ok().json(r))
}

/// Modifies a groups name
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/groups/{id}",
    tag = "groups",
    request_body = NewGroupRequest,
    responses(
        (status = 200, description = "Ok", body = Group),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/groups/{id}")]
#[has_roles("rauthy_admin")]
pub async fn put_group(
    data: web::Data<AppState>,
    id: web::Path<String>,
    group_req: actix_web_validator::Json<NewGroupRequest>,
    req: HttpRequest,
    principal: web::ReqData<Option<Principal>>,
    session_req: web::ReqData<Option<Session>>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::from_req(principal)?;
    principal.validate_rauthy_admin()?;
    if session_req.is_some() {
        Session::extract_validate_csrf(session_req, &req)?;
    }

    Group::update(&data, id.into_inner(), group_req.group.to_owned())
        .await
        .map(|g| HttpResponse::Ok().json(g))
}

/// Deletes a group
///
/// It will be deleted from all currently assigned users too and this operation cannot be reverted.
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    delete,
    path = "/groups/{id}",
    tag = "groups",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[delete("/groups/{id}")]
#[has_roles("rauthy_admin")]
pub async fn delete_group(
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

    Group::delete(&data, id.into_inner())
        .await
        .map(|_| HttpResponse::Ok().finish())
}
