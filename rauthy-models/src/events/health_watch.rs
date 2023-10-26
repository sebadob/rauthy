use crate::app_state::DbPool;
use crate::entity::is_db_alive;
use crate::events::event::Event;
use redhac::{QuorumHealth, QuorumHealthState, QuorumState};
use std::time::Duration;
use tokio::sync::watch;
use tracing::debug;

pub async fn watch_health(
    db: DbPool,
    tx_events: flume::Sender<Event>,
    rx_cache: watch::Receiver<Option<QuorumHealthState>>,
) {
    debug!("Rauthy health watcher started");

    let mut interval = tokio::time::interval(Duration::from_secs(30));
    let mut was_healthy_after_startup = false;
    let mut last_state = false;

    loop {
        interval.tick().await;

        let hs = rx_cache.borrow().clone();
        let cache_healthy = match hs {
            // non-HA cache is always healthy
            None => true,

            Some(hs) => {
                if is_cache_bad(&hs) {
                    // wait for a few seconds and try again before alerting
                    tokio::time::sleep(Duration::from_secs(10)).await;

                    // cannot be None anymore at this point
                    let hs = rx_cache.borrow().clone().unwrap();
                    if is_cache_bad(&hs) && was_healthy_after_startup {
                        tx_events
                            .send_async(Event::rauthy_unhealthy_cache())
                            .await
                            .unwrap();
                        false
                    } else {
                        true
                    }
                } else {
                    true
                }
            }
        };

        let db_healthy = if !is_db_alive(&db).await {
            // wait for a few seconds and try again before alerting
            tokio::time::sleep(Duration::from_secs(10)).await;

            // do not send
            if !is_db_alive(&db).await && was_healthy_after_startup {
                tx_events
                    .send_async(Event::rauthy_unhealthy_db())
                    .await
                    .unwrap();
                false
            } else {
                true
            }
        } else {
            true
        };

        let is_good_now = db_healthy && cache_healthy;
        if !was_healthy_after_startup && is_good_now {
            was_healthy_after_startup = true;
        }

        if is_good_now && is_good_now != last_state {
            // let only the cache leader send healthy message in HA deployment
            tx_events.send_async(Event::rauthy_healthy()).await.unwrap();
        }

        last_state = is_good_now;
    }
}

fn is_cache_bad(hs: &QuorumHealthState) -> bool {
    hs.health == QuorumHealth::Bad
        || hs.state == QuorumState::Undefined
        || hs.state == QuorumState::Retry
}
