use actix_web::web;
use rauthy_models::app_state::AppState;
use redhac::{CacheMethod, CacheNotify};
use tokio::sync::mpsc;
use tracing::debug;

// TODO remove after redhac migration has been completed
pub async fn handle_notify(_data: web::Data<AppState>, mut rx: mpsc::Receiver<CacheNotify>) {
    while let Some(msg) = rx.recv().await {
        match msg.method {
            CacheMethod::Put => {
                debug!(
                    "Remote push to the cache for '{}/{}'",
                    msg.cache_name, msg.entry
                );
            }

            CacheMethod::Del => {
                debug!(
                    "Remote delete from the cache for '{}/{}'",
                    msg.cache_name, msg.entry
                );
            }

            _ => {}
        }
    }
}
