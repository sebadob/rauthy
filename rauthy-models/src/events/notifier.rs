use crate::email;
use crate::email::EMail;
use crate::events::event::{Event, EventLevel, EventType};
use rauthy_common::error_response::ErrorResponse;
use std::env;
use std::sync::OnceLock;
use tokio::sync::mpsc;
use tracing::{info, warn};

static EVENT_EMAIL: OnceLock<String> = OnceLock::new();
static EVENT_NOTIFY_LEVEL: OnceLock<i16> = OnceLock::new();

pub struct EventNotifier;

impl EventNotifier {
    pub fn init() -> Result<(), ErrorResponse> {
        let level = env::var("EVENT_NOTIFY_LEVEL")
            .map(|level|
                level
                    .parse::<EventLevel>()
                    .expect("Cannot parse EVENT_NOTIFY_LEVEL. Possible values: info, notice, warning, critical"))
            .unwrap_or(EventLevel::Notice);
        info!("Event Notification Level: {:?}", level);
        EVENT_NOTIFY_LEVEL
            .set(level.value())
            .expect("EventNotifier::init must only be called once");

        // init email
        // let email = env::var("EVENT_EMAIL").ok()).unwrap();
        if let Ok(email) = env::var("EVENT_EMAIL") {
            info!("Event Notifications will be sent to: {}", email);
            EVENT_EMAIL.set(email).unwrap();
        } else {
            warn!("No Event Notifications will be sent: EVENT_EMAIL is not configured");
        }

        // TODO implement other notification channels / options

        Ok(())
    }

    pub async fn send(tx_email: &mpsc::Sender<EMail>, event: &Event) -> Result<(), ErrorResponse> {
        match event.level {
            EventLevel::Info | EventLevel::Notice => info!("{}", event),
            EventLevel::Warning => warn!("{}", event),
            EventLevel::Critical => warn!("{}", event),
        }

        if let Some(email) = EVENT_EMAIL.get() {
            if event.typ == EventType::Test
                || &event.level.value()
                    >= EVENT_NOTIFY_LEVEL
                        .get()
                        .expect("EventNotifier::init has not been called")
            {
                email::send_email_event(email.clone(), tx_email, event).await;
            }
        }

        Ok(())
    }
}
