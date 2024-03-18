use crate::entity::well_known::WellKnown;
use crate::response::ProviderLookupResponse;
use rauthy_common::constants::RAUTHY_VERSION;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use reqwest::tls;
use serde::{Deserialize, Serialize};
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
    pub secret: Option<String>,
    pub scope: Vec<String>,

    pub token_auth_method_basic: bool,
    pub use_pkce: bool,

    pub root_pem: Option<String>,
    pub logo: Option<Vec<u8>>,
    pub logo_type: Option<String>,
}

impl AuthProvider {
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

        Ok(ProviderLookupResponse {
            issuer: well_known.issuer,
            // TODO optimization (and possibly security enhancement): strip issuer url from all of these?
            // what does the RFC mention about it? MUST they always be on the same sub path?
            authorization_endpoint: well_known.authorization_endpoint,
            token_endpoint: well_known.token_endpoint,
            userinfo_endpoint: well_known.userinfo_endpoint,
            use_pkce: well_known
                .code_challenge_methods_supported
                .iter()
                .any(|c| c == "S256"),
            danger_allow_http,
            danger_allow_insecure,
            // TODO add `scopes_supported` Vec and make them selectable with checkboxes in the UI
            // instead of typing them in?
        })
    }
}
