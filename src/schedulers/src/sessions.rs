use chrono::Utc;
use hiqlite::{Param, params};
use rauthy_common::is_hiqlite;
use rauthy_models::database::DB;
use std::ops::Sub;
use std::time::Duration;
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

        if is_hiqlite() {
            if let Err(err) = DB::hql()
                .execute("DELETE FROM sessions WHERE exp < $1", params!(thres))
                .await
            {
                error!("Session Cleanup Error: {:?}", err)
            }
        } else if let Err(err) = sqlx::query("DELETE FROM sessions WHERE exp < $1")
            .bind(thres)
            .execute(DB::conn_sqlx())
            .await
        {
            error!("Session Cleanup Error: {:?}", err)
        }
    }
}
