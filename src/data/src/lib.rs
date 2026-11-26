// Copyright 2026 Sebastian Dobe <sebastiandobe@mailbox.org>

use crate::entity::sessions::Session;
use actix_web::http::header::{HeaderName, HeaderValue};
use rauthy_api_types::users::ActiveOtp;
use std::fmt::{Display, Formatter};
use std::ops::BitOrAssign;

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
pub mod shutdown;
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

/// A single second factor a user can authenticate with. The value is its bit in `MfaMethods`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum MfaMethod {
    Webauthn = 0b01,
    Otp = 0b10,
}

impl MfaMethod {
    /// All methods, in the order `MfaMethods::iter()` yields them: Webauthn first.
    const ALL: [Self; 2] = [Self::Webauthn, Self::Otp];
}

/// The second factors a login may be completed with. An empty set means no MFA is required.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct MfaMethods(u8);

impl MfaMethods {
    pub const fn empty() -> Self {
        Self(0)
    }

    pub const fn is_empty(self) -> bool {
        self.0 == 0
    }

    pub const fn contains(self, method: MfaMethod) -> bool {
        self.0 & method as u8 != 0
    }

    /// The contained methods, always in the same order: Webauthn first.
    pub fn iter(self) -> impl Iterator<Item = MfaMethod> {
        MfaMethod::ALL
            .into_iter()
            .filter(move |m| self.contains(*m))
    }
}

impl BitOrAssign<MfaMethod> for MfaMethods {
    fn bitor_assign(&mut self, method: MfaMethod) {
        self.0 |= method as u8;
    }
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
    pub mfa_methods: MfaMethods,
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

#[cfg(test)]
mod mfa_methods_tests {
    use super::{MfaMethod, MfaMethods};

    #[test]
    fn empty_contains_nothing() {
        let methods = MfaMethods::empty();
        assert!(methods.is_empty());
        assert!(!methods.contains(MfaMethod::Webauthn));
        assert!(!methods.contains(MfaMethod::Otp));
        assert_eq!(methods.iter().count(), 0);
        assert_eq!(methods, MfaMethods::default());
    }

    #[test]
    fn insert_sets_only_that_method() {
        let mut methods = MfaMethods::empty();
        methods |= MfaMethod::Otp;
        assert!(!methods.is_empty());
        assert!(methods.contains(MfaMethod::Otp));
        assert!(!methods.contains(MfaMethod::Webauthn));
        assert_eq!(methods.iter().collect::<Vec<_>>(), [MfaMethod::Otp]);

        // inserting twice changes nothing
        let once = methods;
        methods |= MfaMethod::Otp;
        assert_eq!(methods, once);
    }

    #[test]
    fn iter_yields_webauthn_first() {
        let mut methods = MfaMethods::empty();
        methods |= MfaMethod::Otp;
        methods |= MfaMethod::Webauthn;
        assert_eq!(
            methods.iter().collect::<Vec<_>>(),
            [MfaMethod::Webauthn, MfaMethod::Otp]
        );
    }
}
