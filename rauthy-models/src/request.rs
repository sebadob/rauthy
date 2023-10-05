use crate::language::Language;
use actix_web::http::header;
use actix_web::HttpRequest;
use css_color::Srgb;
use rauthy_common::constants::{
    RE_ALG, RE_ALNUM, RE_ALNUM_24, RE_ALNUM_48, RE_ALNUM_64, RE_ALNUM_SPACE, RE_APP_ID, RE_ATTR,
    RE_ATTR_DESC, RE_CHALLENGE, RE_CLIENT_NAME, RE_CODE_CHALLENGE, RE_CODE_VERIFIER, RE_FLOWS,
    RE_GROUPS, RE_LOWERCASE, RE_MFA_CODE, RE_URI, RE_USER_NAME,
};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::base64_decode;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::{IntoParams, ToSchema};
use validator::{Validate, ValidationError};

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

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct AuthRequest {
    /// Validation: `^[a-z0-9-_/]{2,128}$`
    #[validate(regex(path = "RE_LOWERCASE", code = "^[a-z0-9-_/]{2,128}$"))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub redirect_uri: String,
    /// Validation: `[a-z0-9-_/]{2,128}`
    #[validate(regex(path = "RE_LOWERCASE", code = "[a-z0-9-_/]{2,128}"))]
    pub response_type: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-s]{2,128}`
    #[validate(regex(path = "RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-s]{2,128}"))]
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

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct EncKeyMigrateRequest {
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub key_id: String,
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
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub password: Option<String>,
    /// Validation: `^[a-z0-9-_/]{2,128}$`
    #[validate(regex(path = "RE_LOWERCASE", code = "^[a-z0-9-_/]{2,128}$"))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub redirect_uri: String,
    /// Validation: `Vec<^[a-zA-Z0-9\\s]$>`
    #[validate(custom(function = "validate_vec_scope"))]
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
    /// Validation: `[a-zA-Z0-9-._~]{43,128}`
    #[validate(regex(path = "RE_CODE_CHALLENGE", code = "[a-zA-Z0-9-._~]{43,128}"))]
    pub code_challenge_method: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct LoginRefreshRequest {
    /// Validation: `^[a-z0-9-_/]{2,128}$`
    #[validate(regex(path = "RE_LOWERCASE", code = "^[a-z0-9-_/]{2,128}$"))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub redirect_uri: String,
    /// Validation: `Vec<^[a-zA-Z0-9\\s]$>`
    #[validate(custom(function = "validate_vec_scope"))]
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
    /// Validation: `[a-zA-Z0-9-._~]{43,128}`
    #[validate(regex(path = "RE_CODE_CHALLENGE", code = "[a-zA-Z0-9-._~]{43,128}"))]
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

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct NewGroupRequest {
    /// Validation: `^[a-z0-9-_/,]{2,32}$`
    #[validate(regex(path = "RE_GROUPS", code = "^[a-z0-9-_/,]{2,32}$"))]
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
    /// Validation: `email`
    #[validate(email)]
    pub email: String,
    /// Validation: `[a-zA-Z0-9]{64}`
    #[validate(regex(path = "RE_ALNUM_64", code = "[a-zA-Z0-9]{64}"))]
    pub magic_link_id: String,
    /// Validation: Applies password policy
    pub password: String,
    /// Validation: `[a-zA-Z0-9]{48}`
    #[validate(regex(path = "RE_ALNUM_48", code = "[a-zA-Z0-9]{48}"))]
    pub mfa_code: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct RequestResetRequest {
    /// Validation: `email`
    #[validate(email)]
    pub email: String,
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
    /// Validation: `Vec<^[a-z0-9-_/,]{2,128}$>`
    #[validate(custom(function = "validate_vec_groups"))]
    pub groups: Option<Vec<String>>,
    /// Validation: `Vec<^[a-z0-9-_/,]{2,128}$>`
    #[validate(custom(function = "validate_vec_groups"))]
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
    pub pow: PowRequest,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct PowRequest {
    /// Validation: `[a-zA-Z0-9]{24}`
    #[validate(regex(path = "RE_ALNUM_24", code = "[a-zA-Z0-9]{24}"))]
    pub challenge: String,
    pub it: u64,
    /// Validation: `[a-zA-Z0-9]{64}`
    #[validate(regex(path = "RE_ALNUM_64", code = "[a-zA-Z0-9]{64}"))]
    pub verifier: String,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct RefreshTokenRequest {
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct NewRoleRequest {
    /// Validation: `^[a-z0-9-_/,]{2,32}$`
    #[validate(regex(path = "RE_GROUPS", code = "^[a-z0-9-_/,]{2,32}$"))]
    pub role: String,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct PasskeyRequest {
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,32}`
    #[validate(regex(path = "RE_USER_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,32}"))]
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ScopeRequest {
    /// Validation: `^[a-z0-9-_/,]{2,32}$`
    #[validate(regex(path = "RE_GROUPS", code = "^[a-z0-9-_/,]{2,32}$"))]
    pub scope: String,
    /// Validation: `^[a-zA-Z0-9-_/]{2,128}$`
    #[validate(custom(function = "validate_vec_attr"))]
    pub attr_include_access: Option<Vec<String>>,
    /// Validation: `^[a-zA-Z0-9-_/]{2,128}$`
    #[validate(custom(function = "validate_vec_attr"))]
    pub attr_include_id: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct TokenRequest {
    /// Validation: `^[a-z0-9-_/]{2,128}$`
    #[validate(regex(path = "RE_LOWERCASE", code = "^[a-z0-9-_/]{2,128}$"))]
    pub grant_type: String,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub code: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub redirect_uri: Option<String>,
    /// Validation: `^[a-z0-9-_/]{2,128}$`
    #[validate(regex(path = "RE_LOWERCASE", code = "^[a-z0-9-_/]{2,128}$"))]
    pub client_id: Option<String>,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub client_secret: Option<String>,
    /// Validation: `[a-zA-Z0-9-\\._~+/=]+`
    #[validate(regex(path = "RE_CODE_VERIFIER", code = "[a-zA-Z0-9-\\._~+/=]+"))]
    pub code_verifier: Option<String>,
    /// Validation: `email`
    #[validate(email)]
    pub username: Option<String>,
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
                    "Bad Authorization header".to_string(),
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
    /// Validation: `^[a-z0-9-_/]{2,128}$`
    #[validate(regex(path = "RE_LOWERCASE", code = "^[a-z0-9-_/]{2,128}$"))]
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
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    #[validate(custom(function = "validate_vec_uri"))]
    pub allowed_origins: Option<Vec<String>>,
    pub enabled: bool,
    /// Validation: `Vec<^(authorization_code|client_credentials|password|refresh_token)$>`
    #[validate(custom(function = "validate_vec_flows"))]
    pub flows_enabled: Vec<String>,
    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    #[validate(regex(path = "RE_ALG", code = "^(RS256|RS384|RS512|EdDSA)$"))]
    pub access_token_alg: String,
    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    #[validate(regex(path = "RE_ALG", code = "^(RS256|RS384|RS512|EdDSA)$"))]
    pub id_token_alg: String,
    pub refresh_token: bool,
    /// Validation: `10 <= auth_code_lifetime <= 300`
    #[validate(range(min = 10, max = 300))]
    pub auth_code_lifetime: i32,
    /// Validation: `60 <= access_token_lifetime <= 86400`
    #[validate(range(min = 60, max = 86400))]
    pub access_token_lifetime: i32,
    /// Validation: `Vec<^[a-z0-9-_/]{2,128}$>`
    #[validate(custom(function = "validate_vec_lowercase"))]
    pub scopes: Vec<String>,
    /// Validation: `Vec<^[a-z0-9-_/]{2,128}$>`
    #[validate(custom(function = "validate_vec_lowercase"))]
    pub default_scopes: Vec<String>,
    /// Validation: `Vec<^(plain|S256)$>`
    #[validate(custom(function = "validate_vec_challenge"))]
    pub challenges: Option<Vec<String>>,
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
    // optional to not break the public API and need a new major release
    pub language: Option<Language>,
    /// Validation: Applies password policy
    pub password: Option<String>,
    /// Validation: `Vec<^[a-z0-9-_/]{2,128}$>`
    #[validate(custom(function = "validate_vec_lowercase"))]
    pub roles: Vec<String>,
    /// Validation: `Vec<^[a-z0-9-_/]{2,128}$>`
    #[validate(custom(function = "validate_vec_lowercase"))]
    pub groups: Option<Vec<String>>,
    pub enabled: bool,
    pub email_verified: bool,
    #[validate(range(min = 1672527600, max = 4070905200))]
    pub user_expires: Option<i64>,
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
    // optional to not break the public API and need a new major release
    pub language: Option<Language>,
    pub password_current: Option<String>,
    pub mfa_code: Option<String>,
    /// Validation: Applies password policy
    pub password_new: Option<String>,
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

fn validate_vec_flows(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;

    if value.is_empty() {
        err = Some("'flows_enabled' cannot be empty when provided");
    } else {
        value.iter().for_each(|v| {
            if !RE_FLOWS.is_match(v) {
                err = Some("^(authorization_code|client_credentials|password|refresh_token)$");
            }
        });
    }

    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

fn validate_vec_lowercase(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_LOWERCASE.is_match(v) {
            err = Some("^[a-z0-9-_/]{2,128}$");
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

fn validate_vec_groups(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_GROUPS.is_match(v) {
            err = Some("^[a-z0-9-_/,]{2,128}$");
        }
    });
    if let Some(e) = err {
        return Err(ValidationError::new(e));
    }
    Ok(())
}

fn validate_vec_scope(value: &[String]) -> Result<(), ValidationError> {
    let mut err = None;
    value.iter().for_each(|v| {
        if !RE_ALNUM_SPACE.is_match(v) {
            err = Some("^[a-zA-Z0-9\\s]$");
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
