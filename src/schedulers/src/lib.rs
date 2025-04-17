use actix_web::web;
use rauthy_models::app_state::AppState;
use std::time::Duration;
use tokio::time;
use tracing::info;

mod app_version;
mod backchannel_logout;
mod devices;
mod dyn_clients;
mod events;
mod jwks;
mod magic_links;
mod passwords;
mod scim_tasks;
mod sessions;
mod tokens;
mod user_login_states;
mod users;

/// Spawn all Rauthy schedulers and periodic tasks
pub async fn spawn(data: web::Data<AppState>) {
    info!("Starting schedulers");

    tokio::spawn(backchannel_logout::backchannel_logout_retry(data.clone()));
    tokio::spawn(scim_tasks::scim_task_retry(data.clone()));
    tokio::spawn(dyn_clients::dyn_client_cleanup());
    tokio::spawn(events::events_cleanup());
    tokio::spawn(devices::devices_cleanup());
    tokio::spawn(magic_links::magic_link_cleanup());
    tokio::spawn(tokens::refresh_tokens_cleanup());
    tokio::spawn(user_login_states::user_login_states_cleanup());
    tokio::spawn(sessions::sessions_cleanup());
    tokio::spawn(jwks::jwks_auto_rotate(data.clone()));
    tokio::spawn(jwks::jwks_cleanup());
    tokio::spawn(passwords::password_expiry_checker(data.clone()));
    tokio::spawn(users::user_expiry_checker(data.clone()));
    tokio::spawn(app_version::app_version_check(data));
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
