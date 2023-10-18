use actix_web::{get, web, Responder};
use actix_web_grants::proc_macro::has_roles;
use actix_web_lab::sse;
use rauthy_common::constants::SSE_KEEP_ALIVE;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::events::listener::EventRouterMsg;
use std::time::Duration;

#[get("/events")]
// #[has_roles("rauthy_admin")] // TODO ADD BACK IN AFTER LOCAL TESTING!!!
pub async fn sse_events(data: web::Data<AppState>) -> Result<impl Responder, ErrorResponse> {
    let (tx, sse) = sse::channel(5);

    if let Err(err) = data
        .tx_events_router
        .send_async(EventRouterMsg::ClientReg {
            ip: "".to_string(),
            tx,
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
