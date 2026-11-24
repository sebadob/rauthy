use crate::common::{
    CLIENT_ID, accept_all, consent_required, cookie_csrf_headers_from_res_direct, get_auth_headers,
    get_backend_url, get_solved_pow, stale_consent,
};
use crate::expect_status;
use rauthy_api_types::auth_providers::{ProviderCallbackRequest, ProviderLoginRequest};
use rauthy_api_types::clients::{NewClientRequest, UpdateClientRequest};
use rauthy_api_types::oidc::{GrantType, JwkKeyPairAlg, LoginConsent};
use rauthy_common::sha256;
use rauthy_common::utils::{base64_url_encode, new_store_id};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::header::{self, HeaderValue, LOCATION};
use std::error::Error;

const PKCE_VERIFIER: &str = "Ajk3hfdVsDu8DpYuZeYbUfNIWSYdq5sKjpDcTWgM7RBsXQEkSmTNXV6wLvHUmJYQ";
const RESOURCE: &str = "https://consent-test.example.com";

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
    // a unique provider per run: its issuer is what the backend enforces as unique, so no other
    // test, and no concurrent run of this one, can collide with it
    let suffix = new_store_id();
    let issuer = format!("{upstream}/issuer-{suffix}");
    let res = http
        .post(format!("{backend_url}/providers/create"))
        .headers(auth_headers.clone())
        .json(&serde_json::json!({
            "name": format!("Callback Test {suffix}"),
            "typ": "custom",
            "enabled": true,
            "issuer": issuer,
            "authorization_endpoint": format!("{upstream}/authorize"),
            "token_endpoint": format!("{upstream}/token"),
            "userinfo_endpoint": format!("{upstream}/userinfo"),
            "use_pkce": true,
            "client_secret_basic": false,
            "client_secret_post": false,
            "auto_onboarding": false,
            "auto_link": false,
            "hide_login": false,
            "client_id": "rauthy",
            "scope": "openid",
        }))
        .send()
        .await?;
    let res = expect_status!(res, 200);
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

    let provider_login_to = async |client_id: &str,
                                   redirect_uri: String,
                                   scopes: Option<Vec<String>>,
                                   resource: Option<&str>,
                                   consent: Option<LoginConsent>| {
        http.post(format!("{backend_url}/providers/login"))
            .headers(session_headers.clone())
            .json(&ProviderLoginRequest {
                email: None,
                client_id: client_id.to_string(),
                redirect_uri,
                scopes,
                state: None,
                nonce: None,
                code_challenge: None,
                code_challenge_method: None,
                resource: resource.map(String::from),
                pow: get_solved_pow().await,
                provider_id: provider_id.clone(),
                pkce_challenge: base64_url_encode(sha256!(PKCE_VERIFIER.as_bytes())),
                handle: None,
                fwda: None,
                consent,
            })
            .send()
            .await
            .unwrap()
    };
    let provider_login = async |consent: Option<LoginConsent>| {
        let redirect_uri = format!("{backend_url}/oidc/callback");
        provider_login_to("rauthy", redirect_uri, None, None, consent).await
    };

    // the consent is asked for exactly the requested scopes with the client's defaults
    let redirect_uri = "http://localhost:3000/oidc/callback".to_string();
    let scopes = Some(vec!["openid".to_string(), "groups".to_string()]);
    let login_groups = |consent| {
        provider_login_to(
            CLIENT_ID,
            redirect_uri.clone(),
            scopes.clone(),
            None,
            consent,
        )
    };
    let content = consent_required(login_groups(None).await).await;
    let names = content.content.scopes.iter().map(|s| s.name.as_str());
    assert_eq!(
        names.collect::<Vec<_>>(),
        ["email", "groups", "openid", "profile"]
    );
    let res = login_groups(Some(accept_all(&content))).await;
    expect_status!(res, 202);

    // a `resource` the client does not allow is refused before the upstream redirect
    let res = provider_login_to(
        CLIENT_ID,
        redirect_uri.clone(),
        scopes.clone(),
        Some(RESOURCE),
        None,
    )
    .await;
    let res = expect_status!(res, 400);
    assert!(res.headers().get(LOCATION).is_none());
    assert_eq!(
        res.json::<ErrorResponse>().await?.error,
        ErrorResponseType::InvalidTarget
    );

    // with an allowed `resource`, the consent is asked for the requested scopes as well
    // a client of its own that allows [`RESOURCE`], which `init_client` does not
    let res_client_id = format!("provider_resource_{}", new_store_id());
    let url_res_client = format!("{backend_url}/clients/{res_client_id}");
    let res = http
        .post(format!("{backend_url}/clients"))
        .headers(auth_headers.clone())
        .json(&NewClientRequest {
            id: res_client_id.clone(),
            secret: None,
            name: Some("Provider Resource".to_string()),
            confidential: false,
            redirect_uris: vec![redirect_uri.clone()],
            post_logout_redirect_uris: None,
            consent: None,
        })
        .send()
        .await?;
    expect_status!(res, 200);
    let res = http
        .put(&url_res_client)
        .headers(auth_headers.clone())
        .json(&UpdateClientRequest {
            name: Some("Provider Resource".to_string()),
            confidential: false,
            redirect_uris: vec![redirect_uri.clone()],
            post_logout_redirect_uris: None,
            allowed_origins: None,
            enabled: true,
            flows_enabled: vec![GrantType::AuthorizationCode],
            access_token_alg: JwkKeyPairAlg::EdDSA,
            id_token_alg: JwkKeyPairAlg::EdDSA,
            auth_code_lifetime: 60,
            access_token_lifetime: 300,
            scopes: ["openid", "email", "groups"].map(String::from).to_vec(),
            default_scopes: vec!["openid".to_string()],
            challenges: Some(vec!["S256".to_string()]),
            force_mfa: false,
            client_uri: None,
            contacts: None,
            backchannel_logout_uri: None,
            restrict_group_prefix: None,
            claims: None,
            claims_at_root: false,
            allowed_resources: Some(vec![RESOURCE.to_string()]),
            default_aud: None,
            consent: None,
            actor_same_client: false,
            scim: None,
        })
        .send()
        .await?;
    expect_status!(res, 200);
    let login_res = |consent| {
        provider_login_to(
            &res_client_id,
            redirect_uri.clone(),
            scopes.clone(),
            Some(RESOURCE),
            consent,
        )
    };
    let content = consent_required(login_res(None).await).await;
    let names = content.content.scopes.iter().map(|s| s.name.as_str());
    assert_eq!(names.collect::<Vec<_>>(), ["groups", "openid"]);
    // built from the consent source of the resource
    assert_eq!(content.content.name.as_deref(), Some("Consent Test API"));
    // and checked against it: the consent to the same login without the resource is stale
    let login_no_res = provider_login_to(
        &res_client_id,
        redirect_uri.clone(),
        scopes.clone(),
        None,
        None,
    );
    let content_no_res = consent_required(login_no_res.await).await;
    assert_eq!(content_no_res.content.name, None);
    assert_eq!(
        consent_required(login_res(Some(accept_all(&content_no_res))).await).await,
        content
    );
    let stale = stale_consent(&content);
    assert_eq!(
        consent_required(login_res(Some(stale)).await).await,
        content
    );
    let res = login_res(Some(accept_all(&content))).await;
    expect_status!(res, 202);
    let res = http
        .delete(&url_res_client)
        .headers(auth_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);

    // a missing or stale consent is refused before the upstream redirect, which would spend
    // the single-use callback on it
    let content = consent_required(provider_login(None).await).await;
    assert!(content.granted_scopes.is_empty());
    let stale = stale_consent(&content);
    assert_eq!(
        consent_required(provider_login(Some(stale)).await).await,
        content
    );

    let res = provider_login(Some(accept_all(&content))).await;
    let res = expect_status!(res, 202);

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
    let res = expect_status!(res, 401);
    assert!(res.text().await?.contains("invalid PKCE verifier"));

    // ... so that even a fully valid request cannot use it anymore
    let res = http
        .post(&url_callback)
        .headers(session_headers.clone())
        .json(&callback(PKCE_VERIFIER))
        .send()
        .await?;
    let res = expect_status!(res, 404);
    assert!(res.text().await?.contains("Callback Code not found"));

    // linking an account issues no auth code: neither the consent nor the client's redirect URI
    // are checked
    let (link_headers, _) = crate::common::session_headers().await;
    let link = async |redirect_uri: String| {
        http.post(format!("{backend_url}/providers/{provider_id}/link"))
            .headers(link_headers.clone())
            .json(&ProviderLoginRequest {
                email: None,
                client_id: "rauthy".to_string(),
                redirect_uri,
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
                fwda: None,
                consent: None,
            })
            .send()
            .await
            .unwrap()
    };
    for redirect_uri in [
        format!("{backend_url}/oidc/callback"),
        "https://unregistered.example.com/callback".to_string(),
    ] {
        let res = link(redirect_uri.clone()).await;
        let res = expect_status!(res, 202);
        let location = res.headers().get(LOCATION).unwrap().to_str()?;
        assert!(
            location.starts_with("http://localhost:1/authorize?"),
            "{redirect_uri}: {location}"
        );
    }

    let res = http
        .delete(format!("{backend_url}/providers/{provider_id}"))
        .headers(auth_headers)
        .send()
        .await?;
    expect_status!(res, 200);

    Ok(())
}
