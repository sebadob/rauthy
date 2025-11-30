use chrono::Utc;
use rauthy_data::database::DB;
use rauthy_data::entity::clients_scim::ClientScim;
use rauthy_data::entity::pam::authorized_keys::AuthorizedKey;
use rauthy_data::entity::pam::users::PamUser;
use rauthy_data::entity::refresh_tokens::RefreshToken;
use rauthy_data::entity::sessions::Session;
use rauthy_data::entity::users::User;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::ErrorResponse;
use rauthy_service::oidc::logout;
use std::time::Duration;
use tokio::time;
use tracing::{debug, error, info};

pub async fn user_expiry_checker() {
    let secs = RauthyConfig::get().vars.database.sched_user_exp_mins as u64;
    let mut interval = tokio::time::interval(Duration::from_secs(secs * 60));
    let cleanup_after_secs = RauthyConfig::get()
        .vars
        .database
        .sched_user_exp_delete_mins
        .map(|s| s as u64 * 60);
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
        if let Err(err) = execute(cleanup_after_secs).await {
            error!("Error during user_expiry_checker: {}", err.message);
        }

        // For some reason, the interval could `.tick()` multiple times,
        // if it finished too quickly.
        time::sleep(Duration::from_secs(3)).await;
    }
}

async fn execute(cleanup_after_secs: Option<u64>) -> Result<(), ErrorResponse> {
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
        logout::execute_backchannel_logout(None, Some(user.id.clone())).await?;

        // expire possibly existing SSH keys
        if let Ok(pam_user) = PamUser::find_by_user_id(user.id).await {
            AuthorizedKey::expire_all_keys_by_uid(pam_user.id).await?;
        }

        // possibly auto-cleanup expired user
        if let Some(secs) = cleanup_after_secs {
            let expired_since_secs = (exp_ts - now).unsigned_abs();
            if expired_since_secs > secs {
                info!(
                    user.id,
                    "Auto cleanup for user after being expired for {} minutes",
                    expired_since_secs / 60,
                );
                if let Err(err) = user.delete().await {
                    error!(user.id, ?err, "auto cleanup - deleting user",);
                }
            }
        }

        ClientScim::create_update_user(user).await?;
    }

    Ok(())
}
