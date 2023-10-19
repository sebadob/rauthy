use actix_web::{delete, get, post, put, web, HttpRequest, HttpResponse};
use actix_web_grants::proc_macro::has_roles;
use rauthy_common::error_response::ErrorResponse;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::principal::Principal;
use rauthy_models::entity::scopes::Scope;
use rauthy_models::entity::sessions::Session;
use rauthy_models::request::ScopeRequest;
use rauthy_models::response::ScopeResponse;

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
#[has_roles("rauthy_admin")]
pub async fn get_scopes(
    data: web::Data<AppState>,
    principal: web::ReqData<Option<Principal>>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::from_req(principal)?;
    principal.validate_rauthy_admin()?;

    Scope::find_all(&data).await.map(|scp| {
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
#[has_roles("rauthy_admin")]
pub async fn post_scope(
    data: web::Data<AppState>,
    req: HttpRequest,
    principal: web::ReqData<Option<Principal>>,
    session_req: web::ReqData<Option<Session>>,
    scope_req: actix_web_validator::Json<ScopeRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::from_req(principal)?;
    principal.validate_rauthy_admin()?;
    if session_req.is_some() {
        Session::extract_validate_csrf(session_req, &req)?;
    }

    Scope::create(&data, scope_req.into_inner())
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
#[has_roles("rauthy_admin")]
pub async fn put_scope(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: HttpRequest,
    principal: web::ReqData<Option<Principal>>,
    session_req: web::ReqData<Option<Session>>,
    scope_req: actix_web_validator::Json<ScopeRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::from_req(principal)?;
    principal.validate_rauthy_admin()?;
    if session_req.is_some() {
        Session::extract_validate_csrf(session_req, &req)?;
    }

    Scope::update(&data, path.as_str(), scope_req.into_inner())
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
#[has_roles("rauthy_admin")]
pub async fn delete_scope(
    data: web::Data<AppState>,
    path: web::Path<String>,
    req: HttpRequest,
    principal: web::ReqData<Option<Principal>>,
    session_req: web::ReqData<Option<Session>>,
) -> Result<HttpResponse, ErrorResponse> {
    let principal = Principal::from_req(principal)?;
    principal.validate_rauthy_admin()?;
    if session_req.is_some() {
        Session::extract_validate_csrf(session_req, &req)?;
    }

    Scope::delete(&data, path.as_str())
        .await
        .map(|_| HttpResponse::Ok().finish())
}
