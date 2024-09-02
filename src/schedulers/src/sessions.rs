use rauthy_models::app_state::DbPool;
use rauthy_models::cache::DB;
use std::ops::Sub;
use std::time::Duration;
use time::OffsetDateTime;
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

        let thres = OffsetDateTime::now_utc()
            .sub(::time::Duration::hours(24))
            .unix_timestamp();

        let res = sqlx::query("delete from sessions where exp < $1")
            .bind(thres)
            .execute(&db)
            .await;

        match res {
            Ok(_) => {}
            Err(err) => error!("Session Cleanup Error: {:?}", err),
        }
    }
}
