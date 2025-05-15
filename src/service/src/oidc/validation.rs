use crate::token_set::{
    AuthCodeFlow, AuthTime, DeviceCodeFlow, DpopFingerprint, TokenScopes, TokenSet,
};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{HttpRequest, web};
use chrono::Utc;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_jwt::claims::{JwtRefreshClaims, JwtTokenType};
use rauthy_jwt::token::JwtToken;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::dpop_proof::DPoPProof;
use rauthy_models::entity::refresh_tokens::RefreshToken;
use rauthy_models::entity::refresh_tokens_devices::RefreshTokenDevice;
use rauthy_models::entity::users::User;
use tracing::debug;

/// Validates request parameters for the authorization and refresh endpoints
pub async fn validate_auth_req_param(
    req: &HttpRequest,
    client_id: &str,
    redirect_uri: &str,
    code_challenge: &Option<String>,
    code_challenge_method: &Option<String>,
) -> Result<(Client, Option<(HeaderName, HeaderValue)>), ErrorResponse> {
    let client = Client::find_maybe_ephemeral(String::from(client_id))
        .await
        .map_err(|mut err| {
            if err.error == ErrorResponseType::NotFound {
                err.message = format!("Client '{client_id}' does not exist").into();
            }
            err
        })?;

    let header = client.get_validated_origin_header(req)?;

    client.validate_redirect_uri(redirect_uri)?;

    if client.challenge.is_some() {
        if code_challenge.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "'code_challenge' is missing",
            ));
        } else {
            // 'plain' is the default method to be assumed by the OAuth specification when it is
            // not further specified.
            let method = if let Some(m) = code_challenge_method {
                m.to_owned()
            } else {
                String::from("plain")
            };
            client.validate_challenge_method(&method)?;
        }
    }

    Ok((client, header))
}

pub async fn validate_refresh_token(
    // when this is some, it will be checked against the 'azp' claim, otherwise skipped and a client
    // will be fetched inside this function
    client_opt: Option<Client>,
    refresh_token: &str,
    data: &web::Data<AppState>,
    req: &HttpRequest,
) -> Result<(TokenSet, Option<String>), ErrorResponse> {
    let mut buf = Vec::with_capacity(256);
    JwtToken::validate_claims_into(
        refresh_token,
        &data.issuer,
        None,
        Some(JwtTokenType::Refresh),
        0,
        &mut buf,
    )
    .await?;
    let claims: JwtRefreshClaims = serde_json::from_slice(&buf)?;

    let client = if let Some(c) = client_opt {
        c
    } else {
        Client::find(claims.common.azp.to_string()).await?
    };
    let header_origin = client.get_validated_origin_header(req)?;

    // validate DPoP proof
    let (dpop_fingerprint, dpop_nonce) = if let Some(cnf) = claims.common.cnf {
        // if the refresh token contains the 'cnf' header, we must validate the DPoP as well
        if let Some(proof) = DPoPProof::opt_validated_from(req, &header_origin).await? {
            let fingerprint = proof.jwk_fingerprint()?;
            if fingerprint != cnf.jkt {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "The refresh token is bound to a missing DPoP proof",
                ));
            }
            debug!("DPoP-Bound refresh token accepted");
            (Some(DpopFingerprint(fingerprint)), proof.claims.nonce)
        } else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "The refresh token is bound to a missing DPoP proof",
            ));
        }
    } else {
        (None, None)
    };

    let mut user = User::find(claims.uid.to_string()).await?;
    user.check_enabled()?;
    user.check_expired()?;

    // validate that it exists in the db and invalidate it afterward
    let (_, validation_str) = refresh_token.split_at(refresh_token.len() - 49);
    let now = Utc::now().timestamp();
    let exp_at_secs = now + data.refresh_grace_time as i64;
    let rt_scope = if let Some(device_id) = &claims.common.did {
        let mut rt = RefreshTokenDevice::find(validation_str).await?;

        if &rt.device_id != device_id {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "'device_id' does not match",
            ));
        }
        if rt.user_id != user.id {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "'user_id' does not match",
            ));
        }

        if rt.exp > exp_at_secs + 1 {
            rt.exp = exp_at_secs;
            rt.save().await?;
        }
        rt.scope
    } else {
        let mut rt = RefreshToken::find(validation_str).await?;
        if rt.exp > exp_at_secs + 1 {
            rt.exp = exp_at_secs;
            rt.save().await?;
        }
        rt.scope
    };

    // at this point, everything has been validated -> we can issue a new TokenSet safely
    debug!("Refresh Token - all good!");

    // set last login
    user.last_login = Some(Utc::now().timestamp());
    user.save(None).await?;

    let auth_time = if let Some(ts) = claims.auth_time {
        AuthTime::given(ts)
    } else {
        // This is not 100% correct but will make the migration from older to new refresh
        // tokens smooth. In a future release, the `auth_time` can be set to required in the claims.
        // Optional for now to still accept older tokens.
        // As soon as no old refresh tokens exist anymore, this branch will never be used anyway.
        AuthTime::now()
    };

    let ts = TokenSet::from_user(
        &user,
        data,
        &client,
        auth_time,
        dpop_fingerprint,
        None,
        rt_scope.map(TokenScopes),
        // TODO think about if we maybe want to have an optional refresh token session binding
        None,
        AuthCodeFlow::No,
        DeviceCodeFlow::No,
    )
    .await?;

    Ok((ts, dpop_nonce))
}
