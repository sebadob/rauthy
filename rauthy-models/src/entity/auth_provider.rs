use crate::app_state::AppState;
use crate::entity::well_known::WellKnown;
use crate::request::ProviderRequest;
use crate::response::ProviderLookupResponse;
use actix_web::web;
use cryptr::EncValue;
use rauthy_common::constants::{CACHE_NAME_12HR, IDX_AUTH_PROVIDER, RAUTHY_VERSION};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::new_store_id;
use redhac::{cache_del, cache_get, cache_get_from, cache_get_value, cache_insert, AckLevel};
use reqwest::tls;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use std::time::Duration;
use tracing::debug;

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
        let scope = Self::cleanup_scope(&payload.scope);
        let secret = Self::secret_encrypted(&payload.client_secret)?;

        let slf = Self {
            id,
            name: payload.name,
            issuer: payload.issuer,
            authorization_endpoint: payload.authorization_endpoint,
            token_endpoint: payload.token_endpoint,
            userinfo_endpoint: payload.userinfo_endpoint,
            client_id: payload.client_id,
            secret,
            scope,
            token_auth_method_basic: payload.token_auth_method_basic,
            use_pkce: payload.use_pkce,
            root_pem: payload.root_pem,

            // TODO when implemented
            logo: None,
            logo_type: None,
        };

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

        Self::invalidate_cache(data).await?;

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

        Self::invalidate_cache(data).await?;

        Ok(())
    }

    pub async fn update(
        data: &web::Data<AppState>,
        id: String,
        payload: ProviderRequest,
    ) -> Result<(), ErrorResponse> {
        let scope = Self::cleanup_scope(&payload.scope);
        let secret = Self::secret_encrypted(&payload.client_secret)?;

        // TODO when implemented: logo = $12, logo_type = $13
        query!(
            r#"UPDATE auth_providers
            SET name = $1, issuer = $2, authorization_endpoint = $3, token_endpoint = $4,
            userinfo_endpoint = $5, client_id = $6, secret = $7, scope = $8,
            token_auth_method_basic = $9, use_pkce = $10, root_pem = $11
            WHERE id = $12"#,
            payload.name,
            payload.issuer,
            payload.authorization_endpoint,
            payload.token_endpoint,
            payload.userinfo_endpoint,
            payload.client_id,
            secret,
            scope,
            payload.token_auth_method_basic,
            payload.use_pkce,
            payload.root_pem,
            // payload.logo,
            // payload.logo_type,
            id,
        )
        .execute(&data.db)
        .await?;

        // just invalidating the cache instead of updating it manually is fine
        // because providers are not updated often
        Self::invalidate_cache(data).await?;

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

    async fn invalidate_cache(data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        cache_del(
            CACHE_NAME_12HR.to_string(),
            Self::cache_idx("all"),
            &data.caches.ha_cache_config,
        )
        .await?;
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
