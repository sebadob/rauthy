use crate::app_state::DbPool;
use crate::email::EMail;
use crate::events::event::{Event, EventLevel, EventType};
use crate::events::ip_blacklist_handler::{IpBlacklist, IpBlacklistReq, IpLoginFailedSet};
use crate::events::notifier::EventNotifier;
use crate::events::EVENT_PERSIST_LEVEL;
use actix_web_lab::sse;
use chrono::DateTime;
use rauthy_common::constants::HA_MODE;
use rauthy_common::constants::{DATABASE_URL, EVENTS_LATEST_LIMIT};
use rauthy_common::error_response::ErrorResponse;
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
        level: EventLevel,
    },
}

pub struct EventListener;

impl EventListener {
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn listen(
        tx_email: mpsc::Sender<EMail>,
        tx_ip_blacklist: flume::Sender<IpBlacklistReq>,
        tx_router: flume::Sender<EventRouterMsg>,
        rx_router: flume::Receiver<EventRouterMsg>,
        rx_event: flume::Receiver<Event>,
        db: DbPool,
    ) -> Result<(), ErrorResponse> {
        debug!("EventListener::listen has been started");

        // having a local copy is a tiny bit faster and needs one less memory lookup
        let is_ha = *HA_MODE;

        if is_ha {
            tokio::spawn(Self::pg_listener(tx_router.clone()));
        }
        tokio::spawn(Self::router(db.clone(), rx_router, tx_ip_blacklist));

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
        if &event.level.value() >= EVENT_PERSIST_LEVEL.get().unwrap() {
            while let Err(err) = event.insert(&db).await {
                error!("Inserting Event into Database: {:?}", err);
                time::sleep(Duration::from_secs(1)).await;
            }
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
        if &event.level.value() >= EVENT_PERSIST_LEVEL.get().unwrap() {
            while let Err(err) = event.insert(&db).await {
                error!("Inserting Event into Database: {:?}", err);
                time::sleep(Duration::from_secs(1)).await;
            }
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
    async fn router(
        db: DbPool,
        rx: flume::Receiver<EventRouterMsg>,
        tx_ip_blacklist: flume::Sender<IpBlacklistReq>,
    ) {
        debug!("EventListener::router_si has been started");

        let mut clients: HashMap<String, (i16, sse::Sender)> = HashMap::with_capacity(4);
        let mut ips_to_remove = Vec::with_capacity(1);
        // Event::find_latest returns the latest events ordered by timestamp desc
        let mut events = Event::find_latest(&db, EVENTS_LATEST_LIMIT as i64)
            .await
            .unwrap_or_default()
            .into_iter()
            .rev()
            .map(|e| (e.level.value(), sse::Data::new(e.as_json())))
            .collect::<VecDeque<(i16, sse::Data)>>();

        while let Ok(msg) = rx.recv_async().await {
            match msg {
                EventRouterMsg::Event(event) => {
                    debug!("received new event in EventListener::router: {:?}", event);

                    // deserialize the event and check for important updates
                    let evt = serde_json::from_str::<Event>(&event)
                        .expect("Event to deserialize correctly");
                    match evt.typ {
                        EventType::InvalidLogins => {
                            tx_ip_blacklist
                                .send_async(IpBlacklistReq::LoginFailedSet(IpLoginFailedSet {
                                    ip: evt.ip.unwrap_or_default(),
                                    invalid_logins: evt.data.unwrap_or_default() as u32,
                                }))
                                .await
                                .unwrap();
                        }
                        EventType::IpBlacklisted => {
                            tx_ip_blacklist
                                .send_async(IpBlacklistReq::Blacklist(IpBlacklist {
                                    ip: evt.ip.unwrap_or_default(),
                                    exp: DateTime::from_timestamp(evt.data.unwrap(), 0)
                                        .unwrap_or_default(),
                                }))
                                .await
                                .unwrap();
                        }
                        EventType::IpBlacklistRemoved => {
                            tx_ip_blacklist
                                .send_async(IpBlacklistReq::BlacklistDelete(
                                    evt.ip.unwrap_or_default(),
                                ))
                                .await
                                .unwrap();
                        }
                        EventType::JwksRotated => {}
                        EventType::NewUserRegistered => {}
                        EventType::NewRauthyAdmin => {}
                        EventType::NewRauthyVersion => {}
                        EventType::PossibleBruteForce => {}
                        EventType::RauthyStarted => {}
                        EventType::RauthyHealthy => {}
                        EventType::RauthyUnhealthy => {}
                        EventType::SecretsMigrated => {}
                        EventType::UserEmailChange => {}
                        EventType::Test => {}
                    }

                    // pre-compute the payload
                    // the incoming data is already in JSON format
                    let payload = sse::Data::new(event);
                    let event_level_value = evt.level.value();

                    // send payload to all clients
                    for (ip, (client_level, tx)) in &clients {
                        if *client_level > event_level_value {
                            // skip the event if the client does not want to receive its level
                            continue;
                        }

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

                    // keep current events max size and push payload
                    if events.len() > EVENTS_LATEST_LIMIT as usize {
                        events.pop_front();
                    }
                    events.push_back((event_level_value, payload));

                    while let Some(ip) = ips_to_remove.pop() {
                        clients.remove(&ip);
                    }
                }

                EventRouterMsg::ClientReg {
                    ip,
                    tx,
                    latest,
                    level,
                } => {
                    info!("New client {} registered for the event listener", ip);
                    let client_level_val = level.value();

                    let mut is_err = false;
                    if let Some(latest) = latest {
                        let latest = latest as usize;

                        let events_filtered = events
                            .iter()
                            .filter(|(level, _payload)| *level >= client_level_val)
                            .map(|(_level, payload)| payload)
                            .collect::<Vec<&sse::Data>>();

                        let evt_len = events_filtered.len();
                        let skip = if latest > evt_len {
                            0
                        } else {
                            evt_len - latest
                        };

                        for event in events_filtered.iter().skip(skip) {
                            match time::timeout(Duration::from_secs(5), tx.send((*event).clone()))
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
                        clients.insert(ip, (client_level_val, tx));
                    }
                }
            }
        }

        panic!("tx for EventRouterMsg has been closed - this should never happen!");
    }
}
