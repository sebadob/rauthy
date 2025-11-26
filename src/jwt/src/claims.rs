use rauthy_api_types::oidc::JktClaim;
use rauthy_data::entity::users::User;
use rauthy_data::entity::users_values::UserValues;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use std::fmt::Write;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtCommonClaims<'a> {
    pub iat: i64,
    pub nbf: i64,
    pub exp: i64,
    pub iss: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jti: Option<&'a str>,
    pub aud: Cow<'a, str>,
    pub sub: Option<&'a str>,
    // pub nonce: Option<&'a str>,
    pub typ: JwtTokenType,
    pub azp: &'a str,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub scope: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub did: Option<&'a str>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub cnf: Option<JktClaim<'a>>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct AddressClaim<'a> {
    pub formatted: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<&'a str>,
    // pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<&'a str>,
}

impl From<AddressClaim<'_>> for rauthy_api_types::oidc::AddressClaim {
    fn from(a: AddressClaim<'_>) -> Self {
        Self {
            formatted: a.formatted,
            street_address: a.street_address.map(String::from),
            locality: a.locality.map(String::from),
            postal_code: a.postal_code.map(String::from),
            country: a.country.map(String::from),
        }
    }
}

impl AddressClaim<'_> {
    pub fn try_build<'a>(user: &'a User, values: &'a UserValues) -> Option<AddressClaim<'a>> {
        let mut slf = AddressClaim {
            formatted: format!("{}\n", user.email_recipient_name()),
            street_address: None,
            locality: None,
            postal_code: None,
            country: None,
        };

        if let Some(street) = &values.street {
            writeln!(slf.formatted, "{street}").expect("AddressClaim to build");
            slf.street_address = Some(street);
        }

        if let Some(zip) = &values.zip {
            slf.postal_code = Some(zip);

            if let Some(city) = &values.city {
                writeln!(slf.formatted, "{zip}, {city}").expect("AddressClaim to build");
                slf.locality = Some(city);
            } else {
                writeln!(slf.formatted, "{zip}").expect("AddressClaim to build");
            }
        }

        if let Some(country) = &values.country {
            writeln!(slf.formatted, "{country}").expect("AddressClaim to build");
            slf.country = Some(country);
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

impl<'a> From<&'a rauthy_api_types::oidc::AddressClaim> for AddressClaim<'a> {
    fn from(value: &rauthy_api_types::oidc::AddressClaim) -> AddressClaim<'_> {
        AddressClaim {
            formatted: value.formatted.to_string(),
            street_address: value.street_address.as_deref(),
            locality: value.locality.as_deref(),
            postal_code: value.postal_code.as_deref(),
            country: value.country.as_deref(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtAccessClaims<'a> {
    #[serde(borrow, flatten)]
    pub common: JwtCommonClaims<'a>,

    // pub scope: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<&'a str>>,
    // user part
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<&'a str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<String, serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtIdClaims<'a> {
    #[serde(borrow, flatten)]
    pub common: JwtCommonClaims<'a>,

    pub amr: Vec<&'a str>,
    pub auth_time: i64,
    pub at_hash: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressClaim<'a>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<&'a str>,
    pub roles: Vec<String>, // TODO change to borrowed data when everything works
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>, // TODO change to borrowed data when everything works
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<String, serde_json::Value>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub webid: Option<Cow<'a, str>>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub zoneinfo: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtLogoutClaims<'a> {
    #[serde(borrow, flatten)]
    pub common: JwtCommonClaims<'a>,

    pub events: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sid: Option<&'a str>,

    // The `nonce` MUST NOT exist in this token. We try to deserialize into an `Option<_>` for easy
    // `.is_none()` validation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nonce: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JwtRefreshClaims<'a> {
    #[serde(borrow, flatten)]
    pub common: JwtCommonClaims<'a>,

    pub uid: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_time: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum JwtTokenType {
    Bearer,
    DPoP,
    Id,
    #[serde(rename = "logout+jwt")]
    Logout,
    Refresh,
}

impl JwtTokenType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Bearer => "Bearer",
            Self::DPoP => "DPoP",
            Self::Id => "Id",
            Self::Logout => "logout+jwt",
            Self::Refresh => "Refresh",
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
                    "Unknown value for 'amr' claim",
                ));
            }
        };
        Ok(slf)
    }
}

impl Display for JwtAmrValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl JwtAmrValue {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pwd => "pwd",
            Self::Mfa => "mfa",
        }
    }
}
