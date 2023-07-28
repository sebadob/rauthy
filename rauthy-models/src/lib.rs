// Rauthy - OpenID Connect and Single Sign-On IdP
// Copyright (C) 2023 Sebastian Dobe <sebastiandobe@mailbox.org>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use crate::entity::sessions::Session;
use actix_web::http::header::{HeaderName, HeaderValue};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub mod app_state;
pub mod email;
pub mod entity;
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
    pub typ: JwtType,
    pub azp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtAccessClaims {
    pub typ: JwtType,
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
    pub custom: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtIdClaims {
    pub azp: String,
    pub typ: JwtType,
    pub amr: Vec<String>,
    pub preferred_username: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    pub roles: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtRefreshClaims {
    pub azp: String,
    pub typ: JwtType,
    pub uid: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum JwtType {
    Bearer,
    Id,
    Refresh,
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
