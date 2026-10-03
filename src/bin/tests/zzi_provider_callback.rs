use crate::common::{
    cookie_csrf_headers_from_res_direct, get_auth_headers, get_backend_url, get_solved_pow,
};
use pretty_assertions::assert_eq;
use rauthy_api_types::auth_providers::{ProviderCallbackRequest, ProviderLoginRequest};
use rauthy_common::sha256;
use rauthy_common::utils::base64_url_encode;
use reqwest::header::{self, HeaderValue, LOCATION};
use std::error::Error;

mod common;

const PKCE_VERIFIER: &str = "Ajk3hfdVsDu8DpYuZeYbUfNIWSYdq5sKjpDcTWgM7RBsXQEkSmTNXV6wLvHUmJYQ";

/// The upstream callback is single-use: once a callback request has been made, any further one
/// with the same callback fails, whatever the outcome of the first one was.
#[tokio::test]
async fn test_provider_callback_single_use() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();
    let http = reqwest::Client::new();

    // The endpoints are never reached: the token request only happens after a fully validated
    // callback, and nothing listens on this port.
    let upstream = "http://localhost:1";
    let res = http
        .post(format!("{backend_url}/providers/create"))
        .headers(auth_headers.clone())
        .json(&serde_json::json!({
            "name": "Callback Test",
            "typ": "custom",
            "enabled": true,
            "issuer": format!("{upstream}/issuer"),
            "authorization_endpoint": format!("{upstream}/authorize"),
            "token_endpoint": format!("{upstream}/token"),
            "userinfo_endpoint": format!("{upstream}/userinfo"),
            "use_pkce": true,
            "client_secret_basic": false,
            "client_secret_post": false,
            "auto_onboarding": false,
            "auto_link": false,
            "client_id": "rauthy",
            "scope": "openid",
        }))
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let provider_id = res.json::<serde_json::Value>().await?["id"]
        .as_str()
        .expect("the provider id")
        .to_string();

    // a session in init state
    let res = http
        .post(format!("{backend_url}/oidc/session"))
        .send()
        .await?;
    let mut session_headers = cookie_csrf_headers_from_res_direct(res).await?;

    let res = http
        .post(format!("{backend_url}/providers/login"))
        .headers(session_headers.clone())
        .json(&ProviderLoginRequest {
            email: None,
            client_id: "rauthy".to_string(),
            redirect_uri: format!("{backend_url}/oidc/callback"),
            scopes: None,
            state: None,
            nonce: None,
            code_challenge: None,
            code_challenge_method: None,
            resource: None,
            pow: get_solved_pow().await,
            provider_id: provider_id.clone(),
            pkce_challenge: base64_url_encode(sha256!(PKCE_VERIFIER.as_bytes())),
            handle: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 202);

    let callback_cookie = res
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .filter_map(|c| c.to_str().ok()?.split_once(';').map(|(c, _)| c.to_string()))
        .find(|c| c.contains("UpstreamAuthCallback="))
        .expect("the upstream callback cookie");
    let location = reqwest::Url::parse(res.headers().get(LOCATION).unwrap().to_str()?)?;
    let callback_id = location
        .query_pairs()
        .find(|(k, _)| k == "state")
        .map(|(_, v)| v.into_owned())
        .expect("state in the upstream Location");
    let xsrf_token = res.text().await?;

    let cookies = format!(
        "{}; {callback_cookie}",
        session_headers.get(header::COOKIE).unwrap().to_str()?
    );
    session_headers.insert(header::COOKIE, HeaderValue::from_str(&cookies)?);

    let callback = |pkce_verifier: &str| ProviderCallbackRequest {
        state: callback_id.clone(),
        code: "upstream_code".to_string(),
        xsrf_token: xsrf_token.clone(),
        pkce_verifier: pkce_verifier.to_string(),
        iss_atproto: None,
    };
    let url_callback = format!("{backend_url}/providers/callback");

    // a failed validation consumes the callback ...
    let res = http
        .post(&url_callback)
        .headers(session_headers.clone())
        .json(&callback("invalidVerifier"))
        .send()
        .await?;
    assert_eq!(res.status(), 401);
    assert!(res.text().await?.contains("invalid PKCE verifier"));

    // ... so that even a fully valid request cannot use it anymore
    let res = http
        .post(&url_callback)
        .headers(session_headers.clone())
        .json(&callback(PKCE_VERIFIER))
        .send()
        .await?;
    assert_eq!(res.status(), 404);
    assert!(res.text().await?.contains("Callback Code not found"));

    let res = http
        .delete(format!("{backend_url}/providers/{provider_id}"))
        .headers(auth_headers)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    Ok(())
}
