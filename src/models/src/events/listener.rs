use crate::database::DB;
use crate::events::EVENT_PERSIST_LEVEL;
use crate::events::event::{Event, EventLevel, EventType};
use crate::events::ip_blacklist_handler::{IpBlacklist, IpBlacklistReq, IpLoginFailedSet};
use crate::events::notifier::EventNotifier;
use actix_web_lab::sse;
use chrono::DateTime;
use rauthy_common::constants::EVENTS_LATEST_LIMIT;
use rauthy_error::ErrorResponse;
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;
use tracing::{debug, error, info};

#[derive(Debug, Clone)]
pub enum EventRouterMsg {
    Event(Event),
    ClientReg {
        ip: String,
        tx: mpsc::Sender<sse::Event>,
        latest: Option<u16>,
        level: EventLevel,
    },
}

pub struct EventListener;

impl EventListener {
    #[tracing::instrument(level = "debug", skip_all)]
    pub async fn listen(
        tx_ip_blacklist: flume::Sender<IpBlacklistReq>,
        tx_router: flume::Sender<EventRouterMsg>,
        rx_router: flume::Receiver<EventRouterMsg>,
        rx_event: flume::Receiver<Event>,
    ) -> Result<(), ErrorResponse> {
        debug!("EventListener::listen has been started");

        tokio::spawn(Self::router(rx_router, tx_ip_blacklist));
        tokio::spawn(Self::raft_events_listener(tx_router));

        while let Ok(event) = rx_event.recv_async().await {
            tokio::spawn(Self::handle_event(event));
        }

        Ok(())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn handle_event(event: Event) {
        // insert into DB
        if &event.level.value() >= EVENT_PERSIST_LEVEL.get().unwrap() {
            while let Err(err) = event.insert().await {
                error!("Inserting Event into Database: {:?}", err);
                time::sleep(Duration::from_secs(1)).await;
            }
        }

        // notify raft members
        let mut fails = 0;
        while let Err(err) = DB::hql().notify(&event).await {
            error!("Publishing Event on Postgres channel: {:?}", err);

            if fails > 10 {
                break;
            } else {
                fails += 1;
                time::sleep(Duration::from_secs(1)).await;
            }
        }

        // send notification
        while let Err(err) = EventNotifier::send(&event).await {
            error!("Sending Event Notification: {:?}", err);
            time::sleep(Duration::from_secs(1)).await;
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn raft_events_listener(tx: flume::Sender<EventRouterMsg>) {
        debug!("EventListener::router_ha has been started");

        while let Ok(event) = DB::hql().listen::<Event>().await {
            debug!("{:?}", event);

            // forward to event router -> payload is already an Event in JSON format
            if let Err(err) = tx.send_async(EventRouterMsg::Event(event)).await {
                error!(
                    "Error sending Event {:?} internally - this should never happen!",
                    err
                );
            }
        }

        info!("raft_events_listener exiting");
    }

    /// The router that will listen to Events coming in via Hiqlite and listen for Client
    /// Registrations via SSE endpoint. It will serialize incoming Events to SSE payload in JSON
    /// format and forward them to all registered clients.
    #[tracing::instrument(level = "debug", skip_all)]
    async fn router(
        rx: flume::Receiver<EventRouterMsg>,
        tx_ip_blacklist: flume::Sender<IpBlacklistReq>,
    ) {
        debug!("EventListener::router has been started");

        let mut clients: HashMap<String, (i16, mpsc::Sender<sse::Event>)> =
            HashMap::with_capacity(1);
        let mut ips_to_remove = Vec::with_capacity(1);

        // TODO in HA deployments (currently only seen with Postgres), we may have duplicate
        // event ids after startup, as soon as the first ever client subscribes to the events stream.
        // The HashSet makes sure we don't send out duplicate data, when we receive a duplicate
        // via `EventRouterMsg::Event(event)`.
        // -> investigate the reason to ultimately get rid of the additional HashSet checks.
        // -> does it maybe make sense to utilize the hiqlite cache here for unified data?
        let mut event_ids: HashSet<String> = HashSet::with_capacity(EVENTS_LATEST_LIMIT as usize);
        // Event::find_latest returns the latest events ordered by timestamp desc
        let mut events = Event::find_latest(EVENTS_LATEST_LIMIT as i64)
            .await
            .unwrap_or_default()
            .into_iter()
            .rev()
            .map(|e| {
                event_ids.insert(e.id.clone());
                (
                    e.level.value(),
                    e.id.clone(),
                    sse::Event::Data(sse::Data::new(e.as_json())),
                )
            })
            .collect::<VecDeque<(i16, String, sse::Event)>>();

        while let Ok(msg) = rx.recv_async().await {
            match msg {
                EventRouterMsg::Event(event) => {
                    debug!("received new event in EventListener::router: {:?}", event);

                    if event_ids.contains(&event.id) {
                        debug!("Duplicate event ID in router: {}", event.id);
                        continue;
                    }

                    let sse_payload =
                        sse::Event::Data(sse::Data::new(serde_json::to_string(&event).unwrap()));

                    // deserialize the event and check for important updates
                    match event.typ {
                        EventType::InvalidLogins => {
                            tx_ip_blacklist
                                .send_async(IpBlacklistReq::LoginFailedSet(IpLoginFailedSet {
                                    ip: event.ip.unwrap_or_default(),
                                    invalid_logins: event.data.unwrap_or_default() as u32,
                                }))
                                .await
                                .unwrap();
                        }
                        EventType::IpBlacklisted => {
                            tx_ip_blacklist
                                .send_async(IpBlacklistReq::Blacklist(IpBlacklist {
                                    ip: event.ip.unwrap_or_default(),
                                    exp: DateTime::from_timestamp(event.data.unwrap(), 0)
                                        .unwrap_or_default(),
                                }))
                                .await
                                .unwrap();
                        }
                        EventType::IpBlacklistRemoved => {
                            tx_ip_blacklist
                                .send_async(IpBlacklistReq::BlacklistDelete(
                                    event.ip.unwrap_or_default(),
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
                        EventType::UserPasswordReset => {}
                        EventType::Test => {}
                        EventType::BackchannelLogoutFailed => {}
                        EventType::ScimTaskFailed => {}
                    }

                    // pre-compute the payload
                    // the incoming data is already in JSON format
                    let event_level_value = event.level.value();

                    // send payload to all clients
                    for (ip, (client_level, tx)) in &clients {
                        if *client_level > event_level_value {
                            // skip the event if the client does not want to receive its level
                            continue;
                        }

                        match time::timeout(Duration::from_secs(5), tx.send(sse_payload.clone()))
                            .await
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
                        if let Some((_, id, _)) = events.pop_front() {
                            event_ids.remove(&id);
                        }
                    }
                    events.push_back((event_level_value, event.id.clone(), sse_payload));
                    event_ids.insert(event.id);

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
                            .filter(|(level, _id, _payload)| *level >= client_level_val)
                            .map(|(_level, _id, payload)| payload)
                            .collect::<Vec<&sse::Event>>();

                        let evt_len = events_filtered.len();
                        let skip = evt_len.saturating_sub(latest);

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
