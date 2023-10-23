use crate::events::event::EventLevel;
use rauthy_common::error_response::ErrorResponse;
use std::env;
use std::sync::OnceLock;
use tracing::{info, warn};

pub mod event;
pub mod listener;
pub mod notifier;

pub static EVENT_EMAIL: OnceLock<String> = OnceLock::new();
pub static EVENT_NOTIFY_LEVEL: OnceLock<i16> = OnceLock::new();
pub static EVENT_PERSIST_LEVEL: OnceLock<i16> = OnceLock::new();

pub fn init_event_vars() -> Result<(), ErrorResponse> {
    let level = env::var("EVENT_NOTIFY_LEVEL")
        .map(|level| {
            level.parse::<EventLevel>().expect(
                "Cannot parse EVENT_NOTIFY_LEVEL. Possible values: info, notice, warning, critical",
            )
        })
        .unwrap_or(EventLevel::Notice);
    info!("Event Notification Level: {:?}", level);
    EVENT_NOTIFY_LEVEL
        .set(level.value())
        .expect("init_event_vars() must only be called once");

    if let Ok(email) = env::var("EVENT_EMAIL") {
        info!("Event Notifications will be sent to: {}", email);
        EVENT_EMAIL.set(email).unwrap();
    } else {
        warn!("No Event Notifications will be sent: EVENT_EMAIL is not configured");
    }

    let level = env::var("EVENT_PERSIST_LEVEL")
        .map(|level| {
            level.parse::<EventLevel>().expect(
                "Cannot parse EVENT_PERSIST_LEVEL. Possible values: info, notice, warning, critical",
            )
        })
        .unwrap_or(EventLevel::Notice);
    info!("Event Persistence Level: {:?}", level);
    EVENT_PERSIST_LEVEL.set(level.value()).unwrap();

    Ok(())
}
