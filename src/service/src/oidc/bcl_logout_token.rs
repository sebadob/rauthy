use actix_web::web;
use chrono::Utc;
use cryptr::utils::secure_random_alnum;
use jwt_simple::algorithms::{EdDSAKeyPairLike, RSAKeyPairLike};
use jwt_simple::claims::Claims;
use jwt_simple::prelude::coarsetime;
use rauthy_common::utils::base64_url_no_pad_decode;
use rauthy_error::ErrorResponse;
use rauthy_error::ErrorResponseType;
use rauthy_models::JwtLogoutClaims;
use rauthy_models::JwtTokenType;
use rauthy_models::app_state::AppState;
use rauthy_models::entity::auth_providers::AuthProvider;
use rauthy_models::entity::jwk::{JWKSPublicKey, JwkKeyPair, JwkKeyPairAlg};
use rauthy_models::sign_jwt;
use serde::{Deserialize, Serialize};
use std::env;
use std::str::FromStr;
use std::string::ToString;
use std::sync::LazyLock;

static EVENT: &str = "http://schemas.openid.net/event/backchannel-logout";

static LOGOUT_TOKEN_LIFETIME: LazyLock<u16> = LazyLock::new(|| {
    env::var("LOGOUT_TOKEN_LIFETIME")
        .unwrap_or_else(|_| "30".to_string())
        .parse::<u16>()
        .expect("Cannot parse LOGOUT_TOKEN_LIFETIME as u16")
});

/// Logout Token for OIDC backchannel logout specified in
/// https://openid.net/specs/openid-connect-backchannel-1_0.html#LogoutToken
#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutToken {
    // default claims
    iss: String,
    aud: String,
    exp: i64,

    // MUST always either not exist or be `logout+jwt`
    typ: Option<JwtTokenType>,
    // alg: JwkKeyPairAlg,
    iat: i64,
    jti: String,
    // MUST always contain `"http://schemas.openid.net/event/backchannel-logout": {}`
    events: serde_json::Value,

    sub: Option<String>,
    sid: Option<String>,

    // The `nonce` MUST NOT exist in this token. We try to deserialize into an `Option<_>` for easy
    // `.is_none()` validation.
    nonce: Option<String>,
}

impl LogoutToken {
    pub async fn new(
        data: &web::Data<AppState>,
        aud: String,
        sub: Option<String>,
        sid: Option<String>,
    ) -> Self {
        let iat = Utc::now().timestamp();
        let exp = iat + *LOGOUT_TOKEN_LIFETIME as i64;

        let events = serde_json::Value::Object(serde_json::Map::from_iter([(
            EVENT.to_string(),
            serde_json::Value::Object(serde_json::Map::new()),
        )]));

        Self {
            iss: data.issuer.clone(),
            aud,
            exp,
            typ: Some(JwtTokenType::Logout),
            iat,
            jti: secure_random_alnum(12),
            events,
            sub,
            sid,
            nonce: None,
        }
    }

    pub async fn into_token(self, alg: JwkKeyPairAlg) -> Result<String, ErrorResponse> {
        let custom_claims = JwtLogoutClaims {
            typ: self.typ,
            iat: self.iat,
            jti: self.jti,
            events: self.events,
            sid: self.sid,
            nonce: None,
        };

        let mut claims = Claims::with_custom_claims(
            custom_claims,
            coarsetime::Duration::from_secs((self.exp - self.iat) as u64),
        )
        .with_audience(self.aud)
        .with_issuer(self.iss);

        if let Some(sub) = self.sub {
            claims = claims.with_subject(sub);
        }

        let kp = JwkKeyPair::find_latest(alg).await?;

        sign_jwt!(kp, claims)
    }

    /// Parse and validate the token as specified in
    /// https://openid.net/specs/openid-connect-backchannel-1_0.html#Validation
    pub async fn from_str_validated(
        data: &web::Data<AppState>,
        logout_token: &str,
    ) -> Result<Self, ErrorResponse> {
        // Before we can actually validate the token, we need to decode it and take a peek at the
        // issuer, so we can validate that we do have the issuer configured as an upstream provider.
        let mut split = logout_token.split(".");

        let header = match split.next() {
            None => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "invalid logout token format - header is missing",
                ));
            }
            Some(b64) => {
                let bytes = base64_url_no_pad_decode(b64)?;
                let s = String::from_utf8_lossy(&bytes);
                serde_json::from_str::<serde_json::Value>(&s)?
            }
        };
        let slf = match split.next() {
            None => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "invalid logout token format - claims are missing",
                ));
            }
            Some(b64) => {
                let bytes = base64_url_no_pad_decode(b64)?;
                let s = String::from_utf8_lossy(&bytes);
                serde_json::from_str::<Self>(&s)?
            }
        };

        // Validate claims that need only inexpensive checks. When we do this before doing actual
        // DB lookups or even JWKS fetches, we can fail fast if something is wrong at this point
        // already.
        let alg = JwkKeyPairAlg::from_str(
            header
                .get("alg")
                .map(|a| a.as_str().unwrap_or_default())
                .unwrap_or_default(),
        )?;

        let kid = header
            .get("kid")
            .map(|a| a.as_str().unwrap_or_default())
            .unwrap_or_default()
            .to_string();
        if kid.is_empty() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "`kid` is missing in token header",
            ));
        }

        if slf.typ.is_some() && slf.typ != Some(JwtTokenType::Logout) {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "invalid token `typ`",
            ));
        }

        // TODO configurable allowed clock skew
        let now = Utc::now().timestamp();
        if slf.exp < now {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "token has expired",
            ));
        }
        if slf.iat > now {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "`iat` must not be in the future",
            ));
        }
        if slf.exp < slf.iat {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "`exp` must not come before `iat`",
            ));
        }
        if slf.exp - slf.iat > 120 {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "logout token has a way too long validity - max 120 seconds allowed",
            ));
        }

        if slf.sub.is_none() && slf.sid.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "at least one of `sub` or `sid` must be given",
            ));
        }
        match slf.events.get(EVENT) {
            None => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "`events.http://schemas.openid.net/event/backchannel-logout` is missing",
                ));
            }
            Some(value) => {
                let s = value.as_str().unwrap_or_default();
                if s != "{}" {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::BadRequest,
                        "invalid value for `events.http://schemas.openid.net/event/backchannel-logout`",
                    ));
                }
            }
        }
        if slf.nonce.is_some() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "`nonce` MUST NOT be given",
            ));
        }

        // We don't really need to care about `jti` caching and validation. Replay attacks can only
        // happen for unencrypted connections, which Rauthy will never work with anyway.

        let provider = AuthProvider::find_by_iss(slf.iss.clone()).await?;
        if provider.jwks_endpoint.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "this provider has no configured `jwks_endpoint` for token validation",
            ));
        }

        if slf.aud != provider.client_id {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "`aud` claim does not match our `client_id",
            ));
        }

        // If we get here, the token we received seems to come from an actual configured upstream
        // auth provider, and we can try to fetch the public key and validate the signature.

        // validates signature
        let jwk =
            JWKSPublicKey::fetch_remote(provider.jwks_endpoint.as_ref().unwrap(), kid).await?;
        if jwk.alg != Some(alg) {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "`alg` mismatch between token header and fetched JWK",
            ));
        }
        jwk.validate_token_signature(logout_token)?;

        Ok(slf)
    }
}
