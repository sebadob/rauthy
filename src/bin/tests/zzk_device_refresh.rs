mod common;

use common::{USERNAME, decode_claims, get_auth_headers, get_backend_url, get_solved_pow};
use pretty_assertions::assert_eq;
use rauthy_api_types::clients::{ClientResponse, NewClientRequest, UpdateClientRequest};
use rauthy_api_types::oidc::{GrantType, TokenRequest};
use rauthy_api_types::users::UserResponseSimple;
use rauthy_service::token_set::TokenSet;
use reqwest::Client;
use serde_json::{Value, json};
use std::error::Error;
use std::time::Duration;
use tokio::time;

const CLIENT: &str = "device-refresh";

/// A public client with the device code and refresh token flows.
async fn create_client(http: &Client) -> Result<(), Box<dyn Error>> {
    let headers = get_auth_headers().await?;
    let url = format!("{}/clients", get_backend_url());
    let url_id = format!("{url}/{CLIENT}");

    // left-over from a prior run
    let _ = http.delete(&url_id).headers(headers.clone()).send().await?;

    let res = http
        .post(&url)
        .headers(headers.clone())
        .json(&NewClientRequest {
            id: CLIENT.to_string(),
            secret: None,
            name: None,
            confidential: false,
            redirect_uris: vec!["http://localhost:3000/oidc/callback".to_string()],
            post_logout_redirect_uris: None,
            consent: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200, "{}", res.text().await?);

    let c = http
        .get(&url_id)
        .headers(headers.clone())
        .send()
        .await?
        .json::<ClientResponse>()
        .await?;
    let res = http
        .put(&url_id)
        .headers(headers)
        .json(&UpdateClientRequest {
            name: c.name,
            confidential: c.confidential,
            redirect_uris: c.redirect_uris,
            post_logout_redirect_uris: c.post_logout_redirect_uris,
            allowed_origins: c.allowed_origins,
            enabled: c.enabled,
            flows_enabled: vec![GrantType::DeviceCode, GrantType::RefreshToken],
            access_token_alg: c.access_token_alg,
            id_token_alg: c.id_token_alg,
            auth_code_lifetime: c.auth_code_lifetime,
            access_token_lifetime: c.access_token_lifetime,
            scopes: c.scopes,
            default_scopes: c.default_scopes,
            challenges: c.challenges,
            force_mfa: c.force_mfa,
            client_uri: c.client_uri,
            contacts: c.contacts,
            backchannel_logout_uri: c.backchannel_logout_uri,
            restrict_group_prefix: c.restrict_group_prefix,
            claims: c.claims,
            claims_at_root: c.claims_at_root,
            allowed_resources: c.allowed_resources,
            default_aud: c.default_aud,
            scim: c.scim,
            consent: c.consent,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200, "{}", res.text().await?);
    Ok(())
}

/// Runs the device code flow on `CLIENT`, accepted by the admin session.
async fn device_token_set(http: &Client) -> Result<TokenSet, Box<dyn Error>> {
    let backend = get_backend_url();

    // the device authorization is rate-limited per IP
    let code = loop {
        let res = http
            .post(format!("{backend}/oidc/device"))
            .form(&[("client_id", CLIENT)])
            .send()
            .await?;
        if res.status() == 429 {
            time::sleep(Duration::from_secs(1)).await;
            continue;
        }
        assert_eq!(res.status(), 200, "{}", res.text().await?);
        break res.json::<Value>().await?;
    };
    let device_code = code["device_code"].as_str().unwrap().to_string();
    let user_code = code["user_code"].as_str().unwrap().to_string();
    let interval = code["interval"].as_u64().unwrap_or(5);

    let res = http
        .post(format!("{backend}/oidc/device/verify"))
        .headers(get_auth_headers().await?)
        .json(&json!({
            "user_code": user_code,
            "pow": get_solved_pow().await,
            "device_accepted": "accept",
        }))
        .send()
        .await?;
    assert_eq!(res.status(), 202, "{}", res.text().await?);

    // the device must respect the poll interval, counted from the authorization
    time::sleep(Duration::from_secs(interval)).await;
    let res = http
        .post(format!("{backend}/oidc/token"))
        .form(&TokenRequest {
            grant_type: GrantType::DeviceCode,
            client_id: Some(CLIENT.to_string()),
            device_code: Some(device_code),
            ..Default::default()
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200, "{}", res.text().await?);
    Ok(res.json::<TokenSet>().await?)
}

async fn refresh(http: &Client, refresh_token: &str) -> reqwest::Response {
    http.post(format!("{}/oidc/token", get_backend_url()))
        .form(&TokenRequest {
            grant_type: GrantType::RefreshToken,
            client_id: Some(CLIENT.to_string()),
            refresh_token: Some(refresh_token.to_string()),
            ..Default::default()
        })
        .send()
        .await
        .unwrap()
}

/// The device code grant has no consent step: its refresh tokens are exempt from the consent
/// check, and stay exempt through the rotation (which drops the `did`).
fn assert_consent_exempt(refresh_token: &str, did: bool) {
    let claims = decode_claims(refresh_token);
    assert_eq!(claims["consent_exempt"], json!(true), "{claims}");
    assert_eq!(claims.get("did").is_some(), did, "{claims}");
}

#[tokio::test]
async fn test_device_refresh_rotation() -> Result<(), Box<dyn Error>> {
    let http = Client::new();
    let backend = get_backend_url();
    let admin = get_auth_headers().await?;
    create_client(&http).await?;

    let admin_id = http
        .get(format!("{backend}/users"))
        .headers(admin.clone())
        .send()
        .await?
        .json::<Vec<UserResponseSimple>>()
        .await?
        .into_iter()
        .find(|u| u.email == USERNAME)
        .unwrap()
        .id;

    // --- one rotation
    let ts = device_token_set(&http).await?;
    let device_rt = ts.refresh_token.unwrap();
    assert_consent_exempt(&device_rt, true);

    let res = refresh(&http, &device_rt).await;
    assert_eq!(res.status(), 200, "{}", res.text().await?);
    let rotated = res.json::<TokenSet>().await?;
    let rotated_rt = rotated.refresh_token.unwrap();
    // a regular refresh token now, still exempt
    assert_consent_exempt(&rotated_rt, false);

    // the source is consumed: a replay fails on the missing row
    let res = refresh(&http, &device_rt).await;
    assert_eq!(res.status(), 404);
    let body = res.text().await?;
    assert!(body.contains("Device Refresh Token not found"), "{body}");

    // the replacement keeps rotating
    let res = refresh(&http, &rotated_rt).await;
    assert_eq!(res.status(), 200, "{}", res.text().await?);
    assert_consent_exempt(&res.json::<TokenSet>().await?.refresh_token.unwrap(), false);

    // --- the device's refresh tokens are deleted between issue and refresh
    let ts = device_token_set(&http).await?;
    let device_rt = ts.refresh_token.unwrap();
    let device_id = decode_claims(&device_rt)["did"]
        .as_str()
        .unwrap()
        .to_string();

    let res = http
        .delete(format!("{backend}/users/{admin_id}/devices"))
        .headers(admin.clone())
        .json(&json!({ "device_id": device_id }))
        .send()
        .await?;
    assert_eq!(res.status(), 200, "{}", res.text().await?);

    // the refresh fails, and leaves no replacement to use
    let res = refresh(&http, &device_rt).await;
    assert_eq!(res.status(), 404);
    let body = res.text().await?;
    assert!(body.contains("Device Refresh Token not found"), "{body}");
    assert!(serde_json::from_str::<TokenSet>(&body).is_err());

    // --- cleanup
    let res = http
        .delete(format!("{backend}/clients/{CLIENT}"))
        .headers(admin)
        .send()
        .await?;
    assert_eq!(res.status(), 200, "{}", res.text().await?);

    Ok(())
}
