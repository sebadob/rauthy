use crate::app_state::DbPool;
use crate::events::event::{Event, EventLevel};
use actix_web_lab::sse;
use rauthy_common::constants::{DATABASE_URL, DB_TYPE};
use rauthy_common::error_response::ErrorResponse;
use rauthy_common::DbType;
use sqlx::postgres::PgListener;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info, warn};

#[derive(Debug, Clone)]
pub enum EventRouterMsg {
    Event(String),
    ClientReg { ip: String, tx: sse::Sender },
}

pub struct EventListener;

impl EventListener {
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn listen(
        tx_router: flume::Sender<EventRouterMsg>,
        rx_router: flume::Receiver<EventRouterMsg>,
        rx_event: flume::Receiver<Event>,
        db: DbPool,
    ) -> Result<(), ErrorResponse> {
        debug!("EventListener::listen has been started");

        // having a local copy is a tiny bit faster and needs one less memory lookup
        // let is_ ha = *HA_MODE;
        // TODO remove after enough testing -> only use postgres in HA_MODE
        let is_ha = *DB_TYPE == DbType::Postgres;

        if is_ha {
            tokio::spawn(Self::pg_listener(tx_router.clone()));
        }
        tokio::spawn(Self::router(rx_router));

        while let Ok(event) = rx_event.recv_async().await {
            if is_ha {
                tokio::spawn(Self::handle_event_ha(event, db.clone()));
            } else {
                tokio::spawn(Self::handle_event_si(event, db.clone(), tx_router.clone()));
            }
        }

        Ok(())
    }

    #[tracing::instrument(level = "debug", skip_all)]
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
        if let Err(err) = tx.send_async(EventRouterMsg::Event(event.as_json())).await {
            error!(
                "Error sending Event {:?} internally - this should never happen!",
                err
            );
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
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

        // notify postgres listeners
        while let Err(err) = sqlx::query(
            r#"SELECT pg_notify(chan, payload)
            FROM (VALUES ('events', $1)) NOTIFIES(chan, payload)"#,
        )
        .bind(event.as_json())
        .execute(&db)
        .await
        {
            error!("Publishing Event on Postgres channel: {:?}", err);
            time::sleep(Duration::from_secs(1)).await;
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn pg_listener(tx: flume::Sender<EventRouterMsg>) {
        debug!("EventListener::router_ha has been started");

        loop {
            let mut listener = match PgListener::connect(&DATABASE_URL).await {
                Ok(l) => l,
                Err(err) => {
                    error!(
                        "Error opening Postgres connection for PgListener: {:?}",
                        err
                    );
                    time::sleep(Duration::from_secs(5)).await;
                    continue;
                }
            };
            if let Err(err) = listener.listen("events").await {
                error!("Error listening to 'events' channel on Postgres: {:?}", err);
                time::sleep(Duration::from_secs(5)).await;
                continue;
            }

            while let Ok(msg) = listener.recv().await {
                debug!("{:?}", msg);

                // forward to event router -> payload is already an Event in JSON format
                if let Err(err) = tx
                    .send_async(EventRouterMsg::Event(msg.payload().to_string()))
                    .await
                {
                    error!(
                        "Error sending Event {:?} internally - this should never happen!",
                        err
                    );
                }
            }

            // try to do an unlisten on all channels even if we had an error and should be
            // disconnected anyway
            let _ = listener.unlisten_all().await;
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn router(rx: flume::Receiver<EventRouterMsg>) {
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
                    let payload = sse::Data::new(event);

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
}
