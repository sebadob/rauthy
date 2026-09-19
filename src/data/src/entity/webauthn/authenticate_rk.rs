use crate::entity::browser_id::BrowserId;
use crate::entity::login_locations::LoginLocation;
use crate::entity::sessions::Session;
use crate::entity::users::User;
use crate::entity::webauthn::auth_data::{WebauthnAdditionalData, WebauthnData};
use crate::entity::webauthn::passkey::PasskeyEntity;
use crate::entity::webauthn::rk_token::ResidentKeyToken;
use crate::rauthy_config::RauthyConfig;
use actix_web::HttpRequest;
use chrono::Utc;
use rauthy_api_types::users::WebauthnAuthStartResponse;
use rauthy_common::utils::get_rand;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::{error, info};
use webauthn_rs::prelude::{Credential, Uuid};
use webauthn_rs_proto::{PublicKeyCredential, UserVerificationPolicy};

/// Starts the user discovery flow via Resident Keys.
pub async fn auth_start_discover() -> Result<WebauthnAuthStartResponse, ErrorResponse> {
    match RauthyConfig::get()
        .webauthn
        .start_passkey_authentication(&[])
    {
        Ok((mut rcr, auth_state)) => {
            let req_exp = RauthyConfig::get().vars.webauthn.req_exp;
            // timeout expected in ms
            rcr.public_key.timeout = Some(req_exp as u32 * 1000);
            rcr.public_key.user_verification = UserVerificationPolicy::Required;

            // cannot be serialized with bincode -> no deserialize from any
            let auth_state_json = serde_json::to_string(&auth_state)?;
            let auth_data = WebauthnData {
                code: get_rand(48),
                auth_state_json,
                data: WebauthnAdditionalData::Discover(None),
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

// CAUTION:
// The `PasskeyAuthenticationLocal` and `AuthenticationStateLocal` MUST be direct copies of the
// `webauthn-rs` structs. We need these copies for make true resident key lookups possible, since
// the original structs have private fields we need to access to solve a chicken-and-egg problem.
// Sadly, the crate does not expose a way to do a resident-key-based login without attestation and
// without knowing the user upfront.
//
// When `webautn-rs` is updated, make sure that these 2 types are in sync!
#[derive(Debug, Serialize, Deserialize)]
struct PasskeyAuthenticationLocal {
    ast: AuthenticationStateLocal,
}

// We only care about the `credentials` type here. Let the others be handled by `serde`.
#[derive(Debug, Serialize, Deserialize)]
struct AuthenticationStateLocal {
    credentials: Vec<Credential>,
    policy: serde_json::Value,
    challenge: serde_json::Value,
    appid: serde_json::Value,
    allow_backup_eligible_upgrade: serde_json::Value,
}

pub async fn auth_finish_discover(
    req: &HttpRequest,
    browser_id: BrowserId,
    session: Option<Session>,
    pubkey_cred: PublicKeyCredential,
    auth_data: WebauthnData,
) -> Result<WebauthnAdditionalData, ErrorResponse> {
    let Some(mut session) = session else {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Webauthn user discvoery requires a session conext",
        ));
    };

    let mut pk_entity = PasskeyEntity::find_rk_by_cred_id(pubkey_cred.get_credential_id()).await?;
    let cred = Credential::from(pk_entity.get_pk());

    // This step is a bit tricky. We need to allow the credential we just discovered in the
    // auth state. `webauthn-rs` does not provide a way to do a full discovery without knowing the
    // user upfront. It always wants us to provide the list of allowed credentials in the auth start
    // phase. However, we want to discover it. If not, we could stick with the default passkey flow
    // and there would be no advantage from using resident keys in the first place.
    //
    // To solve this issue, since the credentials in the auth state are private, we deserialize into
    // a generic JSON we can manipulate, and then convert to the final type again. This is very
    // cumbersome and a bit wasteful, but it's the only solution for this problem right now.
    //
    // SAFETY
    // Doing so is actually safe. We look up the credential ID which needs to exist in our database
    // already. When the validation was successful, we also get the unique user ID from the
    // resident key, which we can validate against our saved data. We generated this user ID
    // during the registration ceremony. The credential ID is already
    //
    // TODO we should use something way faster than JSON after enough testing
    let mut state_local =
        serde_json::from_str::<PasskeyAuthenticationLocal>(&auth_data.auth_state_json)?;
    state_local.ast.credentials.push(cred);
    let json = serde_json::to_string(&state_local)?;
    let auth_state = serde_json::from_str(&json)?;

    match RauthyConfig::get()
        .webauthn
        .finish_passkey_authentication(&pubkey_cred, &auth_state)
    {
        Ok(auth_result) => {
            if !auth_result.user_verified() {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "User Verification missing",
                ));
            }
            let Some(user_handle) = pubkey_cred.response.user_handle else {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "User Handle missing",
                ));
            };
            let user_uuid = Uuid::from_slice(user_handle.as_ref()).unwrap_or_default();

            let pk_uuid =
                Uuid::from_str(&pk_entity.passkey_user_id).expect("to always be a valid UUID");
            if pk_uuid != user_uuid {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "Invalid User Handle",
                ));
            }

            let mut user = User::find(pk_entity.user_id.clone()).await?;
            if let Some(suid) = &session.user_id
                && suid != &user.id
            {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "User ID mismatch for session",
                ));
            }

            session.is_mfa = true;
            session.set_authenticated(&user).await?;
            user.last_login = Some(Utc::now().timestamp());
            user.last_failed_login = None;
            user.failed_login_attempts = None;
            user.save(None).await?;

            LoginLocation::spawn_background_check(user.clone(), req, browser_id.clone())?;

            if auth_result.needs_update() {
                let now = Utc::now().timestamp();
                let mut pk = pk_entity.get_pk();
                if pk.update_credential(&auth_result) == Some(true) {
                    pk_entity.passkey = serde_json::to_string(&pk)?;
                    pk_entity.last_used = now;
                    pk_entity.update_passkey().await?;
                }
            }

            info!(user.id = user.id, "Webauthn Authentication successful");

            let token = ResidentKeyToken::new(session.id, browser_id).await?;
            Ok(WebauthnAdditionalData::Discover(Some(token.code)))
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
