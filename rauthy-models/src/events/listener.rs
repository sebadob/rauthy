use crate::app_state::DbPool;
use crate::email::EMail;
use crate::events::event::Event;
use crate::events::notifier::EventNotifier;
use actix_web_lab::sse;
use rauthy_common::constants::{DATABASE_URL, DB_TYPE, EVENTS_LATEST_LIMIT};
use rauthy_common::error_response::ErrorResponse;
use rauthy_common::DbType;
use sqlx::postgres::PgListener;
use std::collections::{HashMap, VecDeque};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;
use tracing::{debug, error, info};

#[derive(Debug, Clone)]
pub enum EventRouterMsg {
    Event(String),
    ClientReg {
        ip: String,
        tx: sse::Sender,
        latest: Option<u16>,
    },
}

pub struct EventListener;

impl EventListener {
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn listen(
        tx_email: mpsc::Sender<EMail>,
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
        tokio::spawn(Self::router(db.clone(), rx_router));

        while let Ok(event) = rx_event.recv_async().await {
            if is_ha {
                tokio::spawn(Self::handle_event_ha(tx_email.clone(), event, db.clone()));
            } else {
                tokio::spawn(Self::handle_event_si(
                    tx_email.clone(),
                    event,
                    db.clone(),
                    tx_router.clone(),
                ));
            }
        }

        Ok(())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn handle_event_si(
        tx_email: mpsc::Sender<EMail>,
        event: Event,
        db: DbPool,
        tx: flume::Sender<EventRouterMsg>,
    ) {
        // insert into DB
        while let Err(err) = event.insert(&db).await {
            error!("Inserting Event into Database: {:?}", err);
            time::sleep(Duration::from_secs(1)).await;
        }

        // forward to event router
        if let Err(err) = tx.send_async(EventRouterMsg::Event(event.as_json())).await {
            error!(
                "Error sending Event {:?} internally - this should never happen!",
                err
            );
        }

        // send notification
        while let Err(err) = EventNotifier::send(&tx_email, &event).await {
            error!("Sending Event Notification: {:?}", err);
            time::sleep(Duration::from_secs(1)).await;
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn handle_event_ha(tx_email: mpsc::Sender<EMail>, event: Event, db: DbPool) {
        // insert into DB
        while let Err(err) = event.insert(&db).await {
            error!("Inserting Event into Database: {:?}", err);
            time::sleep(Duration::from_secs(1)).await;
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

        // send notification
        while let Err(err) = EventNotifier::send(&tx_email, &event).await {
            error!("Sending Event Notification: {:?}", err);
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
    async fn router(db: DbPool, rx: flume::Receiver<EventRouterMsg>) {
        debug!("EventListener::router_si has been started");

        let mut clients: HashMap<String, sse::Sender> = HashMap::with_capacity(4);
        let mut ips_to_remove = Vec::with_capacity(1);
        // returns the latest events ordered by timestamp desc
        let mut events = Event::find_latest(&db, EVENTS_LATEST_LIMIT as i64)
            .await
            .unwrap_or_default()
            .into_iter()
            .rev()
            .map(|e| sse::Data::new(e.as_json()))
            .collect::<VecDeque<sse::Data>>();

        while let Ok(msg) = rx.recv_async().await {
            match msg {
                EventRouterMsg::Event(event) => {
                    debug!(
                        "received new event in EventListener::router_si: {:?}",
                        event
                    );

                    let payload = sse::Data::new(event);
                    if events.len() > EVENTS_LATEST_LIMIT as usize {
                        events.pop_front();
                    }
                    events.push_back(payload.clone());

                    for (ip, tx) in &clients {
                        match time::timeout(Duration::from_secs(5), tx.send(payload.clone())).await
                        {
                            Ok(tx_res) => {
                                if let Err(err) = tx_res {
                                    error!(
                                        "sending event to client {} from event listener - removing client\n{:?}",
                                        ip, err
                                    );
                                    ips_to_remove.push(ip.clone());
                                }
                            }
                            Err(_) => {
                                error!(
                                    "Timeout reached sending event to client {} - removing client",
                                    ip
                                );
                                ips_to_remove.push(ip.clone());
                            }
                        }
                    }

                    while let Some(ip) = ips_to_remove.pop() {
                        clients.remove(&ip);
                    }
                }

                EventRouterMsg::ClientReg { ip, tx, latest } => {
                    info!("New client {} registered for the event listener", ip);

                    let mut is_err = false;
                    if let Some(latest) = latest {
                        let l = latest as usize;
                        let evt_len = events.len();
                        let skip = if l > evt_len { 0 } else { evt_len - l };

                        for event in events.iter().skip(skip) {
                            match time::timeout(Duration::from_secs(5), tx.send(event.clone()))
                                .await
                            {
                                Ok(tx_res) => {
                                    if let Err(err) = tx_res {
                                        error!(
                                        "sending latest event to client {} after ClientReg - removing client\n{:?}",
                                        ip, err
                                    );
                                        is_err = true;
                                        break;
                                    }
                                }
                                Err(_) => {
                                    error!(
                                    "Timeout reached sending latest events to client {} - removing client",
                                    ip
                                );
                                    is_err = true;
                                    break;
                                }
                            }
                        }
                    }

                    if !is_err {
                        clients.insert(ip, tx);
                    }
                }
            }
        }

        panic!("tx for EventRouterMsg has been closed - this should never happen!");
    }
}
