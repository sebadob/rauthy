use chrono::Utc;
use rauthy_common::password_hasher::HashPassword;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::clients_dyn::ClientDyn;
use rauthy_data::entity::cred_stuff_detect::CredStuffDetect;
use rauthy_data::entity::users::User;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::ErrorResponse;
use std::net::IpAddr;
use tracing::{info, warn};
use zeroize::Zeroize;

/// The user side of a password login, shared by the `password` grant and LDAP binds.
/// The `client` must already be validated by the caller (enabled, secret, `password` flow).
/// `cred_stuff_detect = false` skips the per-IP credential stuffing detection.
pub async fn password_login(
    client: &Client,
    email: String,
    mut password: String,
    ip: IpAddr,
    user_failed_logins: &mut Option<i64>,
    cred_stuff_detect: bool,
) -> Result<User, ErrorResponse> {
    let mut user = match User::find_by_email(email.clone()).await {
        Ok(u) => u,
        Err(err) => {
            if cred_stuff_detect {
                CredStuffDetect::trigger(ip, &email, Some(&password)).await;
            }
            password.zeroize();
            return Err(err);
        }
    };

    match user.validate_password(password.clone()).await {
        Ok(_) => {
            // Very important to do these checks only AFTER a possibly existing password for the user was
            // validated to never leak data. The errors of these 2 checks are forwarded to the UI for better
            // UX, to inform the user about an issue with their account while their password was ok.
            // This means we do the work for hashing the password even for a disabled user, but it leaks
            // no data at all to someone who does not know the correct password.
            user.check_enabled()?;
            user.check_expired()?;

            client.validate_user_groups(&user)?;

            user.last_login = Some(Utc::now().timestamp());
            user.last_failed_login = None;
            user.failed_login_attempts = None;

            // check if the password hash should be upgraded
            let hash_uptodate = user.is_argon2_uptodate(&RauthyConfig::get().argon2_params)?;
            if !hash_uptodate {
                info!("Updating Argon2ID params for user '{}'", &user.email);
                let new_hash = HashPassword::hash_password(password).await?;
                user.password = Some(new_hash);
            } else {
                password.zeroize();
            }

            user.save(None).await?;

            if client.is_dynamic() {
                ClientDyn::update_used(&client.id).await?;
            }

            Ok(user)
        }
        Err(err) => {
            warn!(
                "False Login attempt from Host: '{}' for user: '{}'",
                ip, user.email
            );

            // The password grant receives username + password, so it is subject to credential
            // stuffing just like the authorize flow. Trigger the detection here as well.
            if cred_stuff_detect {
                CredStuffDetect::trigger(ip, &user.email, Some(&password)).await;
            }

            password.zeroize();

            user.last_failed_login = Some(Utc::now().timestamp());
            user.failed_login_attempts = Some(&user.failed_login_attempts.unwrap_or(0) + 1);
            user.save(None).await?;

            *user_failed_logins = user.failed_login_attempts;

            Err(err)
        }
    }
}
