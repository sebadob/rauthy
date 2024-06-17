use crate::entity::api_keys::ApiKeyAccess;
use crate::entity::auth_providers::AuthProviderType;
use crate::entity::jwk::JwkKeyPairAlg;
use crate::events::event::{EventLevel, EventType};
use crate::language::Language;
use actix_web::http::header;
use actix_web::HttpRequest;
use css_color::Srgb;
use rauthy_common::constants::{
    RE_ALNUM, RE_ALNUM_48, RE_ALNUM_64, RE_API_KEY, RE_APP_ID, RE_ATTR, RE_ATTR_DESC, RE_CHALLENGE,
    RE_CITY, RE_CLIENT_ID_EPHEMERAL, RE_CLIENT_NAME, RE_CODE_CHALLENGE, RE_CODE_VERIFIER,
    RE_CONTACT, RE_DATE_STR, RE_GRANT_TYPES, RE_GROUPS, RE_LOWERCASE, RE_MFA_CODE, RE_ORIGIN,
    RE_PEM, RE_PHONE, RE_SCOPE_SPACE, RE_SEARCH, RE_STREET, RE_TOKEN_ENDPOINT_AUTH_METHOD, RE_URI,
    RE_USER_NAME,
};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::base64_decode;
use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use std::str::FromStr;
use utoipa::{IntoParams, ToSchema};
use validator::{Validate, ValidationError};

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ApiKeyRequest {
    /// Validation: `^[a-zA-Z0-9_-/]{2,24}$`
    #[validate(regex(path = "RE_API_KEY", code = "^[a-zA-Z0-9_-/]{2,24}$"))]
    pub name: String,
    /// Unix timestamp in seconds in the future (max year 2099)
    #[validate(range(min = 1672527600, max = 4070905200))]
    pub exp: Option<i64>,
    pub access: Vec<ApiKeyAccess>,
}

// TODO is this not being used anymore? -> check!
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct AuthCodeRequest {
    /// Validation: `^[a-z0-9-_/]{2,128}$`
    #[validate(regex(path = "RE_LOWERCASE", code = "^[a-z0-9-_/]{2,128}$"))]
    pub grant_type: String,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub code: String,
    /// Validation: `^[a-z0-9-_/]{2,128}$`
    #[validate(regex(path = "RE_LOWERCASE", code = "^[a-z0-9-_/]{2,128}$"))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub client_secret: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$"))]
    pub redirect_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct IpBlacklistRequest {
    /// Validation: Ipv4Addr
    #[schema(value_type = str)]
    pub ip: Ipv4Addr,
    /// Unix timestamp in seconds in the future (max year 2099)
    #[validate(range(min = 1672527600, max = 4070905200))]
    pub exp: i64,
}

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct AuthRequest {
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    #[validate(regex(
        path = "RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,256}$"
    ))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub redirect_uri: String,
    /// Validation: `[a-z0-9-_/]{2,128}`
    #[validate(regex(path = "RE_LOWERCASE", code = "[a-z0-9-_/]{2,128}"))]
    pub response_type: String,
    /// Validation: `[a-z0-9-_/:\s*]{0,512}`
    #[validate(regex(path = "RE_SCOPE_SPACE", code = "[a-z0-9-_/:\\s*]{0,512}"))]
    #[serde(default = "default_scope")]
    pub scope: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub state: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub code_challenge: Option<String>,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub code_challenge_method: Option<String>,
    #[validate(range(min = 0))]
    pub max_age: Option<i64>,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub prompt: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ColorsRequest {
    #[validate(length(min = 2, max = 32))]
    pub act1: String,
    #[validate(length(min = 2, max = 32))]
    pub act1a: String,
    #[validate(length(min = 2, max = 32))]
    pub act2: String,
    #[validate(length(min = 2, max = 32))]
    pub act2a: String,
    #[validate(length(min = 2, max = 32))]
    pub acnt: String,
    #[validate(length(min = 2, max = 32))]
    pub acnta: String,
    #[validate(length(min = 2, max = 32))]
    pub ok: String,
    #[validate(length(min = 2, max = 32))]
    pub err: String,
    #[validate(length(min = 2, max = 32))]
    pub glow: String,
    #[validate(length(min = 2, max = 32))]
    pub gmid: String,
    #[validate(length(min = 2, max = 32))]
    pub ghigh: String,
    #[validate(length(min = 2, max = 32))]
    pub text: String,
    #[validate(length(min = 2, max = 32))]
    pub bg: String,
}

impl ColorsRequest {
    pub fn validate_css(&self) -> Result<(), ErrorResponse> {
        Srgb::from_str(&self.act1)?;
        Srgb::from_str(&self.act1a)?;
        Srgb::from_str(&self.act2)?;
        Srgb::from_str(&self.act2a)?;
        Srgb::from_str(&self.acnt)?;
        Srgb::from_str(&self.acnta)?;
        Srgb::from_str(&self.ok)?;
        Srgb::from_str(&self.err)?;
        Srgb::from_str(&self.glow)?;
        Srgb::from_str(&self.gmid)?;
        Srgb::from_str(&self.ghigh)?;
        Srgb::from_str(&self.text)?;
        Srgb::from_str(&self.bg)?;
        Ok(())
    }
}

#[derive(Debug, Clone, Deserialize, Validate, ToSchema)]
pub struct DeviceRequest {
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$"))]
    pub device_id: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct DeviceGrantRequest {
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$"))]
    pub client_id: String,
    /// Validation: max length is 256
    #[validate(length(max = 256))]
    pub client_secret: Option<String>,
    /// Validation: `[a-z0-9-_/:\s*]{0,512}`
    #[validate(regex(path = "RE_SCOPE_SPACE", code = "[a-z0-9-_/:\\s*]{0,512}"))]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum DeviceAcceptedRequest {
    Accept,
    Decline,
    Pending,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct DeviceVerifyRequest {
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub user_code: String,
    /// Validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]+`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+"))]
    pub pow: String,
    /// If `DeviceAcceptedRequest::Pending`, information about the request will be returned.
    /// If `DeviceAcceptedRequest::Accept` - the device will get a Token Set
    /// If `DeviceAcceptedRequest::Decline` - the code request will be deleted and rejected
    pub device_accepted: DeviceAcceptedRequest,
}

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct EncKeyMigrateRequest {
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub key_id: String,
}

/// This request is used for ephemeral clients, which are needed for Solid OIDC for instance.
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct EphemeralClientRequest {
    #[validate(regex(
        path = "RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,256}$"
    ))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub client_name: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub client_uri: Option<String>,
    /// Validation: `Vec<^[a-zA-Z0-9\+.@/]{0,48}$>`
    #[validate(custom(function = "validate_vec_contact"))]
    pub contacts: Option<Vec<String>>,
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    #[validate(custom(function = "validate_vec_uri"))]
    pub redirect_uris: Vec<String>,
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    #[validate(custom(function = "validate_vec_uri"))]
    pub post_logout_redirect_uris: Option<Vec<String>>,
    /// Validation: `Vec<^(authorization_code|client_credentials|password|refresh_token)$>`
    #[validate(custom(function = "validate_vec_grant_type"))]
    pub grant_types: Option<Vec<String>>,
    /// Validation: `60 <= access_token_lifetime <= 86400`
    #[validate(range(min = 60, max = 86400))]
    pub default_max_age: Option<i32>,
    /// Validation: `[a-z0-9-_/:\s*]{0,512}`
    #[validate(regex(path = "RE_SCOPE_SPACE", code = "[a-z0-9-_/:\\s*]{0,512}"))]
    pub scope: Option<String>,
    pub require_auth_time: Option<bool>,

    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    pub access_token_signed_response_alg: Option<JwkKeyPairAlg>,
    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    pub id_token_signed_response_alg: Option<JwkKeyPairAlg>,
}

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct EventsListenParams {
    /// Validation: `0 <= latest <= 1000`
    #[validate(range(min = 0, max = 1000))]
    pub latest: Option<u16>,
    pub level: Option<EventLevel>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct EventsRequest {
    /// Unix timestamp in seconds
    #[validate(range(min = 1672527600, max = 4070905200))]
    pub from: i64,
    /// Unix timestamp in seconds
    #[validate(range(min = 1672527600, max = 4070905200))]
    pub until: Option<i64>,
    pub level: EventLevel,
    pub typ: Option<EventType>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct FedCMAssertionRequest {
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    #[validate(regex(
        path = "RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,128}$"
    ))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub nonce: Option<String>,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub account_id: String,
    /// Whether the user agent has explicitly shown to the user what specific information the
    /// IDP intends to share with the RP (e.g. "idp.example will share your name, email... with
    /// rp.example"), used by the request permission to sign-up algorithm for new users. It is
    /// used as an assurance by the user agent to the IDP that it has indeed shown the terms of
    /// service and privacy policy to the user in the cases where it is required to do so.
    pub disclosure_text_shown: bool,
}

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct FedCMClientMetadataRequest {
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    #[validate(regex(
        path = "RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,128}$"
    ))]
    pub client_id: String,
}

fn default_scope() -> String {
    String::from("openid")
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct I18nRequest {
    pub content: I18nContent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum I18nContent {
    Account,
    Authorize,
    Device,
    EmailChangeConfirm,
    Error,
    Index,
    Logout,
    PasswordReset,
    Register,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    /// Validation: `email`
    #[validate(email)]
    pub email: String,
    /// Validation: Applies password policy - max 256 characters
    #[validate(length(max = 256))]
    pub password: Option<String>,
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    #[validate(regex(
        path = "RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,128}$"
    ))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub redirect_uri: String,
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_scopes"))]
    pub scopes: Option<Vec<String>>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub state: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub nonce: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub code_challenge: Option<String>,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub code_challenge_method: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRefreshRequest {
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    #[validate(regex(
        path = "RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,128}$"
    ))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub redirect_uri: String,
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_scopes"))]
    pub scopes: Option<Vec<String>>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub state: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub nonce: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub code_challenge: Option<String>,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub code_challenge_method: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct LogoutRequest {
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub id_token_hint: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub post_logout_redirect_uri: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub state: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct MfaAwaitRequest {
    /// Validation: `^[a-zA-Z0-9]{48}$`
    #[validate(regex(path = "RE_MFA_CODE", code = "^[a-zA-Z0-9]{48}$"))]
    pub code: String,
    /// Validation: `^[a-zA-Z0-9]{12}$`
    #[validate(regex(path = "RE_APP_ID", code = "^[a-zA-Z0-9]{12}$"))]
    pub req_id: String,
}

#[derive(Debug, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub enum MfaPurpose {
    Login(String),
    PasswordNew,
    PasswordReset,
    Test,
}

#[derive(Debug, Validate, Serialize, Deserialize, ToSchema)]
pub struct NewClientRequest {
    /// Validation: `^[a-z0-9-_/]{2,128}$`
    #[validate(regex(path = "RE_LOWERCASE", code = "^[a-z0-9-_/]{2,128}$"))]
    pub id: String,
    /// Validation: None - will not be deserialized
    #[serde(skip_deserializing)]
    pub secret: Option<Vec<u8>>,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub name: Option<String>,
    /// Validation: bool
    pub confidential: bool,
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    #[validate(custom(function = "validate_vec_uri"))]
    pub redirect_uris: Vec<String>,
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    #[validate(custom(function = "validate_vec_uri"))]
    pub post_logout_redirect_uris: Option<Vec<String>>,
}

// https://openid.net/specs/openid-connect-registration-1_0.html#ClientMetadata
#[derive(Debug, Validate, Serialize, Deserialize, ToSchema)]
pub struct DynamicClientRequest {
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    #[validate(custom(function = "validate_vec_uri"))]
    pub redirect_uris: Vec<String>,
    /// Validation: `Vec<^(authorization_code|client_credentials|password|refresh_token)$>`
    #[validate(custom(function = "validate_vec_grant_types"))]
    pub grant_types: Vec<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub client_name: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub client_uri: Option<String>,
    /// Validation: `Vec<^[a-zA-Z0-9\+.@/]{0,48}$>`
    #[validate(custom(function = "validate_vec_contact"))]
    pub contacts: Option<Vec<String>>,
    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    pub id_token_signed_response_alg: Option<JwkKeyPairAlg>,
    /// Validation: `^(client_secret_post|client_secret_basic|none)$`
    #[validate(regex(
        path = "RE_TOKEN_ENDPOINT_AUTH_METHOD",
        code = "client_secret_post|client_secret_basic|none"
    ))]
    pub token_endpoint_auth_method: Option<String>,
    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    pub token_endpoint_auth_signing_alg: Option<JwkKeyPairAlg>,
    // Rauthy will only accept the following defaults
    // `response_type=code`
    // `subject_type=public`
    // `require_auth_time=true` (always added by Rauthy anyway)
    //
    // The following must never be accepted for security reasons,
    // because the registration may be unauthenticated:
    // - logo_uri
    // - client_uri
    // - policy_uri
    // - tos_uri
    //
    // Unsupported values:
    // - application_type (may come in the future)
    // - contacts (may come in the future)
    // - jwks_uri
    // - jwks
    // - sector_identifier_uri
    // - id_token_encrypted_response_alg
    // - id_token_encrypted_response_enc
    // - userinfo_signed_response_alg
    // - userinfo_encrypted_response_alg
    // - userinfo_encrypted_response_enc
    // - request_object_signing_alg
    // - request_object_encryption_alg
    // - request_object_encryption_enc
    // - default_max_age (can be specified during auth init with `max_age`)
    // - default_acr_values
    // - initiate_login_uri
    // - request_uris (may come in the future with `request_uri` during login)
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub post_logout_redirect_uri: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct NewGroupRequest {
    /// Validation: `^[a-z0-9-_/,:*]{2,64}$`
    #[validate(regex(path = "RE_GROUPS", code = "^[a-z0-9-_/,:*]{2,64}$"))]
    pub group: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct PasswordHashTimesRequest {
    #[validate(range(min = 500))]
    pub target_time: u32,
    #[validate(range(min = 32768))]
    pub m_cost: Option<u32>,
    #[validate(range(min = 2))]
    pub p_cost: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct PasswordPolicyRequest {
    /// Validation: `8 <= length_min <= 128`
    #[validate(range(min = 8, max = 128))]
    pub length_min: i32,
    /// Validation: `8 <= length_max <= 128`
    #[validate(range(min = 8, max = 128))]
    pub length_max: i32,
    /// Validation: `1 <= include_lower_case <= 32`
    #[validate(range(min = 1, max = 32))]
    pub include_lower_case: Option<i32>,
    /// Validation: `1 <= include_upper_case <= 32`
    #[validate(range(min = 1, max = 32))]
    pub include_upper_case: Option<i32>,
    /// Validation: `1 <= include_digits <= 32`
    #[validate(range(min = 1, max = 32))]
    pub include_digits: Option<i32>,
    /// Validation: `1 <= include_special <= 32`
    #[validate(range(min = 1, max = 32))]
    pub include_special: Option<i32>,
    /// Validation: `1 <= valid_days <= 3650`
    #[validate(range(min = 1, max = 3650))]
    pub valid_days: Option<i32>,
    /// Validation: `1 <= not_recently_used <= 10`
    #[validate(range(min = 1, max = 10))]
    pub not_recently_used: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct PasswordResetRequest {
    /// Validation: `[a-zA-Z0-9]{64}`
    #[validate(regex(path = "RE_ALNUM_64", code = "[a-zA-Z0-9]{64}"))]
    pub magic_link_id: String,
    /// Validation: Applies password policy - max 256 characters
    #[validate(length(max = 256))]
    pub password: String,
    /// Validation: `[a-zA-Z0-9]{48}`
    #[validate(regex(path = "RE_ALNUM_48", code = "[a-zA-Z0-9]{48}"))]
    pub mfa_code: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ProviderRequest {
    /// Validation: `[a-zA-Z0-9À-ÿ-\s]{2,128}]`
    #[validate(regex(path = "RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub name: String,
    pub typ: AuthProviderType,
    pub enabled: bool,

    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub issuer: String,
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub authorization_endpoint: String,
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub token_endpoint: String,
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub userinfo_endpoint: String,

    pub danger_allow_insecure: Option<bool>,
    pub use_pkce: bool,

    // This validation is pretty loose, but if we make it too strict,
    // we will most probably get into compatibility issues.
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub client_id: String,
    /// Validation: max length is 256
    #[validate(length(max = 256))]
    pub client_secret: Option<String>,
    /// Validation: `[a-z0-9-_/:\s*]{0,512}`
    #[validate(regex(path = "RE_SCOPE_SPACE", code = "[a-z0-9-_/:\\s*]{0,512}"))]
    pub scope: String,
    /// Validation: `(-----BEGIN CERTIFICATE-----)[a-zA-Z0-9+/=\n]+(-----END CERTIFICATE-----)`
    #[validate(regex(
        path = "RE_PEM",
        code = "(-----BEGIN CERTIFICATE-----)[a-zA-Z0-9+/=\n]+(-----END CERTIFICATE-----)"
    ))]
    pub root_pem: Option<String>,

    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub admin_claim_path: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub admin_claim_value: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub mfa_claim_path: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub mfa_claim_value: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ProviderCallbackRequest {
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub state: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub code: String,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub xsrf_token: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub pkce_verifier: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ProviderLoginRequest {
    // values for the downstream client
    /// Validation: `email`
    #[validate(email)]
    pub email: Option<String>,
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    #[validate(regex(
        path = "RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,128}$"
    ))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub redirect_uri: String,
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_scopes"))]
    pub scopes: Option<Vec<String>>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub state: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub nonce: Option<String>,
    /// Validation: `[a-zA-Z0-9-._~]{43,128}`
    #[validate(regex(path = "RE_CODE_CHALLENGE", code = "[a-zA-Z0-9-._~]{43,128}"))]
    pub code_challenge: Option<String>,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub code_challenge_method: Option<String>,

    // values for the callback from upstream
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub provider_id: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub pkce_challenge: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ProviderLookupRequest {
    /// Validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub issuer: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub metadata_url: Option<String>,
    pub danger_allow_insecure: Option<bool>,
    // no validation since it will throw an error later if not correctly formed
    pub root_pem: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct RequestResetRequest {
    /// Validation: `email`
    #[validate(email)]
    pub email: String,
    /// Redirect URI used after a successful reset - validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub redirect_uri: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct NewUserRequest {
    /// Validation: `email`
    #[validate(email)]
    pub email: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,32}`
    #[validate(regex(path = "RE_USER_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,32}"))]
    pub family_name: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,32}`
    #[validate(regex(path = "RE_USER_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,32}"))]
    pub given_name: String,
    pub language: Language,
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_groups"))]
    pub groups: Option<Vec<String>>,
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_roles"))]
    pub roles: Vec<String>,
    #[validate(range(min = 1672527600, max = 4070905200))]
    pub user_expires: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct NewUserRegistrationRequest {
    #[validate(email)]
    pub email: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,32}`
    #[validate(regex(path = "RE_USER_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,32}"))]
    pub family_name: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,32}`
    #[validate(regex(path = "RE_USER_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,32}"))]
    pub given_name: String,
    /// Validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]+`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+"))]
    pub pow: String,
    /// Validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]+`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+"))]
    pub redirect_uri: Option<String>,
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct NewRoleRequest {
    /// Validation: `^[a-z0-9-_/,:*]{2,64}$`
    #[validate(regex(path = "RE_GROUPS", code = "^[a-z0-9-_/,:*]{2,64}$"))]
    pub role: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct PaginationParams {
    pub page_size: Option<u16>,
    pub offset: Option<u16>,
    pub backwards: Option<bool>,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub continuation_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct PasskeyRequest {
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,32}`
    #[validate(regex(path = "RE_USER_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,32}"))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ScopeRequest {
    // `RE_GROUPS` is correct here
    /// Validation: `^[a-z0-9-_/,:*]{2,64}$`
    #[validate(regex(path = "RE_GROUPS", code = "^[a-z0-9-_/,:*]{2,64}$"))]
    pub scope: String,
    /// Validation: `^[a-zA-Z0-9-_/]{2,128}$`
    #[validate(custom(function = "validate_vec_attr"))]
    pub attr_include_access: Option<Vec<String>>,
    /// Validation: `^[a-zA-Z0-9-_/]{2,128}$`
    #[validate(custom(function = "validate_vec_attr"))]
    pub attr_include_id: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SearchParams {
    /// Data type
    pub ty: SearchParamsType,
    /// Index
    pub idx: SearchParamsIdx,
    /// The actual search query - validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%@]+`
    #[validate(regex(path = "RE_SEARCH", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%@]+"))]
    pub q: String,
    pub limit: Option<u16>,
}

#[derive(Debug, PartialEq, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum SearchParamsIdx {
    // user params
    Id,
    Email,
    // session params
    UserId,
    SessionId,
    Ip,
}

#[derive(Debug, PartialEq, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum SearchParamsType {
    User,
    Session,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct TokenRequest {
    /// Validation: `^(authorization_code|client_credentials|urn:ietf:params:oauth:grant-type:device_code|password|refresh_token)$`
    #[validate(regex(
        path = "RE_GRANT_TYPES",
        code = "^(authorization_code|client_credentials|urn:ietf:params:oauth:grant-type:device_code|password|refresh_token)$"
    ))]
    pub grant_type: String,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub code: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub redirect_uri: Option<String>,
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    #[validate(regex(
        path = "RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,128}$"
    ))]
    pub client_id: Option<String>,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub client_secret: Option<String>,
    /// Validation: `[a-zA-Z0-9-\\._~+/=]+`
    #[validate(regex(path = "RE_CODE_VERIFIER", code = "[a-zA-Z0-9-\\._~+/=]+"))]
    pub code_verifier: Option<String>,
    /// Validation: max length is 256
    #[validate(length(max = 256))]
    pub device_code: Option<String>,
    /// Validation: `email`
    #[validate(email)]
    pub username: Option<String>,
    /// max 256 characters
    #[validate(length(max = 256))]
    pub password: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub refresh_token: Option<String>,
}

impl TokenRequest {
    // by RFC, the client auth can be either sent inside the POST body, or as an Authorization header
    pub fn try_get_client_id_secret(
        &self,
        req: &HttpRequest,
    ) -> Result<(String, Option<String>), ErrorResponse> {
        let auth_header = req.headers().get(header::AUTHORIZATION).map(|h| {
            let (_, b64) = h
                .to_str()
                .unwrap_or_default()
                .split_once(' ')
                .unwrap_or(("", ""));
            b64
        });

        if let Some(header) = auth_header {
            let decoded = String::from_utf8(base64_decode(header)?)?;
            match decoded.split_once(':') {
                None => Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Bad Authorization header",
                )),
                Some((client_id, client_secret)) => {
                    Ok((client_id.to_string(), Some(client_secret.to_string())))
                }
            }
        } else {
            Ok((
                self.client_id.clone().unwrap_or_default(),
                self.client_secret.clone(),
            ))
        }
    }
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct TokenValidationRequest {
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateClientRequest {
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,256}$`
    #[validate(regex(
        path = "RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,256}$"
    ))]
    pub id: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub name: Option<String>,
    pub confidential: bool,
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    #[validate(custom(function = "validate_vec_uri"))]
    pub redirect_uris: Vec<String>,
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    #[validate(custom(function = "validate_vec_uri"))]
    pub post_logout_redirect_uris: Option<Vec<String>>,
    /// Validation: `Vec<^(http|https)://[a-z0-9.:-]+$>`
    #[validate(custom(function = "validate_vec_origin"))]
    pub allowed_origins: Option<Vec<String>>,
    pub enabled: bool,
    /// Validation: `Vec<^(authorization_code|client_credentials|password|refresh_token)$>`
    #[validate(custom(function = "validate_vec_grant_types"))]
    pub flows_enabled: Vec<String>,
    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    pub access_token_alg: JwkKeyPairAlg,
    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    pub id_token_alg: JwkKeyPairAlg,
    /// Validation: `10 <= auth_code_lifetime <= 300`
    #[validate(range(min = 10, max = 300))]
    pub auth_code_lifetime: i32,
    /// Validation: `10 <= access_token_lifetime <= 86400`
    #[validate(range(min = 10, max = 86400))]
    pub access_token_lifetime: i32,
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_scopes"))]
    pub scopes: Vec<String>,
    /// Validation: `Vec<^[a-z0-9-_/:\s]{0,64}$>`
    #[validate(custom(function = "validate_vec_scopes"))]
    pub default_scopes: Vec<String>,
    /// Validation: `Vec<^(plain|S256)$>`
    #[validate(custom(function = "validate_vec_challenge"))]
    pub challenges: Option<Vec<String>>,
    pub force_mfa: bool,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub client_uri: Option<String>,
    /// Validation: `Vec<^[a-zA-Z0-9\+.@/]{0,48}$>`
    #[validate(custom(function = "validate_vec_contact"))]
    pub contacts: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserRequest {
    /// Validation: `email`
    #[validate(email)]
    pub email: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub given_name: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
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
    #[validate(range(min = 1672527600, max = 4070905200))]
    pub user_expires: Option<i64>,
    #[validate]
    pub user_values: Option<UserValuesRequest>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateUserSelfRequest {
    /// Validation: `email`
    #[validate(email)]
    pub email: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub given_name: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub family_name: Option<String>,
    pub language: Option<Language>,
    pub password_current: Option<String>,
    pub mfa_code: Option<String>,
    /// Validation: Applies password policy
    pub password_new: Option<String>,
    #[validate]
    pub user_values: Option<UserValuesRequest>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UserValuesRequest {
    /// Validation: `[0-9]{4}-[0-9]{2}-[0-9]{2}`
    #[validate(regex(path = "RE_DATE_STR", code = "[0-9]{4}-[0-9]{2}-[0-9]{2}"))]
    pub birthdate: Option<String>,
    /// Validation: `+[0-9]{0,32}`
    #[validate(regex(path = "RE_PHONE", code = "+[0-9]{0,32}"))]
    pub phone: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-.\s]{0,48}`
    #[validate(regex(path = "RE_STREET", code = "[a-zA-Z0-9À-ÿ-.\\s]{0,48}"))]
    pub street: Option<String>,
    #[validate(range(min = 1000, max = 9999999))]
    pub zip: Option<i32>,
    /// Validation: `[a-zA-Z0-9À-ÿ-]{0,48}`
    #[validate(regex(path = "RE_CITY", code = "[a-zA-Z0-9À-ÿ-]{0,48}"))]
    pub city: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-]{0,48}`
    #[validate(regex(path = "RE_CITY", code = "[a-zA-Z0-9À-ÿ-]{0,48}"))]
    pub country: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UserAttrConfigRequest {
    /// Validation: `^[a-zA-Z0-9-_/]{2,32}$`
    #[validate(regex(path = "RE_ATTR", code = "^[a-z0-9-_/]{2,32}$"))]
    pub name: String,
    /// Validation: `^[a-zA-Z0-9-_/]{0,128}$`
    #[validate(regex(path = "RE_ATTR_DESC", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub desc: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UserAttrValueRequest {
    /// Validation: `^[a-zA-Z0-9-_/]{2,32}$`
    #[validate(regex(path = "RE_ATTR", code = "^[a-z0-9-_/]{2,32}$"))]
    pub key: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UserAttrValuesUpdateRequest {
    #[validate]
    pub values: Vec<UserAttrValueRequest>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct WebauthnAuthStartRequest {
    pub purpose: MfaPurpose,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct WebauthnAuthFinishRequest {
    /// Validation: `[a-zA-Z0-9]{48}`
    #[validate(regex(path = "RE_ALNUM_48", code = "[a-zA-Z0-9]{48}"))]
    pub code: String,
    /// Note: `ToSchema` does currently not exist for `webauthn_rs::prelude::PublicKeyCredential`
    #[schema(value_type = str)]
    pub data: webauthn_rs::prelude::PublicKeyCredential,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct WebauthnRegStartRequest {
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,32}`
    #[validate(regex(path = "RE_USER_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,32}"))]
    pub passkey_name: String,

    /// Validation: `email`
    #[validate(email)]
    pub email: Option<String>,
    /// Validation: `[a-zA-Z0-9]{64}`
    #[validate(regex(path = "RE_ALNUM_64", code = "[a-zA-Z0-9]{64}"))]
    pub magic_link_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct WebauthnRegFinishRequest {
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,32}`
    #[validate(regex(path = "RE_USER_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,32}"))]
    pub passkey_name: String,
    /// Note: `ToSchema` does currently not exist for `webauthn_rs::prelude::PublicKeyCredential`
    #[schema(value_type = str)]
    pub data: webauthn_rs::prelude::RegisterPublicKeyCredential,
    /// Validation: `[a-zA-Z0-9]{64}`
    #[validate(regex(path = "RE_ALNUM_64", code = "[a-zA-Z0-9]{64}"))]
    pub magic_link_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct WebIdRequest {
    pub custom_triples: Option<String>,
    pub expose_email: bool,
}

// validation helpers

fn validate_vec_attr(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;

    if value.is_empty() {
        err = Some("'validate_vec_attr' cannot be empty when provided");
    } else {
        value.iter().for_each(|v| {
            if !RE_ATTR.is_match(v) {
                err = Some("^[a-z0-9-_/]{2,128}$");
            }
        });
    }

    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

fn validate_vec_challenge(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;

    if value.is_empty() {
        err = Some("'challenges' cannot be empty when provided");
    } else {
        value.iter().for_each(|v| {
            if !RE_CHALLENGE.is_match(v) {
                err = Some("^(plain|S256)$");
            }
        });
    }

    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

fn validate_vec_contact(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_CONTACT.is_match(v) {
            err = Some("^[a-zA-Z0-9\\+.@/]{0,48}$");
        }
    });
    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

fn validate_vec_grant_types(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;

    if value.is_empty() {
        err = Some("'flows_enabled' cannot be empty when provided");
    } else {
        value.iter().for_each(|v| {
            if !RE_GRANT_TYPES.is_match(v) {
                err = Some("^(authorization_code|client_credentials|password|refresh_token)$");
            }
        });
    }

    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

fn validate_vec_origin(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_ORIGIN.is_match(v) {
            err = Some("^(http|https)://[a-z0-9.:-]+$");
        }
    });
    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

fn validate_vec_uri(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_URI.is_match(v) {
            err = Some("^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$");
        }
    });
    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

fn validate_vec_grant_type(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_GRANT_TYPES.is_match(v) {
            err = Some("authorization_code|client_credentials|password|refresh_token");
        }
    });
    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

// validate_vec_groups, _roles and _scopes do the same thing but are 3 functions just to
// be clear in the validation fields above that it does not create confusion, even if they
// all use the same `RE_GROUPS` regex.
fn validate_vec_groups(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_GROUPS.is_match(v) {
            err = Some("^[a-z0-9-_/,:*]{2,64}$");
        }
    });
    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

fn validate_vec_roles(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_GROUPS.is_match(v) {
            err = Some("^[a-z0-9-_/,:*]{2,64}$");
        }
    });
    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

fn validate_vec_scopes(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_GROUPS.is_match(v) {
            err = Some("^[a-z0-9-_/,:*]{2,64}$");
        }
    });
    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::request::ColorsRequest;
    use css_color::Srgb;
    use std::str::FromStr;

    #[test]
    pub fn test_css_color_validation() {
        let req = ColorsRequest {
            act1: "#000".to_string(),
            act1a: "#123456".to_string(),
            act2: "rgb(1,2,3)".to_string(),
            act2a: "rgba(123, 223, 121, .3)".to_string(),
            acnt: "hsl(360 100% 50%)".to_string(),
            acnta: "hsl(123 13% 23%)".to_string(),
            ok: "red".to_string(),
            err: "blue".to_string(),
            glow: "green".to_string(),
            gmid: "aliceblue".to_string(),
            ghigh: "antiquewhite".to_string(),
            text: "hsla(30,20%,60%,0.9)".to_string(),
            bg: "rgb(127  ,    211 , 200    )".to_string(),
        };
        assert!(req.validate_css().is_ok());

        assert!(Srgb::from_str("#00").is_err());
        assert!(Srgb::from_str("#12345").is_err());
        assert!(Srgb::from_str("hsl(360 100%)").is_err());
        assert!(Srgb::from_str(" ").is_err());
    }
}
