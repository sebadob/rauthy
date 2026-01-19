use crate::rauthy_config::RauthyConfig;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::dev::Payload;
use actix_web::{HttpRequest, cookie};
use cryptr::utils::secure_random_alnum;
use rauthy_common::constants::CookieMode;
use rauthy_error::ErrorResponse;
use std::fmt::{Display, Formatter};
use std::pin::Pin;
use tracing::warn;

static COOKIE_NAME: &str = "rbid";
static COOKIE_NAME_HOST: &str = "__Host-rbid";
static COOKIE_NAME_SECURE: &str = "__Secure-rbid";

#[derive(Debug, PartialEq)]
pub enum BrowserIdSetNew {
    Yes,
    No,
}

#[derive(Debug, Default)]
pub struct BrowserId(Option<String>);

impl Display for BrowserId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let v = self.0.as_deref().unwrap_or("None");
        write!(f, "BrowserId({v})")
    }
}

impl BrowserId {
    pub fn new_cookie<'a>() -> Cookie<'a> {
        let access = &RauthyConfig::get().vars.access;
        let path = if access.cookie_set_path { "/auth" } else { "/" };
        let (name, secure, path) = match access.cookie_mode {
            CookieMode::Host => (COOKIE_NAME_HOST, true, "/"),
            CookieMode::Secure => (COOKIE_NAME_SECURE, true, path),
            CookieMode::DangerInsecure => {
                warn!("Building INSECURE cookie - you MUST NEVER use this in production");
                (COOKIE_NAME, false, path)
            }
        };
        let value = secure_random_alnum(32);

        Cookie::build(name, value)
            .secure(secure)
            .http_only(true)
            .same_site(SameSite::Lax)
            .max_age(cookie::time::Duration::days(5 * 365))
            .path(path)
            .finish()
    }

    pub fn as_str(&self) -> Option<&str> {
        self.0.as_deref()
    }

    pub fn inner(self) -> Option<String> {
        self.0
    }

    pub fn needs_set_new(&self) -> BrowserIdSetNew {
        if self.0.is_some() {
            BrowserIdSetNew::No
        } else {
            BrowserIdSetNew::Yes
        }
    }
}

impl actix_web::FromRequest for BrowserId {
    type Error = ErrorResponse;
    type Future = Pin<Box<dyn Future<Output = Result<Self, Self::Error>>>>;

    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let name = match RauthyConfig::get().vars.access.cookie_mode {
            CookieMode::Host => COOKIE_NAME_HOST,
            CookieMode::Secure => COOKIE_NAME_SECURE,
            CookieMode::DangerInsecure => COOKIE_NAME,
        };
        let cookie = req.cookie(name);

        Box::pin(async move {
            if let Some(cookie) = cookie {
                Ok(Self(Some(cookie.value().to_string())))
            } else {
                Ok(Self(None))
            }
        })
    }
}
