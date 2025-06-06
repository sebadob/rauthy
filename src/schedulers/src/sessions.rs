use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_models::database::DB;
use std::ops::Sub;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error};

/// Cleans up old / expired Sessions.
///
/// This does NOT clean up / delete any `user_login_states`. These do not have an FK to `sessions.id`
/// on purpose. This makes it possible to keep them around for longer without wasting too much
/// storage while still being able to target specific `session_id`s if a logout comes in at a later
/// point, instead of logging out everything for the user. This is a big UX improvement in these
/// situations.
pub async fn sessions_cleanup() {
    let mut interval = tokio::time::interval(Duration::from_secs(3595 * 2));

    loop {
        interval.tick().await;

        if !DB::hql().is_leader_cache().await {
            debug!(
                "Running HA mode without being the leader - skipping sessions_cleanup scheduler"
            );
            continue;
        }

        debug!("Running sessions_cleanup scheduler");

        let thres = Utc::now().sub(chrono::Duration::hours(24)).timestamp();
        let sql = "DELETE FROM sessions WHERE exp < $1";

        if is_hiqlite() {
            if let Err(err) = DB::hql().execute(sql, params!(thres)).await {
                error!("Session Cleanup Error: {:?}", err)
            }
        } else if let Err(err) = DB::pg_execute(sql, &[&thres]).await {
            error!("Session Cleanup Error: {:?}", err)
        }

        // For some reason, the interval could `.tick()` multiple times,
        // if it finished too quickly.
        time::sleep(Duration::from_secs(3)).await;
    }
}
