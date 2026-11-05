use crate::entity::sessions::Session;
use crate::rauthy_config::RauthyConfig;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::{HttpRequest, cookie};
use chrono::Utc;
use cryptr::EncValue;
use rauthy_common::utils::{
    base64_decode, base64_encode, base64_url_no_pad_decode, base64_url_no_pad_encode, deserialize,
    real_ip_from_req, serialize,
};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::net::IpAddr;
use tracing::{debug, warn};

/// Prefix for the session id inside the forward-auth cookie. The upstream app sees that cookie, so
/// its value must never decrypt to a plain session id, which would be a valid
/// `__Host-RauthySession`. Session ids are alphanumeric, so a tagged value never matches one.
const FWD_AUTH_TAG: &str = "fwda:";

#[derive(Debug)]
pub struct ForwardAuthSession {
    pub client_id: String,
    pub inner: Session,
}

impl ForwardAuthSession {
    #[inline]
    pub fn build_cookies(
        &self,
        danger_insecure: bool,
    ) -> Result<(Cookie<'_>, Cookie<'_>), ErrorResponse> {
        let max_age = self.inner.exp - Utc::now().timestamp();

        let tagged_sid = tag_sid(&self.client_id, &self.inner.id);
        let cookie_session = build_cookie(
            Self::cookie_name(danger_insecure),
            tagged_sid.as_bytes(),
            max_age,
            SameSite::Lax,
            danger_insecure,
        )?;
        let cookie_csrf = build_cookie(
            Self::cookie_name_csrf(danger_insecure),
            self.inner.csrf_token.as_bytes(),
            max_age,
            SameSite::Strict,
            danger_insecure,
        )?;

        Ok((cookie_session, cookie_csrf))
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
        let plain = String::from_utf8_lossy(decrypted.as_ref());
        // an untagged value is a main session cookie, which is never accepted here
        let (client_id, sid) = untag_sid(&plain)?;

        let session = Session::find(sid.to_string()).await.ok()?;

        Some(Self {
            client_id: client_id.to_string(),
            inner: session,
        })
    }

    #[inline]
    pub fn validate(&self, req: &HttpRequest, client_id: &str) -> Result<(), ErrorResponse> {
        let remote_ip = if RauthyConfig::get().vars.access.session_validate_ip {
            real_ip_from_req(req).ok()
        } else {
            None
        };

        if self.client_id != client_id {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "client.id mismatch",
            ));
        }

        let session_timeout = RauthyConfig::get().vars.lifetimes.session_timeout.as_secs() as i64;
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

        if !constant_time_eq::constant_time_eq(token.as_bytes(), self.inner.csrf_token.as_bytes()) {
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
                debug!(site, "sec-fetch-site forbidden");
                Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "CORS requests forbidden",
                ))
            };
        }

        Ok(())
    }
}

#[inline]
fn tag_sid(client_id: &str, sid: &str) -> String {
    // dynamic clients will have a single `$` in their generated id
    format!("{FWD_AUTH_TAG}{client_id}$${sid}")
}

/// Returns `Some(client_id, session_id)`
#[inline]
fn untag_sid(plain: &str) -> Option<(&str, &str)> {
    let stripped = plain.strip_prefix(FWD_AUTH_TAG)?;
    let (cid, sid) = stripped.split_once("$$")?;
    Some((cid, sid))
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
    name: &'a str,
    value: &[u8],
    max_age: i64,
    same_site: SameSite,
    danger_insecure: bool,
) -> Result<Cookie<'a>, ErrorResponse> {
    let max_age = if max_age < 1 {
        cookie::time::Duration::ZERO
    } else {
        cookie::time::Duration::seconds(max_age)
    };

    let enc = EncValue::encrypt(value)?;
    let value_b64 = base64_encode(enc.into_bytes().as_ref());

    Ok(Cookie::build(name, value_b64)
        .secure(!danger_insecure)
        .http_only(true)
        .same_site(same_site)
        .max_age(max_age)
        .path("/")
        .finish())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api_cookie::ApiCookie;
    use rauthy_common::utils::get_rand;

    #[test]
    fn test_fwd_auth_sid_is_domain_separated() {
        let cid = "my_client";
        let sid = get_rand(32);
        let tagged = tag_sid(cid, &sid);

        // the ext cookie value never equals a session id, so it can't be replayed as one
        assert_ne!(tagged, sid);
        assert!(tagged.contains(':'));
        assert!(sid.chars().all(|c| c.is_ascii_alphanumeric()));

        assert_eq!(untag_sid(&tagged), Some((cid, sid.as_str())));
        // and a main session cookie value is rejected as an ext cookie
        assert_eq!(untag_sid(&sid), None);
    }

    #[test]
    fn test_fwd_auth_cookie_is_not_a_session_cookie() {
        let _ = cryptr::EncKeys::generate_with_id("test".to_string())
            .unwrap()
            .init();

        let fwd = ForwardAuthSession {
            client_id: "my_client".to_string(),
            inner: Session::new(std::time::Duration::from_secs(60), None),
        };
        let (cookie, _) = fwd.build_cookies(true).unwrap();
        let value = ApiCookie::cookie_into_value(Some(cookie)).unwrap();

        assert_ne!(value, fwd.inner.id);
        assert_eq!(
            untag_sid(&value),
            Some((fwd.client_id.as_str(), fwd.inner.id.as_str()))
        );
    }
}
