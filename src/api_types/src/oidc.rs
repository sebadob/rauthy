use crate::cust_validation::validate_vec_scopes;
use crate::generic::PasswordPolicyResponse;
use crate::sessions::SessionState;
use actix_web::HttpRequest;
use actix_web::http::header;
use rauthy_common::regex::{
    RE_ALNUM, RE_BASE64, RE_CLIENT_ID_EPHEMERAL, RE_CODE_CHALLENGE_METHOD, RE_CODE_VERIFIER,
    RE_GRANT_TYPES, RE_LOWERCASE, RE_SCOPE_SPACE, RE_URI,
};
use rauthy_common::utils::base64_decode;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::fmt::{Display, Formatter};
use time::OffsetDateTime;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AddressClaim {
    pub formatted: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    // pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema, IntoParams)]
pub struct AuthRequest {
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    #[validate(regex(
        path = "*RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,256}$"
    ))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub redirect_uri: String,
    /// Validation: `[a-z0-9-_/]{2,128}`
    #[validate(regex(path = "*RE_LOWERCASE", code = "[a-z0-9-_/]{2,128}"))]
    pub response_type: String,
    /// Validation: `[a-zA-Z0-9-_/:\s*]{0,512}`
    #[validate(regex(path = "*RE_SCOPE_SPACE", code = "[a-zA-Z0-9-_/:\\s*]{0,512}"))]
    #[serde(default = "default_scope")]
    pub scope: String,
    /// Validation: max length 2048
    #[validate(length(max = 2048))]
    pub state: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub code_challenge: Option<String>,
    /// Validation: `plain|S256`
    #[validate(regex(path = "*RE_CODE_CHALLENGE_METHOD", code = "plain|S256"))]
    pub code_challenge_method: Option<String>,
    #[validate(range(min = 0))]
    pub max_age: Option<i64>,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "*RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub prompt: Option<String>,
}

#[inline]
fn default_scope() -> String {
    String::from("openid")
}

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct BackchannelLogoutRequest {
    #[validate(regex(path = "*RE_BASE64"))]
    pub logout_token: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CertsParams {
    pub skip_okp: Option<bool>,
}

#[derive(Deserialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
#[serde(rename_all = "lowercase")]
pub enum DeviceAcceptedRequest {
    Accept,
    Decline,
    Pending,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Clone, Serialize))]
pub struct LoginRequest {
    /// Validation: `email`
    #[validate(email)]
    pub email: String,
    /// Validation: Applies password policy - max 256 characters
    #[validate(length(max = 256))]
    pub password: Option<String>,
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    #[validate(regex(
        path = "*RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,128}$"
    ))]
    pub pow: String,
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    #[validate(regex(
        path = "*RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,128}$"
    ))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub redirect_uri: String,
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_scopes"))]
    pub scopes: Option<Vec<String>>,
    /// Validation: max length 2048
    #[validate(length(max = 2048))]
    pub state: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub nonce: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub code_challenge: Option<String>,
    /// Validation: `plain|S256`
    #[validate(regex(path = "*RE_CODE_CHALLENGE_METHOD", code = "plain|S256"))]
    pub code_challenge_method: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct LoginRefreshRequest {
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    #[validate(regex(
        path = "*RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,128}$"
    ))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub redirect_uri: String,
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_scopes"))]
    pub scopes: Option<Vec<String>>,
    /// Validation: max length 2048
    #[validate(length(max = 2048))]
    pub state: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub nonce: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub code_challenge: Option<String>,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "*RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub code_challenge_method: Option<String>,
}

#[derive(Default, Deserialize, Validate, ToSchema, IntoParams)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct LogoutRequest {
    /// Valid `id_token` issued by Rauthy to do an RP Initiated Logout.
    /// https://openid.net/specs/openid-connect-rpinitiated-1_0.html
    ///
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub id_token_hint: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub post_logout_redirect_uri: Option<String>,
    /// Validation: max length 2048
    #[validate(length(max = 2048))]
    pub state: Option<String>,
    /// Logout Token used for OIDC Backchannel Logout
    /// https://openid.net/specs/openid-connect-backchannel-1_0.html#LogoutToken
    ///
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub logout_token: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct DeviceGrantRequest {
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$"))]
    pub client_id: String,
    /// Validation: max length is 256
    #[validate(length(max = 256))]
    pub client_secret: Option<String>,
    /// Validation: `[a-zA-Z0-9-_/:\s*]{0,512}`
    #[validate(regex(path = "*RE_SCOPE_SPACE", code = "[a-zA-Z0-9-_/:\\s*]{0,512}"))]
    pub scope: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct DeviceVerifyRequest {
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "*RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub user_code: String,
    /// Validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]+`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+"))]
    pub pow: String,
    /// If `DeviceAcceptedRequest::Pending`, information about the request will be returned.
    /// If `DeviceAcceptedRequest::Accept` - the device will get a Token Set
    /// If `DeviceAcceptedRequest::Decline` - the code request will be deleted and rejected
    pub device_accepted: DeviceAcceptedRequest,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct TokenRequest {
    /// Validation: `^(authorization_code|client_credentials|urn:ietf:params:oauth:grant-type:device_code|password|refresh_token)$`
    #[validate(regex(
        path = "*RE_GRANT_TYPES",
        code = "^(authorization_code|client_credentials|urn:ietf:params:oauth:grant-type:device_code|password|refresh_token)$"
    ))]
    pub grant_type: String,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "*RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub code: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub redirect_uri: Option<String>,
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    #[validate(regex(
        path = "*RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,128}$"
    ))]
    pub client_id: Option<String>,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "*RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub client_secret: Option<String>,
    /// Validation: `[a-zA-Z0-9-\\._~+/=]+`
    #[validate(regex(path = "*RE_CODE_VERIFIER", code = "[a-zA-Z0-9-\\._~+/=]+"))]
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
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
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

#[derive(Deserialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct TokenRevocationRequest {
    pub token: String,
    pub token_type_hint: Option<String>,
    pub client_id: Option<String>,
    pub client_secret: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct TokenValidationRequest {
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub token: String,
}

#[derive(Serialize, ToSchema)]
pub struct DeviceCodeResponse<'a> {
    pub device_code: &'a str,
    pub user_code: &'a str,
    pub verification_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_uri_complete: Option<String>,
    pub expires_in: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u32>,
}

#[derive(Serialize, ToSchema)]
pub struct DeviceVerifyResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct JktClaim<'a> {
    pub jkt: &'a str,
}

#[derive(Default, PartialEq, Serialize, Deserialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub enum JwkKeyPairAlg {
    RS256,
    RS384,
    RS512,
    #[default]
    EdDSA,
}

impl Display for JwkKeyPairAlg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            JwkKeyPairAlg::RS256 => "RS256",
            JwkKeyPairAlg::RS384 => "RS384",
            JwkKeyPairAlg::RS512 => "RS512",
            JwkKeyPairAlg::EdDSA => "EdDSA",
        };
        write!(f, "{s}")
    }
}

#[derive(Default, Serialize, Deserialize, ToSchema)]
pub enum JwkKeyPairType {
    RSA,
    #[default]
    OKP,
}

#[derive(Serialize, ToSchema)]
pub struct JWKSPublicKeyCerts {
    pub kty: JwkKeyPairType,
    pub alg: JwkKeyPairAlg,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crv: Option<String>, // Ed25519
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub n: Option<String>, // RSA
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e: Option<String>, // RSA
    #[serde(skip_serializing_if = "Option::is_none")]
    pub x: Option<String>, // OKP
}

#[derive(Default, Serialize, ToSchema)]
pub struct JWKSCerts {
    pub keys: Vec<JWKSPublicKeyCerts>,
}

#[derive(Serialize, ToSchema)]
pub struct PasswordResetResponse {
    pub csrf_token: String,
    pub password_policy: PasswordPolicyResponse,
}

#[derive(Serialize, ToSchema)]
pub struct OAuth2ErrorResponse<'a> {
    pub error: OAuth2ErrorTypeResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<Cow<'a, str>>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum OAuth2ErrorTypeResponse {
    InvalidRequest,
    InvalidClient,
    InvalidGrant,
    UnauthorizedClient,
    UnsupportedGrantType,
    InvalidScope,
    // specific to the device grant
    AuthorizationPending,
    SlowDown,
    AccessDenied,
    ExpiredToken,
}

#[derive(Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct SessionInfoResponse<'a> {
    pub id: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Cow<'a, str>>,
    /// format: `OffsetDateTime`
    #[schema(value_type = str)]
    #[serde(with = "time::serde::rfc3339")]
    pub exp: OffsetDateTime,
    /// format: `OffsetDateTime`
    #[schema(value_type = str)]
    #[serde(with = "time::serde::rfc3339")]
    pub timeout: OffsetDateTime,
    pub state: SessionState,
}

#[derive(Default, Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct TokenInfo<'a> {
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Cow<'a, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iat: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nbf: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<i64>,
    #[serde(borrow, skip_serializing_if = "Option::is_none")]
    pub cnf: Option<JktClaim<'a>>,
}
