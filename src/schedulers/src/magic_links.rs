use chrono::Utc;
use hiqlite::macros::params;
use rauthy_common::is_hiqlite;
use rauthy_data::database::DB;
use rauthy_data::entity::user_login_states::UserLoginState;
use rauthy_data::entity::users::User;
use rauthy_error::{ErrorResponse, ErrorResponseType};
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
            error!(error = ?err, "magic_link_cleanup");
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

    let res = if is_hiqlite() {
        DB::hql().execute(sql, params!(exp)).await.map_err(|err| {
            ErrorResponse::new(
                ErrorResponseType::Database,
                format!("Error during Magic Link Cleanup: {err}"),
            )
        })
    } else {
        DB::pg_execute(sql, &[&exp]).await.map_err(|err| {
            ErrorResponse::new(
                ErrorResponseType::Database,
                format!("Error during Magic Link Cleanup: {err}"),
            )
        })
    };

    match res {
        Ok(rows_affected) => {
            if rows_affected > 0 {
                info!(
                    "Cleaned up {rows_affected} users which did not use their initial password reset magic link"
                );
                for _ in 0..rows_affected {
                    User::count_dec().await?;
                }

                // now we can just delete all expired magic links
                let sql = "DELETE FROM magic_links WHERE exp < $1";
                let rows_affected = if is_hiqlite() {
                    DB::hql().execute(sql, params!(exp)).await?
                } else {
                    DB::pg_execute(sql, &[&exp]).await?
                };
                debug!("Cleaned up {rows_affected} expired and used magic links");
            }
        }
        Err(err) => {
            // This should never happen, but we have this catch for better debugging.
            // There was a reported issue that this query can fail, but I was not able to reproduce it.
            // If we ever get here, we want to log some more information to try to debug it in a live
            // env.
            error!("{err}");

            let sql = r#"
SELECT * FROM users
WHERE id IN (
    SELECT DISTINCT user_id
    FROM magic_links
    WHERE exp < $1 AND used = false)
AND password IS NULL AND webauthn_user_id IS NULL"#;

            let users: Vec<User> = if is_hiqlite() {
                DB::hql().query_as(sql, params!(exp)).await?
            } else {
                DB::pg_query(sql, &[&exp], 1).await?
            };

            let mut states = Vec::new();
            for user in &users {
                let login_states = UserLoginState::find_by_user(user.id.clone()).await?;
                states.push(login_states);
            }

            error!(
                r#"

This Magic Link cleanup error should never happen. This is additional debugging
information. It was not possible to cleanup the following users. Please confirm
manually that they are actually fresh, un-initiliazed users that never logged in.
The only table restricting this action should only ever contain data when a user
logged in at least once. In this case though, this query should also never catch
such users.

{users:?}


Did you do anything special / manually for the listed users? Please report at:

https://github.com/sebadob/rauthy/issues/1532

"#
            );

            if !states.is_empty() {
                error!(
                    r#"
Additional debugging information about user magic link cleanup error. The following
login states were found:

{states:?}


This should never happen. There should not be any login states for users without
a password and no passkey. Did you do anything special / manually for the listed
users? Please report at:

https://github.com/sebadob/rauthy/issues/1532

                "#
                );
            }
        }
    }

    Ok(())
}
