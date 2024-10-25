use hiqlite::{params, Param};
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use rauthy_models::app_state::DbPool;
use rauthy_models::hiqlite::DB;
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
        if is_hiqlite() {
            if let Err(err) = cleanup_hiqlite(exp).await {
                error!("{:?}", err);
            }
        } else if let Err(err) = cleanup_sqlx(&db, exp).await {
            error!("{:?}", err);
        }
    }
}

async fn cleanup_hiqlite(exp: i64) -> Result<(), ErrorResponse> {
    let rows_affected = DB::client()
        .execute(
            r#"
DELETE FROM users
WHERE id IN (
    SELECT DISTINCT user_id
    FROM magic_links
    WHERE exp < 1683003398 AND used = false)
AND password IS NULL"#,
            params!(exp),
        )
        .await?;
    debug!(
        "Cleaned up {} users which did not use their initial password reset magic link",
        rows_affected
    );

    // now we can just delete all expired magic links
    let rows_affected = DB::client()
        .execute("DELETE FROM magic_links WHERE exp < $1", params!(exp))
        .await?;
    debug!("Cleaned up {} expired and used magic links", rows_affected);

    Ok(())
}

async fn cleanup_sqlx(db: &DbPool, exp: i64) -> Result<(), ErrorResponse> {
    let res = sqlx::query(
        r#"
DELETE FROM users
WHERE id IN (
    SELECT DISTINCT user_id
    FROM magic_links
    WHERE exp < 1683003398 AND used = false)
AND password IS NULL"#,
    )
    .bind(exp)
    .execute(db)
    .await?;
    debug!(
        "Cleaned up {} users which did not use their initial password reset magic link",
        res.rows_affected()
    );

    // now we can just delete all expired magic links
    let res = sqlx::query("DELETE FROM magic_links WHERE exp < $1")
        .bind(exp)
        .execute(db)
        .await?;
    debug!(
        "Cleaned up {} expired and used magic links",
        res.rows_affected()
    );

    Ok(())
}
