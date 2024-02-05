// Copyright 2024 Sebastian Dobe <sebastiandobe@mailbox.org>

#![forbid(unsafe_code)]

use crate::entity::sessions::Session;
use crate::entity::users::User;
use crate::entity::users_values::UserValues;
use actix_web::http::header::{HeaderName, HeaderValue};
use actix_web::HttpRequest;
use rauthy_common::constants::PROXY_MODE;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use utoipa::ToSchema;

pub mod app_state;
pub mod email;
pub mod entity;
pub mod events;
pub mod i18n;
pub mod language;
pub mod migration;
pub mod request;
pub mod response;
pub mod templates;

pub enum AuthStep {
    LoggedIn(AuthStepLoggedIn),
    AwaitWebauthn(AuthStepAwaitWebauthn),
}

pub struct AuthStepLoggedIn {
    pub has_password_been_hashed: bool,
    pub email: String,
    pub header_loc: (HeaderName, HeaderValue),
    pub header_csrf: (HeaderName, HeaderValue),
    pub header_origin: Option<(HeaderName, HeaderValue)>,
}

pub struct AuthStepAwaitWebauthn {
    pub has_password_been_hashed: bool,
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
}

impl Display for ListenScheme {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ListenScheme::Http => write!(f, "http"),
            ListenScheme::Https => write!(f, "https"),
            ListenScheme::HttpHttps => write!(f, "{{http|https}}"),
        }
    }
}

// This is used for the token info endpoint
#[derive(Debug, Serialize, Deserialize)]
pub struct JwtCommonClaims {
    pub typ: JwtTokenType,
    pub azp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    pub cnf: Option<JktClaim>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct JktClaim {
    pub jkt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AddressClaim {
    pub formatted: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    // pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

impl AddressClaim {
    pub fn try_build(user: &User, values: &UserValues) -> Option<Self> {
        let mut slf = Self {
            formatted: format!("{} {}\n", user.given_name, user.family_name),
            street_address: None,
            locality: None,
            postal_code: None,
            country: None,
        };

        if let Some(street) = &values.street {
            writeln!(slf.formatted, "{}", street).expect("AddressClaim to build");
            slf.street_address = Some(street.clone());
        }

        if let Some(zip) = values.zip {
            slf.postal_code = Some(zip);

            if let Some(city) = &values.city {
                writeln!(slf.formatted, "{}, {}", zip, city).expect("AddressClaim to build");
                slf.locality = Some(city.clone());
            } else {
                writeln!(slf.formatted, "{}", zip).expect("AddressClaim to build");
            }
        }

        if let Some(country) = &values.country {
            writeln!(slf.formatted, "{}", country).expect("AddressClaim to build");
            slf.country = Some(country.clone());
        }

        if slf.street_address.is_some()
            || slf.locality.is_some()
            || slf.postal_code.is_some()
            || slf.country.is_some()
        {
            Some(slf)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtAccessClaims {
    pub typ: JwtTokenType,
    pub azp: String,
    pub scope: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    // user part
    pub uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cnf: Option<JktClaim>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtIdClaims {
    pub azp: String,
    pub typ: JwtTokenType,
    pub amr: Vec<String>,
    pub auth_time: i64,
    pub preferred_username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressClaim>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    pub roles: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cnf: Option<JktClaim>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<String, serde_json::Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtRefreshClaims {
    pub azp: String,
    pub typ: JwtTokenType,
    pub uid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cnf: Option<JktClaim>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum JwtTokenType {
    Bearer,
    DPoP,
    Id,
    Refresh,
}

impl JwtTokenType {
    pub fn as_str(&self) -> &str {
        match self {
            JwtTokenType::Bearer => "Bearer",
            JwtTokenType::DPoP => "DPoP",
            JwtTokenType::Id => "Id",
            JwtTokenType::Refresh => "Refresh",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all(serialize = "lowercase"))]
pub enum JwtAmrValue {
    Pwd,
    Mfa,
}

impl FromStr for JwtAmrValue {
    type Err = ErrorResponse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let slf = match s {
            "pwd" => Self::Pwd,
            "mfa" => Self::Mfa,
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Unknown value for 'amr' claim".to_string(),
                ))
            }
        };
        Ok(slf)
    }
}

impl ToString for JwtAmrValue {
    fn to_string(&self) -> String {
        let s = match self {
            Self::Pwd => "pwd",
            Self::Mfa => "mfa",
        };
        s.to_string()
    }
}

pub fn real_ip_from_req(req: &HttpRequest) -> Option<String> {
    if *PROXY_MODE {
        // TODO maybe make this configurable and extract headers in user-configured order?
        req.connection_info()
            .realip_remote_addr()
            .map(|ip| ip.to_string())
    } else {
        req.connection_info().peer_addr().map(|ip| ip.to_string())
    }
}
