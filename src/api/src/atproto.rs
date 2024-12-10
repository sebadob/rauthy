use actix_web::{
    get,
    http::header::{self, HeaderValue},
    web, HttpResponse,
};
use rauthy_error::ErrorResponse;
use rauthy_models::app_state::AppState;

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
