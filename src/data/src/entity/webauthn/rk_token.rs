use crate::database::{Cache, DB};
use crate::entity::browser_id::BrowserId;
use crate::entity::sessions::Session;
use chrono::Utc;
use rauthy_common::utils::get_rand;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::ops::Add;

/// This token is used for logging in via Resident Keys only.
/// It must be fetched before the actual POST to `/authorize`, and be provided alongside.
#[derive(Debug, Serialize, Deserialize)]
pub struct ResidentKeyToken {
    pub code: String,
    pub exp: i64,
    pub session_id: String,
    pub browser_id: String,
}

impl ResidentKeyToken {
    const LEN: usize = 64;
    const TTL_SECS: i64 = 15;

    fn cache_idx(code: &str) -> String {
        format!("rk_tok_{code}")
    }

    pub async fn new(session_id: String, browser_id: BrowserId) -> Result<Self, ErrorResponse> {
        let slf = Self {
            code: get_rand(Self::LEN),
            exp: Utc::now()
                .add(chrono::Duration::seconds(Self::TTL_SECS))
                .timestamp(),
            session_id,
            browser_id: browser_id.as_str().unwrap_or_default().to_string(),
        };

        DB::hql()
            .put(
                Cache::Webauthn,
                Self::cache_idx(&slf.code),
                &slf,
                Some(Self::TTL_SECS),
            )
            .await?;

        Ok(slf)
    }

    pub async fn get_validated_user_id(
        code: &str,
        session: &Session,
        browser_id: &BrowserId,
    ) -> Result<String, ErrorResponse> {
        if code.len() != Self::LEN {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid Resident Key Token",
            ));
        }

        let Some(slf) = DB::hql()
            .get_remove::<_, _, Self>(Cache::Webauthn, Self::cache_idx(code))
            .await?
        else {
            return Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Resident Key Token not found",
            ));
        };

        if slf.exp < Utc::now().timestamp() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Resident Key has expired",
            ));
        }
        if slf.session_id != session.id {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Resident Key session ID mismatch",
            ));
        }
        // A user can only have a resident key token if the session is authenticated and
        // MFA already. This is set during `auth_finish_discover()` in the step before.
        if !session.is_mfa || session.user_id.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid Session / Resident Key token combination",
            ));
        }

        if slf.browser_id != browser_id.as_str().unwrap_or_default() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Resident Key browser ID mismatch",
            ));
        }

        Ok(session.user_id.clone().unwrap())
    }
}
