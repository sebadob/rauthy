// Copyright 2026 Sebastian Dobe <sebastiandobe@mailbox.org>

use crate::entity::sessions::Session;
use actix_web::http::header::{HeaderName, HeaderValue};
use std::fmt::{Display, Formatter};

pub mod api_cookie;
pub mod database;
pub mod email;
pub mod entity;
pub mod events;
pub mod html;
pub mod ipgeo;
pub mod language;
pub mod migration;
pub mod rauthy_config;
pub mod temp_migrations;
pub mod vault_config;

pub enum AuthStep {
    LoggedIn(AuthStepLoggedIn),
    AwaitToSAccept(AwaitToSAccept),
    AwaitWebauthn(AuthStepAwaitWebauthn),
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

pub struct AuthStepAwaitWebauthn {
    pub code: String,
    pub header_csrf: (HeaderName, HeaderValue),
    pub header_origin: Option<(HeaderName, HeaderValue)>,
    pub user_id: String,
    pub email: String,
    pub exp: u64,
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
