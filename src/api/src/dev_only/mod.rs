use actix_web::{HttpResponse, get, post};
use rauthy_data::entity::browser_id::BrowserId;
use rauthy_error::ErrorResponse;

#[cfg(debug_assertions)]
pub mod dev_handler;

// dev-only endpoint - in prod, values will be inserted into the HTML directly.
// Returns the inner template value, as it would be rendered during prod, inside the body,
// which is different depending on the id.
#[cfg(debug_assertions)]
#[get("/template/{id}")]
pub async fn get_template(
    id: actix_web::web::Path<String>,
    req: actix_web::HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    dev_handler::get_template(id, req).await
}

#[cfg(not(debug_assertions))]
#[get("/template/{id}")]
pub async fn get_template() -> Result<HttpResponse, ErrorResponse> {
    Ok(HttpResponse::NotFound().finish())
}

// This is a dev only endpoint. It is used in very specific scenarios only to work around a
// limitation of the vite proxy. E.h. the `/oidc/logout` endpoint is used with `GET` to return
// HTML, and with POST to actually do the logout. However, the vite proxy (at the time of writing)
// cannot be configured in a fine-grained way to only proxy POST request to the backend and handle
// GET via the dev UI. This endpoint solves this issue to provide a better DX and make everything
// usable during local dev.
//
// DEV_MODE MUST be `true` and the code be compiled with `debug_assertions` to make it work.
#[cfg(debug_assertions)]
#[post("/dev/{typ}")]
pub async fn post_dev_only_endpoints(
    typ: actix_web::web::Path<String>,
    req: actix_web::HttpRequest,
    browser_id: BrowserId,
    payload: actix_web::web::Payload,
) -> Result<HttpResponse, ErrorResponse> {
    dev_handler::post_dev_only_endpoints(typ, req, browser_id, payload).await
}

#[cfg(not(debug_assertions))]
#[post("/dev/{typ}")]
pub async fn post_dev_only_endpoints() -> Result<HttpResponse, ErrorResponse> {
    Ok(HttpResponse::NotFound().finish())
}
