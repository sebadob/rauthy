use chrono::Utc;
use hiqlite_macros::params;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use rauthy_models::database::DB;
use std::ops::Sub;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info};

/// Cleans up old / expired magic links and deletes users, that have never used their
/// 'set first ever password' magic link to keep the database clean in case of an open user registration.
/// Runs every 6 hours.
pub async fn magic_link_cleanup() {
    let mut interval = tokio::time::interval(Duration::from_secs(3600 * 6));

    loop {
        interval.tick().await;

        if !DB::hql().is_leader_cache().await {
            debug!(
                "Running HA mode without being the leader - skipping magic_link_cleanup scheduler"
            );
            continue;
        }

        debug!("Running magic_link_cleanup scheduler");

        // allow 300 seconds of clock skew before cleaning up magic links
        let exp = Utc::now().sub(chrono::Duration::seconds(300)).timestamp();

        // Check for expired and unused magic links that are bound to a user which has no password
        // at all. These users should be deleted since they never cared about the (very important)
        // password E-Mail.
        if let Err(err) = cleanup(exp).await {
            error!("{:?}", err);
        }

        // For some reason, the interval could `.tick()` multiple times,
        // if it finished too quickly.
        time::sleep(Duration::from_secs(3)).await;
    }
}

async fn cleanup(exp: i64) -> Result<(), ErrorResponse> {
    let sql = r#"
DELETE FROM users
WHERE id IN (
    SELECT DISTINCT user_id
    FROM magic_links
    WHERE exp < $1 AND used = false)
AND password IS NULL AND webauthn_user_id IS NULL"#;

    let rows_affected = if is_hiqlite() {
        DB::hql().execute(sql, params!(exp)).await?
    } else {
        DB::pg_execute(sql, &[&exp]).await?
    };

    info!(
        "Cleaned up {} users which did not use their initial password reset magic link",
        rows_affected
    );

    // now we can just delete all expired magic links
    let sql = "DELETE FROM magic_links WHERE exp < $1";
    let rows_affected = if is_hiqlite() {
        DB::hql().execute(sql, params!(exp)).await?
    } else {
        DB::pg_execute(sql, &[&exp]).await?
    };
    debug!("Cleaned up {} expired and used magic links", rows_affected);

    Ok(())
}
