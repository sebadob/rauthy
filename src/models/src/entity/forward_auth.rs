use crate::entity::sessions::Session;
use crate::rauthy_config::RauthyConfig;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpRequest, cookie};
use chrono::Utc;
use cryptr::EncValue;
use rauthy_common::utils::{
    base64_decode, base64_encode, base64_url_no_pad_decode, base64_url_no_pad_encode, deserialize,
    serialize,
};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::net::IpAddr;
use tracing::{debug, warn};

#[derive(Debug)]
pub struct ForwardAuthSession {
    pub inner: Session,
}

impl ForwardAuthSession {
    #[inline]
    pub fn build_cookies(&self, danger_insecure: bool) -> (Cookie<'_>, Cookie<'_>) {
        let max_age = self.inner.exp - Utc::now().timestamp();

        let cookie_session = build_cookie(
            Self::cookie_name(danger_insecure),
            self.inner.id.as_bytes(),
            max_age,
            SameSite::Lax,
            danger_insecure,
        );
        let cookie_csrf = build_cookie(
            Self::cookie_name_csrf(danger_insecure),
            self.inner.csrf_token.as_bytes(),
            max_age,
            SameSite::Strict,
            danger_insecure,
        );

        (cookie_session, cookie_csrf)
    }

    #[inline]
    fn cookie_name(danger_insecure: bool) -> &'static str {
        if danger_insecure {
            "Rauthy-Ext-Session"
        } else {
            "__Host-Rauthy-Ext-Session"
        }
    }

    #[inline]
    fn cookie_name_csrf(danger_insecure: bool) -> &'static str {
        if danger_insecure {
            "Rauthy-Ext-Session-CSRF"
        } else {
            "__Host-Rauthy-Ext-Session-CSRF"
        }
    }

    #[inline]
    pub async fn from_cookie(req: &HttpRequest, danger_insecure: bool) -> Option<Self> {
        let cookie = req.cookie(Self::cookie_name(danger_insecure))?;
        let decoded = base64_decode(cookie.value()).ok()?;
        let decrypted = EncValue::try_from_bytes(decoded).ok()?.decrypt().ok()?;
        let sid = String::from_utf8_lossy(decrypted.as_ref());

        let session = Session::find(sid.to_string()).await.ok()?;

        Some(Self { inner: session })
    }

    #[inline]
    pub fn validate(&self, remote_ip: Option<IpAddr>) -> Result<(), ErrorResponse> {
        if RauthyConfig::get().vars.access.session_validate_ip && remote_ip.is_none() {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Missing peer IP in session validation",
            ));
        }

        let session_timeout = RauthyConfig::get().vars.lifetimes.session_timeout;
        if !self
            .inner
            .is_valid(session_timeout, remote_ip, "NO_EXCEPTION")
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "invalid SessionState",
            ));
        }

        Ok(())
    }

    #[inline]
    pub fn validate_csrf(
        &self,
        req: &HttpRequest,
        danger_insecure: bool,
    ) -> Result<(), ErrorResponse> {
        let Some(cookie) = req.cookie(Self::cookie_name_csrf(danger_insecure)) else {
            debug!("CSRF cookie missing");
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "CSRF Token missing",
            ));
        };
        let decoded = base64_decode(cookie.value())?;
        let decrypted = EncValue::try_from_bytes(decoded)?.decrypt()?;
        let token = String::from_utf8_lossy(decrypted.as_ref());

        if token != self.inner.csrf_token {
            warn!("CSRF Token mismatch");
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "CSRF Token mismatch",
            ));
        };

        if let Some(site) = req.headers().get("sec-fetch-site") {
            let site = site.to_str().unwrap_or_default();

            // site == "none" will be set for user-originated operations
            return if site == "none" || site == "same-origin" {
                Ok(())
            } else {
                debug!("sec-fetch-site forbidden: {}", site);
                Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "CORS requests forbidden",
                ))
            };
        }

        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ForwardAuthCallbackState {
    pub client_id: String,
    pub peer_ip: IpAddr,
    pub origin: String,
    pub forwarded_uri: String,
    pub danger_cookie_insecure: bool,
}

impl TryFrom<&str> for ForwardAuthCallbackState {
    type Error = ErrorResponse;

    #[inline]
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let decoded = base64_url_no_pad_decode(s)?;
        let decrypted = EncValue::try_from_bytes(decoded)?.decrypt()?;
        deserialize(decrypted.as_ref())
    }
}

impl ForwardAuthCallbackState {
    #[inline]
    pub fn encrypted_str(&self) -> Result<String, ErrorResponse> {
        let json = serialize(self)?;
        let enc = EncValue::encrypt(&json)?;
        let b64 = base64_url_no_pad_encode(enc.into_bytes().as_ref());
        Ok(b64)
    }
}

#[inline]
fn build_cookie<'a>(
    name: &'static str,
    value: &'a [u8],
    max_age: i64,
    same_site: SameSite,
    danger_insecure: bool,
) -> Cookie<'a> {
    let max_age = if max_age < 1 {
        cookie::time::Duration::ZERO
    } else {
        cookie::time::Duration::seconds(max_age)
    };

    let enc = EncValue::encrypt(value).unwrap();
    let value_b64 = base64_encode(enc.into_bytes().as_ref());

    Cookie::build(name, value_b64)
        .secure(!danger_insecure)
        .http_only(true)
        .same_site(same_site)
        .max_age(max_age)
        .path("/")
        .finish()
}
