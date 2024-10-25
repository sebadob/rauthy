use chrono::Utc;
use hiqlite::{params, Param};
use rauthy_common::is_hiqlite;
use rauthy_models::app_state::DbPool;
use rauthy_models::hiqlite::DB;
use std::time::Duration;
use tracing::{debug, error};

pub async fn refresh_tokens_cleanup(db: DbPool) {
    let mut interval = tokio::time::interval(Duration::from_secs(3600 * 3));

    loop {
        interval.tick().await;

        if !DB::client().is_leader_cache().await {
            debug!("Running HA mode without being the leader - skipping refresh_tokens_cleanup scheduler");
            continue;
        }

        debug!("Running refresh_tokens_cleanup scheduler");

        let now = Utc::now().timestamp();

        if is_hiqlite() {
            if let Err(err) = DB::client()
                .execute("DELETE FROM refresh_tokens WHERE exp < $1", params!(now))
                .await
            {
                error!("Refresh Token Cleanup Error: {:?}", err)
            }
        }
        if let Err(err) = sqlx::query("DELETE FROM refresh_tokens WHERE exp < $1")
            .bind(now)
            .execute(&db)
            .await
        {
            error!("Refresh Token Cleanup Error: {:?}", err)
        }
    }
}
