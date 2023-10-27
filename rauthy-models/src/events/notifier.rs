use crate::email;
use crate::email::EMail;
use crate::events::event::{Event, EventLevel, EventType};
use crate::events::{EVENT_EMAIL, EVENT_NOTIFY_LEVEL};
use rauthy_common::constants::DEV_MODE;
use rauthy_common::error_response::ErrorResponse;
use tokio::sync::mpsc;
use tracing::{info, warn};

pub struct EventNotifier;

impl EventNotifier {
    pub async fn send(tx_email: &mpsc::Sender<EMail>, event: &Event) -> Result<(), ErrorResponse> {
        match event.level {
            EventLevel::Info | EventLevel::Notice => info!("{}", event),
            EventLevel::Warning => warn!("{}", event),
            EventLevel::Critical => warn!("{}", event),
        }

        if *DEV_MODE {
            warn!(
                "Running in DEV_MODE -> skipping sending out events. New Event to be sent:\n{:?}",
                event
            );
            return Ok(());
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

        // TODO implement other notification channels / options

        Ok(())
    }
}
