use crate::api_cookie::ApiCookie;
use crate::entity::auth_providers::{AuthProvider, AuthProviderTemplate};
use crate::entity::magic_links::{MagicLink, MagicLinkUsage};
use crate::entity::password::PasswordPolicy;
use crate::entity::sessions::Session;
use crate::entity::users::User;
use crate::html::templates::{FrontendAction, HtmlTemplate, TplPasswordReset};
use crate::rauthy_config::RauthyConfig;
use actix_web::cookie::Cookie;
use chrono::Utc;
use rauthy_api_types::generic::PasswordPolicyResponse;
use rauthy_common::constants::{PROVIDER_ATPROTO, PWD_RESET_COOKIE};
use rauthy_common::utils::get_rand;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::cmp::max;
use tracing::warn;

impl HtmlTemplate {
    /// This function is only used during local dev to async resolve template values that are
    /// rendered into the HTML directly in prod.
    pub async fn build_from_str(
        s: &str,
        session: Option<Session>,
    ) -> Result<(Self, Option<Cookie<'_>>), ErrorResponse> {
        match s {
            "tpl_admin_btn_hide" => Ok((
                Self::AdminButtonHide(RauthyConfig::get().vars.access.admin_button_hide),
                None,
            )),
            "tpl_atproto_id" => {
                if RauthyConfig::get().vars.atproto.enable {
                    let provider = AuthProvider::find_by_iss(PROVIDER_ATPROTO.to_string()).await?;
                    Ok((Self::AtprotoId(provider.id), None))
                } else {
                    Err(ErrorResponse::new(
                        ErrorResponseType::NotFound,
                        "atproto disabled",
                    ))
                }
            }
            "tpl_auth_providers" => {
                let json = AuthProviderTemplate::get_all_json_template().await?;
                Ok((Self::AuthProviders(json), None))
            }
            "tpl_client_logo_updated" => Ok((
                Self::ClientLogoUpdated(Some(Utc::now().timestamp_millis())),
                None,
            )),
            "tpl_csrf_token" => {
                if let Some(s) = session {
                    Ok((Self::CsrfToken(s.csrf_token), None))
                } else {
                    Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "no session",
                    ))
                }
            }
            "tpl_device_user_code_length" => Ok((
                Self::DeviceUserCodeLength(max(
                    RauthyConfig::get().vars.device_grant.user_code_length,
                    255,
                ) as u8),
                None,
            )),
            "tpl_email_old" => Ok((Self::EmailOld("OLD@EMAIL.LOCAL".to_string()), None)),
            "tpl_email_new" => Ok((Self::EmailOld("NEW@EMAIL.LOCAL".to_string()), None)),
            "tpl_is_reg_open" => Ok((
                Self::IsRegOpen(RauthyConfig::get().vars.user_registration.enable),
                None,
            )),
            // the LoginAction requires a complex logic + validation.
            // Simply always return None during local dev.
            "tpl_login_action" => Ok((Self::LoginAction(FrontendAction::None), None)),
            // "tpl_client_name" => todo!("extract info from referrer?"),
            // "tpl_client_url" => todo!("extract info from referrer?"),
            "tpl_restricted_email_domain" => Ok((
                Self::RestrictedEmailDomain(
                    RauthyConfig::get()
                        .vars
                        .user_registration
                        .domain_restriction
                        .clone()
                        .unwrap_or_default(),
                ),
                None,
            )),
            "tpl_password_reset" => {
                // To have a good DX when working on the password reset, we want to create a new,
                // valid magic link automatically in DEV mode and return proper values directly.
                // At a valid session should have been created before to make this work.
                warn!(
                    r#"

    The password reset form during local development does not behave like you would expect it to in
    production! To make working on this form less annoying and skip the magic link request / send
    / fetch from mails each time, you need to have a valid session for this form (during local
    development only of course).
    The information from the session will be extracted and a fresh magic link will be generated with
    each template fetch (reload of the page). This makes it possible to work on that form without
    the whole annoy E-Mail flow.

    This means it will always reset the password for the user with the current session, not any other
    user from a possibly existing magic link, for development speed.

                "#
                );

                let user_id = session
                    .expect(
                        "To make the tpl_password_reset work automatically in local dev, \
                    you need to be logged in",
                    )
                    .user_id
                    .clone()
                    .expect(
                        "To make the tpl_password_reset work automatically in local dev, \
                    you need to be logged in",
                    );
                let user = User::find(user_id).await?;

                MagicLink::delete_all_pwd_reset_for_user(user.id.clone()).await?;
                let usage = if user.password.is_none() && !user.has_webauthn_enabled() {
                    MagicLinkUsage::NewUser(None)
                } else {
                    MagicLinkUsage::PasswordReset(None)
                };
                let mut ml = MagicLink::create(
                    user.id.clone(),
                    RauthyConfig::get().vars.lifetimes.magic_link_pwd_reset as i64,
                    usage,
                )
                .await?;
                let cookie_val = get_rand(48);
                ml.cookie = Some(cookie_val);
                ml.save().await?;

                let password_policy = PasswordPolicy::find().await?;
                let tpl = TplPasswordReset {
                    csrf_token: ml.csrf_token.clone(),
                    magic_link_id: ml.id.clone(),
                    needs_mfa: user.has_webauthn_enabled(),
                    password_policy: PasswordPolicyResponse::from(password_policy),
                    user_id: user.id,
                };

                let max_age_secs = ml.exp - Utc::now().timestamp();
                let cookie = ApiCookie::build(PWD_RESET_COOKIE, ml.cookie.unwrap(), max_age_secs);

                Ok((Self::PasswordReset(tpl), Some(cookie)))
            }
            "tpl_user_values_config" => Ok((Self::UserValues, None)),
            _ => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "invalid template id",
            )),
        }
    }
}
