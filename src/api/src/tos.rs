use crate::ReqPrincipal;
use actix_web::web::{Json, Path};
use actix_web::{HttpResponse, get, post};
use rauthy_api_types::tos::{
    ToSAcceptRequest, ToSLatestResponse, ToSRequest, ToSResponse, ToSUserAcceptResponse,
};
use rauthy_data::entity::auth_codes::{AuthCode, AuthCodeToSAwait};
use rauthy_data::entity::tos::ToS;
use rauthy_data::entity::tos_user_accept::ToSUserAccept;
use rauthy_data::entity::users::User;
use rauthy_error::{ErrorResponse, ErrorResponseType};

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

/// GET user accept status for all existing ToS
///
/// **Permissions**
/// - rauthy_admin
#[utoipa::path(
    get,
    path = "/tos/user/{id}",
    tag = "tos",
    responses(
        (status = 200, description = "Ok", body = [ToSUserAcceptResponse]),
        (status = 204, description = "NoContent"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[get("/tos/user/{id}")]
pub async fn get_tos_user_status(
    principal: ReqPrincipal,
    uid: Path<String>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_admin_session()?;

    let res = ToSUserAccept::find_all(uid.into_inner())
        .await?
        .into_iter()
        .map(ToSUserAcceptResponse::from)
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(res))
}

/// Accept an updated ToS for existing accounts
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
pub async fn post_tos_accept(
    principal: ReqPrincipal,
    payload: Json<ToSAcceptRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth_or_init()?;

    // TODO This endpoint should only be called directly after a successful login after a ToS
    //  update. This means we need to handle AuthCode lifetime and re-insert with reduced TTL
    //  most probably.

    let Some(code_await) = AuthCodeToSAwait::find(&payload.accept_code).await? else {
        return Err(ErrorResponse::new(
            ErrorResponseType::NotFound,
            "Invalid ToS accept code",
        ));
    };

    let Some(auth_code) = AuthCode::find(code_await.auth_code).await? else {
        return Err(ErrorResponse::new(
            ErrorResponseType::NotFound,
            "AuthCode does not exist anymore",
        ));
    };

    let user_id = principal.user_id()?.to_string();
    if user_id != auth_code.user_id {
        return Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "Mismatch in UserID",
        ));
    }
    let user = User::find(principal.user_id()?.to_string()).await?;
    user.check_enabled()?;
    user.check_expired()?;

    todo!()
}
