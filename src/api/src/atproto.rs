use actix_web::{
    get,
    http::header::{self, HeaderValue, LOCATION},
    post,
    web::{self, Json},
    HttpResponse,
};
use rauthy_api_types::atproto;
use rauthy_error::ErrorResponse;
use rauthy_models::{
    app_state::AppState,
    entity::{atproto::AtprotoCallback, auth_providers::AuthProviderCallback},
};

use crate::ReqPrincipal;

#[utoipa::path(
    get,
    path = "/atproto/client_metadata",
    tag = "atproto",
    responses(
        (status = 200, description = "OK"),
    ),
)]
#[get("/atproto/client_metadata")]
pub async fn get_client_metadata(data: web::Data<AppState>) -> Result<HttpResponse, ErrorResponse> {
    Ok(HttpResponse::Ok()
        .insert_header((
            header::ACCESS_CONTROL_ALLOW_ORIGIN,
            HeaderValue::from_str("*").unwrap(),
        ))
        .json(&data.atproto.client_metadata))
}

#[utoipa::path(
    post,
    path = "/atproto/login",
    tag = "atproto",
    responses(
        (status = 202, description = "Accepted"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[post("/atproto/login")]
pub async fn post_login(
    data: web::Data<AppState>,
    payload: Json<atproto::LoginRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_session_auth_or_init()?;

    let payload = payload.into_inner();
    let (cookie, xsrf_token, location) =
        <AuthProviderCallback as AtprotoCallback>::login_start(&data, payload).await?;

    Ok(HttpResponse::Accepted()
        .insert_header((LOCATION, location))
        .cookie(cookie)
        .body(xsrf_token))
}
