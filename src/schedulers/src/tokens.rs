use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_data::database::DB;
use std::time::Duration;
use tokio::time;
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
                error!(?err, "Refresh Token Cleanup")
            }
        } else if let Err(err) = DB::pg_execute(sql, &[&now]).await {
            error!(?err, "Refresh Token Cleanup")
        }

        // For some reason, the interval could `.tick()` multiple times,
        // if it finished too quickly.
        time::sleep(Duration::from_secs(3)).await;
    }
}
