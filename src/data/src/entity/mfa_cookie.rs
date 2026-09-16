use std::ops::Add;

use actix_web::cookie::Cookie;
use chrono::Utc;
use cryptr::EncValue;
use rauthy_common::{
    constants::COOKIE_MFA,
    utils::{base64_decode, base64_encode, deserialize, serialize},
};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::{api_cookie::ApiCookie, rauthy_config::RauthyConfig};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MfaCookie {
    pub email: String,
    pub exp: OffsetDateTime,
}

impl MfaCookie {
    pub fn new_webauthn(email: String) -> Self {
        let renew = RauthyConfig::get().vars.webauthn.renew_exp as i64;
        let exp = OffsetDateTime::now_utc().add(::time::Duration::hours(renew));
        Self { email, exp }
    }

    pub fn new_otp(email: String) -> Self {
        let renew = RauthyConfig::get().vars.otp.renew_exp as i64;
        let exp = OffsetDateTime::now_utc().add(::time::Duration::hours(renew));
        Self { email, exp }
    }

    pub fn build(&self) -> Result<Cookie<'_>, ErrorResponse> {
        let set = serialize(self)?;
        let enc = EncValue::encrypt(&set)?.into_bytes();
        let b64 = base64_encode(&enc);

        let max_age = self.exp.unix_timestamp() - Utc::now().timestamp();
        Ok(ApiCookie::build(COOKIE_MFA, b64, max_age))
    }

    pub fn parse_validate(cookie: &Option<String>) -> Result<Self, ErrorResponse> {
        if cookie.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "MFA Cookie is missing",
            ));
        }
        let cookie = cookie.as_ref().unwrap();
        let bytes = base64_decode(cookie)?;
        let dec = EncValue::try_from(bytes)?.decrypt()?;
        let slf = deserialize::<Self>(&dec)?;

        if slf.exp < OffsetDateTime::now_utc() {
            Err(ErrorResponse::new(
                ErrorResponseType::SessionExpired,
                "MFA Cookie has expired",
            ))
        } else {
            Ok(slf)
        }
    }
}
