use crate::database::{Cache, DB};
use chrono::Utc;
use cryptr::utils::secure_random_alnum;
use rauthy_api_types::users::MfaModTokenResponse;
use rauthy_common::constants::IDX_MFA_MOD;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use std::ops::Add;

/// This token will be validated during MFA keys modification for a user.
#[derive(Debug, Serialize, Deserialize)]
pub struct MfaModToken {
    pub id: String,
    pub user_id: String,
    pub exp: i64,
    pub ip: IpAddr,
}

impl MfaModToken {
    pub async fn new(user_id: String, ip: IpAddr) -> Result<Self, ErrorResponse> {
        // TODO make configurable
        let exp_secs = 120;

        let slf = Self {
            id: secure_random_alnum(32),
            user_id,
            exp: Utc::now()
                .add(chrono::Duration::seconds(exp_secs))
                .timestamp(),
            ip,
        };

        DB::hql()
            .put(
                Cache::Webauthn,
                Self::cache_idx(&slf.id),
                &slf,
                Some(exp_secs + 1),
            )
            .await?;

        Ok(slf)
    }

    pub async fn find(id: &str) -> Result<Self, ErrorResponse> {
        if let Some(slf) = DB::hql().get(Cache::Webauthn, Self::cache_idx(id)).await? {
            Ok(slf)
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid MfaModToken id",
            ))
        }
    }
}

impl MfaModToken {
    #[inline]
    pub fn cache_idx(id: &str) -> String {
        format!("{}{}", IDX_MFA_MOD, id)
    }

    pub fn validate(&self, user_id: &str, ip: IpAddr) -> Result<(), ErrorResponse> {
        if self.exp < Utc::now().timestamp() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "MfaMofToken has expired",
            ));
        }

        if self.user_id != user_id {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "MfaMofToken is bound to a different User",
            ));
        }

        if self.ip != ip {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "MfaMofToken is bound to a different IP",
            ));
        }

        Ok(())
    }
}

impl From<MfaModToken> for MfaModTokenResponse {
    fn from(t: MfaModToken) -> Self {
        Self {
            id: t.id,
            user_id: t.user_id,
            exp: t.exp,
            ip: t.ip.to_string(),
        }
    }
}
