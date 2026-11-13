// Copyright 2026 Sebastian Dobe <sebastiandobe@mailbox.org>

use crate::entity::sessions::Session;
use actix_web::http::header::{HeaderName, HeaderValue};
use rauthy_api_types::users::ActiveOtp;
use std::fmt::{Display, Formatter};

pub mod api_cookie;
pub mod database;
pub mod email;
pub mod entity;
pub mod events;
pub mod fido_mds;
pub mod html;
pub mod http_client;
pub mod ipgeo;
pub mod language;
pub mod migration;
pub mod rauthy_config;
pub mod secrets;
pub mod temp_migrations;
pub mod vault_config;

/// Test encryption key IDs. Every unit test in this crate must init keys through
/// `init_test_enc_keys`, because `EncKeys::init` only succeeds once per process.
#[cfg(test)]
pub(crate) const TEST_ENC_KEY_ID: &str = "test";
#[cfg(test)]
pub(crate) const TEST_ENC_KEY_ID_ACTIVE: &str = "testactive";

/// Inits static `EncKeys` with two keys, where the active one is NOT `TEST_ENC_KEY_ID`.
#[cfg(test)]
pub(crate) fn init_test_enc_keys() {
    static INIT: std::sync::Once = std::sync::Once::new();
    INIT.call_once(|| {
        let mut keys = cryptr::EncKeys::generate_with_id(TEST_ENC_KEY_ID.to_string()).unwrap();
        // appending makes the new key the active one
        keys.append_new_random_with_id(TEST_ENC_KEY_ID_ACTIVE.to_string())
            .unwrap();
        keys.init().unwrap();
    });
}

pub enum AuthStep {
    LoggedIn(AuthStepLoggedIn),
    AwaitToSAccept(AwaitToSAccept),
    AwaitMfa(AuthStepAwaitMfa),
    ProviderLink,
}

pub struct AuthStepLoggedIn {
    pub user_id: String,
    pub email: String,
    pub header_loc: (HeaderName, HeaderValue),
    pub header_csrf: (HeaderName, HeaderValue),
    pub header_origin: Option<(HeaderName, HeaderValue)>,
    pub needs_user_update: bool,
}

pub struct AwaitToSAccept {
    pub code: String,
    pub header_csrf: (HeaderName, HeaderValue),
    pub header_origin: Option<(HeaderName, HeaderValue)>,
    pub user_id: String,
}

pub struct AuthStepAwaitMfa {
    pub code: String,
    pub header_csrf: (HeaderName, HeaderValue),
    pub header_origin: Option<(HeaderName, HeaderValue)>,
    pub email: String,
    pub require_webauthn: bool,
    pub require_otp: bool,
    pub active_otps: Vec<ActiveOtp>,
    pub session: Session,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestId {
    pub id: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListenScheme {
    Http,
    Https,
    HttpHttps,
    #[cfg(not(target_os = "windows"))]
    UnixHttp,
    #[cfg(not(target_os = "windows"))]
    UnixHttps,
}

impl Display for ListenScheme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ListenScheme::Http => write!(f, "http"),
            ListenScheme::Https => write!(f, "https"),
            ListenScheme::HttpHttps => write!(f, "{{http|https}}"),
            #[cfg(not(target_os = "windows"))]
            ListenScheme::UnixHttp => write!(f, "unix+http"),
            #[cfg(not(target_os = "windows"))]
            ListenScheme::UnixHttps => write!(f, "unix+https"),
        }
    }
}
