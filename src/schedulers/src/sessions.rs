use chrono::Utc;
use hiqlite::{params, Param};
use rauthy_common::is_hiqlite;
use rauthy_models::app_state::DbPool;
use rauthy_models::database::DB;
use std::ops::Sub;
use std::time::Duration;
use tracing::{debug, error};

// Cleans up old / expired Sessions
pub async fn sessions_cleanup(db: DbPool) {
    let mut interval = tokio::time::interval(Duration::from_secs(3595 * 2));

    loop {
        interval.tick().await;

        if !DB::client().is_leader_cache().await {
            debug!(
                "Running HA mode without being the leader - skipping sessions_cleanup scheduler"
            );
            continue;
        }

        debug!("Running sessions_cleanup scheduler");

        let thres = Utc::now().sub(chrono::Duration::hours(24)).timestamp();

        if is_hiqlite() {
            if let Err(err) = DB::client()
                .execute("DELETE FROM sessions WHERE exp < $1", params!(thres))
                .await
            {
                error!("Session Cleanup Error: {:?}", err)
            }
        } else if let Err(err) = sqlx::query("DELETE FROM sessions WHERE exp < $1")
            .bind(thres)
            .execute(&db)
            .await
        {
            error!("Session Cleanup Error: {:?}", err)
        }
    }
}
