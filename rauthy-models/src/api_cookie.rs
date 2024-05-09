use actix_web::cookie::{Cookie, SameSite};
use actix_web::dev::ServiceRequest;
use actix_web::{cookie, HttpRequest};
use rauthy_common::constants::{CookieMode, COOKIE_MODE, COOKIE_SET_PATH};
use std::borrow::Cow;
use std::fmt::Display;
use tracing::warn;

pub struct ApiCookie;

impl ApiCookie {
    pub fn build<'c, N, V>(name: N, value: V, max_age: i64) -> Cookie<'c>
    where
        N: Into<Cow<'c, str>> + Display,
        V: Into<Cow<'c, str>> + Display,
    {
        let path = if *COOKIE_SET_PATH { "/auth" } else { "/" };
        let (name, secure, path) = match *COOKIE_MODE {
            CookieMode::Host => (format!("__Host-{}", name), true, "/"),
            CookieMode::Secure => (format!("__Secure-{}", name), true, path),
            CookieMode::DangerInsecure => {
                warn!("Building INSECURE session cookie - you MUST NEVER use this in production");
                (name.to_string(), false, path)
            }
        };
        let max_age = if max_age < 1 {
            cookie::time::Duration::ZERO
        } else {
            cookie::time::Duration::seconds(max_age)
        };

        Cookie::build(name, value)
            .secure(secure)
            .http_only(true)
            .same_site(SameSite::Lax)
            .max_age(max_age)
            .path(path)
            .finish()
    }

    pub fn from_req<'c, N>(req: &HttpRequest, cookie_name: N) -> Option<Cookie<'c>>
    where
        N: Into<Cow<'c, str>> + Display,
    {
        let name = match *COOKIE_MODE {
            CookieMode::Host => format!("__Host-{}", cookie_name),
            CookieMode::Secure => format!("__Secure-{}", cookie_name),
            CookieMode::DangerInsecure => cookie_name.to_string(),
        };
        req.cookie(&name)
    }

    pub fn from_svc_req<'c, N>(req: &ServiceRequest, cookie_name: N) -> Option<Cookie<'c>>
    where
        N: Into<Cow<'c, str>> + Display,
    {
        let name = match *COOKIE_MODE {
            CookieMode::Host => format!("__Host-{}", cookie_name),
            CookieMode::Secure => format!("__Secure-{}", cookie_name),
            CookieMode::DangerInsecure => cookie_name.to_string(),
        };
        req.cookie(&name)
    }
}
