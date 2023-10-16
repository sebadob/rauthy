use crate::event::{Event, EventLevel};
use rauthy_common::error_response::ErrorResponse;
use tracing::{info, warn};

pub struct EventListener;

impl EventListener {
    pub async fn listen(rx: flume::Receiver<Event>) -> Result<(), ErrorResponse> {
        while let Ok(event) = rx.recv_async().await {
            match event.level {
                EventLevel::Info | EventLevel::Notice => info!("{}", event),
                EventLevel::Warning => warn!("{}", event),
                EventLevel::Critical => warn!("{}", event),
            }
        }

        Ok(())
    }
}
