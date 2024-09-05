use rauthy_models::app_state::DbPool;
use rauthy_models::cache::DB;
use std::time::Duration;
use time::OffsetDateTime;
use tracing::{debug, error};

// Cleans up old / expired / already used Refresh Tokens
pub async fn refresh_tokens_cleanup(db: DbPool) {
    let mut interval = tokio::time::interval(Duration::from_secs(3600 * 3));

    loop {
        interval.tick().await;

        if !DB::client().is_leader_cache().await {
            debug!("Running HA mode without being the leader - skipping refresh_tokens_cleanup scheduler");
            continue;
        }

        debug!("Running refresh_tokens_cleanup scheduler");

        let now = OffsetDateTime::now_utc().unix_timestamp();
        let res = sqlx::query("delete from refresh_tokens where exp < $1")
            .bind(now)
            .execute(&db)
            .await;

        match res {
            Ok(_) => {}
            Err(err) => error!("Refresh Token Cleanup Error: {:?}", err),
        }
    }
}
