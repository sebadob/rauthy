use crate::database::{Cache, DB};
use crate::entity::users::{AccountType, User};
use crate::entity::webauthn::passkey::PasskeyEntity;
use crate::rauthy_config::RauthyConfig;
use rauthy_api_types::users::{WebauthnRegFinishRequest, WebauthnRegStartRequest};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::{error, info, warn};
use webauthn_rs::prelude::{Credential, PasskeyRegistration, Uuid};
use webauthn_rs_proto::{
    AuthenticatorSelectionCriteria, CreationChallengeResponse, ExtnState, ResidentKeyRequirement,
    UserVerificationPolicy,
};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WebauthnReg {
    pub user_id: String,
    pub passkey_user_id: Uuid,
    pub reg_state: String,
}

pub async fn reg_start(
    user_id: String,
    payload: WebauthnRegStartRequest,
) -> Result<CreationChallengeResponse, ErrorResponse> {
    let user = User::find(user_id).await?;
    let passkey_user_id = if let Some(id) = &user.webauthn_user_id {
        Uuid::from_str(id).expect("corrupted database: user.webauthn_user_id")
    } else {
        Uuid::new_v4()
    };
    let exclude_creds = {
        let pks = PasskeyEntity::find_cred_ids_for_user(&user.id).await?;
        if !pks.is_empty() { Some(pks) } else { None }
    };

    match RauthyConfig::get().webauthn.start_passkey_registration(
        passkey_user_id,
        &user.email,
        &user.email,
        exclude_creds,
    ) {
        Ok((mut ccr, reg_state)) => {
            // timeout expected in ms
            let cfg = &RauthyConfig::get().vars.webauthn;
            ccr.public_key.timeout = Some(cfg.req_exp as u32 * 1000);

            // Any values we overwrite manually here may differ in the `reg_state`. They are hidden
            // from us, any we may double-check and enforce ourselves in the finish step if
            // necessary.
            // However, we cannot use the `ResidentKeyRequirement` in combination with
            // `start_attested_resident_key_registration()`, because it enforces attestation as
            // well, which we don't want right now.

            // The cross-platform is only a hint, not enforced, but it may break some browsers.
            // TODO check on devices providing an internal option if they still
            //  show the internal choice if there is no cross platform attached.
            // let authenticator_attachment = Some(AuthenticatorAttachment::CrossPlatform);
            let authenticator_attachment = None;
            let resident_key = if payload.allow_rk.unwrap_or(false) {
                Some(ResidentKeyRequirement::Preferred)
            } else {
                Some(ResidentKeyRequirement::Discouraged)
            };
            let user_verification = if cfg.force_uv || user.account_type() == AccountType::Passkey {
                UserVerificationPolicy::Required
            } else {
                UserVerificationPolicy::Preferred
            };
            ccr.public_key.authenticator_selection = Some(AuthenticatorSelectionCriteria {
                authenticator_attachment,
                resident_key,
                require_resident_key: false,
                user_verification,
            });

            let cache_idx = format!("reg_{:?}_{}", payload.passkey_name, user.id);
            let reg_data = WebauthnReg {
                user_id: user.id,
                passkey_user_id,
                // the reg_state cannot be serialized with bincode -> missing deserialize from Any
                reg_state: serde_json::to_string(&reg_state)?,
            };
            DB::hql()
                .put(
                    Cache::Webauthn,
                    cache_idx,
                    &reg_data,
                    Some(cfg.req_exp as i64),
                )
                .await?;

            Ok(ccr)
        }

        Err(err) => {
            error!(?err, "Webauthn challenge register");
            Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "Internal error with Webauthn Challenge Registration",
            ))
        }
    }
}

pub async fn reg_finish(
    id: String,
    payload: WebauthnRegFinishRequest,
    is_new_user: bool,
) -> Result<(), ErrorResponse> {
    let mut user = User::find(id).await?;

    let idx = format!("reg_{:?}_{}", payload.passkey_name, user.id);
    let reg_data = match DB::hql()
        .get_remove::<_, _, WebauthnReg>(Cache::Webauthn, idx)
        .await?
    {
        None => {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Webauthn Registration Request not found",
            ));
        }
        Some(data) => data,
    };

    let reg_state = serde_json::from_str::<PasskeyRegistration>(&reg_data.reg_state)?;
    match RauthyConfig::get()
        .webauthn
        .finish_passkey_registration(&payload.data, &reg_state)
    {
        Ok(pk) => {
            // force UV check
            let cfg = &RauthyConfig::get().vars.webauthn;
            let cred = Credential::from(pk.clone());

            if (user.account_type() != AccountType::Password || cfg.force_uv) && !cred.user_verified
            {
                warn!(
                    user.id,
                    "Webauthn Registration Ceremony without User Verification",
                );
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "User Presence only is not allowed - Verification is needed",
                ));
            }

            let user_id = user.id.clone();
            let create_user = if user.webauthn_user_id.is_none() {
                user.webauthn_user_id = Some(reg_data.passkey_user_id.to_string());

                // We need to check if the user is a possibly manually initialized one, and we
                // need to reset a password. This can happen when an admin adds a user, does a
                // manual init to prevent auto-removal, and then the initial password reset link
                // is used to add a passkey instead. In such a situation, the user would have a
                // random password but never logged in even once, and the Magic Link Usage is
                // NewUser.
                if is_new_user && user.password.is_some() && user.last_login.is_none() {
                    info!(
                        "Resetting manually initialized password for user {}",
                        user.email
                    );
                    user.password = None;
                    user.save(None).await?;
                }

                if user.password.is_none() || cfg.no_password_exp {
                    user.password_expires = None;
                }
                Some(user)
            } else {
                None
            };

            let is_rk = {
                match cred.extensions.cred_props {
                    ExtnState::NotRequested | ExtnState::Ignored => None,
                    ExtnState::Set(props) => props.rk,
                    ExtnState::Unsolicited(props) => props.rk,
                    ExtnState::Unsigned(props) => props.rk,
                }
            };

            PasskeyEntity::create(
                user_id.clone(),
                create_user,
                reg_data.passkey_user_id,
                payload.passkey_name,
                pk,
                cred.user_verified,
                is_rk,
            )
            .await?;

            info!(user_id, "New PasskeyEntity saved successfully");
        }
        Err(err) => {
            error!(?err, "Webauthn Reg Finish");
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("{err}"),
            ));
        }
    };

    Ok(())
}
