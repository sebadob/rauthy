use crate::sleep_schedule_next;
use actix_web::web;
use chrono::Utc;
use rauthy_models::app_state::AppState;
use rauthy_models::email::send_pwd_reset_info;
use rauthy_models::entity::users::User;
use rauthy_models::hiqlite::DB;
use std::ops::Add;
use std::str::FromStr;
use tracing::{debug, error};

/// Checks soon expiring passwords and notifies the user accordingly.
/// Runs once every night at 04:30.
pub async fn password_expiry_checker(data: web::Data<AppState>) {
    // sec min hour day_of_month month day_of_week year
    // TODO convert to interval that runs multiple times a day + keep track of sent warnings?
    let schedule = cron::Schedule::from_str("0 30 4 * * * *").unwrap();

    loop {
        sleep_schedule_next(&schedule).await;

        if !DB::client().is_leader_cache().await {
            debug!("Running HA mode without being the leader - skipping password_expiry_checker scheduler");
            continue;
        }

        debug!("Running password_expiry_checker scheduler");

        // warns, if the duration until the expiry is between 9 and 10 days, to only warn once
        // TODO we could keep track about sent warnings in an additional DB, in case the deployment
        // is offline for some longer time
        let now = Utc::now();
        let lower = now.add(chrono::Duration::days(9)).timestamp();
        let upper = now.add(chrono::Duration::days(10)).timestamp();

        match User::find_all(&data).await {
            Ok(users) => {
                // TODO convert into proper query directly after hiqlite migration
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
