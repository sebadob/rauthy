// Copyright 2025 Sebastian Dobe <sebastiandobe@mailbox.org>

use crate::entity::sessions::Session;
use actix_web::http::header::{HeaderName, HeaderValue};
use std::fmt::{Display, Formatter};

pub mod api_cookie;
mod app_config;
pub mod app_state;
pub mod database;
pub mod email;
pub mod entity;
pub mod events;
pub mod html;
pub mod i18n_email;
pub mod language;
pub mod migration;
pub mod temp_migrate_federation_idp;

pub enum AuthStep {
    LoggedIn(AuthStepLoggedIn),
    AwaitWebauthn(AuthStepAwaitWebauthn),
    ProviderLink,
}

pub struct AuthStepLoggedIn {
    pub user_id: String,
    pub email: String,
    pub header_loc: (HeaderName, HeaderValue),
    pub header_csrf: (HeaderName, HeaderValue),
    pub header_origin: Option<(HeaderName, HeaderValue)>,
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
