use crate::{
    AddressClaim, ApiKeyAccess, AuthProviderType, JktClaim, JwkKeyPairAlg, JwkKeyPairType,
    Language, SessionState,
};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::borrow::Cow;
use time::OffsetDateTime;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiKeysResponse {
    pub keys: Vec<ApiKeyResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiKeyResponse {
    pub name: String,
    /// unix timestamp
    pub created: i64,
    /// unix timestamp
    pub expires: Option<i64>,
    pub access: Vec<ApiKeyAccess>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Argon2ParamsResponse {
    pub m_cost: u32,
    pub t_cost: u32,
    pub p_cost: u32,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct BlacklistResponse {
    pub ips: Vec<BlacklistedIp>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct BlacklistedIp {
    pub ip: String,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ClientResponse {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub enabled: bool,
    pub confidential: bool,
    pub redirect_uris: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_logout_redirect_uris: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    pub flows_enabled: Vec<String>,
    pub access_token_alg: String,
    pub id_token_alg: String,
    pub auth_code_lifetime: i32,
    pub access_token_lifetime: i32,
    pub scopes: Vec<String>,
    pub default_scopes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenges: Option<Vec<String>>,
    pub force_mfa: bool,
    pub client_uri: Option<String>,
    pub contacts: Option<Vec<String>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CsrfTokenResponse {
    pub csrf_token: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DeviceResponse {
    pub id: String,
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    pub created: i64,
    pub access_exp: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_exp: Option<i64>,
    pub peer_ip: String,
    pub name: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DeviceCodeResponse<'a> {
    pub device_code: &'a str,
    pub user_code: &'a str,
    pub verification_uri: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_uri_complete: Option<String>,
    pub expires_in: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<u8>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DeviceVerifyResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct DynamicClientResponse {
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    // TODO can we "trust" in a client doing a PUT on Self before en expiry to
    // implement proper forced secret rotation from time to time? -> not mentioned in RFC
    pub client_secret_expires_at: i64,

    pub redirect_uris: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_logout_redirect_uri: Option<String>,

    // only Some(_) after new token has been issued
    // TODO rotate on PUT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_access_token: Option<String>,
    // This is the uri for PUT requests from Self -> only provide if `registration_access_token`
    // has been updated as well
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_client_uri: Option<String>,

    pub grant_types: Vec<String>,
    pub id_token_signed_response_alg: String,
    pub token_endpoint_auth_method: String,
    pub token_endpoint_auth_signing_alg: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ClientSecretResponse {
    pub id: String,
    pub confidential: bool,
    pub secret: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EncKeysResponse<'a> {
    pub active: &'a str,
    pub keys: Vec<&'a str>,
}

#[derive(Debug, Default, Serialize, ToSchema)]
pub struct HealthResponse {
    pub is_db_alive: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = str)]
    pub cache_health: Option<redhac::QuorumHealth>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = str)]
    pub cache_state: Option<redhac::QuorumState>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cache_connected_hosts: Option<usize>,
}

#[derive(Debug, Default, Serialize, ToSchema)]
pub struct JWKSCerts {
    pub keys: Vec<JWKSPublicKeyCerts>,
}

#[derive(Debug, Serialize, ToSchema)]
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
    pub x: Option<String>, // OCT
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LoginTimeResponse {
    pub argon2_params: Argon2ParamsResponse,
    pub login_time: u32,
    pub num_cpus: usize,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct OAuth2ErrorResponse<'a> {
    pub error: OAuth2ErrorTypeResponse,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_description: Option<Cow<'a, str>>,
}

#[derive(Debug, Serialize, ToSchema)]
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

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PasskeyResponse {
    pub name: String,
    /// format: `NaiveDateTime`
    pub registered: i64,
    /// format: `NaiveDateTime`
    pub last_used: i64,
    pub user_verified: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PasswordPolicyResponse {
    pub length_min: i32,
    pub length_max: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_lower_case: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_upper_case: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_digits: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_special: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_recently_used: Option<i32>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ProviderResponse {
    pub id: String,
    pub name: String,
    pub typ: AuthProviderType,
    pub enabled: bool,

    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,

    pub client_id: String,
    pub client_secret: Option<String>,
    pub scope: String,

    pub admin_claim_path: Option<String>,
    pub admin_claim_value: Option<String>,
    pub mfa_claim_path: Option<String>,
    pub mfa_claim_value: Option<String>,

    pub danger_allow_insecure: bool,
    pub use_pkce: bool,

    pub root_pem: Option<String>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ProviderLinkedUserResponse {
    pub id: String,
    pub email: String,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct ProviderLookupResponse<'a> {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub scope: String,
    pub root_pem: &'a Option<String>,
    pub use_pkce: bool,
    pub danger_allow_insecure: bool,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ScopeResponse {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr_include_access: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr_include_id: Option<Vec<String>>,
}

// TODO benchmark, which of these 2 implementations is faster in the end
#[derive(Debug, Serialize, ToSchema)]
pub struct SessionResponse<'a> {
    pub id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<&'a str>,
    pub is_mfa: bool,
    pub state: SessionState,
    pub exp: i64,
    pub last_seen: i64,
    pub remote_ip: Option<&'a str>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
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

// #[derive(Debug, Serialize, ToSchema)]
// pub struct SessionInfoResponse<'a> {
//     pub id: &'a String,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub csrf_token: Option<&'a String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub user_id: Option<&'a String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub roles: Option<&'a String>,
//     #[serde(skip_serializing_if = "Option::is_none")]
//     pub groups: Option<&'a String>,
//     /// format: `OffsetDateTime`
//     #[schema(value_type = str)]
//     #[serde(with = "time::serde::rfc3339")]
//     pub exp: OffsetDateTime,
//     /// format: `OffsetDateTime`
//     #[schema(value_type = str)]
//     #[serde(with = "time::serde::rfc3339")]
//     pub timeout: OffsetDateTime,
//     pub state: &'a SessionState,
// }

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct TokenInfo {
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cnf: Option<JktClaim>,
}

#[derive(Clone, Debug, FromRow, Serialize, Deserialize, ToSchema)]
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
    /// format: `NaiveDateTime`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_expires: Option<i64>,
    /// format: `NaiveDateTime`
    #[schema(value_type = str)]
    pub created_at: i64,
    /// format: `NaiveDateTime`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_login: Option<i64>,
    /// format: `NaiveDateTime`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_failed_login: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_login_attempts: Option<i64>,
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
pub struct AppVersionResponse {
    pub current: String,
    pub last_check: Option<i64>,
    pub latest: Option<String>,
    pub latest_url: Option<String>,
    pub update_available: bool,
}
