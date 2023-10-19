use crate::real_ip_from_req;
use actix_web::{get, web, HttpRequest, Responder};
use actix_web_grants::proc_macro::has_any_permission;
use actix_web_lab::sse;
use rauthy_common::constants::SSE_KEEP_ALIVE;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::api_keys::{AccessGroup, AccessRights, ApiKey};
use rauthy_models::entity::principal::Principal;
use rauthy_models::events::listener::EventRouterMsg;
use rauthy_models::request::EventsListenParams;
use std::time::Duration;
use validator::Validate;

/// Listen to the Events SSE stream
#[utoipa::path(
    get,
    path = "/events",
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
#[get("/events")]
#[has_any_permission("session-auth", "api-key")]
pub async fn sse_events(
    data: web::Data<AppState>,
    api_key: web::ReqData<Option<ApiKey>>,
    principal: web::ReqData<Option<Principal>>,
    req: HttpRequest,
    params: web::Query<EventsListenParams>,
) -> Result<impl Responder, ErrorResponse> {
    params.validate()?;

    if let Some(api_key) = api_key.into_inner() {
        api_key.has_access(AccessGroup::Events, AccessRights::Read)?;
    } else {
        Principal::from_req(principal)?.validate_rauthy_admin()?;
    }

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
