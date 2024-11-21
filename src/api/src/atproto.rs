use actix_web::{
    post,
    web::{self, Json, Query},
    HttpRequest, HttpResponse,
};
use rauthy_api_types::atproto;
use rauthy_error::ErrorResponse;
use rauthy_models::app_state::AppState;

use crate::ReqPrincipal;

#[utoipa::path(
    post,
    path = "/atproto/login",
    tag = "atproto",
    responses(
        (status = 202, description = "Accepted"),
    ),
)]
#[post("/atproto/login")]
pub async fn post_login(
    data: web::Data<AppState>,
    payload: Json<atproto::LoginRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    todo!()
}

#[utoipa::path(
    post,
    path = "/atproto/callback",
    tag = "atproto",
    responses(
        (status = 200, description = "OK"),
    ),
)]
#[post("/atproto/callback")]
pub async fn post_callback(
    data: web::Data<AppState>,
    req: HttpRequest,
    payload: Query<atproto::CallbackRequest>,
    principal: ReqPrincipal,
) -> Result<HttpResponse, ErrorResponse> {
    todo!()
}
