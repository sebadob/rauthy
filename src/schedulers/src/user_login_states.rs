use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_data::database::DB;
use rauthy_data::rauthy_config::RauthyConfig;
use std::time::Duration;
use tokio::time;
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
        let threshold = Utc::now().timestamp()
            - RauthyConfig::get().vars.lifetimes.session_lifetime as i64
            - 3600 * 24;
        let sql = "DELETE FROM user_login_states WHERE timestamp < $1";

        if is_hiqlite() {
            if let Err(err) = DB::hql().execute(sql, params!(threshold)).await {
                error!(?err, "User Login State Cleanup")
            }
        } else if let Err(err) = DB::pg_execute(sql, &[&threshold]).await {
            error!(?err, "User Login State Cleanup")
        }

        // For some reason, the interval could `.tick()` multiple times,
        // if it finished too quickly.
        time::sleep(Duration::from_secs(3)).await;
    }
}
