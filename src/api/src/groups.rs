use crate::ReqPrincipal;
use actix_web::web::Json;
use actix_web::{HttpResponse, delete, get, post, put, web};
use rauthy_api_types::groups::GroupRequest;
use rauthy_error::ErrorResponse;
use rauthy_models::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_models::entity::clients_scim::ClientScim;
use rauthy_models::entity::groups::Group;
use validator::Validate;

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
pub async fn get_groups(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Groups, AccessRights::Read)?;

    Group::find_all()
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
    request_body = GroupRequest,
    responses(
        (status = 200, description = "Ok", body = Group),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/groups")]
pub async fn post_group(
    Json(payload): Json<GroupRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Groups, AccessRights::Create)?;
    payload.validate()?;

    let group = Group::create(payload).await?;

    for client in ClientScim::find_all().await? {
        client.create_update_group(group.clone()).await?;
    }

    Ok(HttpResponse::Ok().json(group))
}

/// Modifies a groups name
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    put,
    path = "/groups/{id}",
    tag = "groups",
    request_body = GroupRequest,
    responses(
        (status = 200, description = "Ok", body = Group),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[put("/groups/{id}")]
pub async fn put_group(
    id: web::Path<String>,
    Json(payload): Json<GroupRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Groups, AccessRights::Update)?;
    payload.validate()?;

    let group = Group::update(id.into_inner(), payload.group).await?;

    for client in ClientScim::find_all().await? {
        client.create_update_group(group.clone()).await?;
    }

    Ok(HttpResponse::Ok().json(group))
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
pub async fn delete_group(
    id: web::Path<String>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Groups, AccessRights::Delete)?;

    let gid = id.into_inner();

    let clients = ClientScim::find_all().await?;
    if !clients.is_empty() {
        let group = Group::find(gid.clone()).await?;
        for client in clients {
            client.delete_group(group.clone()).await?;
        }
    }

    Group::delete(gid).await?;

    Ok(HttpResponse::Ok().finish())
}
