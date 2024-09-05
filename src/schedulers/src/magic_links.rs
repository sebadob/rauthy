use rauthy_models::app_state::DbPool;
use rauthy_models::cache::DB;
use std::time::Duration;
use time::OffsetDateTime;
use tracing::{debug, error};

/// Cleans up old / expired magic links and deletes users, that have never used their
/// 'set first ever password' magic link to keep the database clean in case of an open user registration.
/// Runs every 6 hours.
pub async fn magic_link_cleanup(db: DbPool) {
    let mut interval = tokio::time::interval(Duration::from_secs(3600 * 6));

    loop {
        interval.tick().await;

        if !DB::client().is_leader_cache().await {
            debug!(
                "Running HA mode without being the leader - skipping magic_link_cleanup scheduler"
            );
            continue;
        }

        debug!("Running magic_link_cleanup scheduler");

        // allow 300 seconds of clock skew before cleaning up magic links
        let exp = OffsetDateTime::now_utc().unix_timestamp() - 300;

        // Check for expired and unused magic links that are bound to a user which has no password
        // at all. These users should be deleted since they never cared about the (very important)
        // password E-Mail.
        let res = sqlx::query(
            r#"delete from users where
            id in (select distinct user_id from magic_links where exp < 1683003398 and used = false)
            and password is null"#,
        )
        .bind(exp)
        .execute(&db)
        .await;
        match res {
            Ok(r) => {
                debug!(
                    "Cleaned up {} users which did not use their initial password reset magic link",
                    r.rows_affected()
                );
            }
            Err(err) => error!("Magic link / orphan users cleanup error: {:?}", err),
        }

        // now we can just delete all expired magic links
        let res = sqlx::query("delete from magic_links where exp < $1")
            .bind(exp)
            .execute(&db)
            .await;
        match res {
            Ok(r) => {
                debug!(
                    "Cleaned up {} expired and used magic links",
                    r.rows_affected()
                );
            }
            Err(err) => error!("Magic link cleanup error: {:?}", err),
        }
    }
}
