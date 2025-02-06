use crate::ReqPrincipal;
use actix_web::web::{Json, Query};
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use actix_web_lab::sse;
use chrono::Utc;
use rauthy_api_types::events::{EventResponse, EventsListenParams, EventsRequest};
use rauthy_common::constants::{DEV_MODE, SSE_KEEP_ALIVE};
use rauthy_common::utils::real_ip_from_req;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::api_keys::{AccessGroup, AccessRights};
use rauthy_models::events::event::Event;
use rauthy_models::events::listener::EventRouterMsg;
use std::net::IpAddr;
use std::time::Duration;
use tokio::sync::mpsc;
use validator::Validate;

/// Get events
#[utoipa::path(
    post,
    path = "/events",
    tag = "events",
    request_body = EventsRequest,
    responses(
        (status = 200, description = "Ok", body = [EventResponse]),
        (status = 400, description = "BadRequest", body = ErrorResponse),
        (status = 401, description = "Unauthorized", body = ErrorResponse),
        (status = 403, description = "Forbidden", body = ErrorResponse),
    ),
)]
#[post("/events")]
pub async fn post_events(
    principal: ReqPrincipal,
    Json(payload): Json<EventsRequest>,
) -> Result<HttpResponse, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Events, AccessRights::Read)?;
    payload.validate()?;

    let events = Event::find_all(
        payload.from,
        payload.until.unwrap_or_else(|| Utc::now().timestamp()),
        payload.level.into(),
        payload.typ.map(|t| t.into()),
    )
    .await?
    .into_iter()
    .map(EventResponse::from)
    .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(events))
}

/// Listen to the Events SSE stream
#[utoipa::path(
    get,
    path = "/events/stream",
    tag = "events",
    params(EventsListenParams),
    responses(
        (status = 200, description = "Ok", body = [EventResponse]),
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
    params: Query<EventsListenParams>,
    req: HttpRequest,
) -> Result<impl Responder, ErrorResponse> {
    principal.validate_api_key_or_admin_session(AccessGroup::Events, AccessRights::Read)?;
    params.validate()?;

    let ip = real_ip_from_req(&req)?.to_string();
    let params = params.into_inner();
    let (tx, rx) = mpsc::channel(10);

    let level = params.level.map(|l| l.into()).unwrap_or_default();
    if let Err(err) = data
        .tx_events_router
        .send_async(EventRouterMsg::ClientReg {
            ip,
            tx,
            latest: params.latest,
            level,
        })
        .await
    {
        Err(ErrorResponse::new(
            ErrorResponseType::Internal,
            format!("Cannot register SSE client: {:?}", err),
        ))
    } else {
        Ok(sse::Sse::from_infallible_receiver(rx)
            .with_keep_alive(Duration::from_secs(*SSE_KEEP_ALIVE as u64))
            .with_retry_duration(Duration::from_secs(10)))
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

    Event::test(real_ip_from_req(&req)?)
        .send(&data.tx_events)
        .await?;

    #[cfg(debug_assertions)]
    if *DEV_MODE {
        // in debug + dev mode, we want to generate one event of each type
        // as a helper for UI development
        let ip = "0.0.0.0".parse::<IpAddr>().unwrap();

        Event::invalid_login(1, ip.to_string())
            .send(&data.tx_events)
            .await?;
        Event::brute_force(ip.to_string())
            .send(&data.tx_events)
            .await?;
        Event::ip_blacklisted(Utc::now(), ip.to_string())
            .send(&data.tx_events)
            .await?;
        Event::ip_blacklist_removed(ip.to_string())
            .send(&data.tx_events)
            .await?;
        Event::new_user("test@dummy".to_string(), ip.to_string())
            .send(&data.tx_events)
            .await?;
        Event::new_rauthy_admin("test-admin@dummy".to_string(), ip.to_string())
            .send(&data.tx_events)
            .await?;
        Event::new_rauthy_version("https://github.com/sebadob/rauthy/releases".to_string())
            .send(&data.tx_events)
            .await?;
        Event::jwks_rotated().send(&data.tx_events).await?;
        Event::rauthy_started().send(&data.tx_events).await?;
        Event::rauthy_healthy().send(&data.tx_events).await?;
        Event::rauthy_unhealthy_cache()
            .send(&data.tx_events)
            .await?;
        Event::rauthy_unhealthy_db().send(&data.tx_events).await?;
        Event::secrets_migrated(ip.clone())
            .send(&data.tx_events)
            .await?;
        Event::test(ip.clone()).send(&data.tx_events).await?;

        let old_email = "old@mail";
        let new_mail = "new@mail";
        let text = format!("{} -> {}", old_email, new_mail);
        let text_admin = format!("Change by admin: {} -> {}", old_email, new_mail);
        Event::user_email_change(text, Some(ip.clone()))
            .send(&data.tx_events)
            .await?;
        Event::user_email_change(text_admin, Some(ip.clone()))
            .send(&data.tx_events)
            .await?;

        let text = format!("Reset via Password Reset Form: {}", "dummy@mail");
        let text_admin = format!("Reset done by admin for user {}", "dummy@mail");
        Event::user_password_reset(text, Some(ip.to_string()))
            .send(&data.tx_events)
            .await?;
        Event::user_password_reset(text_admin, Some(ip.to_string()))
            .send(&data.tx_events)
            .await?;
    }

    Ok(HttpResponse::Ok().finish())
}
