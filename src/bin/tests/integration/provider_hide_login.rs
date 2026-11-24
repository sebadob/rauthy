use crate::common::{get_auth_headers, get_backend_url};
use crate::{expect_status, expect_status_json};
use rauthy_common::utils::new_store_id;
use reqwest::header::HeaderMap;
use std::error::Error;

/// Nothing on this port listens: this test never reaches the upstream at all.
const UNREACHABLE: &str = "http://localhost:1";

/// The upstream of this test: nothing listens on port 1, the provider is never reached.
const ISSUER_BASE: &str = "http://localhost:1/hide-login-test";

/// The provider identity of one test run: `name` and `issuer` are both free-form strings, so they
/// travel together and cannot be swapped at a call site.
struct ProviderIdentity {
    name: String,
    issuer: String,
}

/// `ProviderRequest` has no field defaults, so every request sends the full config and only flips
/// `hide_login`.
fn payload(hide_login: bool, identity: &ProviderIdentity) -> serde_json::Value {
    serde_json::json!({
        "name": identity.name,
        "typ": "custom",
        "enabled": true,
        "issuer": identity.issuer,
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

/// The provider of `issuer` from the admin provider list, `None` when it does not exist. The
/// `issuer` is unique, which is what identifies the provider here.
async fn find_by_issuer(
    http: &reqwest::Client,
    url: &str,
    headers: &HeaderMap,
    issuer: &str,
) -> Result<Option<serde_json::Value>, Box<dyn Error>> {
    let res = http
        .post(format!("{url}/providers"))
        .headers(headers.clone())
        .send()
        .await?;
    let providers = expect_status_json!(res, 200, Vec<serde_json::Value>);
    Ok(providers
        .into_iter()
        .find(|p| p["issuer"].as_str() == Some(issuer)))
}

async fn delete_by_issuer(
    http: &reqwest::Client,
    url: &str,
    headers: &HeaderMap,
    issuer: &str,
) -> Result<(), Box<dyn Error>> {
    if let Some(provider) = find_by_issuer(http, url, headers, issuer).await? {
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
    // a unique issuer per run: the provider this test creates is identified by it, and no other
    // test, or a concurrent run of this one, can use the same one
    let identity = ProviderIdentity {
        name: format!("Hide Login Test {}", new_store_id()),
        issuer: format!("{ISSUER_BASE}/{}", new_store_id()),
    };
    let issuer = identity.issuer.clone();

    let res = http
        .post(format!("{backend_url}/providers/create"))
        .headers(auth_headers.clone())
        .json(&payload(true, &identity))
        .send()
        .await?;
    let created = expect_status_json!(res, 200, serde_json::Value);
    let id = created["id"].as_str().expect("the provider id").to_string();

    let provider = find_by_issuer(&http, &backend_url, &auth_headers, &issuer)
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
        .json(&payload(false, &identity))
        .send()
        .await?;
    expect_status!(res, 200);

    let provider = find_by_issuer(&http, &backend_url, &auth_headers, &issuer)
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

    delete_by_issuer(&http, &backend_url, &auth_headers, &issuer).await?;

    Ok(())
}
