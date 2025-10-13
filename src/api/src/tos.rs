use crate::ReqPrincipal;
use actix_web::web::Json;
use actix_web::{HttpResponse, get, post};
use rauthy_api_types::tos::{ToSLatestResponse, ToSRequest, ToSResponse};
use rauthy_data::entity::tos::ToS;
use rauthy_data::entity::users::User;
use rauthy_error::ErrorResponse;

/// Returns all ToS
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/tos",
    tag = "tos",
    responses(
        (status = 200, description = "Ok", body = [ToSResponse]),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/tos")]
pub async fn get_tos(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let resp = ToS::find_all()
        .await?
        .into_iter()
        .map(ToSResponse::from)
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(resp))
}

/// Create and publish new ToS
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    post,
    path = "/tos",
    tag = "tos",
    responses(
        (status = 201, description = "Created"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/tos")]
pub async fn post_tos(
    principal: ReqPrincipal,
    payload: Json<ToSRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let payload = payload.into_inner();
    let user = User::find(principal.user_id()?.to_string()).await?;

    // Note: We do not use an FK here on purpose. We still want to be able to know the email even
    // if this user gets deleted at some point in the future.
    ToS::create(user.email, payload.is_html, payload.content).await?;

    Ok(HttpResponse::Created().finish())
}

/// Returns the latest ToS
#[utoipa::path(
    get,
    path = "/tos/latest",
    tag = "tos",
    responses(
        (status = 200, description = "Ok", body = ToSLatestResponse),
        (status = 204, description = "NoContent"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/tos/latest")]
pub async fn get_tos_latest() -> Result<HttpResponse, ErrorResponse> {
    if let Some(tos) = ToS::find_latest().await? {
        Ok(HttpResponse::Ok().json(ToSLatestResponse::from(tos)))
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}

/// Accept a ToS
///
/// **Permissions**
/// - valid session
#[utoipa::path(
    post,
    path = "/tos/accept/{ts}",
    tag = "tos",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/tos/accept/{ts}")]
pub async fn post_tos_accept(principal: ReqPrincipal) -> Result<HttpResponse, ErrorResponse> {
    // TODO we may run into an issue here if the user needs to accept updated ToS during the login.
    //  In the ideal moment for this, the session may not be in auth state yet, but in between.
    principal.validate_session_auth()?;

    // let user = User::find(principal.user_id()?.to_string()).await?;

    todo!()
}
