use crate::rauthy_error::RauthyError;
use crate::tokens::claims::{CommonClaims, TokenType};
use crate::tokens::token::JwtToken;
use chrono::Utc;
use serde::{Deserialize, Serialize};

const EVENT: &str = "http://schemas.openid.net/event/backchannel-logout";
const LOGOUT_TOKEN_ALLOWED_LIFETIME: u8 = 120;
const LOGOUT_TOKEN_ALLOW_CLOCK_SKEW: u8 = 5;

/// Logout Token for OIDC backchannel logout specified in
/// <https://openid.net/specs/openid-connect-backchannel-1_0.html#LogoutToken>
#[derive(Debug, Serialize, Deserialize)]
pub struct LogoutToken {
    #[serde(flatten)]
    pub common: CommonClaims,

    events: serde_json::Value,
    pub sid: Option<String>,

    // The `nonce` MUST NOT exist in this token. We try to deserialize into an `Option<_>` for easy
    // `.is_none()` validation.
    nonce: Option<String>,
}

impl LogoutToken {
    #[inline(always)]
    pub async fn from_token_validated(token: &str) -> Result<Self, RauthyError> {
        let mut buf = Vec::with_capacity(256);
        JwtToken::validate_claims_into(token, Some(TokenType::Logout), &mut buf).await?;
        let slf = serde_json::from_slice::<Self>(&buf)?;

        match slf.events.get(EVENT) {
            None => {
                return Err(RauthyError::BadRequest(
                    "`events.http://schemas.openid.net/event/backchannel-logout` is missing",
                ));
            }
            Some(value) => {
                if value != &serde_json::Value::Object(serde_json::Map::new()) {
                    return Err(RauthyError::BadRequest(
                        "invalid value for `events.http://schemas.openid.net/event/backchannel-logout`",
                    ));
                }
            }
        }
        if slf.nonce.is_some() {
            return Err(RauthyError::BadRequest("`nonce` MUST NOT be given"));
        }

        let now = Utc::now().timestamp();
        let iat_limit =
            now - LOGOUT_TOKEN_ALLOWED_LIFETIME as i64 - LOGOUT_TOKEN_ALLOW_CLOCK_SKEW as i64;
        let exp_limit = now + LOGOUT_TOKEN_ALLOW_CLOCK_SKEW as i64;

        if slf.common.iat < iat_limit {
            return Err(RauthyError::BadRequest("`iat` is too long ago"));
        }
        if slf.common.exp < exp_limit {
            return Err(RauthyError::BadRequest("token has expired"));
        }
        if slf.common.exp <= slf.common.iat {
            return Err(RauthyError::BadRequest("`exp` must be greater than `iat`"));
        }

        Ok(slf)
    }
}
