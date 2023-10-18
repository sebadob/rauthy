use crate::app_state::DbPool;
use crate::events::event::{Event, EventLevel};
use actix_web_lab::sse;
use rauthy_common::constants::HA_MODE;
use rauthy_common::error_response::ErrorResponse;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone)]
pub enum EventRouterMsg {
    Event(Event),
    ClientReg { ip: String, tx: sse::Sender },
}

pub struct EventListener;

// TODO we could even get rid of the HA functions for the SQLite binary to have it a tiny bit smaller
impl EventListener {
    pub async fn listen(
        tx_router: flume::Sender<EventRouterMsg>,
        rx_router: flume::Receiver<EventRouterMsg>,
        rx_event: flume::Receiver<Event>,
        db: DbPool,
    ) -> Result<(), ErrorResponse> {
        debug!("EventListener::listen has been started");

        if *HA_MODE {
            tokio::spawn(Self::router_ha(rx_router, db.clone()));
        } else {
            tokio::spawn(Self::router_si(rx_router));
        };

        while let Ok(event) = rx_event.recv_async().await {
            if *HA_MODE {
                tokio::spawn(Self::handle_event_ha(event, db.clone()));
            } else {
                tokio::spawn(Self::handle_event_si(event, db.clone(), tx_router.clone()));
            }
        }

        Ok(())
    }

    async fn handle_event_si(event: Event, db: DbPool, tx: flume::Sender<EventRouterMsg>) {
        // insert into DB
        while let Err(err) = event.insert(&db).await {
            error!("Inserting Event into Database: {:?}", err);
            time::sleep(Duration::from_secs(1)).await;
        }

        // log event
        match event.level {
            EventLevel::Info | EventLevel::Notice => info!("{}", event),
            EventLevel::Warning => warn!("{}", event),
            EventLevel::Critical => warn!("{}", event),
        }

        // forward to event router
        if let Err(err) = tx.send_async(EventRouterMsg::Event(event)).await {
            error!(
                "Error sending Event {:?} internally - this should never happen!",
                err
            );
        }
    }

    async fn handle_event_ha(event: Event, db: DbPool) {
        // insert into DB
        while let Err(err) = event.insert(&db).await {
            error!("Inserting Event into Database: {:?}", err);
            time::sleep(Duration::from_secs(1)).await;
        }

        // log event
        match event.level {
            EventLevel::Info | EventLevel::Notice => info!("{}", event),
            EventLevel::Warning => warn!("{}", event),
            EventLevel::Critical => warn!("{}", event),
        }

        // forward to postgres listener
        todo!("handle_event_ha")
    }

    async fn router_si(rx: flume::Receiver<EventRouterMsg>) {
        debug!("EventListener::router_si has been started");

        let mut clients: HashMap<String, sse::Sender> = HashMap::with_capacity(4);
        let mut ips_to_remove = Vec::with_capacity(1);

        while let Ok(msg) = rx.recv_async().await {
            match msg {
                EventRouterMsg::Event(event) => {
                    debug!(
                        "received new event in EventListener::router_si: {:?}",
                        event
                    );
                    let payload = sse::Data::new_json(event).expect("serializing Event");

                    for (ip, tx) in &clients {
                        if let Err(err) = tx.send(payload.clone()).await {
                            error!(
                                "sending event to client {} from event listener - removing client\n{:?}",
                                ip, err
                            );
                            ips_to_remove.push(ip.clone());
                        }
                    }

                    while let Some(ip) = ips_to_remove.pop() {
                        clients.remove(&ip);
                    }
                }

                EventRouterMsg::ClientReg { ip, tx } => {
                    info!("New client {} registered for the event listener", ip);
                    clients.insert(ip, tx);
                }
            }
        }

        panic!("tx for EventRouterMsg has been closed - this should never happen!");
    }

    async fn router_ha(rx: flume::Receiver<EventRouterMsg>, db: DbPool) {
        debug!("EventListener::router_ha has been started");
        todo!("router_ha");
    }
}
