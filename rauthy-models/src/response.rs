use crate::app_state::AppState;
use crate::entity::api_keys::{ApiKey, ApiKeyAccess};
use crate::entity::auth_providers::{AuthProvider, AuthProviderType};
use crate::entity::clients::Client;
use crate::entity::clients_dyn::ClientDyn;
use crate::entity::devices::DeviceEntity;
use crate::entity::jwk::{JWKSPublicKey, JwkKeyPairAlg, JwkKeyPairType, JWKS};
use crate::entity::password::PasswordPolicy;
use crate::entity::scopes::Scope;
use crate::entity::sessions::SessionState;
use crate::entity::user_attr::{UserAttrConfigEntity, UserAttrValueEntity};
use crate::entity::users::{AccountType, User};
use crate::entity::users_values::UserValues;
use crate::entity::webauthn::PasskeyEntity;
use crate::entity::webids::WebId;
use crate::language::Language;
use crate::{AddressClaim, JktClaim};
use actix_web::web;
use rauthy_common::error_response::ErrorResponse;
use rio_api::formatter::TriplesFormatter;
use rio_api::model::{Literal, NamedNode, Subject, Term, Triple};
use rio_turtle::TurtleFormatter;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
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
    pub client_uri: Option<String>,
    pub contacts: Option<Vec<String>>,
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
        let contacts = client.get_contacts();

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
            client_uri: client.client_uri,
            contacts,
        }
    }
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

impl From<DeviceEntity> for DeviceResponse {
    fn from(value: DeviceEntity) -> Self {
        Self {
            id: value.id,
            client_id: value.client_id,
            user_id: value.user_id,
            created: value.created,
            access_exp: value.access_exp,
            refresh_exp: value.refresh_exp,
            peer_ip: value.peer_ip,
            name: value.name,
        }
    }
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

impl DynamicClientResponse {
    pub fn build(
        data: &web::Data<AppState>,
        client: Client,
        client_dyn: ClientDyn,
        map_registration_client_uri: bool,
    ) -> Result<Self, ErrorResponse> {
        let contacts = client.get_contacts();

        let redirect_uris = client.get_redirect_uris();
        let grant_types = client.get_flows();
        let post_logout_redirect_uri = client.get_redirect_uris().first().cloned();

        let client_secret = client.get_secret_cleartext()?;
        let (registration_access_token, registration_client_uri) = if map_registration_client_uri {
            (
                Some(client_dyn.registration_token_plain()?),
                Some(ClientDyn::registration_client_uri(data, &client_dyn.id)),
            )
        } else {
            (None, None)
        };

        Ok(Self {
            client_id: client.id,
            client_name: client.name,
            client_uri: client.client_uri,
            contacts,
            client_secret,
            // TODO check if we can make sure that a client will renew the secret properly -> let it expire then
            client_secret_expires_at: 0,
            redirect_uris,
            post_logout_redirect_uri,
            registration_access_token,
            registration_client_uri,
            grant_types,
            id_token_signed_response_alg: client.id_token_alg,
            token_endpoint_auth_method: client_dyn.token_endpoint_auth_method,
            token_endpoint_auth_signing_alg: client.access_token_alg,
        })
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

impl TryFrom<AuthProvider> for ProviderResponse {
    type Error = ErrorResponse;

    fn try_from(value: AuthProvider) -> Result<Self, Self::Error> {
        let secret = AuthProvider::get_secret_cleartext(&value.secret)?;
        Ok(Self {
            id: value.id,
            name: value.name,
            typ: value.typ,
            enabled: value.enabled,
            issuer: value.issuer,
            authorization_endpoint: value.authorization_endpoint,
            token_endpoint: value.token_endpoint,
            userinfo_endpoint: value.userinfo_endpoint,
            client_id: value.client_id,
            client_secret: secret,
            scope: value.scope,
            admin_claim_path: value.admin_claim_path,
            admin_claim_value: value.admin_claim_value,
            mfa_claim_path: value.mfa_claim_path,
            mfa_claim_value: value.mfa_claim_value,
            danger_allow_insecure: value.allow_insecure_requests,
            use_pkce: value.use_pkce,
            root_pem: value.root_pem,
        })
    }
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

impl From<AccountType> for UserAccountTypeResponse {
    fn from(value: AccountType) -> Self {
        match value {
            AccountType::New => Self::New,
            AccountType::Password => Self::Password,
            AccountType::Passkey => Self::Passkey,
            AccountType::Federated => Self::Federated,
            AccountType::FederatedPasskey => Self::FederatedPasskey,
            AccountType::FederatedPassword => Self::FederatedPassword,
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
    pub webauthn_user_id: Option<String>, // TODO get rid of the webauthn user id ? Not needed at all?
    pub user_values: UserValuesResponse,
    pub auth_provider_id: Option<String>,
    pub federation_uid: Option<String>,
}

impl UserResponse {
    pub fn build(u: User, v: Option<UserValues>) -> Self {
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
            user_values: v.map(UserValuesResponse::from).unwrap_or_default(),
            auth_provider_id: u.auth_provider_id,
            federation_uid: u.federation_uid,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UserResponseSimple {
    pub id: String,
    pub email: String,
}

impl From<User> for UserResponseSimple {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            email: value.email,
        }
    }
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

impl From<UserValues> for UserValuesResponse {
    fn from(value: UserValues) -> Self {
        Self {
            birthdate: value.birthdate,
            phone: value.phone,
            street: value.street,
            zip: value.zip,
            city: value.city,
            country: value.country,
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
pub struct WebIdResponse {
    pub webid: WebId,
    pub issuer: String,
    pub email: String,
    pub given_name: String,
    pub family_name: String,
    pub language: Language,
}

impl WebIdResponse {
    fn triple<'a>(s: impl Into<Subject<'a>>, p: &'a str, o: impl Into<Term<'a>>) -> Triple<'a> {
        Triple {
            subject: s.into(),
            predicate: NamedNode { iri: p },
            object: o.into(),
        }
    }

    /// Serialize the webid response to a graph serializable syntax.
    fn serialize_turtle(
        &self,
        formatter: &mut TurtleFormatter<Vec<u8>>,
    ) -> Result<(), ErrorResponse> {
        let webid = &self.webid;
        let t_user = NamedNode {
            iri: &WebId::resolve_webid_uri(&webid.user_id),
        };
        let t_card = NamedNode {
            iri: &WebId::resolve_webid_card_uri(&webid.user_id),
        };
        let t_type = "http://www.w3.org/1999/02/22-rdf-syntax-ns#type";

        formatter.format(&Self::triple(
            t_card,
            t_type,
            NamedNode {
                iri: "http://xmlns.com/foaf/0.1/PersonalProfileDocument",
            },
        ))?;

        formatter.format(&Self::triple(
            t_card,
            "http://xmlns.com/foaf/0.1/primaryTopic",
            t_user,
        ))?;

        formatter.format(&Self::triple(
            t_user,
            "http://www.w3.org/ns/solid/terms#oidcIssuer",
            NamedNode { iri: &self.issuer },
        ))?;

        // rdf:type
        formatter.format(&Self::triple(
            t_user,
            t_type,
            NamedNode {
                iri: "http://xmlns.com/foaf/0.1/Person",
            },
        ))?;

        // foaf:name
        formatter.format(&Self::triple(
            t_user,
            "http://xmlns.com/foaf/0.1/givenname",
            Literal::Simple {
                value: &self.given_name,
            },
        ))?;
        formatter.format(&Self::triple(
            t_user,
            "http://xmlns.com/foaf/0.1/family_name",
            Literal::Simple {
                value: &self.family_name,
            },
        ))?;

        // foaf:mbox
        if webid.expose_email {
            formatter.format(&Self::triple(
                t_user,
                "http://xmlns.com/foaf/0.1/mbox",
                NamedNode {
                    iri: &format!("mailto:{}", &self.email),
                },
            ))?;
        }

        // Format custom data.
        webid
            .fmt_custom_triples_to_ttl(formatter)
            .expect("Must be valid");

        Ok(())
    }

    pub fn as_turtle(&self) -> Result<String, ErrorResponse> {
        let mut formatter = TurtleFormatter::new(Vec::<u8>::new());
        self.serialize_turtle(&mut formatter)?;

        let finished = formatter.finish()?;
        Ok(String::from_utf8(finished).expect("Must be a valid turtle string."))
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AppVersionResponse {
    pub current: String,
    pub last_check: Option<i64>,
    pub latest: Option<String>,
    pub latest_url: Option<String>,
    pub update_available: bool,
}

#[cfg(test)]
mod tests {
    use rstest::rstest;
    use std::env;

    use crate::{entity::webids::WebId, response::WebIdResponse};

    #[rstest]
    #[case(
        Some(r#"
<http://localhost:8080/auth/webid/za9UxpH7XVxqrtpEbThoqvn2/profile#me>
<http://www.w3.org/ns/solid/terms#oidcIssuer>
<http://localhost:8080/auth/v1> .
"#),
  "<http://localhost:8081/auth/SomeId123/profile> <http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://xmlns.com/foaf/0.1/PersonalProfileDocument> ;\n\t<http://xmlns.com/foaf/0.1/primaryTopic> <http://localhost:8081/auth/SomeId123/profile#me> .\n<http://localhost:8081/auth/SomeId123/profile#me> <http://www.w3.org/ns/solid/terms#oidcIssuer> <http://localhost:8080/auth/v1> ;\n\t<http://www.w3.org/1999/02/22-rdf-syntax-ns#type> <http://xmlns.com/foaf/0.1/Person> ;\n\t<http://xmlns.com/foaf/0.1/givenname> \"Given\" ;\n\t<http://xmlns.com/foaf/0.1/family_name> \"Family\" ;\n\t<http://xmlns.com/foaf/0.1/mbox> <mailto:mail@example.com> .\n<http://localhost:8080/auth/webid/za9UxpH7XVxqrtpEbThoqvn2/profile#me> <http://www.w3.org/ns/solid/terms#oidcIssuer> <http://localhost:8080/auth/v1> .\n"
    )]
    #[ignore] // this is currently ignored, because setting the PUB_URL here interferes with other tests in CI
    fn test_web_id_response(#[case] custom_triples: Option<&str>, #[case] expected_resp: &str) {
        env::set_var("PUB_URL", "localhost:8081".to_string());

        let resp = WebIdResponse {
            webid: WebId::try_new("SomeId123".to_string(), custom_triples, true)
                .expect("Invalid custom triples in test case"),
            issuer: "http://localhost:8080/auth/v1".to_string(),
            email: "mail@example.com".to_string(),
            given_name: "Given".to_string(),
            family_name: "Family".to_string(),
            language: Default::default(),
        };

        assert_eq!(resp.as_turtle().unwrap(), expected_resp);
        // TODO we actually need real test cases with complex custom_triples to make sure
        // the outcome is as expected
    }
}
