use crate::database::DB;
use crate::events::event::{Event, EventLevel};
use crate::events::notifier::EventNotifier;
use crate::rauthy_config::RauthyConfig;
use actix_web_lab::sse;
use rauthy_common::constants::EVENTS_LATEST_LIMIT;
use rauthy_error::ErrorResponse;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
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
        tx_router: flume::Sender<EventRouterMsg>,
        rx_router: flume::Receiver<EventRouterMsg>,
        rx_event: flume::Receiver<Event>,
    ) -> Result<(), ErrorResponse> {
        debug!("EventListener::listen has been started");

        tokio::spawn(Self::router(rx_router));
        tokio::spawn(Self::raft_events_listener(tx_router));

        while let Ok(event) = rx_event.recv_async().await {
            tokio::spawn(Self::handle_event(event));
        }

        Ok(())
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn handle_event(event: Event) {
        // insert into DB
        if event.level.value() >= RauthyConfig::get().vars.events.persist_level.value() {
            while let Err(err) = event.insert().await {
                error!(?err, "Inserting Event into Database");
                time::sleep(Duration::from_secs(1)).await;
            }
        }

        // notify raft members
        let mut fails = 0;
        while let Err(err) = DB::hql().notify(&event).await {
            error!(?err, "Hiqlite::notify()");

            if fails > 10 {
                break;
            } else {
                fails += 1;
                time::sleep(Duration::from_secs(1)).await;
            }
        }

        // send notification
        while let Err(err) = EventNotifier::send(&event).await {
            error!(?err, "Sending Event Notification");
            time::sleep(Duration::from_secs(1)).await;
        }
    }

    #[tracing::instrument(level = "debug", skip_all)]
    async fn raft_events_listener(tx: flume::Sender<EventRouterMsg>) {
        debug!("EventListener::router_ha has been started");

        while let Ok(event) = DB::hql().listen::<Event>().await {
            debug!(?event);

            // forward to event router -> payload is already an Event in JSON format
            if let Err(err) = tx.send_async(EventRouterMsg::Event(event)).await {
                error!(
                    ?err,
                    "Error sending Event internally - this should never happen!",
                );
            }
        }

        info!("raft_events_listener exiting");
    }

    /// The router that will listen to Events coming in via Hiqlite and listen for Client
    /// Registrations via SSE endpoint. It will serialize incoming Events to SSE payload in JSON
    /// format and forward them to all registered clients.
    #[tracing::instrument(level = "debug", skip_all)]
    async fn router(rx: flume::Receiver<EventRouterMsg>) {
        debug!("EventListener::router has been started");

        let mut clients: BTreeMap<String, (i16, mpsc::Sender<sse::Event>)> = BTreeMap::new();
        let mut ips_to_remove = Vec::with_capacity(1);

        let mut event_ids: BTreeSet<String> = BTreeSet::new();
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
                    debug!(?event, "received new event in EventListener::router");

                    if event_ids.contains(&event.id) {
                        debug!("Duplicate event ID in router: {}", event.id);
                        continue;
                    }

                    // pre-compute the payload
                    // the incoming data is already in JSON format
                    let sse_payload =
                        sse::Event::Data(sse::Data::new(serde_json::to_string(&event).unwrap()));
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
                                        ?ip,
                                        ?err,
                                        "sending event to client from event listener - removing \
                                        client",
                                    );
                                    ips_to_remove.push(ip.clone());
                                }
                            }
                            Err(_) => {
                                error!(
                                    ?ip,
                                    "Timeout reached sending event to client - removing client",
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
                    info!(?ip, "New client registered for the event listener");
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
                                            ?ip,
                                            ?err,
                                            "sending latest event to client after ClientReg - \
                                            removing client",
                                        );
                                        is_err = true;
                                        break;
                                    }
                                }
                                Err(_) => {
                                    error!(
                                        ?ip,
                                        "Timeout reached sending latest events to client - removing \
                                        client",
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
