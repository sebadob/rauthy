use crate::rauthy_config::RauthyConfig;
use actix_web::cookie::{Cookie, SameSite};
use actix_web::dev::ServiceRequest;
use actix_web::{HttpRequest, cookie};
use cryptr::EncValue;
use rauthy_common::constants::CookieMode;
use rauthy_common::utils::{base64_decode, base64_encode};
use std::borrow::Cow;
use std::fmt::Display;
use tracing::warn;

pub struct ApiCookie;

impl ApiCookie {
    pub fn build<'c, 'b, N, V>(name: N, value: V, max_age: i64) -> Cookie<'c>
    where
        N: Into<Cow<'c, str>> + Display,
        V: Into<Cow<'b, str>> + Display,
    {
        Self::build_with_same_site(name, value, max_age, SameSite::Lax)
    }

    pub fn build_with_same_site<'c, 'b, N, V>(
        name: N,
        value: V,
        max_age: i64,
        same_site: SameSite,
    ) -> Cookie<'c>
    where
        N: Into<Cow<'c, str>> + Display,
        V: Into<Cow<'b, str>> + Display,
    {
        let access = &RauthyConfig::get().vars.access;
        let path = if access.cookie_set_path { "/auth" } else { "/" };
        let (name, secure, path) = match access.cookie_mode {
            CookieMode::Host => (format!("__Host-{name}"), true, "/"),
            CookieMode::Secure => (format!("__Secure-{name}"), true, path),
            CookieMode::DangerInsecure => {
                warn!("Building INSECURE cookie - you MUST NEVER use this in production");
                (name.to_string(), false, path)
            }
        };
        let max_age = if max_age < 1 {
            cookie::time::Duration::ZERO
        } else {
            cookie::time::Duration::seconds(max_age)
        };

        // we always encrypt any cookie value
        let enc =
            EncValue::encrypt(value.into().as_bytes()).expect("ENC_VALUES not set up correctly");
        let value_b64 = base64_encode(enc.into_bytes().as_ref());

        Cookie::build(name, value_b64)
            .secure(secure)
            .http_only(true)
            .same_site(same_site)
            .max_age(max_age)
            .path(path)
            .finish()
    }

    pub fn from_req<'c, N>(req: &HttpRequest, cookie_name: N) -> Option<String>
    where
        N: Into<Cow<'c, str>> + Display,
    {
        let name = match RauthyConfig::get().vars.access.cookie_mode {
            CookieMode::Host => format!("__Host-{cookie_name}"),
            CookieMode::Secure => format!("__Secure-{cookie_name}"),
            CookieMode::DangerInsecure => cookie_name.to_string(),
        };
        // req.cookie(&name)
        Self::cookie_into_value(req.cookie(&name))
    }

    pub fn from_svc_req<'c, N>(req: &ServiceRequest, cookie_name: N) -> Option<String>
    where
        N: Into<Cow<'c, str>> + Display,
    {
        let name = match RauthyConfig::get().vars.access.cookie_mode {
            CookieMode::Host => format!("__Host-{cookie_name}"),
            CookieMode::Secure => format!("__Secure-{cookie_name}"),
            CookieMode::DangerInsecure => cookie_name.to_string(),
        };
        Self::cookie_into_value(req.cookie(&name))
    }

    pub fn cookie_into_value(cookie: Option<Cookie>) -> Option<String> {
        match cookie {
            None => None,
            Some(cookie) => {
                let val = cookie.value();
                let bytes = base64_decode(val).ok()?;
                let enc = EncValue::try_from(bytes).ok()?;
                let dec = enc.decrypt().ok()?;
                Some(String::from_utf8_lossy(dec.as_ref()).to_string())
            }
        }
    }
}
