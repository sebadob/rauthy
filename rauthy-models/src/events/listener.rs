use crate::app_state::DbPool;
use crate::events::event::{Event, EventLevel};
use rauthy_common::error_response::ErrorResponse;
use std::time::Duration;
use tracing::{error, info, warn};

pub struct EventListener;

impl EventListener {
    pub async fn listen(rx: flume::Receiver<Event>, db: DbPool) -> Result<(), ErrorResponse> {
        while let Ok(event) = rx.recv_async().await {
            tokio::spawn(Self::handle_event(event, db.clone()));
        }

        Ok(())
    }

    async fn handle_event(event: Event, db: DbPool) {
        while let Err(err) = event.insert(&db).await {
            error!("Inserting Event into Database: {:?}", err);
            tokio::time::sleep(Duration::from_secs(1)).await;
        }

        match event.level {
            EventLevel::Info | EventLevel::Notice => info!("{}", event),
            EventLevel::Warning => warn!("{}", event),
            EventLevel::Critical => warn!("{}", event),
        }
    }
}
