use crate::token_set::{
    AuthCodeFlow, AuthTime, DeviceCodeFlow, DpopFingerprint, SessionId, TokenNonce, TokenScopes,
    TokenSet,
};
use actix_web::HttpRequest;
use actix_web::http::header::{
    ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_METHODS, HeaderName, HeaderValue,
};
use chrono::Utc;
use rauthy_api_types::oidc::TokenRequest;
use rauthy_common::constants::HEADER_DPOP_NONCE;
use rauthy_common::utils::{base64_url_encode, real_ip_from_req};
use rauthy_data::entity::auth_codes::AuthCode;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::clients_dyn::ClientDyn;
use rauthy_data::entity::dpop_proof::DPoPProof;
use rauthy_data::entity::sessions::{Session, SessionState};
use rauthy_data::entity::user_login_states::UserLoginState;
use rauthy_data::entity::users::User;
use rauthy_data::events::event::Event;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use ring::digest;
use std::str::FromStr;
use tracing::warn;

#[tracing::instrument(
    skip_all,
    fields(client_id = req_data.client_id, username = req_data.username)
)]
pub async fn grant_type_authorization_code(
    req: HttpRequest,
    req_data: TokenRequest,
) -> Result<(TokenSet, Vec<(HeaderName, HeaderValue)>), ErrorResponse> {
    if req_data.code.is_none() {
        warn!("'code' is missing");
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "'code' is missing",
        ));
    }
    if req_data.redirect_uri.is_none() {
        warn!("'redirect_uri' is missing");
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "'redirect_uri' is missing",
        ));
    }

    // check the client for external origin and oidc flow
    let (client_id, client_secret) = req_data.try_get_client_id_secret(&req)?;
    let client = Client::find_maybe_ephemeral(client_id.clone())
        .await
        .map_err(|_| {
            ErrorResponse::new(
                ErrorResponseType::NotFound,
                format!("Client '{client_id}' not found"),
            )
        })?;
    client.validate_enabled()?;
    let header_origin = client.get_validated_origin_header(&req)?;
    if client.confidential {
        let secret = client_secret.ok_or_else(|| {
            warn!("'client_secret' is missing");
            ErrorResponse::new(ErrorResponseType::BadRequest, "'client_secret' is missing")
        })?;
        client.validate_secret(&secret, &req).await?;
    }
    client.validate_flow("authorization_code")?;
    client.validate_redirect_uri(req_data.redirect_uri.as_deref().unwrap_or_default())?;

    // check for DPoP header
    let mut headers = Vec::new();
    let dpop_fingerprint =
        if let Some(proof) = DPoPProof::opt_validated_from(&req, &header_origin).await? {
            if let Some(nonce) = &proof.claims.nonce {
                headers.push((
                    HeaderName::from_str(HEADER_DPOP_NONCE).unwrap(),
                    HeaderValue::from_str(nonce)?,
                ));
            };
            Some(DpopFingerprint(proof.jwk_fingerprint()?))
        } else {
            None
        };
    if let Some(h) = header_origin {
        headers.push(h);
        headers.push((
            ACCESS_CONTROL_ALLOW_METHODS,
            HeaderValue::from_static("POST"),
        ));
        headers.push((
            ACCESS_CONTROL_ALLOW_CREDENTIALS,
            HeaderValue::from_static("true"),
        ));
    }

    // get the oidc code from the cache
    let idx = req_data.code.as_ref().unwrap().to_owned();
    let code = match AuthCode::find(idx).await? {
        None => {
            warn!(
                "'auth_code' could not be found inside the cache - Host: {}",
                real_ip_from_req(&req)?,
            );
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "'auth_code' could not be found inside the cache",
            ));
        }
        Some(code) => code,
    };
    // validate the oidc code
    if code.client_id != client_id {
        let err = format!("Wrong 'code' for client_id '{client_id}'");
        warn!(err);
        return Err(ErrorResponse::new(ErrorResponseType::Unauthorized, err));
    }
    if code.exp < Utc::now().timestamp() {
        warn!("The Authorization Code has expired");
        return Err(ErrorResponse::new(
            ErrorResponseType::SessionExpired,
            "The Authorization Code has expired",
        ));
    }
    if code.challenge.is_some() {
        if req_data.code_verifier.is_none() {
            warn!("'code_verifier' is missing");
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'code_verifier' is missing",
            ));
        }

        if code.challenge_method.as_ref().unwrap().eq("plain") {
            if !code.challenge.eq(&req_data.code_verifier) {
                warn!("'code_verifier' does not match the challenge");
                return Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    "'code_verifier' does not match the challenge",
                ));
            }
        } else {
            let hash = digest::digest(&digest::SHA256, req_data.code_verifier.unwrap().as_bytes());
            let hash_base64 = base64_url_encode(hash.as_ref());

            if code.challenge != Some(hash_base64) {
                warn!("'code_verifier' does not match the challenge");
                return Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    "'code_verifier' does not match the challenge",
                ));
            }
        }
    }

    let user = User::find(code.user_id.clone()).await?;
    let token_set = TokenSet::from_user(
        &user,
        &client,
        AuthTime::given(user.last_login.unwrap_or_else(|| Utc::now().timestamp())),
        dpop_fingerprint,
        code.nonce.clone().map(TokenNonce),
        Some(TokenScopes(code.scopes.join(" "))),
        code.session_id.clone().map(SessionId),
        AuthCodeFlow::Yes,
        DeviceCodeFlow::No,
    )
    .await?;

    code.delete().await?;

    // update session metadata
    if let Some(sid) = code.session_id.clone() {
        let mut session = Session::find(sid).await?;
        session.set_authenticated(&user).await?;
    }

    if client.is_dynamic() {
        ClientDyn::update_used(&client.id).await?;
    }

    if RauthyConfig::get().vars.events.generate_token_issued {
        Event::token_issued("authorization_code", &client.id, Some(&user.email))
            .send()
            .await?;
    }

    // backchannel logout and login state tracking is not supported for ephemeral clients
    if !client.is_ephemeral() {
        UserLoginState::insert(user.id.clone(), client.id, code.session_id).await?;
    }

    // No location check here, this is done in `POST /authorize` already

    Ok((token_set, headers))
}
