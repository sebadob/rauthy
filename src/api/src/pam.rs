use actix_web::{HttpResponse, get, post};
use actix_web_lab::extract::Json;
use rauthy_api_types::pam::PamPreflightRequest;
use rauthy_error::ErrorResponse;

#[get("/pam/preflight")]
pub async fn get_preflight(
    Json(payload): Json<PamPreflightRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    todo!("Check and return PamPreflightResponse")
}

#[post("/pam/login")]
pub async fn post_login() -> Result<HttpResponse, ErrorResponse> {
    // TODO add a new MfaPurpose for PAM Logins and require a code here
    todo!("Perform login based on conditions for this client + user")
}
