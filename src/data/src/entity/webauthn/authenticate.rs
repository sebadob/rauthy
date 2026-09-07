use crate::entity::auth_codes::AuthCodeToSAwait;
use crate::entity::browser_id::BrowserId;
use crate::entity::login_locations::LoginLocation;
use crate::entity::sessions::Session;
use crate::entity::users::{AccountType, User};
use crate::entity::webauthn::auth_data::{WebauthnAdditionalData, WebauthnData};
use crate::entity::webauthn::auth_req::{WebauthnLoginReq, WebauthnServiceReq};
use crate::entity::webauthn::authenticate_rk::auth_finish_discover;
use crate::entity::webauthn::passkey::PasskeyEntity;
use crate::rauthy_config::RauthyConfig;
use actix_web::HttpRequest;
use chrono::Utc;
use rauthy_api_types::users::{MfaPurpose, WebauthnAuthFinishRequest, WebauthnAuthStartResponse};
use rauthy_common::utils::get_rand;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
use utoipa::ToSchema;
use webauthn_rs::prelude::Passkey;
use webauthn_rs_proto::UserVerificationPolicy;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct WebauthnLoginToSAwaitCode {
    pub await_code: String,
    pub user_id: String,
    pub header_origin: Option<String>,
}

pub async fn auth_start(
    user_id: Option<String>,
    purpose: MfaPurpose,
) -> Result<WebauthnAuthStartResponse, ErrorResponse> {
    // This app_data will be returned to the client upon successful webauthn authentication
    let (add_data, user_id) = match purpose {
        MfaPurpose::Login(code) => {
            debug_assert!(user_id.is_none());
            let d = WebauthnLoginReq::find_remove(code).await?;
            let user_id = d.user_id.clone();
            (WebauthnAdditionalData::Login(d), user_id)
        }
        MfaPurpose::Discover => {
            todo!();
        }
        MfaPurpose::MfaModToken
        | MfaPurpose::PamLogin
        | MfaPurpose::PasswordNew
        | MfaPurpose::PasswordReset => {
            let user_id =
                user_id.expect("user_id should always exist for non-login webauthn starts");
            let svc_req = WebauthnServiceReq::new(user_id.clone());
            svc_req.save().await?;
            (WebauthnAdditionalData::Service(svc_req), user_id)
        }
        MfaPurpose::Test => {
            let user_id =
                user_id.expect("user_id should always exist for non-login webauthn starts");
            (WebauthnAdditionalData::Test(user_id.clone()), user_id)
        }
    };

    let user = User::find(user_id).await?;
    let force_uv =
        user.account_type() == AccountType::Passkey || RauthyConfig::get().vars.webauthn.force_uv;
    let pks = if force_uv {
        // in this case, filter out all presence only keys
        PasskeyEntity::find_for_user_with_uv(&user.id)
            .await?
            .iter()
            .map(|pk_entity| pk_entity.get_pk())
            .collect::<Vec<Passkey>>()
    } else {
        PasskeyEntity::find_for_user(&user.id)
            .await?
            .iter()
            .map(|pk_entity| pk_entity.get_pk())
            .collect::<Vec<Passkey>>()
    };

    if pks.is_empty() {
        // may be the case if the user has presence only keys and the config has changed
        return Err(ErrorResponse::new(
            ErrorResponseType::NotFound,
            "No Security Keys with active user verification found",
        ));
    }

    match RauthyConfig::get()
        .webauthn
        .start_passkey_authentication(pks.as_slice())
    {
        Ok((mut rcr, auth_state)) => {
            let req_exp = RauthyConfig::get().vars.webauthn.req_exp;
            // timeout expected in ms
            rcr.public_key.timeout = Some(req_exp as u32 * 1000);
            if force_uv {
                rcr.public_key.user_verification = UserVerificationPolicy::Required;
            }

            // cannot be serialized with bincode -> no deserialize from any
            let auth_state_json = serde_json::to_string(&auth_state)?;
            let auth_data = WebauthnData {
                code: get_rand(48),
                auth_state_json,
                data: add_data,
            };
            auth_data.save().await?;

            Ok(WebauthnAuthStartResponse {
                code: auth_data.code,
                rcr,
                exp: req_exp as u64,
            })
        }

        Err(err) => {
            error!(?err, "Webauthn challenge authentication");
            Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "Internal error with Webauthn Challenge Authentication",
            ))
        }
    }
}

pub async fn auth_finish(
    req: &HttpRequest,
    browser_id: BrowserId,
    session: Option<Session>,
    payload: WebauthnAuthFinishRequest,
) -> Result<WebauthnAdditionalData, ErrorResponse> {
    let auth_data = WebauthnData::find_remove(payload.code).await?;

    let (user_id, is_login) = match &auth_data.data {
        WebauthnAdditionalData::Login(d) => (&d.user_id, true),
        WebauthnAdditionalData::Discover(_) => {
            // Webauthn auth discovery needs special handling
            return auth_finish_discover(req, browser_id, session, payload.data, auth_data).await;
        }
        WebauthnAdditionalData::Service(d) => (&d.user_id, false),
        WebauthnAdditionalData::Test(user_id) => (user_id, false),
        WebauthnAdditionalData::LoginToSAwait(d) => (&d.user_id, false),
    };

    let mut user = User::find(user_id.clone()).await?;
    let force_uv =
        user.account_type() == AccountType::Passkey || RauthyConfig::get().vars.webauthn.force_uv;

    let pks = PasskeyEntity::find_for_user(&user.id).await?;
    let auth_state = serde_json::from_str(&auth_data.auth_state_json)?;

    match RauthyConfig::get()
        .webauthn
        .finish_passkey_authentication(&payload.data, &auth_state)
    {
        Ok(auth_result) => {
            if force_uv && !auth_result.user_verified() {
                warn!(
                    user.id,
                    "Webauthn Authentication Ceremony without User Verification",
                );
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "User Presence only is not allowed - Verification is needed",
                ));
            }
            let uid = user.id.clone();

            if is_login && let Some(mut session) = session {
                if let Some(suid) = &session.user_id
                    && suid != &user.id
                {
                    // TODO If this happens, this can only be a try to attack and get into another
                    //  user account. We should probably blacklist the source IP after enough
                    //  testing. We must be sure that this can never happen by accident.

                    let WebauthnAdditionalData::Login(data) = auth_data.data else {
                        unreachable!()
                    };
                    data.delete().await?;

                    return Err(ErrorResponse::new(
                        ErrorResponseType::Forbidden,
                        "User ID mismatch for session",
                    ));
                }

                session.set_authenticated(&user).await?;
                user.last_login = Some(Utc::now().timestamp());
                user.last_failed_login = None;
                user.failed_login_attempts = None;
                user.save(None).await?;
            }

            LoginLocation::spawn_background_check(user.clone(), req, browser_id)?;

            if auth_result.needs_update() {
                let now = Utc::now().timestamp();
                for mut pk_entity in pks {
                    let mut pk = pk_entity.get_pk();
                    if pk.update_credential(&auth_result) == Some(true) {
                        pk_entity.passkey = serde_json::to_string(&pk)?;
                        pk_entity.last_used = now;
                        pk_entity.update_passkey().await?;
                    }
                }
            }

            info!(user.id = uid, "Webauthn Authentication successful");

            if let WebauthnAdditionalData::Login(data) = auth_data.data {
                data.delete().await?;

                if let Some(tos_data) = data.tos_await_data {
                    let code_await = AuthCodeToSAwait {
                        auth_code: tos_data.auth_code,
                        await_code: AuthCodeToSAwait::generate_code(),
                        auth_code_lifetime: tos_data.auth_code_lifetime,
                        header_loc: data.header_loc,
                        header_origin: data.header_origin.clone(),
                        needs_user_update: data.needs_user_update,
                    };
                    code_await.save().await?;

                    Ok(WebauthnAdditionalData::LoginToSAwait(
                        WebauthnLoginToSAwaitCode {
                            await_code: code_await.await_code,
                            user_id: uid,
                            header_origin: data.header_origin,
                        },
                    ))
                } else {
                    Ok(WebauthnAdditionalData::Login(data))
                }
            } else {
                Ok(auth_data.data)
            }
        }
        Err(err) => {
            error!(?err, "Webauthn Auth Finish");
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                err.to_string(),
            ))
        }
    }
}
