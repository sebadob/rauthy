use actix_web::{
    HttpResponse, get,
    http::header::{self, HeaderValue},
};
use rauthy_data::{
    entity::atproto::{self},
    rauthy_config::RauthyConfig,
};
use rauthy_error::{ErrorResponse, ErrorResponseType};

/// ATProto public client metadata
///
/// Returns the public client metadata to securely facilitate the authorization process.
#[utoipa::path(
    get,
    path = "/atproto/client_metadata",
    tag = "atproto",
    responses(
        (status = 200, description = "OK"),
    ),
)]
#[get("/atproto/client_metadata")]
pub async fn get_atproto_client_metadata() -> Result<HttpResponse, ErrorResponse> {
    let config = RauthyConfig::get();

    if config.vars.atproto.enable {
        Ok(HttpResponse::Ok()
            .insert_header((
                header::ACCESS_CONTROL_ALLOW_ORIGIN,
                HeaderValue::from_static("*"),
            ))
            .json(&atproto::Client::get().client_metadata))
    } else {
        Err(ErrorResponse::new(
            ErrorResponseType::Internal,
            "The atproto client is disabled on this instance",
        ))
    }
}
