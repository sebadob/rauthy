use crate::app_state::AppState;
use crate::entity::clients::Client;
use crate::entity::well_known::WellKnown;
use crate::request::{ProviderCallbackRequest, ProviderLoginRequest, ProviderRequest};
use crate::response::ProviderLookupResponse;
use actix_web::cookie::Cookie;
use actix_web::http::header::HeaderValue;
use actix_web::{cookie, web};
use cryptr::utils::secure_random_alnum;
use cryptr::EncValue;
use rauthy_common::constants::{
    CACHE_NAME_12HR, CACHE_NAME_AUTH_PROVIDER_CALLBACK, COOKIE_UPSTREAM_CALLBACK,
    IDX_AUTH_PROVIDER, IDX_AUTH_PROVIDER_TEMPLATE, PROVIDER_CALLBACK_URI, RAUTHY_VERSION,
    UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS,
};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::{base64_decode, base64_encode, base64_url_encode, new_store_id};
use redhac::{cache_del, cache_get, cache_get_from, cache_get_value, cache_insert, AckLevel};
use reqwest::tls;
use ring::digest;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use std::fmt::Write;
use std::net::ToSocketAddrs;
use std::time::Duration;
use tracing::{debug, error, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AuthProviderType {
    OIDC,
    // this can be extended in the future tu support special providers that do not
    // work with the oidc standards
}

impl AuthProviderType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::OIDC => "oidc",
        }
    }
}

impl TryFrom<&str> for AuthProviderType {
    type Error = ErrorResponse;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let slf = match value {
            "oidc" => Self::OIDC,
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Invalid AuthProviderType".to_string(),
                ))
            }
        };
        Ok(slf)
    }
}

impl From<String> for AuthProviderType {
    /// Defaults to Self::OIDC in case of an error
    fn from(value: String) -> Self {
        Self::try_from(value.as_str()).unwrap_or(Self::OIDC)
    }
}

/// Upstream Auth Provider for upstream logins without a local Rauthy account
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProvider {
    pub id: String,
    pub name: String,
    pub typ: AuthProviderType,

    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,

    pub client_id: String,
    pub secret: Option<Vec<u8>>,
    pub scope: String,

    pub allow_insecure_requests: bool,
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
            auth_providers (id, name, typ, issuer, authorization_endpoint, token_endpoint,
            userinfo_endpoint, client_id, secret, scope, allow_insecure_requests,
            use_pkce, root_pem, logo, logo_type)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15)"#,
            slf.id,
            slf.name,
            slf.typ.as_str(),
            slf.issuer,
            slf.authorization_endpoint,
            slf.token_endpoint,
            slf.userinfo_endpoint,
            slf.client_id,
            slf.secret,
            slf.scope,
            slf.allow_insecure_requests,
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
        Self::try_from_id_req(id, payload)?.save(data).await
    }

    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        // TODO when implemented: logo = $12, logo_type = $13
        query!(
            r#"UPDATE auth_providers
            SET name = $1, issuer = $2, typ = $3, authorization_endpoint = $4, token_endpoint = $5,
            userinfo_endpoint = $6, client_id = $7, secret = $8, scope = $9,
            allow_insecure_requests = $10, use_pkce = $11, root_pem = $12
            WHERE id = $13"#,
            self.name,
            self.issuer,
            self.typ.as_str(),
            self.authorization_endpoint,
            self.token_endpoint,
            self.userinfo_endpoint,
            self.client_id,
            self.secret,
            self.scope,
            self.allow_insecure_requests,
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

    fn build_client(
        danger_allow_insecure: bool,
        root_pem: Option<&str>,
    ) -> Result<reqwest::Client, ErrorResponse> {
        let client = if danger_allow_insecure {
            reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .connect_timeout(Duration::from_secs(10))
                .user_agent(format!("Rauthy Auth Provider Client v{}", RAUTHY_VERSION))
                .https_only(false)
                .danger_accept_invalid_certs(true)
                .build()?
        } else {
            let mut builder = reqwest::Client::builder()
                .timeout(Duration::from_secs(10))
                .connect_timeout(Duration::from_secs(10))
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
            // for now, this will always be OIDC
            // preparation for future special providers
            typ: AuthProviderType::OIDC,
            issuer: req.issuer,
            authorization_endpoint: req.authorization_endpoint,
            token_endpoint: req.token_endpoint,
            userinfo_endpoint: req.userinfo_endpoint,
            client_id: req.client_id,
            secret,
            scope,
            allow_insecure_requests: req.danger_allow_insecure.unwrap_or(false),
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
        danger_allow_insecure: bool,
        root_pem: Option<&str>,
    ) -> Result<ProviderLookupResponse, ErrorResponse> {
        let client = Self::build_client(danger_allow_insecure, root_pem)?;

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

/// Will be created to start a new upstream login and afterward validate a callback.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthProviderCallback {
    pub callback_id: String,
    pub xsrf_token: String,
    pub typ: AuthProviderType,

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
    pub allow_insecure_requests: bool,
    pub use_pkce: bool,
    pub root_pem: Option<String>,
    // TODO add a nonce upstream as well? -> improvement?
    pub pkce_challenge: String,
}

// CRUD
impl AuthProviderCallback {
    pub async fn delete(
        data: &web::Data<AppState>,
        callback_id: String,
    ) -> Result<(), ErrorResponse> {
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
            typ: provider.typ,

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
            allow_insecure_requests: provider.allow_insecure_requests,
            use_pkce: provider.use_pkce,
            root_pem: provider.root_pem,

            pkce_challenge: payload.pkce_challenge,
        };

        let mut location = format!(
            "{}?client_id={}&redirect_uri={}&response_type=code&scope={}&state={}",
            provider.authorization_endpoint,
            slf.provider_client_id,
            *PROVIDER_CALLBACK_URI,
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

    /// In case of any error, the callback code will be fully deleted for security reasons.
    pub async fn login_finish(
        data: &web::Data<AppState>,
        cookie: Cookie<'_>,
        payload: &ProviderCallbackRequest,
    ) -> Result<Self, ErrorResponse> {
        // the callback id for the cache should be inside the encrypted cookie
        let bytes = base64_decode(cookie.value())?;
        let plain = EncValue::try_from(bytes)?.decrypt()?;
        let callback_id = String::from_utf8_lossy(plain.as_ref()).to_string();

        // validate state
        if callback_id != payload.state {
            Self::delete(data, callback_id).await?;

            error!("`state` does not match");
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "`state` does not match".to_string(),
            ));
        }

        // validate csrf token
        let slf = Self::find(data, callback_id).await?;
        if slf.xsrf_token != payload.xsrf_token {
            Self::delete(data, slf.callback_id).await?;

            error!("invalid CSRF token");
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "invalid CSRF token".to_string(),
            ));
        }

        // validate PKCE verifier
        let hash = digest::digest(&digest::SHA256, payload.pkce_verifier.as_bytes());
        let hash_base64 = base64_url_encode(hash.as_ref());
        if slf.pkce_challenge != hash_base64 {
            Self::delete(data, slf.callback_id).await?;

            error!("invalid PKCE verifier");
            return Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "invalid PKCE verifier".to_string(),
            ));
        }

        // request is valid -> fetch token for the user
        let client =
            AuthProvider::build_client(slf.allow_insecure_requests, slf.root_pem.as_deref())?;
        let res = client
            .get(slf.provider_token_endpoint)
            .basic_auth(
                &slf.provider_client_id,
                AuthProvider::get_secret_cleartext(&slf.provider_secret)?,
            )
            .json(&OidcCodeRequestParams {
                client_id: &slf.provider_client_id,
                client_secret: AuthProvider::get_secret_cleartext(&slf.provider_secret)?,
                code: slf.use_pkce.then_some(&payload.code),
                code_verifier: slf.use_pkce.then_some(&payload.pkce_verifier),
                grant_type: "authorization_code",
                redirect_uri: &*PROVIDER_CALLBACK_URI,
            })
            .send()
            .await?;

        if !res.status().is_success() {
            let status = res.status().as_u16();
            let err = match res.text().await {
                Ok(body) => format!(
                    "HTTP {} during GET /token for upstream auth provider '{}':\n{}",
                    status, slf.provider_client_id, body
                ),
                Err(_) => format!(
                    "HTTP {} during GET /token for upstream auth provider '{}' without any body",
                    status, slf.provider_client_id
                ),
            };
            error!("{}", err);
            return Err(ErrorResponse::new(ErrorResponseType::Internal, err));
        }

        match res.json::<AuthProviderTokenSet>().await {
            Ok(ts) => {
                // At this point, we should have all the tokens.
                // We cannot do any strict validation or deserialization, because upstream
                // providers differ a lot.
                // That's why we will try it via a dynamic approach. This it of course slower
                // than strict deserialization, but it is more versatile and will provide
                // better compatibility with a broader ranger of providers.

                warn!("\n\n{:?}\n", ts);
                todo!();

                // // validate access token
                // let _access_claims =
                //     JwtAccessClaims::from_token_validated(&ts.access_token).await?;
                //
                // // validate id token
                // if ts.id_token.is_none() {
                //     return Err(anyhow::Error::msg("ID token is missing"));
                // }
                // let id_claims = JwtIdClaims::from_token_validated(
                //     ts.id_token.as_deref().unwrap(),
                //     &cookie_state.nonce,
                // )
                // .await?;
                //
                // // reset STATE_COOKIE
                // let cookie = build_lax_cookie_300(
                //     OIDC_STATE_COOKIE,
                //     "",
                //     insecure == OidcCookieInsecure::Yes,
                // );
            }
            Err(err) => {
                let err = format!(
                    "Deserializing /token response from auth provider {}: {}",
                    slf.provider_client_id, err
                );
                error!("{}", err);
                return Err(ErrorResponse::new(ErrorResponseType::Internal, err));
            }
        }

        // let res = client.get(&slf.provider_token_endpoint)
        //         .basic_auth(slf.provider_client_id, slf.provider_secret)
        //     .json()

        todo!();

        // check if user exists locally
        // - if yes, compare values and make sure it is already federated, block otherwise
        // - if not, create a new user

        // check mfa force for the client + user setup and return data accordingly

        // create delete cookie
        // let cookie = cookie::Cookie::build(COOKIE_UPSTREAM_CALLBACK, "")
        //     .secure(true)
        //     .http_only(true)
        //     .same_site(cookie::SameSite::Lax)
        //     .max_age(cookie::time::Duration::ZERO)
        //     .path("/auth")
        //     .finish();

        todo!()
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

#[derive(Debug, Serialize)]
struct OidcCodeRequestParams<'a> {
    client_id: &'a str,
    client_secret: Option<String>,
    code: Option<&'a str>,
    code_verifier: Option<&'a str>,
    grant_type: &'static str,
    redirect_uri: &'a str,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthProviderTokenSet {
    pub access_token: String,
    pub token_type: Option<String>,
    pub id_token: Option<String>,
    pub expires_in: i32,
    pub refresh_token: Option<String>,
}
