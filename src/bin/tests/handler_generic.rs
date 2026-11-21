use crate::common::{get_auth_headers, get_backend_url, get_issuer};
use pretty_assertions::assert_eq;
use serde::Deserialize;
use serde_json::Value;
use std::error::Error;

mod common;

#[tokio::test]
async fn test_get_ping() -> Result<(), Box<dyn Error>> {
    let url = format!("{}/ping", get_backend_url());
    let res = reqwest::get(&url).await?;
    expect_status!(res, 200);
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
    pub authorization_response_iss_parameter_supported: bool,
    pub client_id_metadata_document_supported: bool,
}

#[tokio::test]
async fn test_get_well_known() -> Result<(), Box<dyn Error>> {
    let url = format!("{}/.well-known/openid-configuration", get_backend_url());
    let res = reqwest::get(&url).await?;

    let content = expect_status_json!(res, 200, WellKnown);
    // strip trailing /
    assert_eq!(content.issuer[..content.issuer.len() - 1], get_issuer());
    // RFC 9207
    assert!(content.authorization_response_iss_parameter_supported);
    // public and ephemeral (CIMD) clients authenticate with PKCE only
    assert!(
        content
            .token_endpoint_auth_methods_supported
            .iter()
            .any(|m| m == "none")
    );
    // don't test the rest for now as it might change soon again

    Ok(())
}

// Regression for issue #1643: because the issuer carries a path
// (`.../auth/v1/`), RFC 8414 §3.1 mandates the AS-metadata URL be formed by
// INSERTING `/.well-known/oauth-authorization-server` between host and path.
// claude.ai probes exactly this path-insertion form; it must return the same
// JSON well-known document (not the SPA fallback) so the client can read
// `client_id_metadata_document_supported`.
#[tokio::test]
async fn test_get_well_known_oauth_rfc8414() -> Result<(), Box<dyn Error>> {
    let backend = get_backend_url();
    let root = backend.strip_suffix("/auth/v1").unwrap_or(&backend);

    // Both the no-slash form (probed by claude.ai) and the trailing-slash form
    // (issuer path is `/auth/v1/`) must return the well-known document, not the SPA.
    for url in [
        format!("{root}/.well-known/oauth-authorization-server/auth/v1"),
        format!("{root}/.well-known/oauth-authorization-server/auth/v1/"),
    ] {
        let res = reqwest::get(&url).await?;

        let res = expect_status!(res, 200, "unexpected status for {url}");
        assert_eq!(
            res.headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok()),
            Some("application/json"),
            "unexpected content-type for {url}",
        );
        let content = res.json::<WellKnown>().await?;
        // strip trailing /
        assert_eq!(content.issuer[..content.issuer.len() - 1], get_issuer());
        assert!(content.authorization_response_iss_parameter_supported);
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct BackupListing {
    name: String,
    size: Option<u64>,
}

#[derive(Debug, Deserialize)]
struct BackupListings {
    local: Vec<BackupListing>,
}

/// A local backup download must declare its length and deliver all of it.
#[tokio::test]
async fn test_backup_download_is_complete() -> Result<(), Box<dyn Error>> {
    if std::env::var("HIQLITE").as_deref() == Ok("false") {
        // backups only exist for Hiqlite
        return Ok(());
    }

    let (headers, _) = common::session_headers().await;
    let client = reqwest::Client::new();
    let url = format!("{}/backup", get_backend_url());

    let res = client.post(&url).headers(headers.clone()).send().await?;
    assert!(res.status().is_success(), "{}", res.status());

    let listings: BackupListings = client
        .get(&url)
        .headers(headers.clone())
        .send()
        .await?
        .json()
        .await?;
    assert!(!listings.local.is_empty());

    for listing in &listings.local {
        let res = client
            .get(format!(
                "{}/backup/local/{}",
                get_backend_url(),
                listing.name
            ))
            .headers(headers.clone())
            .send()
            .await?;
        assert_eq!(res.status(), 200);
        assert_eq!(res.content_length(), listing.size);

        let bytes = res.bytes().await?;
        assert_eq!(Some(bytes.len() as u64), listing.size);
        assert!(bytes.starts_with(b"SQLite format 3\0"));
    }

    Ok(())
}

/// The session search idx is `ILIKE` on Postgres and `LIKE` on SQLite, both case-insensitive, and
/// the admin session of `get_auth_headers` always talks to the local backend.
#[tokio::test]
async fn test_search_sessions_by_ip() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let url = format!("{}/search?ty=session&idx=ip&q=127.0.0.1", get_backend_url());

    let res = reqwest::Client::new()
        .get(&url)
        .headers(auth_headers)
        .send()
        .await?;
    let sessions = expect_status_json!(res, 200, Vec<Value>);
    assert!(
        !sessions.is_empty(),
        "the admin session must be found by IP"
    );

    Ok(())
}
