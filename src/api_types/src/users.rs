use crate::cust_validation::{validate_vec_groups, validate_vec_roles};
use crate::generic::Language;
use crate::oidc::AddressClaim;
use rauthy_common::constants::{
    RE_ALNUM_48, RE_ALNUM_64, RE_APP_ID, RE_ATTR, RE_ATTR_DESC, RE_CITY, RE_CLIENT_NAME,
    RE_DATE_STR, RE_MFA_CODE, RE_PHONE, RE_STREET, RE_URI, RE_USER_NAME,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct DeviceRequest {
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$"))]
    pub device_id: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "*RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
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

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum MfaPurpose {
    Login(String),
    PasswordNew,
    PasswordReset,
    Test,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct NewUserRequest {
    /// Validation: `email`
    #[validate(email)]
    pub email: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{1,32}"))]
    pub family_name: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{1,32}"))]
    pub given_name: String,
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
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct NewUserRegistrationRequest {
    #[validate(email)]
    pub email: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{1,32}"))]
    pub family_name: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{1,32}"))]
    pub given_name: String,
    /// Validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]+`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+"))]
    pub pow: String,
    /// Validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]+`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+"))]
    pub redirect_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct PasskeyRequest {
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{1,32}"))]
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct RequestResetRequest {
    /// Validation: `email`
    #[validate(email)]
    pub email: String,
    /// Redirect URI used after a successful reset - validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub redirect_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    /// Validation: `email`
    #[validate(email)]
    pub email: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "*RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub given_name: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "*RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub family_name: String,
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

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserSelfRequest {
    /// Validation: `email`
    #[validate(email)]
    pub email: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "*RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub given_name: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "*RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub family_name: Option<String>,
    pub language: Option<Language>,
    pub password_current: Option<String>,
    pub mfa_code: Option<String>,
    /// Validation: Applies password policy
    pub password_new: Option<String>,
    #[validate(nested)]
    pub user_values: Option<UserValuesRequest>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
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
    #[validate(range(min = 1000, max = 9999999))]
    pub zip: Option<i32>,
    /// Validation: `[a-zA-Z0-9À-ÿ-]{0,48}`
    #[validate(regex(path = "*RE_CITY", code = "[a-zA-Z0-9À-ÿ-]{0,48}"))]
    pub city: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-]{0,48}`
    #[validate(regex(path = "*RE_CITY", code = "[a-zA-Z0-9À-ÿ-]{0,48}"))]
    pub country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UserAttrConfigRequest {
    /// Validation: `^[a-zA-Z0-9-_/]{2,32}$`
    #[validate(regex(path = "*RE_ATTR", code = "^[a-z0-9-_/]{2,32}$"))]
    pub name: String,
    /// Validation: `^[a-zA-Z0-9-_/]{0,128}$`
    #[validate(regex(path = "*RE_ATTR_DESC", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UserAttrValueRequest {
    /// Validation: `^[a-zA-Z0-9-_/]{2,32}$`
    #[validate(regex(path = "*RE_ATTR", code = "^[a-z0-9-_/]{2,32}$"))]
    pub key: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UserAttrValuesUpdateRequest {
    #[validate(nested)]
    pub values: Vec<UserAttrValueRequest>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct WebauthnAuthStartRequest {
    pub purpose: MfaPurpose,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct WebauthnAuthFinishRequest {
    /// Validation: `[a-zA-Z0-9]{48}`
    #[validate(regex(path = "*RE_ALNUM_48", code = "[a-zA-Z0-9]{48}"))]
    pub code: String,
    /// Note: `ToSchema` does currently not exist for `webauthn_rs::prelude::PublicKeyCredential`
    #[schema(value_type = str)]
    pub data: webauthn_rs::prelude::PublicKeyCredential,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct WebauthnRegStartRequest {
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{1,32}"))]
    pub passkey_name: String,
    /// Validation: `[a-zA-Z0-9]{64}`
    #[validate(regex(path = "*RE_ALNUM_64", code = "[a-zA-Z0-9]{64}"))]
    pub magic_link_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct WebauthnRegFinishRequest {
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{1,32}"))]
    pub passkey_name: String,
    /// Note: `ToSchema` does currently not exist for `webauthn_rs::prelude::PublicKeyCredential`
    #[schema(value_type = str)]
    pub data: webauthn_rs::prelude::RegisterPublicKeyCredential,
    /// Validation: `[a-zA-Z0-9]{64}`
    #[validate(regex(path = "*RE_ALNUM_64", code = "[a-zA-Z0-9]{64}"))]
    pub magic_link_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct WebIdRequest {
    pub custom_triples: Option<String>,
    pub expose_email: bool,
}

#[derive(Debug, Serialize, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PasskeyResponse {
    pub name: String,
    /// Unix timestamp in seconds
    pub registered: i64,
    /// Unix timestamp in seconds
    pub last_used: i64,
    pub user_verified: Option<bool>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UserAttrConfigValueResponse {
    pub name: String,
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserAttrConfigResponse {
    pub values: Vec<UserAttrConfigValueResponse>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserAttrValueResponse {
    pub key: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserAttrValuesResponse {
    pub values: Vec<UserAttrValueResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
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

    // scope: phone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    // scope: webid
    #[serde(skip_serializing_if = "Option::is_none")]
    pub webid: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum UserAccountTypeResponse {
    New,
    Password,
    Passkey,
    Federated,
    FederatedPasskey,
    FederatedPassword,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
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
    pub webauthn_user_id: Option<String>, // TODO get rid of the webauthn user id ? Not needed at all?
    pub user_values: UserValuesResponse,
    pub auth_provider_id: Option<String>,
    pub federation_uid: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponseSimple {
    pub id: String,
    pub email: String,
    pub created_at: i64,
    pub last_login: Option<i64>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, ToSchema)]
pub struct UserValuesResponse {
    pub birthdate: Option<String>,
    pub phone: Option<String>,
    pub street: Option<String>,
    pub zip: Option<i32>,
    pub city: Option<String>,
    pub country: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WebId {
    pub user_id: String,
    pub expose_email: bool,
    pub custom_triples: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WebIdResponse {
    pub webid: WebId,
    pub issuer: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub language: Language,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WebauthnAuthStartResponse {
    pub code: String,
    #[schema(value_type = str)]
    pub rcr: webauthn_rs::prelude::RequestChallengeResponse,
    pub user_id: String,
    pub exp: u64,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WebauthnLoginFinishResponse {
    pub loc: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct WebauthnLoginResponse {
    pub code: String,
    pub user_id: String,
    pub exp: u64,
}
