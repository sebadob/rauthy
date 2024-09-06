use crate::token_set::{
    AuthCodeFlow, AuthTime, DeviceCodeFlow, DpopFingerprint, TokenNonce, TokenScopes, TokenSet,
};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{web, HttpRequest};
use chrono::Utc;
use rauthy_api_types::oidc::TokenRequest;
use rauthy_common::constants::HEADER_DPOP_NONCE;
use rauthy_common::utils::{base64_url_encode, real_ip_from_req};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::auth_codes::AuthCode;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::clients_dyn::ClientDyn;
use rauthy_models::entity::dpop_proof::DPoPProof;
use rauthy_models::entity::sessions::{Session, SessionState};
use rauthy_models::entity::users::User;
use ring::digest;
use std::str::FromStr;
use time::OffsetDateTime;
use tracing::warn;

#[tracing::instrument(
    skip_all,
    fields(client_id = req_data.client_id, username = req_data.username)
)]
pub async fn grant_type_authorization_code(
    data: &web::Data<AppState>,
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

    // check the client for external origin and oidc flow
    let (client_id, client_secret) = req_data.try_get_client_id_secret(&req)?;
    let client = Client::find_maybe_ephemeral(data, client_id.clone())
        .await
        .map_err(|_| {
            ErrorResponse::new(
                ErrorResponseType::NotFound,
                format!("Client '{}' not found", client_id),
            )
        })?;
    let header_origin = client.validate_origin(&req, &data.listen_scheme, &data.public_url)?;
    if client.confidential {
        let secret = client_secret.ok_or_else(|| {
            warn!("'client_secret' is missing");
            ErrorResponse::new(ErrorResponseType::BadRequest, "'client_secret' is missing")
        })?;
        client.validate_secret(&secret, &req)?;
    }
    client.validate_flow("authorization_code")?;

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
        let err = format!("Wrong 'code' for client_id '{}'", client_id);
        warn!(err);
        return Err(ErrorResponse::new(ErrorResponseType::Unauthorized, err));
    }
    if code.exp < OffsetDateTime::now_utc().unix_timestamp() {
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

            if !code.challenge.as_ref().unwrap().eq(&hash_base64) {
                warn!("'code_verifier' does not match the challenge");
                return Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    "'code_verifier' does not match the challenge",
                ));
            }
        }
    }

    // We will not perform another `redirect_uri` check at this point, like stated in the RFC.
    // It is just unnecessary because of the way Rauthy handles the flow init during GET /authorize.
    //
    // It is impossible to trick a client to be redirected to another `redirect_uri` than the allowed ones,
    // which are all in control by the original client. The `redirect_uri` for Rauthy in the client config
    // is not optional like mentioned in the RFC, but actually mandatory. It is already checked and validated
    // carefully before the user would even see the login prompt.
    //
    // An additional check at this point does not provide any security benefit but only uses resources.

    let user = User::find(data, code.user_id.clone()).await?;
    let token_set = TokenSet::from_user(
        &user,
        data,
        &client,
        AuthTime::given(user.last_login.unwrap_or_else(|| Utc::now().timestamp())),
        dpop_fingerprint,
        code.nonce.clone().map(TokenNonce),
        Some(TokenScopes(code.scopes.join(" "))),
        AuthCodeFlow::Yes,
        DeviceCodeFlow::No,
    )
    .await?;

    // update session metadata
    if code.session_id.is_some() {
        let sid = code.session_id.as_ref().unwrap().clone();
        let mut session = Session::find(data, sid).await?;

        session.last_seen = OffsetDateTime::now_utc().unix_timestamp();
        session.state = SessionState::Auth.as_str().to_string();
        if let Err(err) = session.validate_user_expiry(&user) {
            code.delete().await?;
            return Err(err);
        }
        session.validate_user_expiry(&user)?;
        session.user_id = Some(user.id);
        session.roles = Some(user.roles);
        session.groups = user.groups;
        session.save(data).await?;
    }
    code.delete().await?;

    // update timestamp if it is a dynamic client
    if client.is_dynamic() {
        ClientDyn::update_used(data, &client.id).await?;
    }

    Ok((token_set, headers))
}
