use actix_web::web;
use rauthy_models::app_state::AppState;
use rauthy_models::migration::s3_backup_init_test;
use redhac::{QuorumHealthState, QuorumState};
use std::time::Duration;
use tokio::sync::watch::Receiver;
use tokio::time;
use tracing::info;

mod app_version;
mod backup;
mod devices;
mod dyn_clients;
mod events;
mod jwks;
mod magic_links;
mod passwords;
mod sessions;
mod tokens;
mod users;

/// Spawn all Rauthy schedulers and periodic tasks
pub async fn spawn(data: web::Data<AppState>) {
    info!("Starting schedulers");

    let rx_health = data.caches.ha_cache_config.rx_health_state.clone();

    // initialize and possibly panic early if anything is mis-configured regarding the s3 storage
    s3_backup_init_test().await;

    tokio::spawn(backup::db_backup(data.db.clone()));
    tokio::spawn(dyn_clients::dyn_client_cleanup(
        data.clone(),
        rx_health.clone(),
    ));
    tokio::spawn(events::events_cleanup(data.db.clone(), rx_health.clone()));
    tokio::spawn(devices::devices_cleanup(data.db.clone(), rx_health.clone()));
    tokio::spawn(magic_links::magic_link_cleanup(
        data.db.clone(),
        rx_health.clone(),
    ));
    tokio::spawn(tokens::refresh_tokens_cleanup(
        data.db.clone(),
        rx_health.clone(),
    ));
    tokio::spawn(sessions::sessions_cleanup(
        data.db.clone(),
        rx_health.clone(),
    ));
    tokio::spawn(jwks::jwks_auto_rotate(data.clone(), rx_health.clone()));
    tokio::spawn(jwks::jwks_cleanup(data.clone(), rx_health.clone()));
    tokio::spawn(passwords::password_expiry_checker(
        data.clone(),
        rx_health.clone(),
    ));
    tokio::spawn(users::user_expiry_checker(data.clone(), rx_health.clone()));
    tokio::spawn(app_version::app_version_check(data, rx_health));
}

/// sleeps until the next scheduled event
async fn sleep_schedule_next(schedule: &cron::Schedule) {
    // this 10 sec sleep is done to prevent an overlap with the calculation in some cases
    time::sleep(Duration::from_secs(10)).await;

    let next = schedule.upcoming(chrono::Local).next().unwrap();
    let until = next.signed_duration_since(chrono::Local::now());

    // we are adding a future date here --> safe to cast from i64 to u64
    time::sleep(Duration::from_secs(until.num_seconds() as u64)).await;
}

fn is_ha_leader(rx_health: &Receiver<Option<QuorumHealthState>>) -> Option<bool> {
    let health_state = rx_health.borrow();
    health_state
        .as_ref()
        .map(|state| state.state == QuorumState::Leader)
}
