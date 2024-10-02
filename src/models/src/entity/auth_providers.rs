use crate::api_cookie::ApiCookie;
use crate::app_state::AppState;
use crate::cache::{Cache, DB};
use crate::entity::auth_codes::AuthCode;
use crate::entity::clients::Client;
use crate::entity::sessions::Session;
use crate::entity::users::User;
use crate::entity::users_values::UserValues;
use crate::entity::webauthn::WebauthnLoginReq;
use crate::language::Language;
use crate::{AuthStep, AuthStepAwaitWebauthn, AuthStepLoggedIn};
use actix_web::cookie::Cookie;
use actix_web::http::header;
use actix_web::http::header::HeaderValue;
use actix_web::{web, HttpRequest};
use cryptr::utils::secure_random_alnum;
use cryptr::EncValue;
use image::EncodableLayout;
use itertools::Itertools;
use rauthy_api_types::auth_providers::{
    ProviderCallbackRequest, ProviderLoginRequest, ProviderLookupRequest, ProviderRequest,
};
use rauthy_api_types::auth_providers::{
    ProviderLinkedUserResponse, ProviderLookupResponse, ProviderResponse,
};
use rauthy_api_types::users::UserValuesRequest;
use rauthy_common::constants::{
    APPLICATION_JSON, CACHE_TTL_APP, CACHE_TTL_AUTH_PROVIDER_CALLBACK, COOKIE_UPSTREAM_CALLBACK,
    IDX_AUTH_PROVIDER, IDX_AUTH_PROVIDER_TEMPLATE, PROVIDER_CALLBACK_URI,
    PROVIDER_CALLBACK_URI_ENCODED, PROVIDER_LINK_COOKIE, RAUTHY_VERSION,
    UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS, WEBAUTHN_REQ_EXP,
};
use rauthy_common::utils::{
    base64_decode, base64_encode, base64_url_encode, base64_url_no_pad_decode, get_rand,
    new_store_id,
};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::header::{ACCEPT, AUTHORIZATION};
use reqwest::tls;
use ring::digest;
use serde::{Deserialize, Serialize};
use serde_json::value;
use serde_json_path::JsonPath;
use sqlx::{query, query_as, FromRow};
use std::borrow::Cow;
use std::fmt::Write;
use std::str::FromStr;
use std::time::Duration;
use time::OffsetDateTime;
use tracing::{debug, error};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "varchar")]
#[sqlx(rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum AuthProviderType {
    Custom,
    Github,
    Google,
    OIDC,
}

impl AuthProviderType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Custom => "custom",
            Self::Github => "github",
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
            "github" => Self::Github,
            "google" => Self::Google,
            "oidc" => Self::OIDC,
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Invalid AuthProviderType",
                ))
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
            rauthy_api_types::auth_providers::AuthProviderType::Github => Self::Github,
            rauthy_api_types::auth_providers::AuthProviderType::Google => Self::Google,
            rauthy_api_types::auth_providers::AuthProviderType::OIDC => Self::OIDC,
        }
    }
}

impl From<AuthProviderType> for rauthy_api_types::auth_providers::AuthProviderType {
    fn from(value: AuthProviderType) -> Self {
        match value {
            AuthProviderType::Custom => Self::Custom,
            AuthProviderType::Github => Self::Github,
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
        let slf = bincode::deserialize::<AuthProviderLinkCookie>(bytes.as_slice())?;
        Ok(slf)
    }
}

impl AuthProviderLinkCookie {
    pub fn build_cookie(&self) -> Result<Cookie<'_>, ErrorResponse> {
        let bytes = bincode::serialize(self)?;
        let value = base64_encode(&bytes);
        Ok(ApiCookie::build(PROVIDER_LINK_COOKIE, value, 300))
    }

    pub fn deletion_cookie<'a>() -> Cookie<'a> {
        ApiCookie::build(PROVIDER_LINK_COOKIE, "", 0)
    }
}

/// Upstream Auth Provider for upstream logins without a local Rauthy account
#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct AuthProvider {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub typ: AuthProviderType,

    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,

    pub client_id: String,
    pub secret: Option<Vec<u8>>,
    pub scope: String,

    pub admin_claim_path: Option<String>,
    pub admin_claim_value: Option<String>,
    pub mfa_claim_path: Option<String>,
    pub mfa_claim_value: Option<String>,

    pub allow_insecure_requests: bool,
    pub use_pkce: bool,

    pub root_pem: Option<String>,
}

impl AuthProvider {
    pub async fn create(
        data: &web::Data<AppState>,
        payload: ProviderRequest,
    ) -> Result<Self, ErrorResponse> {
        let slf = Self::try_from_id_req(new_store_id(), payload)?;
        let typ = slf.typ.as_str();

        query!(
            r#"INSERT INTO
            auth_providers (id, name, enabled, typ, issuer, authorization_endpoint, token_endpoint,
            userinfo_endpoint, client_id, secret, scope, admin_claim_path, admin_claim_value,
            mfa_claim_path, mfa_claim_value, allow_insecure_requests, use_pkce, root_pem)
            VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)"#,
            slf.id,
            slf.name,
            slf.enabled,
            typ,
            slf.issuer,
            slf.authorization_endpoint,
            slf.token_endpoint,
            slf.userinfo_endpoint,
            slf.client_id,
            slf.secret,
            slf.scope,
            slf.admin_claim_path,
            slf.admin_claim_value,
            slf.mfa_claim_path,
            slf.mfa_claim_value,
            slf.allow_insecure_requests,
            slf.use_pkce,
            slf.root_pem,
        )
        .execute(&data.db)
        .await?;

        Self::invalidate_cache_all(data).await?;

        DB::client()
            .put(Cache::App, Self::cache_idx(&slf.id), &slf, CACHE_TTL_APP)
            .await?;

        Ok(slf)
    }

    pub async fn find(data: &web::Data<AppState>, id: &str) -> Result<Self, ErrorResponse> {
        let client = DB::client();
        if let Some(slf) = client.get(Cache::App, Self::cache_idx(id)).await? {
            return Ok(slf);
        }

        let slf = query_as!(Self, "SELECT * FROM auth_providers WHERE id = $1", id)
            .fetch_one(&data.db)
            .await?;

        client
            .put(Cache::App, Self::cache_idx(id), &slf, CACHE_TTL_APP)
            .await?;

        Ok(slf)
    }

    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let client = DB::client();
        if let Some(res) = client.get(Cache::App, Self::cache_idx("all")).await? {
            return Ok(res);
        }

        let res = query_as!(Self, "SELECT * FROM auth_providers")
            .fetch_all(&data.db)
            .await?;

        // needed for rendering each single login page -> always cache this
        client
            .put(Cache::App, Self::cache_idx("all"), &res, CACHE_TTL_APP)
            .await?;

        Ok(res)
    }

    pub async fn find_linked_users(
        data: &web::Data<AppState>,
        id: &str,
    ) -> Result<Vec<ProviderLinkedUserResponse>, ErrorResponse> {
        let users = query_as!(
            ProviderLinkedUserResponse,
            "SELECT id, email FROM users WHERE auth_provider_id = $1",
            id
        )
        .fetch_all(&data.db)
        .await?;

        Ok(users)
    }

    pub async fn delete(data: &web::Data<AppState>, id: &str) -> Result<(), ErrorResponse> {
        query!("DELETE FROM auth_providers WHERE id = $1", id)
            .execute(&data.db)
            .await?;

        Self::invalidate_cache_all(data).await?;
        DB::client().delete(Cache::App, Self::cache_idx(id)).await?;

        Ok(())
    }

    pub async fn update(
        data: &web::Data<AppState>,
        id: String,
        payload: ProviderRequest,
    ) -> Result<(), ErrorResponse> {
        Self::try_from_id_req(id, payload)?.save(data).await
    }

    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        let typ = self.typ.as_str();
        query!(
            r#"UPDATE auth_providers
            SET name = $1, enabled = $2, issuer = $3, typ = $4, authorization_endpoint = $5,
            token_endpoint = $6, userinfo_endpoint = $7, client_id = $8, secret = $9, scope = $10,
            admin_claim_path = $11, admin_claim_value = $12, mfa_claim_path = $13,
            mfa_claim_value = $14, allow_insecure_requests = $15, use_pkce = $16, root_pem = $17
            WHERE id = $18"#,
            self.name,
            self.enabled,
            self.issuer,
            typ,
            self.authorization_endpoint,
            self.token_endpoint,
            self.userinfo_endpoint,
            self.client_id,
            self.secret,
            self.scope,
            self.admin_claim_path,
            self.admin_claim_value,
            self.mfa_claim_path,
            self.mfa_claim_value,
            self.allow_insecure_requests,
            self.use_pkce,
            self.root_pem,
            self.id,
        )
        .execute(&data.db)
        .await?;

        Self::invalidate_cache_all(data).await?;
        DB::client()
            .put(Cache::App, Self::cache_idx(&self.id), self, CACHE_TTL_APP)
            .await?;

        Ok(())
    }
}

impl AuthProvider {
    #[inline(always)]
    fn cache_idx(id: &str) -> String {
        format!("{}_{}", IDX_AUTH_PROVIDER, id)
    }

    fn cleanup_scope(scope: &str) -> String {
        scope
            .split(' ')
            // filter this way to catch multiple spaces in a row
            .filter_map(|s| if !s.is_empty() { Some(s.trim()) } else { None })
            .collect::<Vec<&str>>()
            .join("+")
    }

    fn build_client(
        danger_allow_insecure: bool,
        root_pem: Option<&str>,
    ) -> Result<reqwest::Client, ErrorResponse> {
        let client = if danger_allow_insecure {
            reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .connect_timeout(Duration::from_secs(10))
                .tcp_keepalive(Duration::from_secs(30))
                .user_agent(format!("Rauthy Auth Provider Client v{}", RAUTHY_VERSION))
                .https_only(false)
                .danger_accept_invalid_certs(true)
                .build()?
        } else {
            let mut builder = reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .connect_timeout(Duration::from_secs(10))
                .tcp_keepalive(Duration::from_secs(30))
                .min_tls_version(tls::Version::TLS_1_3)
                .tls_built_in_root_certs(true)
                .user_agent(format!("Rauthy Auth Provider Client v{}", RAUTHY_VERSION))
                .https_only(true);
            if let Some(pem) = root_pem {
                let root_cert = reqwest::tls::Certificate::from_pem(pem.as_bytes())?;
                builder = builder.add_root_certificate(root_cert);
            }
            builder.build()?
        };

        Ok(client)
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

            client_id: req.client_id,
            secret,
            scope,

            admin_claim_path: req.admin_claim_path,
            admin_claim_value: req.admin_claim_value,
            mfa_claim_path: req.mfa_claim_path,
            mfa_claim_value: req.mfa_claim_value,

            allow_insecure_requests: req.danger_allow_insecure.unwrap_or(false),
            use_pkce: req.use_pkce,
            root_pem: req.root_pem,
        })
    }

    async fn invalidate_cache_all(data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        DB::client()
            .delete(Cache::App, Self::cache_idx("all"))
            .await?;

        // Directly update the template cache preemptively.
        // This is needed all the time anyway.
        AuthProviderTemplate::update_cache(data).await?;

        Ok(())
    }

    pub async fn lookup_config(
        payload: &ProviderLookupRequest,
    ) -> Result<ProviderLookupResponse, ErrorResponse> {
        let client = Self::build_client(
            payload.danger_allow_insecure.unwrap_or(false),
            payload.root_pem.as_deref(),
        )?;

        let url = if let Some(url) = &payload.metadata_url {
            Cow::from(url)
        } else if let Some(iss) = &payload.issuer {
            let url = if iss.ends_with('/') {
                format!("{}.well-known/openid-configuration", iss)
            } else {
                format!("{}/.well-known/openid-configuration", iss)
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
            Cow::from(format!("https://{}", url))
        };

        debug!("AuthProvider lookup to {}", config_url);
        let res = client.get(config_url.as_ref()).send().await?;
        let status = res.status();
        if !status.is_success() {
            let body = res.text().await?;
            return Err(ErrorResponse::new(
                ErrorResponseType::Connection,
                format!(
                    "HTTP {} when trying provider config lookup to {}: {}",
                    config_url, status, body
                ),
            ));
        }

        let well_known = res.json::<WellKnownLookup>().await.map_err(|err| {
            ErrorResponse::new(
                // TODO we could make this more UX friendly in the future and return a link to the
                // docs, when they exist
                ErrorResponseType::BadRequest,
                format!(
                    "The provider does not support the mandatory openid-configuration: {}",
                    err
                ),
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

        Ok(ProviderLookupResponse {
            issuer: well_known.issuer,
            // TODO optimization (and possibly security enhancement): strip issuer url from all of these?
            // what does the RFC mention about it? MUST they always be on the same sub path?
            authorization_endpoint: well_known.authorization_endpoint,
            token_endpoint: well_known.token_endpoint,
            userinfo_endpoint: well_known.userinfo_endpoint,
            root_pem: &payload.root_pem,
            use_pkce: well_known
                .code_challenge_methods_supported
                .iter()
                .any(|c| c == "S256"),
            danger_allow_insecure: payload.danger_allow_insecure.unwrap_or(false),
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

    pub fn get_secret_cleartext(secret: &Option<Vec<u8>>) -> Result<Option<String>, ErrorResponse> {
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
        let secret = AuthProvider::get_secret_cleartext(&value.secret)?;
        Ok(Self {
            id: value.id,
            name: value.name,
            typ: value.typ.into(),
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
        DB::client()
            .delete(Cache::AuthProviderCallback, callback_id)
            .await?;

        Ok(())
    }

    async fn find(callback_id: String) -> Result<Self, ErrorResponse> {
        let opt: Option<Self> = DB::client()
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

    async fn save(&self) -> Result<(), ErrorResponse> {
        DB::client()
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

impl AuthProviderCallback {
    /// returns (encrypted cookie, xsrf token, location header, optional allowed origins)
    pub async fn login_start(
        data: &web::Data<AppState>,
        payload: ProviderLoginRequest,
    ) -> Result<(Cookie, String, HeaderValue), ErrorResponse> {
        let provider = AuthProvider::find(data, &payload.provider_id).await?;
        let client = Client::find(data, payload.client_id).await?;

        let slf = Self {
            callback_id: secure_random_alnum(32),
            xsrf_token: secure_random_alnum(32),
            typ: provider.typ,

            req_client_id: client.id,
            req_scopes: payload.scopes,
            req_redirect_uri: payload.redirect_uri,
            req_state: payload.state,
            req_nonce: payload.nonce,
            req_code_challenge: payload.code_challenge,
            req_code_challenge_method: payload.code_challenge_method,

            provider_id: provider.id,

            pkce_challenge: payload.pkce_challenge,
        };

        let mut location = format!(
            "{}{}client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            provider.authorization_endpoint,
            // append parameters if there are already some parameters
            if provider.authorization_endpoint.contains('?') {
                '&'
            } else {
                '?'
            },
            provider.client_id,
            *PROVIDER_CALLBACK_URI_ENCODED,
            provider.scope,
            slf.callback_id
        );
        if provider.use_pkce {
            write!(
                location,
                "&code_challenge={}&code_challenge_method=S256",
                slf.pkce_challenge
            )
            .expect("write to always succeed");
        }

        let cookie = ApiCookie::build(
            COOKIE_UPSTREAM_CALLBACK,
            &slf.callback_id,
            UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS as i64,
        );

        slf.save().await?;

        Ok((
            cookie,
            slf.xsrf_token,
            HeaderValue::from_str(&location).expect("Location HeaderValue to be correct"),
        ))
    }

    /// In case of any error, the callback code will be fully deleted for security reasons.
    pub async fn login_finish<'a>(
        data: &'a web::Data<AppState>,
        req: &'a HttpRequest,
        payload: &'a ProviderCallbackRequest,
        mut session: Session,
    ) -> Result<(AuthStep, Cookie<'a>), ErrorResponse> {
        // the callback id for the cache should be inside the encrypted cookie
        let callback_id = ApiCookie::from_req(req, COOKIE_UPSTREAM_CALLBACK).ok_or_else(|| {
            ErrorResponse::new(
                ErrorResponseType::Forbidden,
                "Missing encrypted callback cookie",
            )
        })?;

        // validate state
        if callback_id != payload.state {
            Self::delete(callback_id).await?;

            error!("`state` does not match");
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "`state` does not match",
            ));
        }
        debug!("callback state is valid");

        // validate csrf token
        let slf = Self::find(callback_id).await?;
        if slf.xsrf_token != payload.xsrf_token {
            Self::delete(slf.callback_id).await?;

            error!("invalid CSRF token");
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "invalid CSRF token",
            ));
        }
        debug!("callback csrf token is valid");

        // validate PKCE verifier
        let hash = digest::digest(&digest::SHA256, payload.pkce_verifier.as_bytes());
        let hash_base64 = base64_url_encode(hash.as_ref());
        if slf.pkce_challenge != hash_base64 {
            Self::delete(slf.callback_id).await?;

            error!("invalid PKCE verifier");
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "invalid PKCE verifier",
            ));
        }
        debug!("callback pkce verifier is valid");

        // request is valid -> fetch token for the user
        let provider = AuthProvider::find(data, &slf.provider_id).await?;
        let client = AuthProvider::build_client(
            provider.allow_insecure_requests,
            provider.root_pem.as_deref(),
        )?;
        let payload = OidcCodeRequestParams {
            client_id: &provider.client_id,
            client_secret: AuthProvider::get_secret_cleartext(&provider.secret)?,
            code: &payload.code,
            code_verifier: provider.use_pkce.then_some(&payload.pkce_verifier),
            grant_type: "authorization_code",
            redirect_uri: &PROVIDER_CALLBACK_URI,
        };
        let res = client
            .post(&provider.token_endpoint)
            .header(ACCEPT, APPLICATION_JSON)
            .basic_auth(
                &provider.client_id,
                AuthProvider::get_secret_cleartext(&provider.secret)?,
            )
            .form(&payload)
            .send()
            .await?;

        let status = res.status().as_u16();
        debug!("POST /token auth provider status: {}", status);

        // return early if we got any error
        if !res.status().is_success() {
            let err = match res.text().await {
                Ok(body) => format!(
                    "HTTP {} during POST {} for upstream auth provider '{}'\n{}",
                    status, provider.token_endpoint, provider.client_id, body
                ),
                Err(_) => format!(
                    "HTTP {} during POST {} for upstream auth provider '{}' without any body",
                    status, provider.token_endpoint, provider.client_id
                ),
            };
            error!("{}", err);
            return Err(ErrorResponse::new(ErrorResponseType::Internal, err));
        }

        // extract a possibly existing provider link cookie for
        // linking an existing account to a provider
        let link_cookie = ApiCookie::from_req(req, PROVIDER_LINK_COOKIE)
            .and_then(|value| AuthProviderLinkCookie::try_from(value.as_str()).ok());

        // deserialize payload and validate the information
        let (user, provider_mfa_login) = match res.json::<AuthProviderTokenSet>().await {
            Ok(ts) => {
                if let Some(err) = ts.error {
                    let msg = format!(
                        "/token request error: {}: {}",
                        err,
                        ts.error_description.unwrap_or_default()
                    );
                    error!("{}", msg);
                    return Err(ErrorResponse::new(ErrorResponseType::Internal, msg));
                }

                // in case of a standard OIDC provider, we only care about the ID token
                if let Some(id_token) = ts.id_token {
                    let claims_bytes = AuthProviderIdClaims::self_as_bytes_from_token(&id_token)?;
                    let claims = AuthProviderIdClaims::try_from(claims_bytes.as_slice())?;
                    claims
                        .validate_update_user(data, &provider, &link_cookie)
                        .await?
                } else if let Some(access_token) = ts.access_token {
                    // the id_token only exists, if we actually have an OIDC provider.
                    // If we only get an access token, we need to do another request to the
                    // userinfo endpoint
                    let res = client
                        .get(&provider.userinfo_endpoint)
                        .header(AUTHORIZATION, format!("Bearer {}", access_token))
                        .header(ACCEPT, APPLICATION_JSON)
                        .send()
                        .await?;

                    let status = res.status().as_u16();
                    debug!("GET /userinfo auth provider status: {}", status);

                    let res_bytes = res.bytes().await?;
                    let claims = AuthProviderIdClaims::try_from(res_bytes.as_bytes())?;
                    claims
                        .validate_update_user(data, &provider, &link_cookie)
                        .await?
                } else {
                    let err = "Neither `access_token` nor `id_token` existed";
                    error!("{}", err);
                    return Err(ErrorResponse::new(ErrorResponseType::BadRequest, err));
                }
            }
            Err(err) => {
                let err = format!(
                    "Deserializing /token response from auth provider {}: {}",
                    provider.client_id, err
                );
                error!("{}", err);
                return Err(ErrorResponse::new(ErrorResponseType::Internal, err));
            }
        };

        user.check_enabled()?;
        user.check_expired()?;

        if link_cookie.is_some() {
            // If this is the case, we don't need to validate any further client values.
            // We will not generate a new auth code at all -> this is just a request to federate
            // an existing account. The federation has been done in the step above already.
            return Ok((
                AuthStep::ProviderLink,
                AuthProviderLinkCookie::deletion_cookie(),
            ));
        }

        // validate client values
        let client = Client::find_maybe_ephemeral(data, slf.req_client_id).await?;
        let force_mfa = client.force_mfa();
        if force_mfa {
            if provider_mfa_login == ProviderMfaLogin::No && !user.has_webauthn_enabled() {
                return Err(ErrorResponse::new(
                    ErrorResponseType::MfaRequired,
                    "MFA is required for this client",
                ));
            }
            session.set_mfa(data, true).await?;
        }
        client.validate_redirect_uri(&slf.req_redirect_uri)?;
        client.validate_code_challenge(&slf.req_code_challenge, &slf.req_code_challenge_method)?;
        let header_origin = client.validate_origin(req, &data.listen_scheme, &data.public_url)?;

        // ######################################
        // all good, we can generate an auth code

        // authorization code
        let code_lifetime = if force_mfa && user.has_webauthn_enabled() {
            client.auth_code_lifetime + *WEBAUTHN_REQ_EXP as i32
        } else {
            client.auth_code_lifetime
        };
        let scopes = client.sanitize_login_scopes(&slf.req_scopes)?;
        let code = AuthCode::new(
            user.id.clone(),
            client.id,
            Some(session.id.clone()),
            slf.req_code_challenge,
            slf.req_code_challenge_method,
            slf.req_nonce,
            scopes,
            code_lifetime,
        );
        code.save().await?;

        // location header
        let mut loc = format!("{}?code={}", slf.req_redirect_uri, code.id);
        if let Some(state) = slf.req_state {
            write!(loc, "&state={}", state)?;
        };

        let auth_step = if user.has_webauthn_enabled() {
            let step = AuthStepAwaitWebauthn {
                code: get_rand(48),
                header_csrf: Session::get_csrf_header(&session.csrf_token),
                header_origin,
                user_id: user.id.clone(),
                email: user.email,
                exp: *WEBAUTHN_REQ_EXP,
                session,
            };

            WebauthnLoginReq {
                code: step.code.clone(),
                user_id: user.id,
                header_loc: loc,
                header_origin: step
                    .header_origin
                    .as_ref()
                    .map(|h| h.1.to_str().unwrap().to_string()),
            }
            .save()
            .await?;

            AuthStep::AwaitWebauthn(step)
        } else {
            AuthStep::LoggedIn(AuthStepLoggedIn {
                user_id: user.id,
                email: user.email,
                header_loc: (header::LOCATION, HeaderValue::from_str(&loc)?),
                header_csrf: Session::get_csrf_header(&session.csrf_token),
                header_origin,
            })
        };

        // callback data deletion cookie
        let cookie = ApiCookie::build(COOKIE_UPSTREAM_CALLBACK, "", 0);
        Ok((auth_step, cookie))
    }
}

/// Auth Provider as template value for SSR of the Login page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProviderTemplate {
    pub id: String,
    pub name: String,
    // pub logo: Option<Vec<u8>>,
    // pub logo_type: Option<String>,
}

impl AuthProviderTemplate {
    pub async fn get_all_json_template(
        data: &web::Data<AppState>,
    ) -> Result<Option<String>, ErrorResponse> {
        let client = DB::client();
        if let Some(slf) = client.get(Cache::App, IDX_AUTH_PROVIDER_TEMPLATE).await? {
            return Ok(slf);
        }

        let providers = AuthProvider::find_all(data)
            .await?
            .into_iter()
            // We don't want to even show disabled providers
            .filter(|p| p.enabled)
            .map(|p| Self {
                id: p.id,
                name: p.name,
            })
            .collect::<Vec<Self>>();

        let json = if providers.is_empty() {
            None
        } else {
            Some(serde_json::to_string(&providers)?)
        };
        client
            .put(Cache::App, IDX_AUTH_PROVIDER_TEMPLATE, &json, CACHE_TTL_APP)
            .await?;

        Ok(json)
    }

    async fn invalidate_cache() -> Result<(), ErrorResponse> {
        DB::client()
            .delete(Cache::App, IDX_AUTH_PROVIDER_TEMPLATE)
            .await?;

        Ok(())
    }

    async fn update_cache(data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        Self::invalidate_cache().await?;
        Self::get_all_json_template(data).await?;
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AuthProviderAddressClaims<'a> {
    pub formatted: Option<&'a str>,
    pub street_address: Option<&'a str>,
    pub locality: Option<&'a str>,
    pub postal_code: Option<i32>,
    pub country: Option<&'a str>,
}

#[derive(Debug, PartialEq)]
enum ProviderMfaLogin {
    Yes,
    No,
}

#[derive(Debug, Deserialize)]
struct AuthProviderIdClaims<'a> {
    // pub iss: &'a str,
    // json values because some providers provide String, some int
    sub: Option<serde_json::Value>,
    id: Option<serde_json::Value>,
    uid: Option<serde_json::Value>,

    // aud / azp is not being validated, because it works with OIDC only anyway
    // aud: Option<&'a str>,
    // azp: Option<&'a str>,
    // even though `email` is mandatory for Rauthy, we set it to optional for
    // the deserialization to have more control over the error message being returned
    email: Option<Cow<'a, str>>,
    email_verified: Option<bool>,

    name: Option<Cow<'a, str>>,
    given_name: Option<Cow<'a, str>>,
    family_name: Option<Cow<'a, str>>,

    address: Option<AuthProviderAddressClaims<'a>>,
    birthdate: Option<Cow<'a, str>>,
    locale: Option<Cow<'a, str>>,
    phone: Option<Cow<'a, str>>,

    json_bytes: Option<&'a [u8]>,
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
            let (given_name, _) = name.split_once(' ').unwrap_or(("N/A", "N/A"));
            given_name
        } else {
            "N/A"
        }
    }

    fn family_name(&self) -> &str {
        if let Some(family_name) = &self.family_name {
            family_name
        } else if let Some(name) = &self.name {
            let (_, family_name) = name.split_once(' ').unwrap_or(("N/A", "N/A"));
            family_name
        } else {
            "N/A"
        }
    }

    fn self_as_bytes_from_token(token: &str) -> Result<Vec<u8>, ErrorResponse> {
        let mut parts = token.split('.');
        let _header = parts.next().ok_or_else(|| {
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "incorrect ID did not contain claims".to_string(),
            )
        })?;
        let claims = parts.next().ok_or_else(|| {
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "ID token was unsigned".to_string(),
            )
        })?;
        debug!("upstream ID token claims:\n{}", claims);
        let json_bytes = base64_url_no_pad_decode(claims)?;
        Ok(json_bytes)
    }

    async fn validate_update_user(
        &self,
        data: &web::Data<AppState>,
        provider: &AuthProvider,
        link_cookie: &Option<AuthProviderLinkCookie>,
    ) -> Result<(User, ProviderMfaLogin), ErrorResponse> {
        if self.email.is_none() {
            let err = "No `email` in ID token claims. This is a mandatory claim";
            error!("{}", err);
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                err.to_string(),
            ));
        }

        let claims_user_id = if let Some(sub) = &self.sub {
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
        }
        // We need to create a real string here, since we don't know what json type we get.
        // Any json number would become a String too, which is what we need for compatibility.
        .to_string();

        let user_opt = match User::find_by_federation(data, &provider.id, &claims_user_id).await {
            Ok(user) => {
                debug!(
                    "found already existing user by federation lookup: {:?}",
                    user
                );
                Some(user)
            }
            Err(_) => {
                debug!("did not find already existing user by federation lookup - making sure email does not exist");
                // If a federated user with this information does not exist, we will create
                // a new one in the following code, but we should make sure, that the email,
                // which is a key value for Rauthy, does not yet exist for another user.
                // On conflict, the DB would return an error anyway, but the error message is
                // rather cryptic for a normal user.
                if let Ok(mut user) =
                    User::find_by_email(data, self.email.as_ref().unwrap().to_string()).await
                {
                    // TODO check if creating a new link for an existing user is allowed
                    if let Some(link) = link_cookie {
                        if link.provider_id != provider.id {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "bad provider_id in link cookie".to_string(),
                            ));
                        }

                        if link.user_id != user.id {
                            // In this case, the link cookie exists from another user session.
                            // It is possible to build this situation manually with access to
                            // multiple accounts.
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "bad user_id in link cookie".to_string(),
                            ));
                        }

                        // finally, this is our condition we allow linking for existing accs
                        if link.user_email != user.email {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::BadRequest,
                                "Invalid E-Mail".to_string(),
                            ));
                        }

                        // If we got here, everything was fine, and we can create the link.
                        // No need to `.save()` here, will be done later anyway with other updates.
                        user.auth_provider_id = Some(provider.id.clone());
                        user.federation_uid = Some(claims_user_id.clone());

                        Some(user)
                    } else {
                        return Err(ErrorResponse::new(ErrorResponseType::Forbidden, format!(
                            "User with email '{}' already exists but is not linked to this provider.",
                            user.email
                        )));
                    }
                } else {
                    None
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
                    "Misconfigured Auth Provider - admin claim path without value".to_string(),
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
                            format!("\"{}\"", value)
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
                    error!("Error parsing JsonPath from: '{}\nError: {}", path, err);
                }
            }
        }

        // check if mfa has been used by upstream claim
        let mut provider_mfa_login = ProviderMfaLogin::No;
        if let Some(path) = &provider.mfa_claim_path {
            if provider.mfa_claim_value.is_none() {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Internal,
                    "Misconfigured Auth Provider - mfa claim path without value".to_string(),
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
                            format!("\"{}\"", value)
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
                    error!("Error parsing JsonPath from: '{}\nError: {}", path, err);
                }
            }
        }

        let now = OffsetDateTime::now_utc().unix_timestamp();
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
                user.save(data, old_email, None).await?;

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
            if user.family_name.as_str() != family_name {
                user.family_name = family_name.to_string();
            }

            // should this user be a rauthy admin?
            let roles = user.get_roles();
            let roles_str = roles.iter().map(|r| r.as_str()).collect::<Vec<&str>>();

            // We will only re-map the rauthy_admin role if the claim mapping is configured.
            // Otherwise, we would remove an admin role from a user it has been manually added for,
            // which would not be the expected outcome.
            if let Some(should_be_admin) = should_be_rauthy_admin {
                if should_be_admin {
                    if !roles_str.contains(&"rauthy_admin") {
                        let mut new_roles = Vec::with_capacity(roles.len() + 1);
                        new_roles.push("rauthy_admin".to_string());
                        roles.into_iter().for_each(|r| new_roles.push(r));
                        user.roles = new_roles.join(",");
                    }
                } else if roles_str.contains(&"rauthy_admin") {
                    if roles.len() == 1 {
                        user.roles = "".to_string();
                    } else {
                        user.roles = roles.into_iter().filter(|r| r != "rauthy_admin").join(",");
                    }
                }
            }

            // update the user on our side
            user.last_login = Some(now);
            user.last_failed_login = None;
            user.failed_login_attempts = None;

            user.save(data, old_email, None).await?;
            user
        } else {
            // Create a new federated user
            let new_user = User {
                email: self.email.as_ref().unwrap().to_string(),
                given_name: self.given_name().to_string(),
                family_name: self.family_name().to_string(),
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
            User::create_federated(data, new_user).await?
        };

        // check if we got additional values from the token
        let mut found_values = false;
        let mut user_values = match UserValues::find(data, &user.id).await? {
            Some(values) => UserValuesRequest {
                birthdate: values.birthdate,
                phone: values.phone,
                street: values.street,
                zip: values.zip,
                city: values.city,
                country: values.country,
            },
            None => UserValuesRequest {
                birthdate: None,
                phone: None,
                street: None,
                zip: None,
                city: None,
                country: None,
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
                user_values.zip = Some(zip);
            }
            found_values = true;
        }
        if found_values {
            UserValues::upsert(data, user.id.clone(), user_values).await?;
        }

        Ok((user, provider_mfa_login))
    }
}

#[derive(Debug, Deserialize)]
struct AuthProviderTokenSet {
    pub access_token: Option<String>,
    // pub token_type: Option<String>,
    pub id_token: Option<String>,
    pub error: Option<String>,
    pub error_description: Option<String>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use cryptr::EncKeys;

    #[test]
    fn test_auth_provider_link_cookie() {
        dotenvy::from_filename("rauthy.test.cfg").ok();
        let _ = EncKeys::from_env().unwrap().init();

        let value = AuthProviderLinkCookie {
            provider_id: "my_id_1337".to_string(),
            user_id: "batman123".to_string(),
            user_email: "batman@gotham.io".to_string(),
        };

        let cookie = value.build_cookie().unwrap();

        let cookie_val = ApiCookie::cookie_into_value(Some(cookie)).unwrap();
        let res = AuthProviderLinkCookie::try_from(cookie_val.as_str()).unwrap();

        assert_eq!(value.provider_id, res.provider_id);
        assert_eq!(value.user_id, res.user_id);
        assert_eq!(value.user_email, res.user_email);
    }

    // ... just to understand the query syntax
    #[test]
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
        assert!(AuthProviderIdClaims::try_from(claims_bytes.as_bytes()).is_ok());
    }
}
