use crate::app_state::AppState;
use crate::entity::clients::Client;
use crate::entity::well_known::WellKnown;
use crate::request::{ProviderLoginRequest, ProviderRequest};
use crate::response::ProviderLookupResponse;
use actix_web::cookie::Cookie;
use actix_web::http::header::HeaderValue;
use actix_web::{cookie, web};
use cryptr::utils::secure_random_alnum;
use cryptr::EncValue;
use rauthy_common::constants::{
    CACHE_NAME_12HR, CACHE_NAME_AUTH_PROVIDER_CALLBACK, COOKIE_UPSTREAM_CALLBACK,
    IDX_AUTH_PROVIDER, IDX_AUTH_PROVIDER_TEMPLATE, PUB_URL_WITH_SCHEME_ENCODED, RAUTHY_VERSION,
    UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS,
};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::{base64_encode, new_store_id};
use redhac::{cache_del, cache_get, cache_get_from, cache_get_value, cache_insert, AckLevel};
use reqwest::tls;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use std::fmt::Write;
use std::time::Duration;
use tracing::debug;

/// Upstream Auth Provider for upstream logins without a local Rauthy account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProvider {
    pub id: String,
    pub name: String,

    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,

    pub client_id: String,
    pub secret: Option<Vec<u8>>,
    pub scope: String,

    pub token_auth_method_basic: bool,
    pub use_pkce: bool,

    pub root_pem: Option<String>,
    pub logo: Option<Vec<u8>>,
    pub logo_type: Option<String>,
}

impl AuthProvider {
    pub async fn create(
        data: &web::Data<AppState>,
        payload: ProviderRequest,
    ) -> Result<Self, ErrorResponse> {
        let id = new_store_id();
        // let scope = Self::cleanup_scope(&payload.scope);
        // let secret = Self::secret_encrypted(&payload.client_secret)?;
        //
        // let slf = Self {
        //     id,
        //     name: payload.name,
        //     issuer: payload.issuer,
        //     authorization_endpoint: payload.authorization_endpoint,
        //     token_endpoint: payload.token_endpoint,
        //     userinfo_endpoint: payload.userinfo_endpoint,
        //     client_id: payload.client_id,
        //     secret,
        //     scope,
        //     token_auth_method_basic: payload.token_auth_method_basic,
        //     use_pkce: payload.use_pkce,
        //     root_pem: payload.root_pem,
        //
        //     // TODO when implemented
        //     logo: None,
        //     logo_type: None,
        // };
        let slf = Self::try_from_id_req(id, payload)?;

        query!(
            r#"
            INSERT INTO
            auth_providers (id, name, issuer, authorization_endpoint, token_endpoint, userinfo_endpoint,
            client_id, secret, scope, token_auth_method_basic, use_pkce, root_pem, logo, logo_type)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)"#,
            slf.id,
            slf.name,
            slf.issuer,
            slf.authorization_endpoint,
            slf.token_endpoint,
            slf.userinfo_endpoint,
            slf.client_id,
            slf.secret,
            slf.scope,
            slf.token_auth_method_basic,
            slf.use_pkce,
            slf.root_pem,
            slf.logo,
            slf.logo_type,
        )
        .execute(&data.db)
        .await?;

        cache_insert(
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx(&slf.id),
            &data.caches.ha_cache_config,
            &slf,
            AckLevel::Quorum,
        )
        .await?;

        Self::invalidate_cache_all(data).await?;

        Ok(slf)
    }

    pub async fn find(data: &web::Data<AppState>, id: &str) -> Result<Self, ErrorResponse> {
        if let Some(res) = cache_get!(
            Self,
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx(&id),
            &data.caches.ha_cache_config,
            false
        )
        .await?
        {
            return Ok(res);
        }

        let slf = query_as!(Self, "SELECT * FROM auth_providers WHERE id = $1", id)
            .fetch_one(&data.db)
            .await?;

        cache_insert(
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx(id),
            &data.caches.ha_cache_config,
            &slf,
            AckLevel::Quorum,
        )
        .await?;

        Ok(slf)
    }

    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        if let Some(res) = cache_get!(
            Vec<Self>,
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx("all"),
            &data.caches.ha_cache_config,
            false
        )
        .await?
        {
            return Ok(res);
        }

        let res = query_as!(Self, "SELECT * FROM auth_providers")
            .fetch_all(&data.db)
            .await?;

        // needed for rendering each single login page -> always cache this
        cache_insert(
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx("all"),
            &data.caches.ha_cache_config,
            &res,
            AckLevel::Quorum,
        )
        .await?;

        Ok(res)
    }

    pub async fn delete(data: &web::Data<AppState>, id: &str) -> Result<(), ErrorResponse> {
        query!("DELETE FROM auth_providers WHERE id = $1", id)
            .execute(&data.db)
            .await?;

        Self::invalidate_cache_all(data).await?;
        cache_del(
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx(id),
            &data.caches.ha_cache_config,
        )
        .await?;

        Ok(())
    }

    pub async fn update(
        data: &web::Data<AppState>,
        id: String,
        payload: ProviderRequest,
    ) -> Result<(), ErrorResponse> {
        // let scope = Self::cleanup_scope(&payload.scope);
        // let secret = Self::secret_encrypted(&payload.client_secret)?;
        //
        // Self {
        //     id,
        //     name: payload.name,
        //     issuer: payload.issuer,
        //     authorization_endpoint: payload.authorization_endpoint,
        //     token_endpoint: payload.token_endpoint,
        //     userinfo_endpoint: payload.userinfo_endpoint,
        //     client_id: payload.client_id,
        //     secret,
        //     scope,
        //     token_auth_method_basic: payload.token_auth_method_basic,
        //     use_pkce: payload.use_pkce,
        //     root_pem: payload.root_pem,
        //
        //     logo: None,
        //     logo_type: None,
        //     // TODO when implemented in the UI
        //     // logo: payload.,
        //     // logo_type: payload.use_pkce,
        // }
        // .save(data)
        // .await

        Self::try_from_id_req(id, payload)?.save(data).await
    }

    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        // TODO when implemented: logo = $12, logo_type = $13
        query!(
            r#"UPDATE auth_providers
            SET name = $1, issuer = $2, authorization_endpoint = $3, token_endpoint = $4,
            userinfo_endpoint = $5, client_id = $6, secret = $7, scope = $8,
            token_auth_method_basic = $9, use_pkce = $10, root_pem = $11
            WHERE id = $12"#,
            self.name,
            self.issuer,
            self.authorization_endpoint,
            self.token_endpoint,
            self.userinfo_endpoint,
            self.client_id,
            self.secret,
            self.scope,
            self.token_auth_method_basic,
            self.use_pkce,
            self.root_pem,
            // self.logo,
            // self.logo_type,
            self.id,
        )
        .execute(&data.db)
        .await?;

        Self::invalidate_cache_all(data).await?;

        cache_insert(
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx(&self.id),
            &data.caches.ha_cache_config,
            &self,
            AckLevel::Quorum,
        )
        .await?;

        Ok(())
    }
}

impl AuthProvider {
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

    fn try_from_id_req(id: String, req: ProviderRequest) -> Result<Self, ErrorResponse> {
        let scope = Self::cleanup_scope(&req.scope);
        let secret = Self::secret_encrypted(&req.client_secret)?;

        Ok(Self {
            id,
            name: req.name,
            issuer: req.issuer,
            authorization_endpoint: req.authorization_endpoint,
            token_endpoint: req.token_endpoint,
            userinfo_endpoint: req.userinfo_endpoint,
            client_id: req.client_id,
            secret,
            scope,
            token_auth_method_basic: req.token_auth_method_basic,
            use_pkce: req.use_pkce,
            root_pem: req.root_pem,

            logo: None,
            logo_type: None,
            // TODO when implemented in the UI
            // logo: payload.,
            // logo_type: payload.use_pkce,
        })
    }

    async fn invalidate_cache_all(data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        cache_del(
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx("all"),
            &data.caches.ha_cache_config,
        )
        .await?;
        AuthProviderTemplate::invalidate_cache(data).await?;
        Ok(())
    }

    pub async fn lookup_config(
        issuer: &str,
        danger_allow_http: bool,
        danger_allow_insecure: bool,
    ) -> Result<ProviderLookupResponse, ErrorResponse> {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(10))
            .min_tls_version(tls::Version::TLS_1_3)
            .tls_built_in_root_certs(true)
            .user_agent(format!("Rauthy Auth Provider Client v{}", RAUTHY_VERSION))
            // if necessary, make insecure connections work for local dev, testing, internal networks, ...
            .https_only(!danger_allow_http)
            .danger_accept_invalid_certs(danger_allow_insecure)
            .build()?;

        let issuer_url = if issuer.starts_with("http://") || issuer.starts_with("https://") {
            issuer.to_string()
        } else {
            // we always assume https connections, if the scheme is not given
            format!("https://{}", issuer)
        };
        // for now, we only support the default openid-configuration location
        let config_url = if issuer_url.ends_with('/') {
            format!("{}.well-known/openid-configuration", issuer_url)
        } else {
            format!("{}/.well-known/openid-configuration", issuer_url)
        };

        debug!("AuthProvider lookup to {}", config_url);
        let res = client.get(&config_url).send().await?;
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

        let well_known = res.json::<WellKnown>().await.map_err(|err| {
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
            token_auth_method_basic: !well_known
                .token_endpoint_auth_methods_supported
                .iter()
                .any(|m| m == "client_secret_post"),
            use_pkce: well_known
                .code_challenge_methods_supported
                .iter()
                .any(|c| c == "S256"),
            danger_allow_http,
            danger_allow_insecure,
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

    pub fn get_secret_cleartext(&self) -> Result<Option<String>, ErrorResponse> {
        if let Some(secret) = &self.secret {
            let bytes = EncValue::try_from(secret.clone())?.decrypt()?;
            let cleartext = String::from_utf8_lossy(bytes.as_ref()).to_string();
            Ok(Some(cleartext))
        } else {
            Ok(None)
        }
    }
}

/// Will be created to start a new upstream login and afterward validate a callback.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProviderCallback {
    pub callback_id: String,
    pub xsrf_token: String,

    pub client_id: String,
    pub client_scope: Vec<String>,
    pub client_force_mfa: bool,
    pub client_redirect_uri: String,
    pub client_state: Option<String>,
    pub client_nonce: Option<String>,
    pub client_code_challenge: Option<String>,
    pub client_code_challenge_method: Option<String>,

    pub provider_issuer: String,
    pub provider_token_endpoint: String,
    // pub userinfo_endpoint: String,
    pub provider_client_id: String,
    pub provider_secret: Option<Vec<u8>>,
    pub token_auth_method_basic: bool,
    pub use_pkce: bool,
    pub root_pem: Option<String>,
    // TODO add a nonce upstream as well? -> improvement?
    pub pkce_challenge: String,
}

impl AuthProviderCallback {
    /// returns (encrypted cookie, xsrf token, location header, optional allowed origins)
    pub async fn login_start(
        data: &web::Data<AppState>,
        payload: ProviderLoginRequest,
    ) -> Result<(Cookie, String, HeaderValue, Option<Vec<String>>), ErrorResponse> {
        let provider = AuthProvider::find(data, &payload.provider_id).await?;

        let client = Client::find(data, payload.client_id).await?;
        let client_scope = client.sanitize_login_scopes(&payload.scopes)?;
        let allowed_origins = client.get_allowed_origins();

        let slf = Self {
            callback_id: secure_random_alnum(32),
            xsrf_token: secure_random_alnum(32),

            client_id: client.id,
            client_scope,
            client_force_mfa: client.force_mfa,
            client_redirect_uri: payload.redirect_uri,
            client_state: payload.state,
            client_nonce: payload.nonce,
            client_code_challenge: payload.code_challenge,
            client_code_challenge_method: payload.code_challenge_method,

            provider_issuer: provider.issuer,
            provider_token_endpoint: provider.token_endpoint,
            provider_client_id: provider.client_id,
            provider_secret: provider.secret,
            token_auth_method_basic: provider.token_auth_method_basic,
            use_pkce: provider.use_pkce,
            root_pem: provider.root_pem,

            pkce_challenge: payload.pkce_challenge,
        };

        let callback_uri = format!(
            "{}%2Fauth%2Fv1%2Fproviders%2Fcallback",
            *PUB_URL_WITH_SCHEME_ENCODED
        );
        let mut location = format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            provider.authorization_endpoint,
            slf.provider_client_id,
            callback_uri,
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

        let id_enc = EncValue::encrypt(slf.callback_id.as_bytes())?;
        let id_b64 = base64_encode(id_enc.into_bytes().as_ref());
        let cookie = cookie::Cookie::build(COOKIE_UPSTREAM_CALLBACK, id_b64)
            .secure(true)
            .http_only(true)
            .same_site(cookie::SameSite::Lax)
            .max_age(cookie::time::Duration::seconds(
                UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS as i64,
            ))
            .path("/auth")
            .finish();

        slf.save(data).await?;

        Ok((
            cookie,
            slf.xsrf_token,
            HeaderValue::from_str(&location).expect("Location HeaderValue to be correct"),
            allowed_origins,
        ))
    }

    pub async fn login_finish(
        _web: &web::Data<AppState>,
        _cookie: &str,
        _xsrf_token: &str,
        _pkce_verifier: String,
    ) -> Result<Self, ErrorResponse> {
        // validate values for Self

        // fetch token for the user

        // check if user exists locally
        // - if yes, compare values and make sure it is already federated, block otherwise
        // - if not, create a new user

        // check mfa force for the client + user setup and return data accordingly

        // create delete cookie
        let cookie = cookie::Cookie::build(COOKIE_UPSTREAM_CALLBACK, "")
            .secure(true)
            .http_only(true)
            .same_site(cookie::SameSite::Lax)
            .max_age(cookie::time::Duration::ZERO)
            .path("/auth")
            .finish();

        todo!()
    }

    async fn delete(data: &web::Data<AppState>, callback_id: String) -> Result<(), ErrorResponse> {
        cache_del(
            CACHE_NAME_AUTH_PROVIDER_CALLBACK.to_string(),
            callback_id,
            &data.caches.ha_cache_config,
        )
        .await?;
        Ok(())
    }

    async fn find(data: &web::Data<AppState>, callback_id: String) -> Result<Self, ErrorResponse> {
        match cache_get!(
            Self,
            CACHE_NAME_AUTH_PROVIDER_CALLBACK.to_string(),
            callback_id,
            &data.caches.ha_cache_config,
            true
        )
        .await?
        {
            None => Err(ErrorResponse::new(
                ErrorResponseType::NotFound,
                "Callback Code not found - timeout reached?".to_string(),
            )),
            Some(slf) => Ok(slf),
        }
    }

    async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        cache_insert(
            CACHE_NAME_AUTH_PROVIDER_CALLBACK.to_string(),
            self.callback_id.clone(),
            &data.caches.ha_cache_config,
            &self,
            // Once is good enough and faster -> random ID each time cannot produce conflicts
            AckLevel::Once,
        )
        .await?;
        Ok(())
    }
}

/// Auth Provider as template value for SSR of the Login page
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProviderTemplate {
    pub id: String,
    pub name: String,
    pub use_pkce: bool,
    // pub logo: Option<Vec<u8>>,
    // pub logo_type: Option<String>,
}

impl AuthProviderTemplate {
    async fn invalidate_cache(data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        cache_del(
            CACHE_NAME_12HR.to_string(),
            IDX_AUTH_PROVIDER_TEMPLATE.to_string(),
            &data.caches.ha_cache_config,
        )
        .await?;
        Ok(())
    }

    pub async fn get_all_json_template(
        data: &web::Data<AppState>,
    ) -> Result<Option<String>, ErrorResponse> {
        if let Some(res) = cache_get!(
            Option<String>,
            CACHE_NAME_12HR.to_string(),
            IDX_AUTH_PROVIDER_TEMPLATE.to_string(),
            &data.caches.ha_cache_config,
            false
        )
        .await?
        {
            return Ok(res);
        }

        let providers = AuthProvider::find_all(data)
            .await?
            .into_iter()
            .map(|p| Self {
                id: p.id,
                name: p.name,
                use_pkce: p.use_pkce,
            })
            .collect::<Vec<Self>>();

        let json = if providers.is_empty() {
            None
        } else {
            Some(serde_json::to_string(&providers)?)
        };
        cache_insert(
            CACHE_NAME_12HR.to_string(),
            IDX_AUTH_PROVIDER_TEMPLATE.to_string(),
            &data.caches.ha_cache_config,
            &json,
            AckLevel::Quorum,
        )
        .await?;

        Ok(json)
    }
}
