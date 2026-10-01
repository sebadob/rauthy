use crate::entity::auth_codes::AuthCodeToSAwait;
use crate::entity::browser_id::BrowserId;
use crate::entity::login_locations::LoginLocation;
use crate::entity::sessions::Session;
use crate::entity::users::User;
use crate::entity::webauthn::auth_data::{WebauthnAdditionalData, WebauthnData};
use crate::entity::webauthn::auth_req::{WebauthnLoginReq, WebauthnServiceReq};
use crate::entity::webauthn::authenticate_rk::auth_finish_discover;
use crate::entity::webauthn::ceremony::{AuthenticationState, requires_uv};
use crate::entity::webauthn::passkey::PasskeyEntity;
use crate::entity::webauthn::{force_mds_attestation, verify_attestation};
use crate::rauthy_config::RauthyConfig;
use actix_web::HttpRequest;
use chrono::Utc;
use rauthy_api_types::users::{MfaPurpose, WebauthnAuthFinishRequest, WebauthnAuthStartResponse};
use rauthy_common::utils::get_rand;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::cmp::min;
use tracing::{error, info, warn};
use utoipa::ToSchema;
use webauthn_rs::prelude::Passkey;

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
            // Discoverable credentials never use this route.
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "A discoverable credential auth request must use a different endpoint",
            ));
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
    let config = &RauthyConfig::get().vars.webauthn;
    let force_uv = requires_uv(user.account_type(), config.force_uv);
    let force_attestation = force_mds_attestation();

    let pks = {
        let entities = if force_uv {
            // in this case, filter out all presence only keys
            PasskeyEntity::find_for_user_with_uv(&user.id).await?
        } else {
            PasskeyEntity::find_for_user(&user.id).await?
        };
        // When attestation is forced, keys without a stored AAGUID cannot be verified against
        // the FIDO MDS dataset, so they are excluded up front to fail fast instead of after
        // the ceremony.
        entities
            .into_iter()
            .filter(|pk_entity| !force_attestation || pk_entity.aaguid.is_some())
            .map(|pk_entity| pk_entity.get_pk())
            .collect::<Vec<Passkey>>()
    };

    if pks.is_empty() {
        // may be the case if the user has presence only keys, unattested keys and the config
        // has changed since registration
        let msg = match (force_uv, force_attestation) {
            (true, true) => {
                "No Security Keys with active user verification and a stored AAGUID found"
            }
            (true, false) => "No Security Keys with active user verification found",
            (false, true) => "No Security Keys with a stored AAGUID found",
            (false, false) => "No Security Keys found",
        };
        return Err(ErrorResponse::new(ErrorResponseType::NotFound, msg));
    }

    match AuthenticationState::start(&RauthyConfig::get().webauthn, pks, force_uv) {
        Ok((mut rcr, auth_state)) => {
            let req_exp = RauthyConfig::get().vars.webauthn.req_exp;
            // timeout expected in ms
            rcr.public_key.timeout = Some(min(req_exp.as_millis(), u32::MAX as u128) as u32);

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
                exp: req_exp.as_secs(),
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
    let force_uv = requires_uv(
        user.account_type(),
        RauthyConfig::get().vars.webauthn.force_uv,
    );

    let pks = PasskeyEntity::find_for_user(&user.id).await?;
    let auth_state = serde_json::from_str::<AuthenticationState>(&auth_data.auth_state_json)?;

    match auth_state.finish(&RauthyConfig::get().webauthn, &payload.data) {
        Ok(auth_result) => {
            // At this point, if the passkey entity has a stored aaguid, it was attested during
            // registration. We don't need to re-validate certificates each time. It will also be
            // set to NULL if the cert is being removed from the MDS dataset.
            let Some(mut pk_entity) = pks
                .into_iter()
                .find(|e| e.credential_id.as_slice() == auth_result.cred_id().as_ref())
            else {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Webauthn CredID not found in authentication result",
                ));
            };

            verify_attestation(
                pk_entity.aaguid.as_deref(),
                &pk_entity.user_id,
                &pk_entity.name,
            )
            .await?;

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
                let mut pk = pk_entity.get_pk();
                if pk.update_credential(&auth_result) == Some(true) {
                    pk_entity.passkey = serde_json::to_string(&pk)?;
                    pk_entity.last_used = now;
                    pk_entity.update_passkey().await?;
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
