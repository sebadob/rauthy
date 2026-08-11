use crate::database::DB;
use crate::events::event::{Event, EventLevel};
use crate::events::notifier::EventNotifier;
use crate::rauthy_config::RauthyConfig;
use actix_web_lab::sse;
use rauthy_common::constants::EVENTS_LATEST_LIMIT;
use rauthy_error::ErrorResponse;
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;
use tokio::sync::mpsc;
use tokio::time;
use tracing::{debug, error, info, warn};

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

        // Cap the number of concurrently processed events so an event storm cannot grow
        // the spawned-task fan-out without limit. Once the permits are exhausted the
        // consumer stops dequeuing, the bounded channel fills up, and producers drop
        // events with a warning (see Event::send) instead of growing memory.
        let sem = Arc::new(Semaphore::new(256));
        while let Ok(event) = rx_event.recv_async().await {
            let sem = Arc::clone(&sem);
            let permit = match sem.acquire_owned().await {
                Ok(permit) => permit,
                // the semaphore is only closed at shutdown - treat as end of stream
                Err(_) => break,
            };
            tokio::spawn(async move {
                let _permit = permit;
                Self::handle_event(event).await;
            });
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
            // best-effort: drop (with a warning) if the bounded router queue is saturated,
            // so the HA raft listener never blocks on a full queue
            if let Err(err) = tx.try_send(EventRouterMsg::Event(event)) {
                warn!(
                    ?err,
                    "Event router queue full - dropping event from raft listener",
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
                    if events.len() > EVENTS_LATEST_LIMIT as usize
                        && let Some((_, id, _)) = events.pop_front()
                    {
                        event_ids.remove(&id);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::event::EventType;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn test_event() -> Event {
        Event::new(EventLevel::Info, EventType::InvalidLogins, None, None, None)
    }

    // Covers the consumer-loop change (acquire-before-spawn with a permit semaphore): the
    // number of concurrently processed events is structurally capped at the permit count,
    // so an event storm cannot grow the spawned-task fan-out without limit. The assertion
    // is structural (semaphore), not timing-based; the wait is a bounded completion wait.
    #[tokio::test]
    async fn test_event_consumer_caps_concurrency() {
        let (tx, rx) = flume::bounded::<Event>(2);
        for _ in 0..2 {
            let _ = tx.try_send(test_event());
        }
        drop(tx);

        let sem = Arc::new(Semaphore::new(1));
        let in_flight = Arc::new(AtomicUsize::new(0));
        let max_in_flight = Arc::new(AtomicUsize::new(0));
        let (done_tx, done_rx) = flume::unbounded::<()>();

        let handle = tokio::spawn({
            let rx = rx;
            let sem = Arc::clone(&sem);
            let in_flight = Arc::clone(&in_flight);
            let max = Arc::clone(&max_in_flight);
            let done_tx = done_tx;
            async move {
                while let Ok(event) = rx.recv_async().await {
                    let sem = Arc::clone(&sem);
                    let in_flight = Arc::clone(&in_flight);
                    let max = Arc::clone(&max);
                    let done_tx = done_tx.clone();
                    let permit = sem.acquire_owned().await.unwrap();
                    tokio::spawn(async move {
                        let _permit = permit;
                        let _ = event;
                        let now = in_flight.fetch_add(1, Ordering::SeqCst) + 1;
                        max.fetch_max(now, Ordering::SeqCst);
                        in_flight.fetch_sub(1, Ordering::SeqCst);
                        let _ = done_tx.send(());
                    });
                }
            }
        });

        let _ = tokio::time::timeout(Duration::from_secs(5), async {
            for _ in 0..2 {
                let _ = done_rx.recv_async().await;
            }
        })
        .await
        .expect("both events should be processed");

        assert!(max_in_flight.load(Ordering::SeqCst) <= 1);
        handle.abort();
    }
}
