use chrono::Utc;
use hiqlite::{Param, params};
use rauthy_common::is_hiqlite;
use rauthy_models::database::DB;
use std::time::Duration;
use tracing::{debug, error};

pub async fn refresh_tokens_cleanup() {
    let mut interval = tokio::time::interval(Duration::from_secs(3600 * 3));

    loop {
        interval.tick().await;

        if !DB::hql().is_leader_cache().await {
            debug!(
                "Running HA mode without being the leader - skipping refresh_tokens_cleanup scheduler"
            );
            continue;
        }

        debug!("Running refresh_tokens_cleanup scheduler");

        let now = Utc::now().timestamp();
        let sql = "DELETE FROM refresh_tokens WHERE exp < $1";

        if is_hiqlite() {
            if let Err(err) = DB::hql().execute(sql, params!(now)).await {
                error!("Refresh Token Cleanup Error: {:?}", err)
            }
        } else if let Err(err) = DB::pg_execute(sql, &[&now]).await {
            error!("Refresh Token Cleanup Error: {:?}", err)
        }
    }
}
