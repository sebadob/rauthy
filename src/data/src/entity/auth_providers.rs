use crate::api_cookie::ApiCookie;
use crate::database::{Cache, DB};
use crate::entity::logos::{Logo, LogoType};
use crate::entity::users::User;
use crate::entity::users_values::UserValues;
use crate::language::Language;
use crate::rauthy_config::RauthyConfig;
use actix_web::cookie::Cookie;
use chrono::Utc;
use cryptr::EncValue;
use hiqlite::Row;
use hiqlite_macros::params;
use itertools::Itertools;
use rauthy_api_types::auth_providers::{
    ProviderLinkedUserResponse, ProviderLookupResponse, ProviderResponse,
};
use rauthy_api_types::auth_providers::{ProviderLookupRequest, ProviderRequest};
use rauthy_api_types::users::UserValuesRequest;
use rauthy_common::constants::{
    CACHE_TTL_APP, CACHE_TTL_AUTH_PROVIDER_CALLBACK, IDX_AUTH_PROVIDER, IDX_AUTH_PROVIDER_TEMPLATE,
    PROVIDER_ATPROTO, PROVIDER_LINK_COOKIE,
};
use rauthy_common::utils::{
    base64_decode, base64_encode, base64_url_no_pad_decode, deserialize, new_store_id, serialize,
};
use rauthy_common::{http_client, is_hiqlite};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use serde_json::{Value, value};
use serde_json_path::JsonPath;
use std::borrow::Cow;
use std::str::FromStr;
use tracing::{debug, error};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, postgres_types::FromSql)]
#[postgres(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AuthProviderType {
    Custom,
    GitHub,
    Google,
    OIDC,
}

impl AuthProviderType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Custom => "custom",
            Self::GitHub => "github",
            Self::Google => "google",
            Self::OIDC => "oidc",
        }
    }
}

impl TryFrom<&str> for AuthProviderType {
    type Error = ErrorResponse;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let slf = match value {
            "custom" => Self::Custom,
            "github" => Self::GitHub,
            "google" => Self::Google,
            "oidc" => Self::OIDC,
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Invalid AuthProviderType",
                ));
            }
        };
        Ok(slf)
    }
}

impl From<String> for AuthProviderType {
    /// Defaults to Self::OIDC in case of an error
    fn from(value: String) -> Self {
        Self::try_from(value.as_str()).unwrap_or(Self::Custom)
    }
}

impl From<rauthy_api_types::auth_providers::AuthProviderType> for AuthProviderType {
    fn from(value: rauthy_api_types::auth_providers::AuthProviderType) -> Self {
        match value {
            rauthy_api_types::auth_providers::AuthProviderType::Custom => Self::Custom,
            rauthy_api_types::auth_providers::AuthProviderType::GitHub => Self::GitHub,
            rauthy_api_types::auth_providers::AuthProviderType::Google => Self::Google,
            rauthy_api_types::auth_providers::AuthProviderType::OIDC => Self::OIDC,
        }
    }
}

impl From<AuthProviderType> for rauthy_api_types::auth_providers::AuthProviderType {
    fn from(value: AuthProviderType) -> Self {
        match value {
            AuthProviderType::Custom => Self::Custom,
            AuthProviderType::GitHub => Self::GitHub,
            AuthProviderType::Google => Self::Google,
            AuthProviderType::OIDC => Self::OIDC,
        }
    }
}

/// Minimal version of the OpenID metadata. This is used for upstream oauth2 lookup.
/// Only includes the data we care about when doing a config lookup.
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct WellKnownLookup {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub jwks_uri: String,
    pub scopes_supported: Vec<String>,
    pub token_endpoint_auth_methods_supported: Vec<String>,
    #[serde(default)]
    pub code_challenge_methods_supported: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthProviderLinkCookie {
    pub provider_id: String,
    pub user_id: String,
    pub user_email: String,
}

impl TryFrom<&str> for AuthProviderLinkCookie {
    type Error = ErrorResponse;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let bytes = base64_decode(value)?;
        let slf = deserialize::<AuthProviderLinkCookie>(bytes.as_slice())?;
        Ok(slf)
    }
}

impl AuthProviderLinkCookie {
    pub fn build_cookie(&self) -> Result<Cookie<'_>, ErrorResponse> {
        let bytes = serialize(self)?;
        let value = base64_encode(&bytes);
        Ok(ApiCookie::build(PROVIDER_LINK_COOKIE, value, 300))
    }

    pub fn deletion_cookie<'a>() -> Cookie<'a> {
        ApiCookie::build(PROVIDER_LINK_COOKIE, "", 0)
    }
}

/// Upstream Auth Provider for upstream logins without a local Rauthy account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProvider {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub typ: AuthProviderType,

    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub jwks_endpoint: Option<String>,

    pub client_id: String,
    pub secret: Option<Vec<u8>>,
    pub scope: String,

    pub admin_claim_path: Option<String>,
    pub admin_claim_value: Option<String>,
    pub mfa_claim_path: Option<String>,
    pub mfa_claim_value: Option<String>,

    pub use_pkce: bool,
    pub client_secret_basic: bool,
    pub client_secret_post: bool,
    pub auto_onboarding: bool,
    pub auto_link: bool,
}

impl<'r> From<hiqlite::Row<'r>> for AuthProvider {
    fn from(mut row: Row<'r>) -> Self {
        let typ_str: String = row.get("typ");
        let typ = AuthProviderType::from(typ_str);

        Self {
            id: row.get("id"),
            name: row.get("name"),
            enabled: row.get("enabled"),
            typ,
            issuer: row.get("issuer"),
            authorization_endpoint: row.get("authorization_endpoint"),
            token_endpoint: row.get("token_endpoint"),
            userinfo_endpoint: row.get("userinfo_endpoint"),
            jwks_endpoint: row.get("jwks_endpoint"),
            client_id: row.get("client_id"),
            secret: row.get("secret"),
            scope: row.get("scope"),
            admin_claim_path: row.get("admin_claim_path"),
            admin_claim_value: row.get("admin_claim_value"),
            mfa_claim_path: row.get("mfa_claim_path"),
            mfa_claim_value: row.get("mfa_claim_value"),
            use_pkce: row.get("use_pkce"),
            client_secret_basic: row.get("client_secret_basic"),
            client_secret_post: row.get("client_secret_post"),
            auto_onboarding: row.get("auto_onboarding"),
            auto_link: row.get("auto_link"),
        }
    }
}

impl From<tokio_postgres::Row> for AuthProvider {
    fn from(row: tokio_postgres::Row) -> Self {
        let typ_str: String = row.get("typ");
        let typ = AuthProviderType::from(typ_str);

        Self {
            id: row.get("id"),
            name: row.get("name"),
            enabled: row.get("enabled"),
            typ,
            issuer: row.get("issuer"),
            authorization_endpoint: row.get("authorization_endpoint"),
            token_endpoint: row.get("token_endpoint"),
            userinfo_endpoint: row.get("userinfo_endpoint"),
            jwks_endpoint: row.get("jwks_endpoint"),
            client_id: row.get("client_id"),
            secret: row.get("secret"),
            scope: row.get("scope"),
            admin_claim_path: row.get("admin_claim_path"),
            admin_claim_value: row.get("admin_claim_value"),
            mfa_claim_path: row.get("mfa_claim_path"),
            mfa_claim_value: row.get("mfa_claim_value"),
            use_pkce: row.get("use_pkce"),
            client_secret_basic: row.get("client_secret_basic"),
            client_secret_post: row.get("client_secret_post"),
            auto_onboarding: row.get("auto_onboarding"),
            auto_link: row.get("auto_link"),
        }
    }
}

impl AuthProvider {
    pub async fn create(payload: ProviderRequest) -> Result<Self, ErrorResponse> {
        let slf = Self::try_from_id_req(new_store_id(), payload)?;
        let typ = slf.typ.as_str();

        let sql = r#"
INSERT INTO
auth_providers (id, name, enabled, typ, issuer, authorization_endpoint, token_endpoint,
userinfo_endpoint, jwks_endpoint, client_id, secret, scope, admin_claim_path, admin_claim_value,
mfa_claim_path, mfa_claim_value, use_pkce, client_secret_basic, client_secret_post, auto_onboarding,
auto_link)
VALUES
($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21)"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        &slf.id,
                        &slf.name,
                        slf.enabled,
                        typ,
                        &slf.issuer,
                        &slf.authorization_endpoint,
                        &slf.token_endpoint,
                        &slf.userinfo_endpoint,
                        &slf.jwks_endpoint,
                        &slf.client_id,
                        &slf.secret,
                        &slf.scope,
                        &slf.admin_claim_path,
                        &slf.admin_claim_value,
                        &slf.mfa_claim_path,
                        &slf.mfa_claim_value,
                        slf.use_pkce,
                        slf.client_secret_basic,
                        slf.client_secret_post,
                        slf.auto_onboarding,
                        slf.auto_link
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &slf.id,
                    &slf.name,
                    &slf.enabled,
                    &typ,
                    &slf.issuer,
                    &slf.authorization_endpoint,
                    &slf.token_endpoint,
                    &slf.userinfo_endpoint,
                    &slf.jwks_endpoint,
                    &slf.client_id,
                    &slf.secret,
                    &slf.scope,
                    &slf.admin_claim_path,
                    &slf.admin_claim_value,
                    &slf.mfa_claim_path,
                    &slf.mfa_claim_value,
                    &slf.use_pkce,
                    &slf.client_secret_basic,
                    &slf.client_secret_post,
                    &slf.auto_onboarding,
                    &slf.auto_link,
                ],
            )
            .await?;
        };

        Self::invalidate_cache_all().await?;

        DB::hql()
            .put(Cache::App, Self::cache_idx(&slf.id), &slf, CACHE_TTL_APP)
            .await?;

        Ok(slf)
    }

    pub async fn find(id: &str) -> Result<Self, ErrorResponse> {
        let client = DB::hql();
        if let Some(slf) = client.get(Cache::App, Self::cache_idx(id)).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM auth_providers WHERE id = $1";
        let slf = if is_hiqlite() {
            client.query_map_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        client
            .put(Cache::App, Self::cache_idx(id), &slf, CACHE_TTL_APP)
            .await?;

        Ok(slf)
    }

    /// Tries to find an Auth Provider by the given `iss`. This function does not use any caching.
    pub async fn find_by_iss(iss: String) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM auth_providers WHERE issuer = $1";
        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(iss)).await?
        } else {
            DB::pg_query_one(sql, &[&iss]).await?
        };

        Ok(slf)
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let client = DB::hql();
        if let Some(res) = client.get(Cache::App, Self::cache_idx("all")).await? {
            return Ok(res);
        }

        let sql = "SELECT * FROM auth_providers";
        let mut res: Vec<Self> = if is_hiqlite() {
            client.query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };

        if !RauthyConfig::get().vars.atproto.enable {
            res.retain(|p| p.issuer != PROVIDER_ATPROTO);
        }

        // needed for rendering each single login page -> always cache this
        client
            .put(Cache::App, Self::cache_idx("all"), &res, CACHE_TTL_APP)
            .await?;

        Ok(res)
    }

    pub async fn find_linked_users(
        id: &str,
    ) -> Result<Vec<ProviderLinkedUserResponse>, ErrorResponse> {
        let sql = "SELECT id, email FROM users WHERE auth_provider_id = $1";
        let users = if is_hiqlite() {
            DB::hql().query_as(sql, params!(id)).await?
        } else {
            DB::pg_query(sql, &[&id], 0).await?
        };

        Ok(users)
    }

    pub async fn delete(id: &str) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM auth_providers WHERE id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(id)).await?;
        } else {
            DB::pg_execute(sql, &[]).await?;
        }

        Self::invalidate_cache_all().await?;
        DB::hql().delete(Cache::App, Self::cache_idx(id)).await?;

        Ok(())
    }

    pub async fn update(id: String, payload: ProviderRequest) -> Result<(), ErrorResponse> {
        Self::try_from_id_req(id, payload)?.save().await
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let typ = self.typ.as_str();

        let sql = r#"
UPDATE auth_providers
SET name = $1, enabled = $2, issuer = $3, typ = $4, authorization_endpoint = $5,
token_endpoint = $6, userinfo_endpoint = $7, jwks_endpoint = $8, client_id = $9, secret = $10,
scope = $11, admin_claim_path = $12, admin_claim_value = $13, mfa_claim_path = $14,
mfa_claim_value = $15, use_pkce = $16, client_secret_basic = $17, client_secret_post = $18,
auto_onboarding = $19, auto_link = $20
WHERE id = $21"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        self.name.clone(),
                        self.enabled,
                        self.issuer.clone(),
                        typ.to_string(),
                        self.authorization_endpoint.clone(),
                        self.token_endpoint.clone(),
                        self.userinfo_endpoint.clone(),
                        self.jwks_endpoint.clone(),
                        self.client_id.clone(),
                        self.secret.clone(),
                        self.scope.clone(),
                        self.admin_claim_path.clone(),
                        self.admin_claim_value.clone(),
                        self.mfa_claim_path.clone(),
                        self.mfa_claim_value.clone(),
                        self.use_pkce,
                        self.client_secret_basic,
                        self.client_secret_post,
                        self.auto_onboarding,
                        self.auto_link,
                        self.id.clone()
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &self.name,
                    &self.enabled,
                    &self.issuer,
                    &typ,
                    &self.authorization_endpoint,
                    &self.token_endpoint,
                    &self.userinfo_endpoint,
                    &self.jwks_endpoint,
                    &self.client_id,
                    &self.secret,
                    &self.scope,
                    &self.admin_claim_path,
                    &self.admin_claim_value,
                    &self.mfa_claim_path,
                    &self.mfa_claim_value,
                    &self.use_pkce,
                    &self.client_secret_basic,
                    &self.client_secret_post,
                    &self.auto_onboarding,
                    &self.auto_link,
                    &self.id,
                ],
            )
            .await?;
        }

        Self::invalidate_cache_all().await?;
        DB::hql()
            .put(Cache::App, Self::cache_idx(&self.id), self, CACHE_TTL_APP)
            .await?;

        Ok(())
    }
}

impl AuthProvider {
    #[inline(always)]
    pub fn cache_idx(id: &str) -> String {
        format!("{IDX_AUTH_PROVIDER}_{id}")
    }

    fn cleanup_scope(scope: &str) -> String {
        scope
            .split(' ')
            // filter this way to catch multiple spaces in a row
            .filter_map(|s| if !s.is_empty() { Some(s.trim()) } else { None })
            .collect::<Vec<&str>>()
            .join("+")
    }

    fn try_from_id_req(id: String, req: ProviderRequest) -> Result<Self, ErrorResponse> {
        let scope = Self::cleanup_scope(&req.scope);
        let secret = Self::secret_encrypted(&req.client_secret)?;

        Ok(Self {
            id,
            name: req.name,
            enabled: req.enabled,
            typ: req.typ.into(),
            issuer: req.issuer,
            authorization_endpoint: req.authorization_endpoint,
            token_endpoint: req.token_endpoint,
            userinfo_endpoint: req.userinfo_endpoint,
            jwks_endpoint: req.jwks_endpoint,

            client_id: req.client_id,
            secret,
            scope,

            admin_claim_path: req.admin_claim_path,
            admin_claim_value: req.admin_claim_value,
            mfa_claim_path: req.mfa_claim_path,
            mfa_claim_value: req.mfa_claim_value,

            use_pkce: req.use_pkce,
            client_secret_basic: req.client_secret_basic,
            client_secret_post: req.client_secret_post,
            auto_onboarding: req.auto_onboarding,
            auto_link: req.auto_link,
        })
    }

    async fn invalidate_cache_all() -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::App, Self::cache_idx("all")).await?;

        // We don't really need to clean all HTML caches, but rebuilding all of them
        // is a lot easier to maintain and auth providers are not updated often anyway.
        DB::hql().clear_cache(Cache::Html).await?;

        // Directly update the template cache preemptively.
        // This is needed all the time anyway.
        AuthProviderTemplate::update_cache().await?;

        Ok(())
    }

    pub async fn lookup_config(
        payload: &ProviderLookupRequest,
    ) -> Result<ProviderLookupResponse, ErrorResponse> {
        let url = if let Some(url) = &payload.metadata_url {
            Cow::from(url)
        } else if let Some(iss) = &payload.issuer {
            let url = if iss.ends_with('/') {
                format!("{iss}.well-known/openid-configuration")
            } else {
                format!("{iss}/.well-known/openid-configuration")
            };
            Cow::from(url)
        } else {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "either `metadata_url` or `issuer` must be given",
            ));
        };
        let config_url = if url.starts_with("http://") || url.starts_with("https://") {
            url
        } else {
            // we always assume https connections, if the scheme is not given
            Cow::from(format!("https://{url}"))
        };

        debug!("AuthProvider lookup to {}", config_url);
        let res = http_client().get(config_url.as_ref()).send().await?;
        let status = res.status();
        if !status.is_success() {
            let body = res.text().await?;
            return Err(ErrorResponse::new(
                ErrorResponseType::Connection,
                format!("HTTP {config_url} when trying provider config lookup to {status}: {body}"),
            ));
        }

        let well_known = res.json::<WellKnownLookup>().await.map_err(|err| {
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!("The provider does not support the mandatory openid-configuration: {err}"),
            )
        })?;

        let scopes_supported = well_known
            .scopes_supported
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<&str>>();
        let mut scope = String::with_capacity(24);
        if scopes_supported.contains(&"openid") {
            scope.push_str("openid ");
        }
        if scopes_supported.contains(&"profile") {
            scope.push_str("profile ");
        }
        if scopes_supported.contains(&"email") {
            scope.push_str("email ");
        }

        let client_secret_basic = well_known
            .token_endpoint_auth_methods_supported
            .iter()
            .any(|m| m.as_str() == "client_secret_basic");
        let client_secret_post = well_known
            .token_endpoint_auth_methods_supported
            .iter()
            .any(|m| m.as_str() == "client_secret_post");

        Ok(ProviderLookupResponse {
            issuer: well_known.issuer,
            // TODO optimization (and possibly security enhancement): strip issuer url from all of these?
            // what does the RFC mention about it? MUST they always be on the same sub path?
            authorization_endpoint: well_known.authorization_endpoint,
            token_endpoint: well_known.token_endpoint,
            userinfo_endpoint: well_known.userinfo_endpoint,
            jwks_endpoint: Some(well_known.jwks_uri),
            use_pkce: well_known
                .code_challenge_methods_supported
                .iter()
                .any(|c| c == "S256"),
            client_secret_basic,
            client_secret_post,
            scope,
            // TODO add `scopes_supported` Vec and make them selectable with checkboxes in the UI
            // instead of typing them in?
        })
    }

    fn secret_encrypted(secret: &Option<String>) -> Result<Option<Vec<u8>>, ErrorResponse> {
        if let Some(secret) = &secret {
            Ok(Some(
                EncValue::encrypt(secret.as_bytes())?.into_bytes().to_vec(),
            ))
        } else {
            Ok(None)
        }
    }

    pub fn secret_cleartext(secret: &Option<Vec<u8>>) -> Result<Option<String>, ErrorResponse> {
        if let Some(secret) = &secret {
            let bytes = EncValue::try_from(secret.clone())?.decrypt()?;
            let cleartext = String::from_utf8_lossy(bytes.as_ref()).to_string();
            Ok(Some(cleartext))
        } else {
            Ok(None)
        }
    }
}

impl TryFrom<AuthProvider> for ProviderResponse {
    type Error = ErrorResponse;

    fn try_from(value: AuthProvider) -> Result<Self, Self::Error> {
        let secret = AuthProvider::secret_cleartext(&value.secret)?;
        Ok(Self {
            id: value.id,
            name: value.name,
            typ: value.typ.into(),
            enabled: value.enabled,
            issuer: value.issuer,
            authorization_endpoint: value.authorization_endpoint,
            token_endpoint: value.token_endpoint,
            userinfo_endpoint: value.userinfo_endpoint,
            jwks_endpoint: value.jwks_endpoint,
            client_id: value.client_id,
            client_secret: secret,
            scope: value.scope,
            admin_claim_path: value.admin_claim_path,
            admin_claim_value: value.admin_claim_value,
            mfa_claim_path: value.mfa_claim_path,
            mfa_claim_value: value.mfa_claim_value,
            use_pkce: value.use_pkce,
            client_secret_basic: value.client_secret_basic,
            client_secret_post: value.client_secret_post,
            auto_onboarding: value.auto_onboarding,
            auto_link: value.auto_link,
        })
    }
}

/// Will be created to start a new upstream login and afterward validate a callback.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProviderCallback {
    pub callback_id: String,
    pub xsrf_token: String,
    pub typ: AuthProviderType,

    pub req_client_id: String,
    pub req_scopes: Option<Vec<String>>,
    pub req_redirect_uri: String,
    pub req_state: Option<String>,
    pub req_nonce: Option<String>,
    pub req_code_challenge: Option<String>,
    pub req_code_challenge_method: Option<String>,

    pub provider_id: String,

    // TODO add a nonce upstream as well? -> improvement?
    pub pkce_challenge: String,
}

// CRUD
impl AuthProviderCallback {
    pub async fn delete(callback_id: String) -> Result<(), ErrorResponse> {
        DB::hql()
            .delete(Cache::AuthProviderCallback, callback_id)
            .await?;

        Ok(())
    }

    pub async fn find(callback_id: String) -> Result<Self, ErrorResponse> {
        let opt: Option<Self> = DB::hql()
            .get(Cache::AuthProviderCallback, callback_id)
            .await?;

        match opt {
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Callback Code not found - timeout reached?",
            )),
            Some(slf) => Ok(slf),
        }
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        DB::hql()
            .put(
                Cache::AuthProviderCallback,
                self.callback_id.clone(),
                self,
                CACHE_TTL_AUTH_PROVIDER_CALLBACK,
            )
            .await?;

        Ok(())
    }
}

/// Auth Provider as template value for SSR of the Login page
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthProviderTemplate {
    pub id: String,
    pub name: String,
    pub updated: i64,
}

impl AuthProviderTemplate {
    pub async fn get_all_json_template() -> Result<String, ErrorResponse> {
        let client = DB::hql();
        if let Some(slf) = client.get(Cache::App, IDX_AUTH_PROVIDER_TEMPLATE).await? {
            return Ok(slf);
        }

        let providers = AuthProvider::find_all()
            .await?
            .into_iter()
            .filter(|p| p.enabled)
            .collect::<Vec<_>>();
        let mut slf = Vec::with_capacity(providers.len());
        for provider in providers {
            let updated = Logo::find_cached(&provider.id, &LogoType::AuthProvider)
                .await
                .map(|l| l.updated)
                .unwrap_or(0);

            slf.push(Self {
                id: provider.id,
                name: provider.name,
                updated,
            });
        }
        let json = serde_json::to_string(&slf)?;

        client
            .put(Cache::App, IDX_AUTH_PROVIDER_TEMPLATE, &json, CACHE_TTL_APP)
            .await?;

        Ok(json)
    }

    async fn invalidate_cache() -> Result<(), ErrorResponse> {
        DB::hql()
            .delete(Cache::App, IDX_AUTH_PROVIDER_TEMPLATE)
            .await?;

        Ok(())
    }

    pub async fn update_cache() -> Result<(), ErrorResponse> {
        Self::invalidate_cache().await?;
        Self::get_all_json_template().await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthProviderAddressClaims<'a> {
    pub formatted: Option<&'a str>,
    pub street_address: Option<&'a str>,
    pub locality: Option<&'a str>,
    pub postal_code: Option<&'a str>,
    pub country: Option<&'a str>,
}

#[derive(Debug, PartialEq)]
pub enum ProviderMfaLogin {
    Yes,
    No,
}

#[derive(Debug, PartialEq)]
pub enum NewFederatedUserCreated {
    Yes,
    No,
}

#[derive(Debug, Default, Deserialize)]
pub struct AuthProviderIdClaims<'a> {
    // pub iss: &'a str,
    // json values because some providers provide String, some int
    pub sub: Option<serde_json::Value>,
    pub id: Option<serde_json::Value>,
    pub uid: Option<serde_json::Value>,

    // aud / azp is not being validated, because it works with OIDC only anyway
    // aud: Option<&'a str>,
    // azp: Option<&'a str>,
    // even though `email` is mandatory for Rauthy, we set it to optional for
    // the deserialization to have more control over the error message being returned
    pub email: Option<Cow<'a, str>>,
    pub email_verified: Option<bool>,

    pub name: Option<Cow<'a, str>>,
    pub given_name: Option<Cow<'a, str>>,
    pub family_name: Option<Cow<'a, str>>,

    pub address: Option<AuthProviderAddressClaims<'a>>,
    pub birthdate: Option<Cow<'a, str>>,
    pub locale: Option<Cow<'a, str>>,
    pub phone: Option<Cow<'a, str>>,

    // some providers (e.g. Github) provide the username as `login`
    pub login: Option<Cow<'a, str>>,
    pub preferred_username: Option<Cow<'a, str>>,
    pub zoneinfo: Option<Cow<'a, str>>,

    pub json_bytes: Option<&'a [u8]>,
}

impl<'a> TryFrom<&'a [u8]> for AuthProviderIdClaims<'a> {
    type Error = ErrorResponse;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let mut slf = serde_json::from_slice::<AuthProviderIdClaims>(value)?;
        slf.json_bytes = Some(value);
        Ok(slf)
    }
}

impl AuthProviderIdClaims<'_> {
    fn given_name(&self) -> &str {
        if let Some(given_name) = &self.given_name {
            given_name
        } else if let Some(name) = &self.name {
            let (given_name, _) = name.split_once(' ').unwrap_or((name, ""));
            given_name
        } else {
            ""
        }
    }

    fn family_name(&self) -> Option<&str> {
        if let Some(family_name) = &self.family_name {
            Some(family_name)
        } else if let Some(name) = &self.name {
            if let Some((_, family_name)) = name.split_once(' ') {
                Some(family_name)
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn self_as_bytes_from_token(token: &str) -> Result<Vec<u8>, ErrorResponse> {
        let mut parts = token.split('.');
        let _header = parts.next().ok_or_else(|| {
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "incorrect ID did not contain claims",
            )
        })?;
        let claims = parts.next().ok_or_else(|| {
            ErrorResponse::new(ErrorResponseType::BadRequest, "ID token was unsigned")
        })?;
        debug!("upstream ID token claims: {claims}");
        let json_bytes = base64_url_no_pad_decode(claims)?;
        Ok(json_bytes)
    }

    pub async fn validate_update_user(
        &self,
        provider: &AuthProvider,
        link_cookie: &Option<AuthProviderLinkCookie>,
    ) -> Result<(User, ProviderMfaLogin, NewFederatedUserCreated), ErrorResponse> {
        if self.email.is_none() {
            let err = "No `email` in ID token claims. This is a mandatory claim";
            error!("{err}");
            return Err(ErrorResponse::new(ErrorResponseType::BadRequest, err));
        }

        let claims_user_id_json = if let Some(sub) = &self.sub {
            sub
        } else if let Some(id) = &self.id {
            id
        } else if let Some(uid) = &self.uid {
            uid
        } else {
            let err = "Cannot find any user id in the response";
            error!("{}", err);
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                err.to_string(),
            ));
        };

        // We need to create a real string here, since we don't know what json type we get.
        // Any json number would become a String too, which is what we need for compatibility.
        let claims_user_id = match claims_user_id_json {
            Value::Number(num) => num.to_string(),
            Value::String(s) => s.to_string(),
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    "Invalid value for the Upstream User ID",
                ));
            }
        };

        let (user_opt, new_federated_user) = match User::find_by_federation(
            &provider.id,
            &claims_user_id,
        )
        .await
        {
            Ok(user) => {
                debug!("found already existing user by federation lookup: {user:?}");
                (Some(user), NewFederatedUserCreated::No)
            }
            Err(_) => {
                debug!("did not find already existing user by federation lookup");
                if let Ok(mut user) =
                    User::find_by_email(self.email.as_ref().unwrap().to_string()).await
                {
                    if let Some(link) = link_cookie {
                        if link.provider_id != provider.id {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "bad provider_id in link cookie",
                            ));
                        }

                        if link.user_id != user.id {
                            // In this case, the link cookie exists from another user session.
                            // It is possible to build this situation manually with access to
                            // multiple accounts.
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "bad user_id in link cookie",
                            ));
                        }

                        // finally, this is our condition we allow linking for existing accs
                        if link.user_email != user.email {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "Invalid E-Mail",
                            ));
                        }

                        // If we got here, everything was fine, and we can create the link.
                        // No need to `.save()` here, will be done later anyway with other updates.
                        user.auth_provider_id = Some(provider.id.clone());
                        user.federation_uid = Some(claims_user_id.clone());

                        (Some(user), NewFederatedUserCreated::No)
                    } else if provider.auto_link
                        && user.federation_uid.is_none()
                        && user.auth_provider_id.is_none()
                    {
                        user.auth_provider_id = Some(provider.id.clone());
                        user.federation_uid = Some(claims_user_id.clone());

                        (Some(user), NewFederatedUserCreated::No)
                    } else {
                        return Err(ErrorResponse::new(
                            ErrorResponseType::Forbidden,
                            format!(
                                "User with email '{}' already exists but is not linked to this provider.",
                                user.email
                            ),
                        ));
                    }
                } else if !provider.auto_onboarding {
                    return Err(ErrorResponse::new(
                        ErrorResponseType::NotFound,
                        "User not found",
                    ));
                } else {
                    // a new user will be created further down
                    (None, NewFederatedUserCreated::Yes)
                }
            }
        };
        debug!("user_opt:\n{:?}", user_opt);

        // `rauthy_admin` role mapping by upstream claim
        let mut should_be_rauthy_admin = None;
        if let Some(path) = &provider.admin_claim_path {
            if provider.admin_claim_value.is_none() {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Internal,
                    "Misconfigured Auth Provider - admin claim path without value",
                ));
            }

            debug!("try validating admin_claim_path: {:?}", path);
            match JsonPath::parse(path) {
                Ok(path) => {
                    let json_str = String::from_utf8_lossy(self.json_bytes.unwrap());
                    let json =
                        value::Value::from_str(json_str.as_ref()).expect("json to build fine");
                    let admin_value =
                        value::Value::from(provider.admin_claim_value.as_deref().unwrap())
                            .to_string();
                    should_be_rauthy_admin = Some(false);
                    for value in path.query(&json).all() {
                        // We actually need this allocation to String to get bigger compatibility.
                        // This way, we can accept not only string, but we would for instance
                        // also interpret a given bool as string.
                        let value = if !value.is_string() {
                            format!("\"{value}\"")
                        } else {
                            value.to_string()
                        };
                        if value == admin_value {
                            should_be_rauthy_admin = Some(true);
                            break;
                        }
                    }
                }
                Err(err) => {
                    error!("Error parsing JsonPath from: '{path}', Error: {err}");
                }
            }
        }

        // check if mfa has been used by upstream claim
        let mut provider_mfa_login = ProviderMfaLogin::No;
        if let Some(path) = &provider.mfa_claim_path {
            if provider.mfa_claim_value.is_none() {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Internal,
                    "Misconfigured Auth Provider - mfa claim path without value",
                ));
            }

            debug!("try validating mfa_claim_path: {:?}", path);
            match JsonPath::parse(path) {
                Ok(path) => {
                    let json_str = String::from_utf8_lossy(self.json_bytes.unwrap());
                    let json =
                        value::Value::from_str(json_str.as_ref()).expect("json to build fine");
                    let mfa_value =
                        value::Value::from(provider.mfa_claim_value.as_deref().unwrap())
                            .to_string();

                    for value in path.query(&json).all() {
                        // We actually need this allocation to String to get bigger compatibility.
                        // This way, we can accept not only string, but we would for instance
                        // also interpret a given bool as string.
                        let value = if !value.is_string() {
                            format!("\"{value}\"")
                        } else {
                            value.to_string()
                        };
                        if value == mfa_value {
                            provider_mfa_login = ProviderMfaLogin::Yes;
                            break;
                        }
                    }
                }
                Err(err) => {
                    error!("Error parsing JsonPath from: '{path}', Error: {err}");
                }
            }
        }

        let now = Utc::now().timestamp();
        let user = if let Some(mut user) = user_opt {
            let mut old_email = None;
            let mut forbidden_error = None;

            // validate federation_uid
            // we must reject any upstream login, if a non-federated local user with the same email
            // exists, as it could lead to an account takeover
            if user.federation_uid.is_none()
                || user.federation_uid.as_deref() != Some(&claims_user_id)
            {
                forbidden_error = Some("non-federated user or ID mismatch");
            }

            // validate auth_provider_id
            if user.auth_provider_id.as_deref() != Some(&provider.id) {
                forbidden_error = Some("invalid login from wrong auth provider");
            }

            if let Some(err) = forbidden_error {
                user.last_failed_login = Some(now);
                user.failed_login_attempts =
                    Some(user.failed_login_attempts.unwrap_or_default() + 1);
                user.save(old_email).await?;

                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    err.to_string(),
                ));
            }

            // check / update email
            if Some(user.email.as_str()) != self.email.as_deref() {
                old_email = Some(user.email);
                user.email = self.email.as_ref().unwrap().to_string();
            }

            // check other existing values and possibly update them
            let given_name = self.given_name();
            if user.given_name.as_str() != given_name {
                user.given_name = given_name.to_string();
            }
            let family_name = self.family_name();
            if user.family_name.as_deref() != family_name {
                user.family_name = family_name.map(String::from);
            }

            // should this user be a rauthy admin?
            let roles = user.roles_iter().collect::<Vec<_>>();

            // We will only re-map the rauthy_admin role if the claim mapping is configured.
            // Otherwise, we would remove an admin role from a user it has been manually added for,
            // which would not be the expected outcome.
            if let Some(should_be_admin) = should_be_rauthy_admin {
                if should_be_admin {
                    if !roles.contains(&"rauthy_admin") {
                        let is_empty = roles.is_empty();
                        drop(roles);
                        if is_empty {
                            user.roles.push_str("rauthy_admin");
                        } else {
                            user.roles.push_str(",rauthy_admin");
                        }
                    }
                } else if roles.contains(&"rauthy_admin") {
                    if roles.len() == 1 {
                        user.roles = "".to_string();
                    } else {
                        user.roles = roles.into_iter().filter(|r| r != &"rauthy_admin").join(",");
                    }
                }
            }

            // update the user on our side
            user.last_login = Some(now);
            user.last_failed_login = None;
            user.failed_login_attempts = None;

            user.save(old_email).await?;
            user
        } else {
            // Create a new federated user
            let new_user = User {
                email: self.email.as_ref().unwrap().to_string(),
                given_name: self.given_name().to_string(),
                family_name: self.family_name().map(String::from),
                roles: should_be_rauthy_admin
                    .map(|should_be_admin| {
                        if should_be_admin {
                            "rauthy_admin".to_string()
                        } else {
                            String::default()
                        }
                    })
                    .unwrap_or_default(),
                enabled: true,
                email_verified: self.email_verified.unwrap_or(false),
                last_login: Some(now),
                language: self
                    .locale
                    .as_ref()
                    .map(|l| Language::from(l.as_ref()))
                    .unwrap_or_default(),
                auth_provider_id: Some(provider.id.clone()),
                federation_uid: Some(claims_user_id.to_string()),
                ..Default::default()
            };
            User::create_federated(new_user).await?
        };

        // check if we got additional values from the token
        let mut found_values = false;
        let mut user_values = match UserValues::find(&user.id).await? {
            Some(values) => UserValuesRequest {
                birthdate: values.birthdate,
                phone: values.phone,
                street: values.street,
                zip: values.zip,
                city: values.city,
                country: values.country,
                tz: values.tz,
            },
            None => UserValuesRequest {
                birthdate: None,
                phone: None,
                street: None,
                zip: None,
                city: None,
                country: None,
                tz: None,
            },
        };
        if let Some(bday) = &self.birthdate {
            user_values.birthdate = Some(bday.to_string());
            found_values = true;
        }
        if let Some(phone) = &self.phone {
            user_values.phone = Some(phone.to_string());
            found_values = true;
        }
        if let Some(addr) = &self.address {
            if let Some(street) = addr.street_address {
                user_values.street = Some(street.to_string());
            }
            if let Some(country) = addr.country {
                user_values.country = Some(country.to_string());
            }
            if let Some(zip) = addr.postal_code {
                user_values.zip = Some(zip.to_string());
            }
            found_values = true;
        }
        if let Some(tz) = &self.zoneinfo {
            user_values.tz = Some(tz.to_string());
            found_values = true;
        }

        let preferred_username = if let Some(username) = &self.preferred_username {
            UserValues::validate_preferred_username_free(username.to_string())
                .await
                .is_ok()
                .then_some(username.to_string())
        } else if let Some(username) = &self.login {
            UserValues::validate_preferred_username_free(username.to_string())
                .await
                .is_ok()
                .then_some(username.to_string())
        } else {
            None
        };
        if let Some(username) = preferred_username {
            // We ignore the result in case of a race condition.
            // Better continue without setting the username than failing.
            // The user can edit it later on anyway.
            let _ = UserValues::upsert_preferred_username(user.id.clone(), username).await;
        }

        if found_values {
            UserValues::upsert(user.id.clone(), user_values).await?;
        }

        Ok((user, provider_mfa_login, new_federated_user))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // exists only to understand the query syntax and experiment with it
    #[test]
    #[ignore]
    fn test_json_path() {
        let value = serde_json::json!({
            "foo": {
                "bar": ["baz", "bop", 23],
                "bor": "yes"
            },
            "bor": "yup"
        });

        let path = JsonPath::parse("$.foo.bar[*]").unwrap();
        let nodes = path.query(&value).all();
        assert_eq!(nodes.get(0).unwrap().as_str(), Some("baz"));
        assert_eq!(nodes.get(1).unwrap().as_str(), Some("bop"));
        assert_eq!(
            nodes.get(2).unwrap().as_number(),
            Some(&serde_json::Number::from(23))
        );

        let path = JsonPath::parse("$.foo.*").unwrap();
        let nodes = path.query(&value).all();
        assert_eq!(nodes.len(), 2);

        // works in the same way (in this case -> array) as `$.foo.bar[*]`
        let path = JsonPath::parse("$.foo.bar.*").unwrap();
        let nodes = path.query(&value).all();
        assert_eq!(nodes.len(), 3);
        assert_eq!(nodes.get(1).unwrap().as_str(), Some("bop"));

        let path = JsonPath::parse("$.foo.bor").unwrap();
        let nodes = path.query(&value).all();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes.get(0).unwrap().as_str(), Some("yes"));

        // we cannot query for single values with the wildcard in the end
        // -> add 2 possible cases in the checking code for best UX
        let path = JsonPath::parse("$.foo.bor.*").unwrap();
        let nodes = path.query(&value).all();
        assert_eq!(nodes.len(), 0);

        // wildcards work in the front as well
        let path = JsonPath::parse("$.*.bor").unwrap();
        let nodes = path.query(&value).all();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes.get(0).unwrap().as_str(), Some("yes"));
    }

    #[test]
    fn test_id_token_deserialization() {
        // this raw token contains unicode encoded chars
        let raw = "eyJhbGciOiJSUzI1NiIsImtpZCI6InlPeXYtWG1tUVYtUTNrUUVCbkRHVlFGb2hoVTNyaTFiRkY0VEg4VGZYTFUiLCJ0eXAiOiJKV1QifQ.eyJhdWQiOlsiaHR0cHM6Ly9vcGVuaWRjb25uZWN0Lm5ldCJdLCJlbWFpbCI6InRlc3RcdTAwMjZAYXBpdG1hbi5jb20iLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiZXhwIjoxNzE4MjAwNjc2LCJpYXQiOjE3MTgxMTQyNzYsImlzcyI6Imh0dHBzOi8vbGFzdGxvZ2luLmlvIiwibmFtZSI6InRlc3RcdTAwMjYiLCJub25jZSI6IiIsInN1YiI6InRlc3RcdTAwMjZAYXBpdG1hbi5jb20ifQ.IchRsMNdE4isQ9ug8GqFtc_TRkZ3wA1KQw4dqnwqqVgNMcbY3qTuufwRHW-zoLDxQZkNOCl0niJolrLGmSfeSAo0fRtRMrkq41b7hhcnfE2MLBTZbI3E8m2mbxdgGlBJxeOBFdIwhN3meuioxiqnoAdCh1aw8p2dj7TuBaHpNXdREUcB76U1ZXHqSmYFWZDVROzMgVb7PjL2ikPcjksWf-jlFIRrEdbRXm9DygL9LAUFI10IjCZePFv8wjmr0aJS1pXAXYn31hoSWaH0xxoDCmO4t4aBafL392zX6is0i3uRYSu-3L0GJRCT7qmbm_FiGvEqfFQoILslKRvzWG74Ww";
        let claims_bytes = AuthProviderIdClaims::self_as_bytes_from_token(raw).unwrap();
        assert!(AuthProviderIdClaims::try_from(claims_bytes.as_ref()).is_ok());
    }
}
