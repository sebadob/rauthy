use crate::token_set::{
    AuthCodeFlow, AuthTime, DeviceCodeFlow, DpopFingerprint, TokenScopes, TokenSet,
};
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::{web, HttpRequest};
use jwt_simple::claims;
use jwt_simple::claims::JWTClaims;
use jwt_simple::common::VerificationOptions;
use jwt_simple::prelude::*;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use rauthy_models::entity::dpop_proof::DPoPProof;
use rauthy_models::entity::jwk::{JwkKeyPair, JwkKeyPairAlg};
use rauthy_models::entity::refresh_tokens::RefreshToken;
use rauthy_models::entity::refresh_tokens_devices::RefreshTokenDevice;
use rauthy_models::entity::users::User;
use rauthy_models::{validate_jwt, JwtRefreshClaims, JwtTokenType};
use std::collections::HashSet;
use time::OffsetDateTime;
use tracing::debug;

/// Validates request parameters for the authorization and refresh endpoints
pub async fn validate_auth_req_param(
    data: &web::Data<AppState>,
    req: &HttpRequest,
    client_id: &str,
    redirect_uri: &str,
    code_challenge: &Option<String>,
    code_challenge_method: &Option<String>,
) -> Result<(Client, Option<(HeaderName, HeaderValue)>), ErrorResponse> {
    // client exists
    let client = Client::find_maybe_ephemeral(data, String::from(client_id)).await?;

    // allowed origin
    let header = client.validate_origin(req, &data.listen_scheme, &data.public_url)?;

    // allowed redirect uris
    client.validate_redirect_uri(redirect_uri)?;

    // code challenge + method
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

/// Validates a given JWT Access Token
pub async fn validate_token<T: serde::Serialize + for<'de> ::serde::Deserialize<'de>>(
    data: &web::Data<AppState>,
    token: &str,
) -> Result<claims::JWTClaims<T>, ErrorResponse> {
    let options = jwt_simple::prelude::VerificationOptions {
        // allowed_audiences: Some(HashSet::from_strings(&[&])), // TODO
        allowed_issuers: Some(HashSet::from_strings(&[&data.issuer])),
        ..Default::default()
    };

    // extract metadata
    let kid = JwkKeyPair::kid_from_token(token)?;

    // retrieve jwk for kid
    let kp = JwkKeyPair::find(data, kid).await?;
    validate_jwt!(T, kp, token, options)

    // TODO check roles if we add more users / roles
}

pub async fn validate_refresh_token(
    // when this is some, it will be checked against the 'azp' claim, otherwise skipped and a client
    // will be fetched inside this function
    client_opt: Option<Client>,
    refresh_token: &str,
    data: &web::Data<AppState>,
    req: &HttpRequest,
) -> Result<(TokenSet, Option<String>), ErrorResponse> {
    let options = VerificationOptions {
        // allowed_audiences: Some(HashSet::from_strings(&[&])), // TODO change after making client non-opt
        allowed_issuers: Some(HashSet::from_strings(&[&data.issuer])),
        ..Default::default()
    };

    // extract metadata
    let kid = JwkKeyPair::kid_from_token(refresh_token)?;

    // retrieve jwk for kid
    let kp = JwkKeyPair::find(data, kid).await?;
    let claims: JWTClaims<JwtRefreshClaims> =
        validate_jwt!(JwtRefreshClaims, kp, refresh_token, options)?;

    // validate typ
    if claims.custom.typ != JwtTokenType::Refresh {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Provided Token is not a valid refresh token",
        ));
    }

    // get uid
    let uid = claims.custom.uid;

    // get azp / client
    let client = if let Some(c) = client_opt {
        c
    } else {
        Client::find(data, claims.custom.azp.clone()).await?
    };
    if client.id != claims.custom.azp {
        return Err(ErrorResponse::new(
            ErrorResponseType::BadRequest,
            "Invalid 'azp'",
        ));
    }
    let header_origin = client.validate_origin(req, &data.listen_scheme, &data.public_url)?;

    // validate DPoP proof
    let (dpop_fingerprint, dpop_nonce) = if let Some(cnf) = claims.custom.cnf {
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

    let mut user = User::find(data, uid).await?;
    user.check_enabled()?;
    user.check_expired()?;

    // validate that it exists in the db and invalidate it afterward
    let (_, validation_str) = refresh_token.split_at(refresh_token.len() - 49);
    let now = OffsetDateTime::now_utc().unix_timestamp();
    let exp_at_secs = now + data.refresh_grace_time as i64;
    let rt_scope = if let Some(device_id) = &claims.custom.did {
        let mut rt = RefreshTokenDevice::find(data, validation_str).await?;

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
            rt.save(data).await?;
        }
        rt.scope
    } else {
        let mut rt = RefreshToken::find(data, validation_str).await?;
        if rt.exp > exp_at_secs + 1 {
            rt.exp = exp_at_secs;
            rt.save(data).await?;
        }
        rt.scope
    };

    // at this point, everything has been validated -> we can issue a new TokenSet safely
    debug!("Refresh Token - all good!");

    // set last login
    user.last_login = Some(OffsetDateTime::now_utc().unix_timestamp());
    user.save(data, None, None).await?;

    let auth_time = if let Some(ts) = claims.custom.auth_time {
        AuthTime::given(ts)
    } else {
        // TODO this is not 100% correct but will make the migration from older to new refresh
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
        AuthCodeFlow::No,
        DeviceCodeFlow::No,
    )
    .await?;

    Ok((ts, dpop_nonce))
}
