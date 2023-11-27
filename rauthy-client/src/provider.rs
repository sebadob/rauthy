use crate::jwks::JwksMsg;
use crate::oidc_config::{JwtClaim, RauthyConfig};
use crate::{DangerAcceptInvalidCerts, RauthyHttpsOnly, VERSION};
use jwt_simple::common::VerificationOptions;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::OnceLock;
use std::time::Duration;

pub(crate) static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();
static OIDC_CONFIG: OnceLock<OidcProviderConfig> = OnceLock::new();

#[derive(Debug)]
pub(crate) struct OidcProviderConfig {
    pub auth_url_base: String,
    pub client_id: String,
    pub email_verified: bool,
    pub provider: OidcProvider,
    pub redirect_uri: String,
    pub secret: Option<String>,
    pub admin_claim: Option<JwtClaim>,
    pub user_claim: JwtClaim,
    pub verification_options: VerificationOptions,
}

impl OidcProviderConfig {
    #[allow(clippy::too_many_arguments)]
    async fn build_from_values(
        redirect_uri: String,
        iss: String,
        scope: String,
        client_id: String,
        allowed_audiences: HashSet<String>,
        email_verified: bool,
        secret: Option<String>,
        admin_claim: Option<JwtClaim>,
        user_claim: JwtClaim,
    ) -> anyhow::Result<Self> {
        let append = if iss.ends_with('/') {
            ".well-known/openid-configuration"
        } else {
            "/.well-known/openid-configuration"
        };
        let oidc_config_url = format!("{}{}", iss, append);
        let provider = OidcProvider::fetch(&oidc_config_url).await?;
        // update JWKS handler
        JwksMsg::NewJwksUri(provider.jwks_uri.clone()).send()?;

        let auth_endpoint = &provider.authorization_endpoint;
        let auth_url_base = format!(
            "{auth_endpoint}?client_id={client_id}&redirect_uri={redirect_uri}&\
            response_type=code&code_challenge_method=S256&scope={scope}"
        );

        let verification_options = VerificationOptions {
            allowed_audiences: Some(allowed_audiences),
            allowed_issuers: Some(HashSet::from([iss])),
            ..Default::default()
        };

        Ok(Self {
            auth_url_base,
            client_id,
            email_verified,
            provider,
            redirect_uri,
            secret,
            admin_claim,
            user_claim,
            verification_options,
        })
    }
}

/// The configured Rauthy OIDC Provider
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize)]
pub struct OidcProvider {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub introspection_endpoint: String,
    pub userinfo_endpoint: String,
    pub end_session_endpoint: String,
    pub jwks_uri: String,
    // pub registration_endpoint: String,
    // pub check_session_iframe: String,
    pub grant_types_supported: Vec<Flows>,
    pub response_types_supported: Vec<String>,
    pub id_token_signing_alg_values_supported: Vec<Algorithm>,
    pub token_endpoint_auth_signing_alg_values_supported: Vec<Algorithm>,
    pub claims_supported: Vec<String>,
    pub scopes_supported: Vec<String>,
    pub code_challenge_methods_supported: Vec<Challenge>,
}

impl OidcProvider {
    #[inline]
    pub(crate) fn config<'a>() -> anyhow::Result<&'a OidcProviderConfig> {
        match OIDC_CONFIG.get() {
            None => Err(anyhow::Error::msg("OidcProvider has not been initialized")),
            Some(c) => Ok(c),
        }
    }

    async fn fetch(oidc_config_endpoint: &str) -> anyhow::Result<Self> {
        let slf = Self::client()
            .get(oidc_config_endpoint)
            .send()
            .await?
            .json::<Self>()
            .await?;

        Ok(slf)
    }

    pub async fn setup_from_config(config: RauthyConfig, redirect_uri: &str) -> anyhow::Result<()> {
        let callback = redirect_uri.replace(':', "%3A").replace('/', "%2F");
        let scope = config.scope.join("+");
        let config = OidcProviderConfig::build_from_values(
            callback,
            config.iss,
            scope,
            config.client_id,
            config.allowed_audiences,
            config.email_verified,
            config.secret,
            config.admin_claim,
            config.user_claim,
        )
        .await?;

        OIDC_CONFIG.set(config).map_err(|_| {
            anyhow::Error::msg("OidcProvider::setup_from_config must only be called once")
        })?;

        Ok(())
    }

    // TODO add setup_from_env fn

    pub fn init_client(
        root_certificate: Option<reqwest::Certificate>,
        https_only: RauthyHttpsOnly,
        danger_accept_invalid_certs: DangerAcceptInvalidCerts,
    ) -> anyhow::Result<()> {
        let mut c = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .connect_timeout(Duration::from_secs(10))
            .https_only(https_only.bool())
            .danger_accept_invalid_certs(danger_accept_invalid_certs.bool())
            .user_agent(format!("Rauthy OIDC Client v{}", VERSION))
            .brotli(true)
            .http2_prior_knowledge()
            .timeout(Duration::from_secs(10));

        if let Some(root) = root_certificate {
            c = c.add_root_certificate(root);
        }

        HTTP_CLIENT.set(c.build().unwrap()).map_err(|_| {
            anyhow::Error::msg("OidcProvider::init_client must only be called once")
        })?;

        Ok(())
    }

    #[inline]
    pub(crate) fn client<'a>() -> &'a reqwest::Client {
        HTTP_CLIENT.get().expect(
            "OIDC Client has not been initialized - run OidcProvider::init_client() at startup",
        )
    }
}

/// Rauthy-supported token algorithms
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Algorithm {
    RS256,
    RS384,
    RS512,
    EdDSA,
}

/// Rauthy-supported PKCE challenges
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Challenge {
    plain,
    S256,
}

/// OIDC login flows
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Flows {
    authorization_code,
    client_credentials,
    password,
    refresh_token,
}
