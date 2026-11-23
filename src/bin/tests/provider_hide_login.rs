use crate::common::{get_auth_headers, get_backend_url};
use reqwest::header::HeaderMap;
use std::error::Error;

mod common;

const ISSUER: &str = "http://localhost:1/hide-login-test";

/// Nothing on this port listens: this test never reaches the upstream at all.
const UNREACHABLE: &str = "http://localhost:1";

/// `ProviderRequest` has no field defaults, so every request sends the full config and only flips
/// `hide_login`.
fn payload(hide_login: bool) -> serde_json::Value {
    serde_json::json!({
        "name": "Hide Login Test",
        "typ": "custom",
        "enabled": true,
        "issuer": ISSUER,
        "authorization_endpoint": format!("{UNREACHABLE}/authorize"),
        "token_endpoint": format!("{UNREACHABLE}/token"),
        "userinfo_endpoint": format!("{UNREACHABLE}/userinfo"),
        "use_pkce": true,
        "client_secret_basic": false,
        "client_secret_post": false,
        "auto_onboarding": false,
        "auto_link": false,
        "hide_login": hide_login,
        "client_id": "rauthy",
        "scope": "openid",
    })
}

/// The provider of [`ISSUER`] from the admin provider list, `None` when it does not exist. The
/// `issuer` is unique, which is what identifies the provider here.
async fn find_by_issuer(
    http: &reqwest::Client,
    url: &str,
    headers: &HeaderMap,
) -> Result<Option<serde_json::Value>, Box<dyn Error>> {
    let res = http
        .post(format!("{url}/providers"))
        .headers(headers.clone())
        .send()
        .await?;
    let providers = expect_status_json!(res, 200, Vec<serde_json::Value>);
    Ok(providers
        .into_iter()
        .find(|p| p["issuer"].as_str() == Some(ISSUER)))
}

async fn delete_by_issuer(
    http: &reqwest::Client,
    url: &str,
    headers: &HeaderMap,
) -> Result<(), Box<dyn Error>> {
    if let Some(provider) = find_by_issuer(http, url, headers).await? {
        let id = provider["id"].as_str().expect("the provider id");
        let res = http
            .delete(format!("{url}/providers/{id}"))
            .headers(headers.clone())
            .send()
            .await?;
        expect_status!(res, 200);
    }
    Ok(())
}

/// The `hide_login` of the provider in the templated `GET /providers/minimal` list, which is what
/// the login and registration pages are built from.
async fn minimal_hide_login(
    http: &reqwest::Client,
    url: &str,
    id: &str,
) -> Result<bool, Box<dyn Error>> {
    let res = http.get(format!("{url}/providers/minimal")).send().await?;
    let providers = expect_status_json!(res, 200, Vec<serde_json::Value>);
    providers
        .into_iter()
        .find(|p| p["id"].as_str() == Some(id))
        .map(|p| p["hide_login"].as_bool().expect("the hide_login flag"))
        .ok_or_else(|| "provider missing from /providers/minimal".into())
}

#[tokio::test]
async fn test_provider_hide_login() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();
    let http = reqwest::Client::new();

    // A leftover from a previous, failed run would skew the very first assertion.
    delete_by_issuer(&http, &backend_url, &auth_headers).await?;

    let res = http
        .post(format!("{backend_url}/providers/create"))
        .headers(auth_headers.clone())
        .json(&payload(true))
        .send()
        .await?;
    let created = expect_status_json!(res, 200, serde_json::Value);
    let id = created["id"].as_str().expect("the provider id").to_string();

    let provider = find_by_issuer(&http, &backend_url, &auth_headers)
        .await?
        .expect("the created provider");
    assert_eq!(
        provider["hide_login"].as_bool(),
        Some(true),
        "a provider created with `hide_login: true`",
    );
    assert!(
        minimal_hide_login(&http, &backend_url, &id).await?,
        "the provider is part of the login page template",
    );

    // A hidden provider stays in `/providers/minimal`: an `idp_hint` login resolves the provider
    // from that list, only the rendered button is filtered on the page itself.
    let res = http
        .put(format!("{backend_url}/providers/{id}"))
        .headers(auth_headers.clone())
        .json(&payload(false))
        .send()
        .await?;
    expect_status!(res, 200);

    let provider = find_by_issuer(&http, &backend_url, &auth_headers)
        .await?
        .expect("the updated provider");
    assert_eq!(
        provider["hide_login"].as_bool(),
        Some(false),
        "the flag round-trips on update",
    );
    assert!(
        !minimal_hide_login(&http, &backend_url, &id).await?,
        "the template carries the updated flag",
    );

    delete_by_issuer(&http, &backend_url, &auth_headers).await?;

    Ok(())
}
