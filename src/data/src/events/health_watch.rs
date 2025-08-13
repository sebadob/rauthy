use crate::database::DB;
use crate::entity::is_db_alive;
use crate::events::event::Event;
use crate::rauthy_config::RauthyConfig;
use std::time::Duration;
use tracing::debug;

pub async fn watch_health() {
    debug!("Rauthy health watcher started");

    let mut interval = tokio::time::interval(Duration::from_secs(30));
    let mut was_healthy_after_startup = false;
    let mut last_state = false;
    let tx_events = RauthyConfig::get().tx_events.clone();

    loop {
        interval.tick().await;

        let cache_healthy = DB::hql().is_healthy_cache().await.is_ok();

        let db_healthy = if !is_db_alive().await {
            // wait for a few seconds and try again before alerting
            tokio::time::sleep(Duration::from_secs(10)).await;

            // do not send
            if !is_db_alive().await && was_healthy_after_startup {
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

        if is_good_now && !last_state {
            // let only the cache leader send healthy message in HA deployment
            tx_events.send_async(Event::rauthy_healthy()).await.unwrap();
        }

        last_state = is_good_now;
    }
}
