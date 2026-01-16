use crate::common::{get_backend_url, get_issuer};
use pretty_assertions::assert_eq;
use serde::Deserialize;
use std::error::Error;

mod common;

#[tokio::test]
async fn test_get_ping() -> Result<(), Box<dyn Error>> {
    let url = format!("{}/ping", get_backend_url());
    let res = reqwest::get(&url).await?;
    assert_eq!(res.status(), 200);
    Ok(())
}

// Re-defined and copy & pasted here to be able to get rid of lots of
// memory allocations in prod, because `Deserialize` will not work with
// `'static` lifetimes.
#[derive(Debug, Deserialize)]
pub struct WellKnown {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub backchannel_logout_supported: bool,
    pub backchannel_logout_session_supported: bool,
    pub device_authorization_endpoint: String,
    pub token_endpoint: String,
    pub introspection_endpoint: String,
    pub userinfo_endpoint: String,
    pub end_session_endpoint: String,
    pub registration_endpoint: Option<String>,
    pub jwks_uri: String,
    pub grant_types_supported: Vec<String>,
    pub response_types_supported: Vec<String>,
    pub subject_types_supported: Vec<String>,
    pub id_token_signing_alg_values_supported: Vec<String>,
    pub token_endpoint_auth_methods_supported: Vec<String>,
    pub token_endpoint_auth_signing_alg_values_supported: Vec<String>,
    pub claims_supported: Vec<String>,
    pub claim_types_supported: Vec<String>,
    pub scopes_supported: Vec<String>,
    pub code_challenge_methods_supported: Vec<String>,
    pub dpop_signing_alg_values_supported: Vec<String>,
    pub service_documentation: String,
    pub ui_locales_supported: Vec<String>,
    pub claims_parameter_supported: bool,
}

#[tokio::test]
async fn test_get_well_known() -> Result<(), Box<dyn Error>> {
    let url = format!("{}/.well-known/openid-configuration", get_backend_url());
    let res = reqwest::get(&url).await?;

    assert_eq!(res.status(), 200);
    let content = res.json::<WellKnown>().await?;
    assert_eq!(content.issuer, get_issuer());
    // don't test the rest for now as it might change soon again

    Ok(())
}
