mod common;

use common::{
    CLIENT_ID, decode_claims, get_auth_headers, get_backend_url, session_headers_for_client,
};
use rauthy_api_types::clients::ClientSecretResponse;
use rauthy_api_types::oidc::{GrantType, LogoutRequest, TokenRequest};
use rauthy_service::token_set::TokenSet;
use std::error::Error;

async fn client_secret() -> String {
    reqwest::Client::new()
        .post(format!("{}/clients/{CLIENT_ID}/secret", get_backend_url()))
        .headers(get_auth_headers().await.unwrap())
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json::<ClientSecretResponse>()
        .await
        .unwrap()
        .secret
        .unwrap()
}

async fn refresh(http: &reqwest::Client, tokens: &TokenSet, secret: &str) -> reqwest::Response {
    http.post(format!("{}/oidc/token", get_backend_url()))
        .form(&TokenRequest {
            grant_type: GrantType::RefreshToken,
            client_id: Some(CLIENT_ID.to_string()),
            client_secret: Some(secret.to_string()),
            refresh_token: tokens.refresh_token.clone(),
            ..Default::default()
        })
        .send()
        .await
        .unwrap()
}

#[tokio::test]
async fn test_refresh_preserves_session_id() -> Result<(), Box<dyn Error>> {
    let secret = client_secret().await;
    let http = reqwest::Client::new();
    let (_, original) = session_headers_for_client(
        CLIENT_ID,
        Some(secret.as_str()),
        "http://localhost:3000/oidc/callback",
    )
    .await;
    let sid = decode_claims(original.id_token.as_ref().unwrap())["sid"].clone();
    assert!(sid.is_string());

    let response = refresh(&http, &original, &secret).await;
    assert_eq!(response.status(), 200);
    let first: TokenSet = response.json().await?;
    let response = refresh(&http, &first, &secret).await;
    assert_eq!(response.status(), 200);
    let rotated: TokenSet = response.json().await?;
    assert_eq!(
        decode_claims(rotated.id_token.as_ref().unwrap())["sid"],
        sid
    );
    Ok(())
}

#[tokio::test]
async fn test_concurrent_refresh_has_one_winner() -> Result<(), Box<dyn Error>> {
    let secret = client_secret().await;
    let http = reqwest::Client::new();
    let (_, original) = session_headers_for_client(
        CLIENT_ID,
        Some(secret.as_str()),
        "http://localhost:3000/oidc/callback",
    )
    .await;
    let (first, second) = tokio::join!(
        refresh(&http, &original, &secret),
        refresh(&http, &original, &secret)
    );
    assert_ne!(first.status().is_success(), second.status().is_success());
    let (winner, loser) = if first.status().is_success() {
        (first, second)
    } else {
        (second, first)
    };
    assert_eq!(winner.status(), 200);
    assert_eq!(loser.status(), 404);
    assert_eq!(refresh(&http, &original, &secret).await.status(), 404);
    let rotated: TokenSet = winner.json().await?;
    let inspected = http
        .post(format!("{}/oidc/introspect", get_backend_url()))
        .basic_auth(CLIENT_ID, Some(secret.as_str()))
        .form(&[("token", &rotated.access_token)])
        .send()
        .await?;
    assert_eq!(inspected.status(), 200);
    assert_eq!(inspected.json::<serde_json::Value>().await?["active"], true);
    assert_eq!(refresh(&http, &rotated, &secret).await.status(), 200);
    Ok(())
}

#[tokio::test]
async fn test_logout_before_refresh_rejects_rotation() -> Result<(), Box<dyn Error>> {
    let secret = client_secret().await;
    let http = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let (_, original) = session_headers_for_client(
        CLIENT_ID,
        Some(secret.as_str()),
        "http://localhost:3000/oidc/callback",
    )
    .await;
    let response = http
        .post(format!("{}/oidc/logout", get_backend_url()))
        .form(&LogoutRequest {
            id_token_hint: original.id_token.clone(),
            post_logout_redirect_uri: None,
            state: None,
            logout_token: None,
        })
        .send()
        .await?;
    assert_eq!(response.status(), 200);
    assert_eq!(refresh(&http, &original, &secret).await.status(), 404);
    Ok(())
}

#[tokio::test]
async fn test_session_logout_revokes_rotated_tokens() -> Result<(), Box<dyn Error>> {
    let secret = client_secret().await;
    let http = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let (_, original) = session_headers_for_client(
        CLIENT_ID,
        Some(secret.as_str()),
        "http://localhost:3000/oidc/callback",
    )
    .await;
    let (_, independent) = session_headers_for_client(
        CLIENT_ID,
        Some(secret.as_str()),
        "http://localhost:3000/oidc/callback",
    )
    .await;
    let response = refresh(&http, &original, &secret).await;
    assert_eq!(response.status(), 200);
    let first: TokenSet = response.json().await?;
    let response = refresh(&http, &first, &secret).await;
    assert_eq!(response.status(), 200);
    let rotated: TokenSet = response.json().await?;
    let inspect_url = format!("{}/oidc/introspect", get_backend_url());
    let before = http
        .post(&inspect_url)
        .basic_auth(CLIENT_ID, Some(secret.as_str()))
        .form(&[("token", &rotated.access_token)])
        .send()
        .await?;
    assert_eq!(before.status(), 200);
    assert_eq!(before.json::<serde_json::Value>().await?["active"], true);

    let logout = http
        .post(format!("{}/oidc/logout", get_backend_url()))
        .form(&LogoutRequest {
            id_token_hint: original.id_token,
            post_logout_redirect_uri: None,
            state: None,
            logout_token: None,
        })
        .send()
        .await?;
    assert_eq!(logout.status(), 200);

    let after = http
        .post(&inspect_url)
        .basic_auth(CLIENT_ID, Some(secret.as_str()))
        .form(&[("token", &rotated.access_token)])
        .send()
        .await?;
    assert_eq!(after.status(), 401);
    assert_eq!(refresh(&http, &rotated, &secret).await.status(), 404);
    assert_eq!(refresh(&http, &independent, &secret).await.status(), 200);
    Ok(())
}

#[tokio::test]
async fn test_concurrent_logout_revokes_returned_refresh() -> Result<(), Box<dyn Error>> {
    let secret = client_secret().await;
    let http = reqwest::Client::new();
    let (_, original) = session_headers_for_client(
        CLIENT_ID,
        Some(secret.as_str()),
        "http://localhost:3000/oidc/callback",
    )
    .await;
    let logout = http
        .post(format!("{}/oidc/logout", get_backend_url()))
        .form(&LogoutRequest {
            id_token_hint: original.id_token.clone(),
            post_logout_redirect_uri: None,
            state: None,
            logout_token: None,
        });
    let (rotated, logout) = tokio::join!(refresh(&http, &original, &secret), logout.send());
    assert_eq!(logout?.status(), 200);
    if rotated.status().is_success() {
        let rotated: TokenSet = rotated.json().await?;
        let inspected = http
            .post(format!("{}/oidc/introspect", get_backend_url()))
            .basic_auth(CLIENT_ID, Some(secret.as_str()))
            .form(&[("token", &rotated.access_token)])
            .send()
            .await?;
        assert_eq!(inspected.status(), 401);
        assert_eq!(refresh(&http, &rotated, &secret).await.status(), 404);
    } else {
        assert_eq!(rotated.status(), 404);
    }
    Ok(())
}
