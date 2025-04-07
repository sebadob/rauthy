use crate::sleep_schedule_next;
use actix_web::web;
use chrono::Utc;
use hiqlite::{Param, params};
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use rauthy_models::app_state::AppState;
use rauthy_models::database::DB;
use rauthy_models::email::send_pwd_reset_info;
use rauthy_models::entity::users::User;
use std::ops::Add;
use std::str::FromStr;
use tracing::{debug, error};

/// Checks soon expiring passwords and notifies the user accordingly.
/// Runs once every night at 04:30.
pub async fn password_expiry_checker(data: web::Data<AppState>) {
    // sec min hour day_of_month month day_of_week year
    // TODO convert to interval that runs multiple times a day + keep track of sent warnings?
    // -> would require persisting sent emails and other things though
    let schedule = cron::Schedule::from_str("0 30 4 * * * *").unwrap();

    loop {
        sleep_schedule_next(&schedule).await;

        if !DB::hql().is_leader_cache().await {
            debug!(
                "Running HA mode without being the leader - skipping password_expiry_checker scheduler"
            );
            continue;
        }

        debug!("Running password_expiry_checker scheduler");

        // warns, if the duration until the expiry is between 9 and 10 days, to only warn once
        // TODO we could keep track about sent warnings in an additional DB, in case the deployment
        // is offline for some longer time
        let now = Utc::now();
        let lower = now.add(chrono::Duration::days(9)).timestamp();
        let upper = now.add(chrono::Duration::days(10)).timestamp();

        let expiring_users: Result<Vec<User>, ErrorResponse> = if is_hiqlite() {
            DB::hql()
                .query_as(
                    "SELECT * FROM users WHERE password_expires <= $1 AND password_expires > $2",
                    params!(upper, lower),
                )
                .await
                .map_err(ErrorResponse::from)
        } else {
            sqlx::query_as!(
                User,
                "SELECT * FROM users WHERE password_expires <= $1 AND password_expires > $2",
                upper,
                lower
            )
            .fetch_all(DB::conn_sqlx())
            .await
            .map_err(ErrorResponse::from)
        };

        match expiring_users {
            Ok(users_to_notify) => {
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
