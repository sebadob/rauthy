use crate::api_cookie::ApiCookie;
use crate::database::{Cache, DB};
use crate::entity::groups::Group;
use crate::entity::logos::{Logo, LogoType};
use crate::entity::roles::Role;
use crate::entity::users::User;
use crate::entity::users_values::UserValues;
use crate::entity::{atproto, auth_provider_cust_impls};
use crate::language::Language;
use crate::rauthy_config::RauthyConfig;
use actix_web::cookie::Cookie;
use atrium_api::xrpc::http::header::{ACCEPT, AUTHORIZATION};
use atrium_common::store::Store;
use chrono::Utc;
use cryptr::EncValue;
use dashmap::DashMap;
use hiqlite::macros::{FromRow, params};
use itertools::Itertools;
use rauthy_api_types::auth_providers::{
    ClaimMapping, ClaimMappingTyp, ProviderCallbackRequest, ProviderLinkedUserResponse,
    ProviderLookupRequest, ProviderLookupResponse, ProviderRequest, ProviderResponse,
};
use rauthy_api_types::oidc::LoginConsent;
use rauthy_api_types::users::UserValuesRequest;
use rauthy_common::constants::{
    APPLICATION_JSON, CACHE_TTL_APP, CACHE_TTL_AUTH_PROVIDER_CALLBACK, CLAIM_MAPPINGS_MAX_RULES,
    IDX_AUTH_PROVIDER, IDX_AUTH_PROVIDER_TEMPLATE, PROVIDER_ATPROTO, PROVIDER_LINK_COOKIE,
    RAUTHY_ADMIN_GROUP_PREFIX, RAUTHY_ADMIN_ROLE,
};
use rauthy_common::json_path::{ClaimNodes, JsonPath, Matcher};
use rauthy_common::regex::{RE_GROUPS, RE_ROLES_SCOPES};
use rauthy_common::utils::{
    base64_decode, base64_encode, base64_url_no_pad_decode, deserialize, new_store_id, serialize,
};
use rauthy_common::{http_client, is_hiqlite};
use rauthy_derive::FromPgRow;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use std::collections::BTreeSet;
use std::str::FromStr;
use std::sync::{Arc, LazyLock};
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

impl FromStr for AuthProviderType {
    type Err = ErrorResponse;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
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
        Self::from_str(value.as_str()).unwrap_or(Self::Custom)
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

impl FromStr for AuthProviderLinkCookie {
    type Err = ErrorResponse;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
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

/// A claim path and its matcher, compiled: the login path evaluates this.
#[derive(Debug)]
pub struct ClaimRule {
    pub path: JsonPath,
    pub matcher: Matcher,
}

/// A compiled claim rule plus the local role / group name it manages.
#[derive(Debug)]
pub struct TargetRule {
    pub rule: ClaimRule,
    pub target: String,
}

/// All claim rules of one provider, compiled once.
///
/// The fingerprint over the provider's claim columns is what keeps the cache coherent across
/// cluster nodes without a TTL: every federated login reads a fresh provider row from the DB
/// (`find_by_iss`), so the first login after a config change on any node hashes a different
/// fingerprint and recompiles.
///
/// The admin and the MFA rule grant a fixed name and are the targets of the provider's own
/// `admin_claim_*` / `mfa_claim_*` fields, so they carry no target.
#[derive(Debug)]
pub struct CompiledMappings {
    fingerprint: u64,
    admin: Option<ClaimRule>,
    mfa: Option<ClaimRule>,
    roles: Vec<TargetRule>,
    groups: Vec<TargetRule>,
}

/// Compiled claim mappings, keyed by [`AuthProvider::cache_idx`].
///
/// Guard rule: clone the `Arc` out of the `Ref` and drop the `Ref` before touching the map
/// again — a nested `get` on the same shard deadlocks.
static CLAIM_MAPPINGS: LazyLock<DashMap<String, Arc<CompiledMappings>, ahash::RandomState>> =
    LazyLock::new(|| {
        DashMap::with_hasher_and_shard_amount(
            ahash::RandomState::default(),
            usize::from(RauthyConfig::get().vars.server.http_workers) * 2,
        )
    });

/// The fingerprint hasher. A single shared `RandomState`, because per-call seeds would make
/// every fingerprint unique and the cache would recompile on every login.
static FP_HASHER: LazyLock<ahash::RandomState> = LazyLock::new(ahash::RandomState::new);

impl ClaimRule {
    /// Parses the path and compiles the matcher of one rule. Both are the expensive parts of a
    /// claim rule, and neither of them happens on a federated login.
    fn compile(path: &str, value: &str) -> Result<Self, ErrorResponse> {
        Ok(Self {
            path: JsonPath::parse(path)?,
            matcher: Matcher::from_claim_value(value)?,
        })
    }

    /// True when any node this rule's path selects matches its matcher.
    fn matches<'a>(&self, claims: &'a Value, nodes: &mut ClaimNodes<'a>, buf: &mut String) -> bool {
        self.path.eval(claims, nodes);
        nodes.matches(&self.matcher, buf)
    }
}

impl CompiledMappings {
    fn compile(provider: &AuthProvider) -> Result<Self, ErrorResponse> {
        let admin = provider
            .admin_claim_path
            .as_deref()
            .zip(provider.admin_claim_value.as_deref())
            .map(|(path, value)| ClaimRule::compile(path, value))
            .transpose()?;
        let mfa = provider
            .mfa_claim_path
            .as_deref()
            .zip(provider.mfa_claim_value.as_deref())
            .map(|(path, value)| ClaimRule::compile(path, value))
            .transpose()?;

        let mut roles = Vec::new();
        let mut groups = Vec::new();
        for mapping in provider.claim_mappings()? {
            let rule = TargetRule {
                rule: ClaimRule::compile(&mapping.claim_path, &mapping.claim_value)?,
                target: mapping.target,
            };
            match mapping.typ {
                ClaimMappingTyp::Role => roles.push(rule),
                ClaimMappingTyp::Group => groups.push(rule),
            }
        }

        Ok(Self {
            fingerprint: provider.fingerprint(),
            admin,
            mfa,
            roles,
            groups,
        })
    }

    /// True when the provider manages any role name at all. The login path skips the whole
    /// reconcile when a provider has no admin claim, no MFA claim and no rules.
    pub fn has_rules(&self) -> bool {
        self.admin.is_some()
            || self.mfa.is_some()
            || !self.roles.is_empty()
            || !self.groups.is_empty()
    }
}

/// Upstream Auth Provider for upstream logins without a local Rauthy account
#[derive(Debug, Clone, Serialize, Deserialize, FromRow, FromPgRow)]
pub struct AuthProvider {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    #[column(from_string)]
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
    /// `Vec<ClaimMapping>` serialized as JSON. Deserialization happens in
    /// [`AuthProvider::claim_mappings`], never in the row derive: `#[column(parse)]` would
    /// `unwrap()` on a corrupt row. `#[serde(default)]` keeps a row written by the previous
    /// version readable.
    #[serde(default)]
    pub claim_mappings: Option<String>,

    pub use_pkce: bool,
    pub client_secret_basic: bool,
    pub client_secret_post: bool,
    pub auto_onboarding: bool,
    pub auto_link: bool,
    pub hide_login: bool,
}

impl AuthProvider {
    pub async fn create(payload: ProviderRequest) -> Result<Self, ErrorResponse> {
        Self::validate_claim_targets(&payload).await?;
        let slf = Self::try_from_id_req(new_store_id(), payload)?;
        let typ = slf.typ.as_str();

        let sql = r#"
INSERT INTO
auth_providers (id, name, enabled, typ, issuer, authorization_endpoint, token_endpoint,
userinfo_endpoint, jwks_endpoint, client_id, secret, scope, admin_claim_path, admin_claim_value,
mfa_claim_path, mfa_claim_value, claim_mappings, use_pkce, client_secret_basic,
client_secret_post, auto_onboarding, auto_link, hide_login)
VALUES
($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20,
$21, $22, $23)"#;

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
                        &slf.claim_mappings,
                        slf.use_pkce,
                        slf.client_secret_basic,
                        slf.client_secret_post,
                        slf.auto_onboarding,
                        slf.auto_link,
                        slf.hide_login
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
                    &slf.claim_mappings,
                    &slf.use_pkce,
                    &slf.client_secret_basic,
                    &slf.client_secret_post,
                    &slf.auto_onboarding,
                    &slf.auto_link,
                    &slf.hide_login,
                ],
            )
            .await?;
        }

        Self::invalidate_cache_all(None).await?;
        slf.cache_compiled_mappings()?;

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

        res.sort_by(|a, b| a.name.cmp(&b.name));

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
            DB::pg_execute(sql, &[&id]).await?;
        }

        Self::invalidate_cache_all(Some(id)).await?;
        CLAIM_MAPPINGS.remove(&Self::cache_idx(id));

        Ok(())
    }

    pub async fn update(id: String, payload: ProviderRequest) -> Result<(), ErrorResponse> {
        Self::validate_claim_targets(&payload).await?;
        Self::try_from_id_req(id, payload)?.save().await
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let typ = self.typ.as_str();

        let sql = r#"
UPDATE auth_providers
SET name = $1, enabled = $2, issuer = $3, typ = $4, authorization_endpoint = $5,
token_endpoint = $6, userinfo_endpoint = $7, jwks_endpoint = $8, client_id = $9, secret = $10,
scope = $11, admin_claim_path = $12, admin_claim_value = $13, mfa_claim_path = $14,
mfa_claim_value = $15, claim_mappings = $16, use_pkce = $17, client_secret_basic = $18,
client_secret_post = $19, auto_onboarding = $20, auto_link = $21, hide_login = $22
WHERE id = $23"#;

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
                        self.claim_mappings.clone(),
                        self.use_pkce,
                        self.client_secret_basic,
                        self.client_secret_post,
                        self.auto_onboarding,
                        self.auto_link,
                        self.hide_login,
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
                    &self.claim_mappings,
                    &self.use_pkce,
                    &self.client_secret_basic,
                    &self.client_secret_post,
                    &self.auto_onboarding,
                    &self.auto_link,
                    &self.hide_login,
                    &self.id,
                ],
            )
            .await?;
        }

        Self::invalidate_cache_all(None).await?;
        self.cache_compiled_mappings()?;

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

    /// The provider's claim mapping rules, deserialized from the JSON column. Empty when
    /// the provider has no rules. A corrupt row is an `Err`, never a panic.
    pub fn claim_mappings(&self) -> Result<Vec<ClaimMapping>, ErrorResponse> {
        self.claim_mappings
            .as_deref()
            .map(|json| {
                serde_json::from_str::<Vec<ClaimMapping>>(json).map_err(|err| {
                    ErrorResponse::new(
                        ErrorResponseType::Internal,
                        format!("Cannot deserialize claim_mappings of {}: {err}", self.id),
                    )
                })
            })
            .unwrap_or_else(|| Ok(Vec::new()))
    }

    /// The provider's compiled claim rules: the paths and every `~` regex of a provider are
    /// compiled once, keyed by [`AuthProvider::cache_idx`], and reused by all later federated
    /// logins. On a fingerprint miss, which is the first login after a config change on this node
    /// or the first login after a restart, the rules are compiled and inserted.
    ///
    /// The login path holds a fresh provider row from `find_by_iss`, so comparing the fingerprint
    /// of that row is what makes the cache self-correcting across cluster nodes without a TTL.
    pub fn compiled_mappings(&self) -> Result<Arc<CompiledMappings>, ErrorResponse> {
        let key = Self::cache_idx(&self.id);
        let fingerprint = self.fingerprint();

        // The `Ref` of the shard is dropped before anything else touches the map.
        let cached = CLAIM_MAPPINGS.get(&key).and_then(|entry| {
            (entry.fingerprint == fingerprint).then(|| Arc::clone(entry.value()))
        });
        if let Some(compiled) = cached {
            return Ok(compiled);
        }

        let compiled = Arc::new(CompiledMappings::compile(self)?);
        CLAIM_MAPPINGS.insert(key, Arc::clone(&compiled));
        Ok(compiled)
    }

    /// Compiles and caches the provider's claim rules. Called from the write seams next to
    /// `invalidate_cache_all`, so a config change is live on the very next login.
    fn cache_compiled_mappings(&self) -> Result<(), ErrorResponse> {
        let compiled = CompiledMappings::compile(self)?;
        CLAIM_MAPPINGS.insert(Self::cache_idx(&self.id), Arc::new(compiled));
        Ok(())
    }

    /// The content fingerprint of everything the claim matcher reads. `AuthProvider` has no
    /// `updated` column, so a row version cannot key the compiled cache — the content does.
    fn fingerprint(&self) -> u64 {
        let hasher = &*FP_HASHER;
        let mut hash = hasher.hash_one(&self.admin_claim_path);
        for part in [
            &self.admin_claim_value,
            &self.mfa_claim_path,
            &self.mfa_claim_value,
            &self.claim_mappings,
        ] {
            hash = hash.rotate_left(1) ^ hasher.hash_one(part);
        }
        hash
    }

    /// Every target of a claim rule must exist before the config is written: Rauthy never creates a
    /// role or a group on a login path. A `rauthy_admin:<group>` target needs the group behind it,
    /// any other target needs its role or group. The error names the missing name.
    /// Both lookups are cached.
    async fn validate_claim_targets(req: &ProviderRequest) -> Result<(), ErrorResponse> {
        let mappings = req.claim_mappings.as_deref().unwrap_or_default();
        if mappings.is_empty() {
            return Ok(());
        }

        let roles = Role::find_all()
            .await?
            .into_iter()
            .map(|r| r.name)
            .collect::<BTreeSet<String>>();
        let groups = Group::find_all()
            .await?
            .into_iter()
            .map(|g| g.name)
            .collect::<BTreeSet<String>>();

        for mapping in mappings {
            match mapping.target.strip_prefix(RAUTHY_ADMIN_GROUP_PREFIX) {
                // A group admin is a role name, never a row in the `roles` table, so its group is
                // what must exist. Rauthy never creates a group on a login path.
                Some(group) => {
                    if group.is_empty()
                        || !groups
                            .iter()
                            .any(|name| group_admin_matches_group(name, group))
                    {
                        return Err(ErrorResponse::new(
                            ErrorResponseType::BadRequest,
                            format!(
                                "Claim mapping target has no group behind it: {}",
                                mapping.target
                            ),
                        ));
                    }
                }
                None => {
                    let known = match mapping.typ {
                        ClaimMappingTyp::Role => &roles,
                        ClaimMappingTyp::Group => &groups,
                    };
                    if !known.contains(&mapping.target) {
                        return Err(ErrorResponse::new(
                            ErrorResponseType::BadRequest,
                            format!("Claim mapping target does not exist: {}", mapping.target),
                        ));
                    }
                }
            }
        }

        Ok(())
    }

    /// Validates the claim config of a request and serializes the rules into the column value.
    ///
    /// Every path is parsed and every matcher is compiled here, so an unparseable path and an
    /// invalid regex both fail the config save, naming the rule. A `path` and its `value` must
    /// always be set together.
    fn validate_claim_mappings(req: &ProviderRequest) -> Result<Option<String>, ErrorResponse> {
        for (name, path, value) in [
            ("admin_claim", &req.admin_claim_path, &req.admin_claim_value),
            ("mfa_claim", &req.mfa_claim_path, &req.mfa_claim_value),
        ] {
            let pair = (
                path.as_deref().unwrap_or_default(),
                value.as_deref().unwrap_or_default(),
            );
            if pair.0.is_empty() != pair.1.is_empty() {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!("`{name}` path and value must be set together"),
                ));
            }
            if let (Some(path), Some(value)) = (path, value) {
                ClaimRule::compile(path, value).map_err(|err| Self::claim_rule_err(name, err))?;
            }
        }

        let mappings = req.claim_mappings.as_deref().unwrap_or_default();
        if mappings.len() > CLAIM_MAPPINGS_MAX_RULES {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!(
                    "An auth provider supports at most {CLAIM_MAPPINGS_MAX_RULES} claim mappings"
                ),
            ));
        }

        for mapping in mappings {
            // The full admin role is only ever granted by the dedicated admin claim mapping. Its
            // group-admin delegates are valid generic targets, and they always are role names.
            if mapping.target == RAUTHY_ADMIN_ROLE {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!(
                        "`{RAUTHY_ADMIN_ROLE}` can only be mapped by the dedicated admin claim"
                    ),
                ));
            }
            if mapping.target.starts_with(RAUTHY_ADMIN_GROUP_PREFIX)
                && mapping.typ == ClaimMappingTyp::Group
            {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!(
                        "A group admin target `{}` is a role, it cannot be a group mapping",
                        mapping.target
                    ),
                ));
            }

            // Targets are written into the comma-separated `roles` / `groups` columns of a user, so
            // a name containing a `,` would come back as several members. Both name rules allow it.
            if mapping.target.contains(',') {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!(
                        "Claim mapping targets must not contain a `,`: {}",
                        mapping.target
                    ),
                ));
            }

            let name_rule = match mapping.typ {
                ClaimMappingTyp::Role => &*RE_ROLES_SCOPES,
                ClaimMappingTyp::Group => &*RE_GROUPS,
            };
            if !name_rule.is_match(&mapping.target) {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    format!(
                        "Invalid {} target `{}`",
                        match mapping.typ {
                            ClaimMappingTyp::Role => "role",
                            ClaimMappingTyp::Group => "group",
                        },
                        mapping.target
                    ),
                ));
            }

            // The path is parsed and the matcher compiled here, exactly as on the login path, so
            // neither can fail there.
            ClaimRule::compile(&mapping.claim_path, &mapping.claim_value)
                .map_err(|err| Self::claim_rule_err(&mapping.target, err))?;
        }

        if mappings.is_empty() {
            return Ok(None);
        }
        serde_json::to_string(mappings).map(Some).map_err(|err| {
            ErrorResponse::new(
                ErrorResponseType::Internal,
                format!("Cannot serialize claim_mappings: {err}"),
            )
        })
    }

    fn claim_rule_err(target: &str, err: ErrorResponse) -> ErrorResponse {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Claim mapping for `{target}`: {}", err.message),
        )
    }

    fn try_from_id_req(id: String, req: ProviderRequest) -> Result<Self, ErrorResponse> {
        let claim_mappings = Self::validate_claim_mappings(&req)?;

        let scope = Self::cleanup_scope(&req.scope);
        let secret = Self::secret_encrypted(req.client_secret.as_deref())?;

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
            claim_mappings,

            use_pkce: req.use_pkce,
            client_secret_basic: req.client_secret_basic,
            client_secret_post: req.client_secret_post,
            auto_onboarding: req.auto_onboarding,
            auto_link: req.auto_link,
            hide_login: req.hide_login,
        })
    }

    /// Invalidates every auth providers cache, plus the config of `affected_id`, and rebuilds
    /// the template. The template rebuild must stay after the delay, it is what makes the
    /// fresh template win over a fill that raced with this write.
    async fn invalidate_cache_all(affected_id: Option<&str>) -> Result<(), ErrorResponse> {
        let affected = affected_id
            .map(|id| (Cache::App, Some(Cow::from(Self::cache_idx(id)))))
            .into_iter();
        let cache_entries = [
            (Cache::App, Some(Cow::from(Self::cache_idx("all")))),
            (Cache::App, Some(Cow::from(IDX_AUTH_PROVIDER_TEMPLATE))),
        ]
        .into_iter()
        .chain(affected);
        DB::cache_delete_delayed(cache_entries).await?;

        // We don't really need to clean all HTML caches, but rebuilding all of them
        // is a lot easier to maintain and auth providers are not updated often anyway.
        DB::hql().clear_cache(Cache::Html).await?;

        // Directly update the template cache preemptively.
        // This is needed all the time anyway.
        AuthProviderTemplate::get_all_json_template().await?;

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

    fn secret_encrypted(secret: Option<&str>) -> Result<Option<Vec<u8>>, ErrorResponse> {
        if let Some(secret) = secret {
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
        let claim_mappings = value.claim_mappings()?;
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
            claim_mappings,
            use_pkce: value.use_pkce,
            client_secret_basic: value.client_secret_basic,
            client_secret_post: value.client_secret_post,
            auto_onboarding: value.auto_onboarding,
            auto_link: value.auto_link,
            hide_login: value.hide_login,
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
    /// RFC 8707 resource indicator from the authorization request (#1702).
    pub req_resource: Option<String>,
    pub req_consent: Option<LoginConsent>,

    pub provider_id: String,

    pub pkce_challenge: String,
    pub is_forward_auth: Option<bool>,
}

// CRUD
impl AuthProviderCallback {
    /// Atomically takes the callback out of the cache, so that it can only ever be used once,
    /// even with concurrent requests.
    pub async fn find_remove(callback_id: String) -> Result<Self, ErrorResponse> {
        let opt: Option<Self> = DB::hql()
            .get_remove(Cache::AuthProviderCallback, callback_id)
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

#[derive(Debug, Serialize)]
struct OidcCodeRequestParams<'a> {
    client_id: &'a str,
    client_secret: Option<String>,
    code: &'a str,
    code_verifier: Option<&'a str>,
    grant_type: &'static str,
    redirect_uri: &'a str,
}

#[derive(Debug, Deserialize)]
struct AuthProviderTokenSet {
    access_token: Option<String>,
    // token_type: Option<String>,
    id_token: Option<String>,
    error: Option<String>,
    error_description: Option<String>,
}

impl AuthProviderCallback {
    pub async fn extract_user(
        &self,
        provider: &AuthProvider,
        link_cookie: &Option<AuthProviderLinkCookie>,
        payload: &ProviderCallbackRequest,
    ) -> Result<(User, ProviderMfaLogin, NewFederatedUserCreated), ErrorResponse> {
        let mut payload = OidcCodeRequestParams {
            // a client MAY add the `client_id`, but it MUST add it when it's public
            client_id: &provider.client_id,
            client_secret: None,
            code: &payload.code,
            code_verifier: provider.use_pkce.then_some(&payload.pkce_verifier),
            grant_type: "authorization_code",
            redirect_uri: &RauthyConfig::get().provider_callback_uri,
        };
        if provider.client_secret_post {
            payload.client_secret = AuthProvider::secret_cleartext(&provider.secret)?;
        }

        let res = {
            let mut builder = http_client()
                .post(&provider.token_endpoint)
                .header(ACCEPT, APPLICATION_JSON);

            if provider.client_secret_basic {
                builder = builder.basic_auth(
                    &provider.client_id,
                    AuthProvider::secret_cleartext(&provider.secret)?,
                );
            }

            builder
        }
        .form(&payload)
        .send()
        .await?;

        let status = res.status().as_u16();
        debug!("POST /token auth provider status: {status}");

        // return early if we got any error
        if !res.status().is_success() {
            let err = match res.text().await {
                Ok(body) => format!(
                    "HTTP {status} during POST {} for upstream auth provider '{}'\n{body}",
                    provider.token_endpoint, provider.client_id
                ),
                Err(_) => format!(
                    "HTTP {status} during POST {} for upstream auth provider '{}' without any body",
                    provider.token_endpoint, provider.client_id
                ),
            };
            error!("{}", err);
            return Err(ErrorResponse::new(ErrorResponseType::Internal, err));
        }

        let ts = match res.json::<AuthProviderTokenSet>().await {
            Ok(ts) => ts,
            Err(err) => {
                let err = format!(
                    "Deserializing /token response from auth provider {}: {err}",
                    provider.client_id
                );
                error!("{err}");
                return Err(ErrorResponse::new(ErrorResponseType::Internal, err));
            }
        };

        if let Some(err) = ts.error {
            let msg = format!(
                "/token request error: {err}: {}",
                ts.error_description.unwrap_or_default()
            );
            error!("{msg}");
            return Err(ErrorResponse::new(ErrorResponseType::Internal, msg));
        }

        if let Some(id_token) = ts.id_token {
            let claims_bytes = AuthProviderIdClaims::self_as_bytes_from_token(&id_token)?;

            // Some providers like Discord send pretty useless id_tokens that do not even contain
            // the requested claims. If anything fails to extract at least the bare minimum, we want
            // to go on and try fetching userinfo using the access token below.
            match AuthProviderIdClaims::try_from(claims_bytes.as_slice()) {
                Ok(claims) => match claims.validate_update_user(provider, link_cookie).await {
                    Ok(res) => return Ok(res),
                    Err(err) => {
                        debug!("Error validating the user extracted from the id_claims: {err}");
                    }
                },
                Err(err) => {
                    debug!("Failed to extract claims from id_token: {err}. Trying access token.");
                }
            }
        }

        if let Some(access_token) = ts.access_token {
            // the id_token only exists, if we actually have an OIDC provider.
            // If we only get an access token, we need to do another request to the userinfo endpoint
            let res = http_client()
                .get(&provider.userinfo_endpoint)
                .header(AUTHORIZATION, format!("Bearer {access_token}"))
                .header(ACCEPT, APPLICATION_JSON)
                .send()
                .await?;

            let status = res.status().as_u16();
            debug!("GET /userinfo auth provider status: {status}");

            let res_bytes = res.bytes().await?;
            let mut claims = AuthProviderIdClaims::try_from(res_bytes.as_ref())?;

            if claims.email.is_none() && provider.typ == AuthProviderType::GitHub {
                auth_provider_cust_impls::get_github_private_email(&access_token, &mut claims)
                    .await?;
            }

            claims.validate_update_user(provider, link_cookie).await
        } else {
            let err = "Neither `access_token` nor `id_token` existed";
            error!("{err}");
            Err(ErrorResponse::new(ErrorResponseType::BadRequest, err))
        }
    }

    pub async fn extract_user_at_proto(
        &self,
        provider: &AuthProvider,
        link_cookie: &Option<AuthProviderLinkCookie>,
        payload: &ProviderCallbackRequest,
    ) -> Result<(User, ProviderMfaLogin, NewFederatedUserCreated), ErrorResponse> {
        let atproto = atproto::Client::get();

        let params = atrium_oauth::CallbackParams {
            code: payload.code.clone(),
            state: Some(payload.state.clone()),
            iss: payload.iss_atproto.clone(),
        };
        // return early if we got any error
        let (session_manager, app_state) = atproto.callback(params).await.map_err(|error| {
            error!(%error, "failed to complete authorization callback for ATProto");

            ErrorResponse::new(
                ErrorResponseType::Internal,
                "failed to complete authorization callback for ATProto",
            )
        })?;

        let Some(app_state) = app_state else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "missing callback state for ATProto",
            ));
        };

        if !constant_time_eq::constant_time_eq(app_state.as_bytes(), self.callback_id.as_bytes()) {
            return Err(ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "callback state mismatch for ATProto",
            ));
        }

        let agent = atrium_api::agent::Agent::new(session_manager);

        let Some(did) = agent.did().await else {
            error!("missing DID for ATProto session");
            return Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "missing DID for ATProto session",
            ));
        };

        let Some(session) = DB.get(&did).await.map_err(|error| {
            error!(%error, "failed to get session for ATProto callback");

            ErrorResponse::new(
                ErrorResponseType::Internal,
                "failed to get session for ATProto callback",
            )
        })?
        else {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "failed to complete authorization callback for atproto",
            ));
        };

        let data = match agent.api.com.atproto.server.get_session().await {
            Ok(atrium_api::types::Object { data, .. }) => data,
            Err(error) => {
                error!(%error, "failed to get session for ATProto callback");

                return Err(ErrorResponse::new(
                    ErrorResponseType::Internal,
                    "failed to get session for ATProto callback",
                ));
            }
        };

        let claims = AuthProviderIdClaims {
            sub: Some(Value::String(session.token_set.sub.to_string())),
            email: data.email.map(Cow::from),
            email_verified: data.email_confirmed,
            ..Default::default()
        };

        claims.validate_update_user(provider, link_cookie).await
    }
}

/// Auth Provider as template value for SSR of the Login page
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthProviderTemplate {
    pub id: String,
    pub name: String,
    pub updated: i64,
    /// Not rendered as a login button. Part of the template, never filtered out of it, because
    /// a login via the `idp_hint` query parameter resolves the provider from this very list.
    pub hide_login: bool,
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
                hide_login: provider.hide_login,
            });
        }
        let json = serde_json::to_string(&slf)?;

        client
            .put(Cache::App, IDX_AUTH_PROVIDER_TEMPLATE, &json, CACHE_TTL_APP)
            .await?;

        Ok(json)
    }

    async fn invalidate_cache() -> Result<(), ErrorResponse> {
        DB::cache_delete_delayed([(Cache::App, Some(Cow::from(IDX_AUTH_PROVIDER_TEMPLATE)))])
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

    /// The claims JSON a login carries, parsed only when the provider has claim rules to evaluate.
    /// A callback that carries no claims payload at all, like ATProto, is no claims and not an
    /// error.
    fn claims_json(&self, has_rules: bool) -> Result<Option<Value>, ErrorResponse> {
        match self.json_bytes {
            Some(bytes) if has_rules => {
                serde_json::from_slice::<Value>(bytes)
                    .map(Some)
                    .map_err(|err| {
                        ErrorResponse::new(
                            ErrorResponseType::BadRequest,
                            format!("Cannot parse JSON response from upstream: {err}"),
                        )
                    })
            }
            _ => Ok(None),
        }
    }

    fn apply_provider_email(&self, user: &mut User, email: String) -> Option<String> {
        if user.email == email {
            if self.email_verified == Some(true) {
                user.email_verified = true;
            }

            None
        } else {
            let old_email = std::mem::replace(&mut user.email, email);
            user.email_verified = self.email_verified.unwrap_or(false);

            Some(old_email)
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

        // All local lookups and storage are lowercase-based (`User::find_by_email`,
        // local registration), so normalize the upstream claim once here.
        // Otherwise, mixed-case addresses could break email lookups or create duplicate accounts.
        let email = self.email.as_ref().unwrap().to_lowercase();

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
            Value::String(s) => s.clone(),
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
                if let Ok(mut user) = User::find_by_email(email.clone()).await {
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

        // Paths and regexes are compiled at config save, a federated login only evaluates them.
        // A provider without claim rules, and a login that carries no claims JSON at all (ATProto),
        // evaluate nothing.
        let mappings = provider.compiled_mappings()?;
        let claims = self.claims_json(mappings.has_rules())?;

        // A name is only ever removed when this provider maps it. A name inside the mapping set is
        // provider-owned, and that includes a name an admin granted to that user by hand. A target
        // is added when any of its rules matched, and removed only when none of them matched.
        let mut roles_add: BTreeSet<&str> = BTreeSet::new();
        let mut roles_managed: BTreeSet<&str> = BTreeSet::new();
        let mut groups_add: BTreeSet<&str> = BTreeSet::new();
        let mut groups_managed: BTreeSet<&str> = BTreeSet::new();

        // check if mfa has been used by upstream claim
        let provider_mfa_login = match &claims {
            Some(claims) => {
                let mut nodes = ClaimNodes::with_capacity(16);
                let mut claims_buf = String::with_capacity(16);

                if let Some(rule) = &mappings.admin {
                    roles_managed.insert(RAUTHY_ADMIN_ROLE);
                    if rule.matches(claims, &mut nodes, &mut claims_buf) {
                        roles_add.insert(RAUTHY_ADMIN_ROLE);
                    }
                }
                for target in &mappings.roles {
                    roles_managed.insert(target.target.as_str());
                    if target.rule.matches(claims, &mut nodes, &mut claims_buf) {
                        roles_add.insert(target.target.as_str());
                    }
                }
                for target in &mappings.groups {
                    groups_managed.insert(target.target.as_str());
                    if target.rule.matches(claims, &mut nodes, &mut claims_buf) {
                        groups_add.insert(target.target.as_str());
                    }
                }

                // A target that was deleted after the config save is dropped here, never
                // auto-created on a login path. Only names that matched need the check, and the
                // cached role and group lists are only read when something matched at all.
                if !roles_add.is_empty() {
                    let all_roles = Role::find_all().await?;
                    // A group admin role is only meaningful for a group that exists.
                    let all_groups = if roles_add
                        .iter()
                        .any(|name| name.starts_with(RAUTHY_ADMIN_GROUP_PREFIX))
                    {
                        Some(Group::find_all().await?)
                    } else {
                        None
                    };
                    roles_add.retain(|name| match name.strip_prefix(RAUTHY_ADMIN_GROUP_PREFIX) {
                        Some(group) => all_groups.as_deref().is_some_and(|groups| {
                            groups
                                .iter()
                                .any(|g| group_admin_matches_group(&g.name, group))
                        }),
                        None => all_roles.iter().any(|role| role.name == *name),
                    });
                }
                if !groups_add.is_empty() {
                    let all_groups = Group::find_all().await?;
                    groups_add.retain(|name| all_groups.iter().any(|group| group.name == *name));
                }

                if let Some(rule) = &mappings.mfa
                    && rule.matches(claims, &mut nodes, &mut claims_buf)
                {
                    ProviderMfaLogin::Yes
                } else {
                    ProviderMfaLogin::No
                }
            }
            None => ProviderMfaLogin::No,
        };

        let now = Utc::now().timestamp();
        let user = if let Some(mut user) = user_opt {
            let mut forbidden_error = None;

            // validate federation_uid
            // we must reject any upstream login, if a non-federated local user with the same email
            // exists, as it could lead to an account takeover
            if user.federation_uid.is_none()
                || user.federation_uid.as_deref() != Some(claims_user_id.as_str())
            {
                forbidden_error = Some("non-federated user or ID mismatch");
            }

            // validate auth_provider_id
            if user.auth_provider_id.as_deref() != Some(provider.id.as_str()) {
                forbidden_error = Some("invalid login from wrong auth provider");
            }

            if let Some(err) = forbidden_error {
                user.save_login_failure().await?;

                return Err(ErrorResponse::new(
                    ErrorResponseType::Forbidden,
                    err.to_string(),
                ));
            }

            let old_email = self.apply_provider_email(&mut user, email);

            // check other existing values and possibly update them
            let given_name = self.given_name();
            if user.given_name.as_str() != given_name {
                user.given_name = given_name.to_string();
            }
            let family_name = self.family_name();
            if user.family_name.as_deref() != family_name {
                user.family_name = family_name.map(String::from);
            }

            // Empty entries are dropped: `"".split(',')` yields one empty string.
            let roles: BTreeSet<&str> = user.roles_iter().filter(|r| !r.is_empty()).collect();
            user.roles = reconcile(&roles, &roles_managed, &roles_add);
            let groups: BTreeSet<&str> = user.groups_iter().filter(|g| !g.is_empty()).collect();
            let groups_csv = reconcile(&groups, &groups_managed, &groups_add);
            user.groups = (!groups_csv.is_empty()).then_some(groups_csv);

            // update the user on our side
            user.last_login = Some(now);
            user.last_failed_login = None;
            user.failed_login_attempts = None;

            user.save(old_email).await?;
            user
        } else {
            // Create a new federated user
            let new_user = User {
                email: email.clone(),
                given_name: self.given_name().to_string(),
                family_name: self.family_name().map(String::from),
                roles: roles_add.iter().join(","),
                groups: if groups_add.is_empty() {
                    None
                } else {
                    Some(groups_add.iter().join(","))
                },
                enabled: true,
                email_verified: self.email_verified.unwrap_or(false),
                last_login: Some(now),
                language: self
                    .locale
                    .as_ref()
                    .map(|l| Language::from(l.as_ref()))
                    .unwrap_or_default(),
                auth_provider_id: Some(provider.id.clone()),
                federation_uid: Some(claims_user_id.clone()),
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
            Some(username.to_string())
        } else {
            self.login.as_ref().map(|l| l.to_string())
        };
        if let Some(username) = preferred_username {
            // Check via local read first, which is a lot cheaper than write through the Raft.
            if UserValues::validate_preferred_username_free(username.clone())
                .await
                .is_ok()
            {
                // We ignore the result in case of a race condition.
                // Better continue without setting the username than failing.
                // The user can edit it later on anyway.
                let _ = UserValues::upsert_preferred_username(user.id.clone(), username).await;
            }
        }

        if found_values {
            UserValues::upsert(user.id.clone(), user_values).await?;
        }

        Ok((user, provider_mfa_login, new_federated_user))
    }
}

/// Reconcile a user's role / group list against a provider's claim rules. A name is only ever
/// removed when it is part of `managed`, which is exactly the set of targets the provider's own
/// rules name. Anything a local admin granted, and anything another provider manages, stays
/// exactly as it is. Matched targets are added.
/// Iterating the `BTreeSet` keeps the stored CSV byte-stable across nodes.
fn reconcile<'a>(
    current: &BTreeSet<&'a str>,
    managed: &BTreeSet<&'a str>,
    matched: &BTreeSet<&'a str>,
) -> String {
    let mut out = current.clone();
    out.extend(matched.iter().copied());
    for name in managed.difference(matched) {
        out.remove(name);
    }
    out.iter().join(",")
}

/// Match a group name against the group part of a `rauthy_admin:<group>` role, exactly like the
/// group admin roles are matched in `Principal`: the name matches itself, a trailing `*` makes it
/// a prefix glob, and empty group names never match.
fn group_admin_matches_group(name: &str, group: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    match group.strip_suffix('*') {
        Some(prefix) => name.starts_with(prefix),
        None => name == group,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Both a login without a claims payload (ATProto) and a provider without claim rules mean "no
    /// claims": no parse, no error, and never an unwrap on `None`.
    #[test]
    fn claims_json_needs_both_rules_and_payload() -> Result<(), ErrorResponse> {
        let json: &[u8] = br#"{"sub": "1", "roles": ["admin"]}"#;

        let with_payload = AuthProviderIdClaims {
            json_bytes: Some(json),
            ..Default::default()
        };
        assert!(with_payload.claims_json(true)?.is_some());
        assert!(
            with_payload.claims_json(false)?.is_none(),
            "a provider without rules parses nothing"
        );

        let atproto = AuthProviderIdClaims {
            email: Some(Cow::Borrowed("a@b.c")),
            ..Default::default()
        };
        assert!(
            atproto.claims_json(true)?.is_none(),
            "a callback without a claims payload is no claims"
        );
        assert!(
            atproto.claims_json(false)?.is_none(),
            "no payload and no rules is no claims"
        );

        let broken = AuthProviderIdClaims {
            json_bytes: Some(b"not json"),
            ..Default::default()
        };
        assert!(broken.claims_json(true).is_err());

        Ok(())
    }

    /// The provider's rules only ever add and remove their own targets. A name an admin granted
    /// locally is never in `managed`, so it can never be removed, and a name without a rule is
    /// never added.
    #[test]
    fn reconcile_only_touches_provider_managed_names() {
        let current: BTreeSet<&str> = ["api", "local_admin", "tgchat:admin", "other_prov"]
            .into_iter()
            .collect();
        let managed: BTreeSet<&str> = ["tgchat:admin", "tgchat:read"].into_iter().collect();

        // `tgchat:admin` lost its match, `tgchat:read` gained one: both moves are provider-owned.
        let matched: BTreeSet<&str> = ["tgchat:read"].into_iter().collect();
        assert_eq!(
            reconcile(&current, &managed, &matched),
            "api,local_admin,other_prov,tgchat:read"
        );

        // A match adds, and the locally granted names stay untouched in every case.
        let matched: BTreeSet<&str> = ["tgchat:admin", "tgchat:read"].into_iter().collect();
        assert_eq!(
            reconcile(&current, &managed, &matched),
            "api,local_admin,other_prov,tgchat:admin,tgchat:read"
        );

        // A provider without any rule never changes the user.
        let no_rules: BTreeSet<&str> = BTreeSet::new();
        assert_eq!(
            reconcile(&current, &no_rules, &no_rules),
            "api,local_admin,other_prov,tgchat:admin"
        );

        // New user: there is nothing to keep, the order is the sorted one.
        assert_eq!(
            reconcile(&no_rules, &managed, &matched),
            "tgchat:admin,tgchat:read"
        );
    }

    /// Several rules can name the same target. A single match wins over any number of misses.
    #[test]
    fn reconcile_adds_a_target_that_any_of_its_rules_matched() {
        let current: BTreeSet<&str> = BTreeSet::new();
        let managed: BTreeSet<&str> = ["dev"].into_iter().collect();
        let matched: BTreeSet<&str> = ["dev"].into_iter().collect();
        assert_eq!(reconcile(&current, &managed, &matched), "dev");

        let no_match: BTreeSet<&str> = BTreeSet::new();
        assert_eq!(reconcile(&current, &managed, &no_match), "");
    }

    /// The group part of a group admin target matches like the group admin roles in `Principal`:
    /// the name itself, a prefix glob with a trailing `*`, and `*` for every group.
    #[test]
    fn group_admin_target_matches_its_groups() {
        assert!(group_admin_matches_group("dev", "dev"));
        assert!(!group_admin_matches_group("devx", "dev"));
        assert!(group_admin_matches_group("dev-read", "dev-*"));
        assert!(group_admin_matches_group("dev", "dev*"));
        assert!(group_admin_matches_group("anything", "*"));
        assert!(
            !group_admin_matches_group("", "dev"),
            "an empty group name is never managed"
        );
        assert!(
            !group_admin_matches_group("", "*"),
            "the global glob does not match an empty group name"
        );
    }

    fn user_with_email(email: &str, email_verified: bool) -> User {
        User {
            email: email.to_string(),
            email_verified,
            ..Default::default()
        }
    }

    fn claims_with_email_verified(email_verified: Option<bool>) -> AuthProviderIdClaims<'static> {
        AuthProviderIdClaims {
            email_verified,
            ..Default::default()
        }
    }

    #[test]
    fn provider_verifies_matching_email() {
        let claims = claims_with_email_verified(Some(true));
        let mut user = user_with_email("user@example.com", false);

        let old_email = claims.apply_provider_email(&mut user, "user@example.com".to_string());

        assert!(user.email_verified);
        assert_eq!(old_email, None);
    }

    #[test]
    fn unverified_provider_claim_does_not_verify_matching_email() {
        let claims = claims_with_email_verified(Some(false));
        let mut user = user_with_email("user@example.com", false);

        claims.apply_provider_email(&mut user, "user@example.com".to_string());

        assert!(!user.email_verified);
    }

    #[test]
    fn missing_provider_claim_does_not_verify_matching_email() {
        let claims = claims_with_email_verified(None);
        let mut user = user_with_email("user@example.com", false);

        claims.apply_provider_email(&mut user, "user@example.com".to_string());

        assert!(!user.email_verified);
    }

    #[test]
    fn unverified_provider_claim_does_not_revoke_matching_email() {
        let claims = claims_with_email_verified(Some(false));
        let mut user = user_with_email("user@example.com", true);

        claims.apply_provider_email(&mut user, "user@example.com".to_string());

        assert!(user.email_verified);
    }

    #[test]
    fn missing_provider_claim_does_not_revoke_matching_email() {
        let claims = claims_with_email_verified(None);
        let mut user = user_with_email("user@example.com", true);

        claims.apply_provider_email(&mut user, "user@example.com".to_string());

        assert!(user.email_verified);
    }

    #[test]
    fn verified_provider_claim_verifies_changed_email() {
        let claims = claims_with_email_verified(Some(true));
        let mut user = user_with_email("old@example.com", false);

        let old_email = claims.apply_provider_email(&mut user, "new@example.com".to_string());

        assert_eq!(user.email, "new@example.com");
        assert!(user.email_verified);
        assert_eq!(old_email.as_deref(), Some("old@example.com"));
    }

    #[test]
    fn unverified_provider_claim_does_not_carry_verification_to_changed_email() {
        let claims = claims_with_email_verified(Some(false));
        let mut user = user_with_email("old@example.com", true);

        let old_email = claims.apply_provider_email(&mut user, "new@example.com".to_string());

        assert_eq!(user.email, "new@example.com");
        assert!(!user.email_verified);
        assert_eq!(old_email.as_deref(), Some("old@example.com"));
    }

    #[test]
    fn missing_provider_claim_does_not_carry_verification_to_changed_email() {
        let claims = claims_with_email_verified(None);
        let mut user = user_with_email("old@example.com", true);

        let old_email = claims.apply_provider_email(&mut user, "new@example.com".to_string());

        assert_eq!(user.email, "new@example.com");
        assert!(!user.email_verified);
        assert_eq!(old_email.as_deref(), Some("old@example.com"));
    }

    #[test]
    fn test_id_token_deserialization() {
        // this raw token contains unicode encoded chars
        let raw = "eyJhbGciOiJSUzI1NiIsImtpZCI6InlPeXYtWG1tUVYtUTNrUUVCbkRHVlFGb2hoVTNyaTFiRkY0VEg4VGZYTFUiLCJ0eXAiOiJKV1QifQ.eyJhdWQiOlsiaHR0cHM6Ly9vcGVuaWRjb25uZWN0Lm5ldCJdLCJlbWFpbCI6InRlc3RcdTAwMjZAYXBpdG1hbi5jb20iLCJlbWFpbF92ZXJpZmllZCI6dHJ1ZSwiZXhwIjoxNzE4MjAwNjc2LCJpYXQiOjE3MTgxMTQyNzYsImlzcyI6Imh0dHBzOi8vbGFzdGxvZ2luLmlvIiwibmFtZSI6InRlc3RcdTAwMjYiLCJub25jZSI6IiIsInN1YiI6InRlc3RcdTAwMjZAYXBpdG1hbi5jb20ifQ.IchRsMNdE4isQ9ug8GqFtc_TRkZ3wA1KQw4dqnwqqVgNMcbY3qTuufwRHW-zoLDxQZkNOCl0niJolrLGmSfeSAo0fRtRMrkq41b7hhcnfE2MLBTZbI3E8m2mbxdgGlBJxeOBFdIwhN3meuioxiqnoAdCh1aw8p2dj7TuBaHpNXdREUcB76U1ZXHqSmYFWZDVROzMgVb7PjL2ikPcjksWf-jlFIRrEdbRXm9DygL9LAUFI10IjCZePFv8wjmr0aJS1pXAXYn31hoSWaH0xxoDCmO4t4aBafL392zX6is0i3uRYSu-3L0GJRCT7qmbm_FiGvEqfFQoILslKRvzWG74Ww";
        let claims_bytes = AuthProviderIdClaims::self_as_bytes_from_token(raw).unwrap();
        assert!(AuthProviderIdClaims::try_from(claims_bytes.as_ref()).is_ok());
    }
}
