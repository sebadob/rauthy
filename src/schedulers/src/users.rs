use actix_web::web;
use chrono::Utc;
use rauthy_error::ErrorResponse;
use rauthy_models::app_state::AppState;
use rauthy_models::database::DB;
use rauthy_models::entity::clients_scim::ClientScim;
use rauthy_models::entity::refresh_tokens::RefreshToken;
use rauthy_models::entity::sessions::Session;
use rauthy_models::entity::users::User;
use rauthy_service::oidc::logout;
use std::env;
use std::time::Duration;
use tracing::{debug, error, info};

pub async fn user_expiry_checker(data: web::Data<AppState>) {
    let secs = env::var("SCHED_USER_EXP_MINS")
        .as_deref()
        .unwrap_or("60")
        .parse::<u64>()
        .expect("Cannot parse 'SCHED_USER_EXP_MINS' to u64");
    let mut interval = tokio::time::interval(Duration::from_secs(secs * 60));
    let cleanup_after_secs = env::var("SCHED_USER_EXP_DELETE_MINS")
        .map(|s| {
            s.parse::<u64>()
                .expect("Cannot parse 'SCHED_USER_EXP_DELETE_MINS' to u64")
                * 60
        })
        .ok();
    if cleanup_after_secs.is_none() {
        info!("Auto cleanup for expired users disabled");
    }

    loop {
        interval.tick().await;

        if !DB::hql().is_leader_cache().await {
            debug!(
                "Running HA mode without being the leader - skipping user_expiry_checker scheduler"
            );
            continue;
        }

        debug!("Running user_expiry_checker scheduler");
        if let Err(err) = execute(&data, cleanup_after_secs).await {
            error!("Error during user_expiry_checker: {}", err.message);
        }
    }
}

async fn execute(
    data: &web::Data<AppState>,
    cleanup_after_secs: Option<u64>,
) -> Result<(), ErrorResponse> {
    let now = Utc::now().timestamp();
    for mut user in User::find_expired().await? {
        debug!("Found expired user {}: {}", user.id, user.email);

        let exp_ts = if let Some(ts) = user.user_expires {
            if now < ts {
                error!(
                    "Got not yet expired user in user_expiry_checker - this should never happen"
                );
                continue;
            }
            ts
        } else {
            error!("Got non-expiring user in user_expiry_checker - this should never happen");
            continue;
        };

        user.enabled = false;
        user.save(None).await?;

        Session::invalidate_for_user(&user.id).await?;
        RefreshToken::invalidate_for_user(&user.id).await?;
        logout::execute_backchannel_logout(data, None, Some(user.id.clone())).await?;

        // possibly auto-cleanup expired user
        if let Some(secs) = cleanup_after_secs {
            let expired_since_secs = (exp_ts - now).unsigned_abs();
            if expired_since_secs > secs {
                info!(
                    "Auto cleanup for user {} after being expired for {} minutes",
                    user.id,
                    expired_since_secs / 60
                );
                if let Err(err) = user.delete().await {
                    error!(
                        "Error during auto cleanup - deleting user {}: {:?}",
                        user.id, err
                    );
                }
            }
        }

        ClientScim::create_update_user(user).await?;
    }

    Ok(())
}
