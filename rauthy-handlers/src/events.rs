use crate::{real_ip_from_req, ReqPrincipal};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web_lab::sse;
use rauthy_common::constants::SSE_KEEP_ALIVE;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_models::events::event::Event;
use rauthy_models::events::listener::EventRouterMsg;
use rauthy_models::request::EventsListenParams;
use std::time::Duration;
use validator::Validate;

/// Listen to the Events SSE stream
#[utoipa::path(
    get,
    path = "/events/stream",
    tag = "events",
    params(EventsListenParams),
    responses(
        (status = 200, description = "Ok"),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
        (status = 404, description = "NotFound", body = ErrorResponse),
    ),
)]
#[get("/events/stream")]
pub async fn sse_events(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
    params: web::Query<EventsListenParams>,
    req: HttpRequest,
) -> Result<impl Responder, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Events, AccessRights::Read)?;

    params.validate()?;

    let (tx, sse) = sse::channel(10);

    match real_ip_from_req(&req) {
        None => Err(ErrorResponse::new(
            ErrorResponseType::NotFound,
            "Cannot extract client IP from HttpRequest. This is an internal network error."
                .to_string(),
        )),
        Some(ip) => {
            let params = params.into_inner();
            if let Err(err) = data
                .tx_events_router
                .send_async(EventRouterMsg::ClientReg {
                    ip,
                    tx,
                    latest: params.latest,
                    level: params.level.unwrap_or_default(),
                })
                .await
            {
                Err(ErrorResponse::new(
                    ErrorResponseType::Internal,
                    format!("Cannot register SSE client: {:?}", err),
                ))
            } else {
                Ok(sse.with_keep_alive(Duration::from_secs(*SSE_KEEP_ALIVE as u64)))
            }
        }
    }
}

/// Create a TEST Event
#[utoipa::path(
    post,
    path = "/events/test",
    tag = "events",
    responses(
        (status = 200, description = "Ok"),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/events/test")]
pub async fn post_event_test(
    data: web::Data<AppState>,
    principal: ReqPrincipal,
    req: HttpRequest,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Events, AccessRights::Create)?;

    Event::test(real_ip_from_req(&req))
        .send(&data.tx_events)
        .await?;

    Ok(HttpResponse::Ok().finish())
}
