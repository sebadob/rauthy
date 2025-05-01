use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::constants::SESSION_LIFETIME;
use rauthy_common::is_hiqlite;
use rauthy_models::database::DB;
use std::time::Duration;
use tracing::{debug, error};

pub async fn user_login_states_cleanup() {
    let mut interval = tokio::time::interval(Duration::from_secs(3600 * 8));

    loop {
        interval.tick().await;

        if !DB::hql().is_leader_cache().await {
            debug!(
                "Running HA mode without being the leader - skipping user_login_states_cleanup scheduler"
            );
            continue;
        }

        debug!("Running user_login_states_cleanup scheduler");

        // It makes no sense to keep login states around for longer than the maximum allowed
        // total lifetime for sessions. Adds 1 day of grace time already.
        let threshold = Utc::now().timestamp() - *SESSION_LIFETIME as i64 - 3600 * 24;
        let sql = "DELETE FROM user_login_states WHERE timestamp < $1";

        if is_hiqlite() {
            if let Err(err) = DB::hql().execute(sql, params!(threshold)).await {
                error!("User Login State Cleanup Error: {:?}", err)
            }
        } else if let Err(err) = DB::pg_execute(sql, &[&threshold]).await {
            error!("User Login State Cleanup Error: {:?}", err)
        }
    }
}
