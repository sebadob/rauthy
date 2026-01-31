use crate::cust_validation::{validate_vec_groups, validate_vec_roles};
use crate::generic::Language;
use crate::oidc::AddressClaim;
use rauthy_common::regex::{
    RE_ALNUM, RE_ALNUM_48, RE_ALNUM_64, RE_APP_ID, RE_ATTR, RE_ATTR_DESC, RE_CITY, RE_CLIENT_NAME,
    RE_DATE_STR, RE_MFA_CODE, RE_PHONE, RE_PREFERRED_USERNAME, RE_STREET, RE_URI, RE_USER_NAME,
};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, ToSchema)]
pub struct UserPictureConfig {
    pub upload_allowed: bool,
    pub content_len_limit: u32,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct DeviceRequest {
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$"))]
    pub device_id: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "*RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ɏ-\\s]{2,128}"))]
    pub name: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct MfaAwaitRequest {
    /// Validation: `^[a-zA-Z0-9]{48}$`
    #[validate(regex(path = "*RE_MFA_CODE", code = "^[a-zA-Z0-9]{48}$"))]
    pub code: String,
    /// Validation: `^[a-zA-Z0-9]{12}$`
    #[validate(regex(path = "*RE_APP_ID", code = "^[a-zA-Z0-9]{12}$"))]
    pub req_id: String,
}

#[derive(PartialEq, Eq, Deserialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub enum MfaPurpose {
    Login(String),
    MfaModToken,
    PamLogin,
    PasswordNew,
    PasswordReset,
    Test,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct NewUserRequest {
    /// Validation: `email`
    #[validate(email)]
    pub email: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-'\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ɏ-'\\s]{1,32}"))]
    pub family_name: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-'\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ɏ-'\\s]{1,32}"))]
    pub given_name: Option<String>,
    pub language: Language,
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_groups"))]
    pub groups: Option<Vec<String>>,
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_roles"))]
    pub roles: Vec<String>,
    // TODO max validation for inner i64 is broken in the macro in v0.18.1
    // #[validate(range(min = 1719784800, max = 4070905200))]
    /// Unix timestamp in seconds
    #[validate(range(min = 1719784800))]
    pub user_expires: Option<i64>,
    /// Validation: Valid Timezone in the format of `Europe/Berlin`
    #[validate(length(max = 48))]
    pub tz: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct NewUserRegistrationRequest {
    #[validate(email)]
    pub email: String,
    /// Validation: `[user_values.preferred_username] -> regex_rust`
    #[validate(regex(path = "RE_PREFERRED_USERNAME"))]
    pub preferred_username: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-'\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ɏ-'\\s]{1,32}"))]
    pub family_name: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-'\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ɏ-'\\s]{1,32}"))]
    pub given_name: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]+`
    #[validate(nested)]
    pub user_values: Option<UserValuesRequest>,
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+"))]
    pub pow: String,
    /// Validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]+`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+"))]
    pub redirect_uri: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct PasskeyRequest {
    /// Validation: `[a-zA-Z0-9À-ÿ-'\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ɏ-'\\s]{1,32}"))]
    pub name: String,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct PasswordResetRequest {
    /// Validation: `[a-zA-Z0-9]{64}`
    #[validate(regex(path = "*RE_ALNUM_64", code = "[a-zA-Z0-9]{64}"))]
    pub magic_link_id: String,
    /// Validation: Applies password policy - max 256 characters
    #[validate(length(max = 256))]
    pub password: String,
    /// Validation: `[a-zA-Z0-9]{48}`
    #[validate(regex(path = "*RE_ALNUM_48", code = "[a-zA-Z0-9]{48}"))]
    pub mfa_code: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct PreferredUsernameRequest {
    /// Validation: `[user_values.preferred_username] -> regex_rust`
    #[validate(regex(path = "RE_PREFERRED_USERNAME"))]
    pub preferred_username: Option<String>,
    /// Can only be set with an active `rauthy_admin` session
    pub force_overwrite: Option<bool>,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct RequestResetRequest {
    /// Validation: `email`
    #[validate(email)]
    pub email: String,
    /// Redirect URI used after a successful reset - validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub redirect_uri: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]+`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+"))]
    pub pow: String,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct UpdateUserRequest {
    /// Validation: `email`
    #[validate(email)]
    pub email: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-'\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ɏ-'\\s]{1,32}"))]
    pub given_name: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-'\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ɏ-'\\s]{1,32}"))]
    pub family_name: Option<String>,
    pub language: Option<Language>,
    /// Validation: Applies password policy - max 256 characters
    #[validate(length(max = 256))]
    pub password: Option<String>,
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_roles"))]
    pub roles: Vec<String>,
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_groups"))]
    pub groups: Option<Vec<String>>,
    pub enabled: bool,
    pub email_verified: bool,
    // TODO max validation for inner i64 is broken in the macro in v0.18.1
    // #[validate(range(min = 1719784800, max = 4070905200))]
    /// Unix timestamp in seconds
    #[validate(range(min = 1719784800))]
    pub user_expires: Option<i64>,
    #[validate(nested)]
    pub user_values: Option<UserValuesRequest>,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct UpdateUserSelfRequest {
    /// Validation: `email`
    #[validate(email)]
    pub email: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-'\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ɏ-'\\s]{1,32}"))]
    pub given_name: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-'\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ɏ-'\\s]{1,32}"))]
    pub family_name: Option<String>,
    pub language: Option<Language>,
    #[validate(length(max = 256))]
    pub password_current: Option<String>,
    pub mfa_code: Option<String>,
    /// Validation: Applies password policy
    #[validate(length(max = 256))]
    pub password_new: Option<String>,
    #[validate(nested)]
    pub user_values: Option<UserValuesRequest>,
}

#[derive(Debug, Default, PartialEq, Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct UserValuesRequest {
    /// Validation: `[0-9]{4}-[0-9]{2}-[0-9]{2}`
    #[validate(regex(path = "*RE_DATE_STR", code = "[0-9]{4}-[0-9]{2}-[0-9]{2}"))]
    pub birthdate: Option<String>,
    /// Validation: `+[0-9]{0,32}`
    #[validate(regex(path = "*RE_PHONE", code = "+[0-9]{0,32}"))]
    pub phone: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-.\s]{0,48}`
    #[validate(regex(path = "*RE_STREET", code = "[a-zA-Z0-9À-ÿ-.\\s]{0,48}"))]
    pub street: Option<String>,
    /// Validation: `[a-zA-Z0-9]`, max length 24
    #[validate(regex(path = "*RE_ALNUM", code = "[a-zA-Z0-9]"), length(max = 24))]
    pub zip: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-]{0,48}`
    #[validate(regex(path = "*RE_CITY", code = "[a-zA-Z0-9À-ÿ-]{0,48}"))]
    pub city: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-]{0,48}`
    #[validate(regex(path = "*RE_CITY", code = "[a-zA-Z0-9À-ÿ-]{0,48}"))]
    pub country: Option<String>,
    /// Validation: Valid Timezone in the format of `Europe/Berlin`
    #[validate(length(max = 48))]
    pub tz: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum UserAttrConfigTyp {
    Email,
}

impl TryFrom<&str> for UserAttrConfigTyp {
    type Error = ErrorResponse;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let slf = match value {
            "email" => Self::Email,
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "invalid value for UserAttrConfigTyp",
                ));
            }
        };
        Ok(slf)
    }
}

impl UserAttrConfigTyp {
    pub fn as_str(&self) -> &str {
        match self {
            UserAttrConfigTyp::Email => "email",
        }
    }
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct UserAttrConfigRequest {
    /// Validation: `^[a-zA-Z0-9-_/]{2,32}$`
    #[validate(regex(path = "*RE_ATTR", code = "^[a-z0-9-_/]{2,32}$"))]
    pub name: String,
    /// Validation: `^[a-zA-Z0-9-_/]{0,128}$`
    #[validate(regex(path = "*RE_ATTR_DESC", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub desc: Option<String>,
    pub default_value: Option<serde_json::Value>,
    /// Currently ignored - will be implemented in a future version
    pub typ: Option<UserAttrConfigTyp>,
    pub user_editable: Option<bool>,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct UserAttrValueRequest {
    /// Validation: `^[a-zA-Z0-9-_/]{2,32}$`
    #[validate(regex(path = "*RE_ATTR", code = "^[a-z0-9-_/]{2,32}$"))]
    pub key: String,
    pub value: serde_json::Value,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct UserAttrValuesUpdateRequest {
    #[validate(nested)]
    pub values: Vec<UserAttrValueRequest>,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct UserRevokeParams {
    /// Validation: IpAddr
    #[schema(value_type = str)]
    pub ip: IpAddr,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct MfaModTokenRequest {
    #[validate(length(max = 256))]
    pub password: Option<String>,
    #[validate(length(min = 48, max = 48))]
    pub mfa_code: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct WebauthnAuthStartRequest {
    pub purpose: MfaPurpose,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct WebauthnAuthFinishRequest {
    /// Validation: `[a-zA-Z0-9]{48}`
    #[validate(regex(path = "*RE_ALNUM_48", code = "[a-zA-Z0-9]{48}"))]
    pub code: String,
    /// Note: `ToSchema` does currently not exist for `webauthn_rs::prelude::PublicKeyCredential`
    #[schema(value_type = str)]
    pub data: webauthn_rs::prelude::PublicKeyCredential,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct WebauthnDeleteRequest {
    #[validate(length(min = 32, max = 32))]
    pub mfa_mod_token_id: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct WebauthnRegStartRequest {
    /// Validation: `[a-zA-Z0-9À-ÿ-'\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ɏ-'\\s]{1,32}"))]
    pub passkey_name: String,
    /// Validation: `[a-zA-Z0-9]{64}`
    #[validate(regex(path = "*RE_ALNUM_64", code = "[a-zA-Z0-9]{64}"))]
    pub magic_link_id: Option<String>,
    #[validate(length(min = 32, max = 32))]
    pub mfa_mod_token_id: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct WebauthnRegFinishRequest {
    /// Validation: `[a-zA-Z0-9À-ÿ-'\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ɏ-'\\s]{1,32}"))]
    pub passkey_name: String,
    /// Note: `ToSchema` does currently not exist for `webauthn_rs::prelude::PublicKeyCredential`
    #[schema(value_type = str)]
    pub data: webauthn_rs::prelude::RegisterPublicKeyCredential,
    /// Validation: `[a-zA-Z0-9]{64}`
    #[validate(regex(path = "*RE_ALNUM_64", code = "[a-zA-Z0-9]{64}"))]
    pub magic_link_id: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct WebIdRequest {
    pub custom_triples: Option<String>,
    pub expose_email: bool,
}

#[derive(Serialize, ToSchema)]
pub struct DeviceResponse {
    pub id: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// Unix timestamp in seconds
    pub created: i64,
    /// Unix timestamp in seconds
    pub access_exp: i64,
    /// Unix timestamp in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_exp: Option<i64>,
    pub peer_ip: String,
    pub name: String,
}

#[derive(Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct MfaModTokenResponse {
    pub id: String,
    pub user_id: String,
    pub exp: i64,
    pub ip: String,
}

#[derive(Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct PasskeyResponse {
    pub name: String,
    /// Unix timestamp in seconds
    pub registered: i64,
    /// Unix timestamp in seconds
    pub last_used: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_verified: Option<bool>,
}

#[derive(Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct UserAttrConfigValueResponse {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typ: Option<UserAttrConfigTyp>,
    pub user_editable: bool,
}

#[derive(Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct UserAttrConfigResponse {
    pub values: Vec<UserAttrConfigValueResponse>,
}

#[derive(Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct UserAttrValueResponse {
    pub key: String,
    pub value: serde_json::Value,
}

#[derive(Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct UserAttrValuesResponse {
    pub values: Vec<UserAttrValueResponse>,
}

#[derive(Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct UserEditableAttrResponse {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub typ: Option<UserAttrConfigTyp>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
}

#[derive(Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct UserEditableAttrsResponse {
    pub values: Vec<UserEditableAttrResponse>,
}

#[derive(Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct Userinfo {
    pub id: String,
    pub sub: String,
    pub name: String,
    pub roles: Vec<String>,
    pub mfa_enabled: bool,

    // scope: address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<AddressClaim>,

    // scope: email
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,

    // scope: groups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,

    // scope: profile
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zoneinfo: Option<String>,

    // scope: phone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    // scope: webid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webid: Option<String>,
}

#[derive(Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
#[serde(rename_all = "snake_case")]
pub enum UserAccountTypeResponse {
    New,
    Password,
    Passkey,
    Federated,
    FederatedPasskey,
    FederatedPassword,
}

#[derive(Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    pub language: Language,
    pub roles: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<String>>,
    pub enabled: bool,
    pub email_verified: bool,
    /// Unix timestamp in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expires: Option<i64>,
    /// Unix timestamp in seconds
    pub created_at: i64,
    /// Unix timestamp in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login: Option<i64>,
    /// Unix timestamp in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failed_login: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_login_attempts: Option<i64>,
    /// Unix timestamp in seconds
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_expires: Option<i64>,
    pub account_type: UserAccountTypeResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webauthn_user_id: Option<String>, // TODO get rid of the webauthn user id ? Not needed at all?
    pub user_values: UserValuesResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_provider_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub federation_uid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture_id: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserResponseSimple {
    pub id: String,
    pub email: String,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub created_at: i64,
    pub last_login: Option<i64>,
    pub picture_id: Option<String>,
}

impl From<hiqlite::Row<'_>> for UserResponseSimple {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        let name: String = row.get("given_name");
        let given_name = if name.is_empty() { None } else { Some(name) };

        Self {
            id: row.get("id"),
            email: row.get("email"),
            given_name,
            family_name: row.get("family_name"),
            created_at: row.get("created_at"),
            last_login: row.get("last_login"),
            picture_id: row.get("picture_id"),
        }
    }
}

impl From<tokio_postgres::Row> for UserResponseSimple {
    fn from(row: tokio_postgres::Row) -> Self {
        let name: String = row.get("given_name");
        let given_name = if name.is_empty() { None } else { Some(name) };

        Self {
            id: row.get("id"),
            email: row.get("email"),
            given_name,
            family_name: row.get("family_name"),
            created_at: row.get("created_at"),
            last_login: row.get("last_login"),
            picture_id: row.get("picture_id"),
        }
    }
}

#[derive(Default, Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct UserValuesResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birthdate: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tz: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct WebId {
    pub user_id: String,
    pub expose_email: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_triples: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct WebIdResponse {
    pub webid: WebId,
    pub issuer: String,
    pub email: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    pub language: Language,
}

#[derive(Serialize, ToSchema)]
pub struct WebauthnAuthStartResponse {
    pub code: String,
    #[schema(value_type = str)]
    pub rcr: webauthn_rs::prelude::RequestChallengeResponse,
    pub user_id: String,
    pub exp: u64,
}

#[derive(Serialize, ToSchema)]
pub struct WebauthnLoginFinishResponse {
    pub loc: String,
}

#[derive(Serialize, ToSchema)]
pub struct WebauthnLoginResponse {
    pub code: String,
    pub user_id: String,
    pub exp: u64,
}
