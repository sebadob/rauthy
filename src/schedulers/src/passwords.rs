use chrono::Utc;
use hiqlite::macros::params;
use rauthy_common::is_hiqlite;
use rauthy_data::database::DB;
use rauthy_data::email::password_reset_info::send_pwd_reset_info;
use rauthy_data::entity::users::User;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::ErrorResponse;
use std::ops::{Add, Sub};
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, warn};

/// Checks soon expiring passwords and notifies the user accordingly.
/// Runs once every night at 04:30.
pub async fn password_expiry_checker() {
    time::sleep(Duration::from_secs(5)).await;
    let mut interval = time::interval(Duration::from_secs(4 * 3600));

    loop {
        interval.tick().await;

        if !DB::hql().is_leader_cache().await {
            debug!(
                "Running HA mode without being the leader - skipping password_expiry_checker scheduler"
            );
            continue;
        }

        tracing::warn!("Running password_expiry_checker scheduler");

        if let Err(err) = execute().await {
            error!("Error running password_expiry_checker scheduler: {:?}", err);
        }
    }
}

async fn execute() -> Result<(), ErrorResponse> {
    let now = Utc::now();
    let days = RauthyConfig::get().vars.email.password_exp_days;

    let past = now.sub(chrono::Duration::days(days as i64)).timestamp();
    let fut = now.add(chrono::Duration::days(days as i64)).timestamp();

    // This query also needs to look into the past just in case we missed
    // a cleanup for `pwd_exp_mails` somewhere.
    let sql = r#"SELECT * FROM users
WHERE password_expires <= $1 AND NOT EXISTS (
    SELECT 1 FROM pwd_exp_mails WHERE user_id = id AND mail_sent_ts > $2
)"#;

    let expiring_users: Result<Vec<User>, ErrorResponse> = if is_hiqlite() {
        DB::hql()
            .query_as(sql, params!(fut, past))
            .await
            .map_err(ErrorResponse::from)
    } else {
        DB::pg_query(sql, &[&fut, &past], 0).await
    };

    match expiring_users {
        Ok(users_to_notify) => {
            warn!("Users about to expire: {:?}", users_to_notify);
            for user in users_to_notify {
                send_pwd_reset_info(user).await;
            }
        }

        Err(err) => {
            error!("password_expiry_checker error: {}", err.message);
        }
    };

    Ok(())
}
