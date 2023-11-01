use crate::entity::api_keys::{ApiKey, ApiKeyAccess};
use crate::entity::clients::Client;
use crate::entity::jwk::{JWKSPublicKey, JwkKeyPairAlg, JwkKeyPairType, JWKS};
use crate::entity::password::PasswordPolicy;
use crate::entity::scopes::Scope;
use crate::entity::sessions::SessionState;
use crate::entity::user_attr::{UserAttrConfigEntity, UserAttrValueEntity};
use crate::entity::users::{AccountType, User};
use crate::entity::webauthn::PasskeyEntity;
use crate::language::Language;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use tracing::debug;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiKeysResponse {
    pub keys: Vec<ApiKeyResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct ApiKeyResponse {
    pub name: String,
    /// unix timestamp
    pub created: i64,
    /// unix timestamp
    pub expires: Option<i64>,
    pub access: Vec<ApiKeyAccess>,
}

impl From<ApiKey> for ApiKeyResponse {
    fn from(value: ApiKey) -> Self {
        Self {
            name: value.name,
            created: value.created,
            expires: value.expires,
            access: value.access,
        }
    }
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
    pub refresh_token: bool,
    pub auth_code_lifetime: i32,
    pub access_token_lifetime: i32,
    pub scopes: Vec<String>,
    pub default_scopes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenges: Option<Vec<String>>,
    pub force_mfa: bool,
}

impl From<Client> for ClientResponse {
    fn from(client: Client) -> Self {
        let redirect_uris = client.get_redirect_uris();
        let post_logout_redirect_uris = client.get_post_logout_uris();
        let allowed_origins = client.get_allowed_origins();
        let flows_enabled = client.get_flows();
        let scopes = client.get_scopes();
        let default_scopes = client.get_default_scopes();
        let challenges = client.get_challenges();

        Self {
            id: client.id,
            name: client.name,
            enabled: client.enabled,
            confidential: client.confidential,
            redirect_uris,
            post_logout_redirect_uris,
            allowed_origins,
            flows_enabled,
            access_token_alg: client.access_token_alg,
            id_token_alg: client.id_token_alg,
            refresh_token: client.refresh_token,
            auth_code_lifetime: client.auth_code_lifetime,
            access_token_lifetime: client.access_token_lifetime,
            scopes,
            default_scopes,
            challenges,
            force_mfa: client.force_mfa,
        }
    }
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

impl From<JWKS> for JWKSCerts {
    fn from(jwks: JWKS) -> Self {
        let mut keys = Vec::with_capacity(jwks.keys.len());
        for k in jwks.keys {
            keys.push(JWKSPublicKeyCerts::from(k));
        }
        Self { keys }
    }
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

impl From<JWKSPublicKey> for JWKSPublicKeyCerts {
    fn from(pk: JWKSPublicKey) -> Self {
        Self {
            kty: pk.kty,
            alg: pk.alg.unwrap_or_default(),
            crv: pk.crv,
            kid: pk.kid,
            n: pk.n,
            e: pk.e,
            x: pk.x,
        }
    }
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LoginTimeResponse {
    pub argon2_params: Argon2ParamsResponse,
    pub login_time: u32,
    pub num_cpus: usize,
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

impl From<PasskeyEntity> for PasskeyResponse {
    fn from(value: PasskeyEntity) -> Self {
        Self {
            name: value.name,
            registered: value.registered,
            last_used: value.last_used,
            user_verified: value.user_verified,
        }
    }
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

impl From<PasswordPolicy> for PasswordPolicyResponse {
    fn from(r: PasswordPolicy) -> Self {
        Self {
            length_min: r.length_min,
            length_max: r.length_max,
            include_lower_case: r.include_lower_case,
            include_upper_case: r.include_upper_case,
            include_digits: r.include_digits,
            include_special: r.include_special,
            valid_days: r.valid_days,
            not_recently_used: r.not_recently_used,
        }
    }
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

impl From<Scope> for ScopeResponse {
    fn from(value: Scope) -> Self {
        let attr_include_access = value
            .attr_include_access
            .map(|attr| attr.split(',').map(String::from).collect());
        let attr_include_id = value
            .attr_include_id
            .map(|attr| attr.split(',').map(String::from).collect());

        Self {
            id: value.id,
            name: value.name,
            attr_include_access,
            attr_include_id,
        }
    }
}

// TODO benchmark, which of these 2 implementations is faster in the end
#[derive(Debug, Serialize, ToSchema)]
pub struct SessionResponse<'a> {
    pub id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<&'a str>,
    pub is_mfa: bool,
    pub state: &'a SessionState,
    pub exp: i64,
    pub last_seen: i64,
    pub remote_ip: Option<&'a str>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionInfoResponse<'a> {
    pub id: &'a String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub csrf_token: Option<&'a String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<&'a String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<&'a String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<&'a String>,
    /// format: `OffsetDateTime`
    #[schema(value_type = str)]
    #[serde(with = "time::serde::rfc3339")]
    pub exp: OffsetDateTime,
    /// format: `OffsetDateTime`
    #[schema(value_type = str)]
    #[serde(with = "time::serde::rfc3339")]
    pub timeout: OffsetDateTime,
}

#[derive(Debug, Serialize, ToSchema)]
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
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserAttrConfigResponse {
    pub values: Vec<UserAttrConfigEntity>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserAttrValueResponse {
    pub key: String,
    pub value: serde_json::Value,
}

impl From<UserAttrValueEntity> for UserAttrValueResponse {
    fn from(value: UserAttrValueEntity) -> Self {
        debug!("{:?}", value);
        let val = serde_json::from_slice(&value.value).unwrap();
        debug!("{:?}", val);
        Self {
            key: value.key,
            value: val,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserAttrValuesResponse {
    pub values: Vec<UserAttrValueResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct Userinfo {
    pub id: String,
    pub sub: String,
    pub email: String,
    pub email_verified: bool,
    pub name: String,
    pub roles: Vec<String>,
    pub groups: Vec<String>,
    pub preferred_username: String,
    pub given_name: String,
    pub family_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum UserAccountTypeResponse {
    New,
    Password,
    Passkey,
}

impl From<AccountType> for UserAccountTypeResponse {
    fn from(value: AccountType) -> Self {
        match value {
            AccountType::New => Self::New,
            AccountType::Password => Self::Password,
            AccountType::Passkey => Self::Passkey,
        }
    }
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
    pub webauthn_user_id: Option<String>,
}

impl From<User> for UserResponse {
    fn from(u: User) -> Self {
        let roles = u.get_roles();
        let groups = if u.groups.is_some() {
            Some(u.get_groups())
        } else {
            None
        };
        let account_type = UserAccountTypeResponse::from(u.account_type());

        Self {
            id: u.id,
            email: u.email,
            given_name: u.given_name,
            family_name: u.family_name,
            language: u.language,
            roles,
            groups,
            enabled: u.enabled,
            email_verified: u.email_verified,
            password_expires: u.password_expires,
            created_at: u.created_at,
            last_login: u.last_login,
            last_failed_login: u.last_failed_login,
            failed_login_attempts: u.failed_login_attempts,
            user_expires: u.user_expires,
            account_type,
            webauthn_user_id: u.webauthn_user_id,
        }
    }
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
pub struct AppVersionResponse {
    pub current: String,
    pub last_check: Option<i64>,
    pub latest: Option<String>,
    pub latest_url: Option<String>,
    pub update_available: bool,
}
