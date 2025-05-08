use crate::jwks::{JwkKeyPairAlg, JwkPublicKey};
use crate::provider::OidcProvider;
use crate::rauthy_error::RauthyError;
use crate::token_set::JwtTokenType;
use crate::{base64_url_no_pad_decode, validate_jwt};
use chrono::Utc;
use jwt_simple::algorithms::{EdDSAPublicKeyLike, RSAPublicKeyLike};
use jwt_simple::claims;
use jwt_simple::prelude::{Deserialize, Serialize};
use std::borrow::Cow;

static EVENT: &str = "http://schemas.openid.net/event/backchannel-logout";
static GRANT_TYPE_DEVICE_CODE: &str = "urn:ietf:params:oauth:grant-type:device_code";
static LOGOUT_TOKEN_ALLOWED_LIFETIME: u8 = 120;
static LOGOUT_TOKEN_ALLOW_CLOCK_SKEW: u8 = 5;

/// Logout Token for OIDC backchannel logout specified in
/// https://openid.net/specs/openid-connect-backchannel-1_0.html#LogoutToken
#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutToken {
    // default claims
    pub iss: String,
    pub aud: String,
    pub exp: i64,

    // MUST always either not exist or be `logout+jwt`
    typ: Option<JwtTokenType>,
    // alg: JwkKeyPairAlg,
    pub iat: i64,
    pub jti: String,
    // MUST always contain `"http://schemas.openid.net/event/backchannel-logout": {}`
    events: serde_json::Value,

    pub sub: Option<String>,
    pub sid: Option<String>,

    // The `nonce` MUST NOT exist in this token. We try to deserialize into an `Option<_>` for easy
    // `.is_none()` validation.
    nonce: Option<String>,
}

impl LogoutToken {
    /// Parse and validate the token as specified in
    /// https://openid.net/specs/openid-connect-backchannel-1_0.html#Validation
    pub async fn from_str_validated(logout_token: &str) -> Result<Self, RauthyError> {
        let (header, slf) = Self::build_from_str(logout_token)?;
        let kid = slf.validate_claims(header)?;

        let jwk = JwkPublicKey::get_for_kid(&kid).await?;
        let config = OidcProvider::config()?;
        let claims: claims::JWTClaims<Self> =
            validate_jwt!(Self, jwk, logout_token, config.verification_options.clone())?;
        todo!("aud validation for LogoutToken / rework for auto-deserialize claims");

        Ok(slf)
    }

    /// CAUTION: DO NOT use this function directly outside of tests. It DOES NOT VALIDATE the input.
    /// Only use `from_str_validated()`!
    ///
    /// Returns `(jwt_header, Self)`
    #[inline]
    pub(crate) fn build_from_str(
        logout_token: &str,
    ) -> Result<(serde_json::Value, Self), RauthyError> {
        // Before we can actually validate the token, we need to decode it and take a peek at the
        // issuer, so we can validate that we do have the issuer configured as an upstream provider.
        let mut split = logout_token.split(".");

        let header = match split.next() {
            None => {
                return Err(RauthyError::BadRequest(
                    "invalid logout token format - header is missing",
                ));
            }
            Some(b64) => {
                let bytes = base64_url_no_pad_decode(b64)?;
                let s = String::from_utf8_lossy(&bytes);
                serde_json::from_str::<serde_json::Value>(&s)?
            }
        };

        match split.next() {
            None => Err(RauthyError::BadRequest(
                "invalid logout token format - header is missing",
            )),
            Some(b64) => {
                let bytes = base64_url_no_pad_decode(b64)?;
                let s = String::from_utf8_lossy(&bytes);
                Ok((header, serde_json::from_str::<Self>(&s)?))
            }
        }
    }

    #[inline]
    fn validate_claims(&self, header: serde_json::Value) -> Result<String, RauthyError> {
        // let alg = JwkKeyPairAlg::from_str(
        //     header
        //         .get("alg")
        //         .map(|a| a.as_str().unwrap_or_default())
        //         .unwrap_or_default(),
        // )?;

        let kid = header
            .get("kid")
            .map(|a| a.as_str().unwrap_or_default())
            .unwrap_or_default()
            .to_string();
        if kid.is_empty() {
            return Err(RauthyError::BadRequest("`kid` is missing in token header"));
        }

        if self.typ.is_some() && self.typ != Some(JwtTokenType::Logout) {
            return Err(RauthyError::BadRequest("invalid token `typ`"));
        }

        let now = Utc::now().timestamp();
        let iat_limit =
            now - LOGOUT_TOKEN_ALLOWED_LIFETIME as i64 - LOGOUT_TOKEN_ALLOW_CLOCK_SKEW as i64;
        let exp_limit = now + LOGOUT_TOKEN_ALLOW_CLOCK_SKEW as i64;

        if self.iat < iat_limit {
            return Err(RauthyError::BadRequest("`iat` is too long ago"));
        }
        if self.exp < exp_limit {
            return Err(RauthyError::BadRequest("token has expired"));
        }
        if self.exp <= self.iat {
            return Err(RauthyError::BadRequest("`exp` must be greater than `iat`"));
        }

        if self.sub.is_none() && self.sid.is_none() {
            return Err(RauthyError::BadRequest(
                "at least one of `sub` or `sid` must be given",
            ));
        }
        match self.events.get(EVENT) {
            None => {
                return Err(RauthyError::BadRequest(
                    "`events.http://schemas.openid.net/event/backchannel-logout` is missing",
                ));
            }
            Some(value) => {
                if value != &serde_json::Value::Object(serde_json::Map::new()) {
                    return Err(RauthyError::BadRequest("invalid value for `events.http://schemas.openid.net/event/backchannel-logout`"));
                }
            }
        }
        if self.nonce.is_some() {
            return Err(RauthyError::BadRequest("`nonce` MUST NOT be given"));
        }

        // We don't really need to care about `jti` caching and validation. Replay attacks can only
        // happen for unencrypted connections, which Rauthy will never work with anyway.

        Ok(kid)
    }
}
