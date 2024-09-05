use crate::sleep_schedule_next;
use actix_web::web;
use rauthy_models::app_state::AppState;
use rauthy_models::cache::DB;
use rauthy_models::email::send_pwd_reset_info;
use rauthy_models::entity::users::User;
use std::ops::Add;
use std::str::FromStr;
use time::OffsetDateTime;
use tracing::{debug, error};

/// Checks soon expiring passwords and notifies the user accordingly.
/// Runs once every night at 04:30.
pub async fn password_expiry_checker(data: web::Data<AppState>) {
    // sec min hour day_of_month month day_of_week year
    let schedule = cron::Schedule::from_str("0 30 4 * * * *").unwrap();

    loop {
        sleep_schedule_next(&schedule).await;

        if !DB::client().is_leader_cache().await {
            debug!("Running HA mode without being the leader - skipping password_expiry_checker scheduler");
            continue;
        }

        debug!("Running password_expiry_checker scheduler");

        // warns, if the duration until the expiry is between 9 and 10 days, to only warn once
        let now = OffsetDateTime::now_utc();
        let lower = now.add(::time::Duration::days(9)).unix_timestamp();
        let upper = now.add(::time::Duration::days(10)).unix_timestamp();

        match User::find_all(&data).await {
            Ok(users) => {
                let users_to_notify = users
                    .into_iter()
                    .filter(|user| {
                        if let Some(exp) = user.password_expires {
                            exp <= upper && exp > lower
                        } else {
                            false
                        }
                    })
                    .collect::<Vec<User>>();

                for user in users_to_notify {
                    send_pwd_reset_info(&data, &user).await;
                    debug!("User {} notified about password expiry", user.email);
                }
            }

            Err(err) => {
                error!("password_expiry_checker error: {}", err.message);
            }
        };
    }
}
