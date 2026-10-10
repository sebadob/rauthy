mod common;

use common::{
    CLIENT_ID, PASSWORD, USERNAME, code_state_from_headers, cookie_csrf_headers_from_res_direct,
    get_auth_headers, get_backend_url, get_solved_pow, session_headers_for_client,
};
use rauthy_api_types::clients::{
    ClientResponse, ClientSecretResponse, NewClientRequest, UpdateClientRequest,
};
use rauthy_api_types::oidc::{GrantType, LoginRequest, TokenRequest};
use rauthy_common::sha256;
use rauthy_common::utils::base64_url_encode;
use rauthy_service::token_set::TokenSet;
use std::error::Error;

const REDIRECT_URI: &str = "http://localhost:3000/oidc/callback";
const CODE_VERIFIER: &str = "oDXug9zfYqfz8ejcqMpALRPXfW8QhbKV2AVuScAt8xrLKDAmaRYQ4yRi2uqcH9ys";

/// Creates a public client with the authorization code and refresh token flows.
async fn create_client(id: &str) -> Result<(), Box<dyn Error>> {
    let headers = get_auth_headers().await?;
    let http = reqwest::Client::new();
    let url = format!("{}/clients", get_backend_url());

    let res = http
        .post(&url)
        .headers(headers.clone())
        .json(&NewClientRequest {
            id: id.to_string(),
            secret: None,
            name: None,
            confidential: false,
            redirect_uris: vec![REDIRECT_URI.to_string()],
            post_logout_redirect_uris: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200, "{}", res.text().await?);

    let url_id = format!("{url}/{id}");
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
            flows_enabled: vec![GrantType::AuthorizationCode, GrantType::RefreshToken],
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
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200, "{}", res.text().await?);
    Ok(())
}

async fn delete_client(id: &str) -> Result<(), Box<dyn Error>> {
    let res = reqwest::Client::new()
        .delete(format!("{}/clients/{id}", get_backend_url()))
        .headers(get_auth_headers().await?)
        .send()
        .await?;
    assert_eq!(res.status(), 200, "{}", res.text().await?);
    Ok(())
}

/// Logs in to `client_id` and returns the authorization code without exchanging it.
async fn auth_code(client_id: &str) -> Result<String, Box<dyn Error>> {
    let backend_url = get_backend_url();
    let http = reqwest::Client::new();

    let res = http
        .post(format!("{backend_url}/oidc/session"))
        .send()
        .await?;
    assert!(res.status().is_success());
    let headers = cookie_csrf_headers_from_res_direct(res).await?;

    let challenge = base64_url_encode(sha256!(CODE_VERIFIER.as_bytes()));
    let url_auth = format!(
        "{backend_url}/oidc/authorize?client_id={client_id}&redirect_uri={REDIRECT_URI}\
        &response_type=code&code_challenge={challenge}&code_challenge_method=S256"
    );
    let res = http
        .post(&url_auth)
        .headers(headers)
        .json(&LoginRequest {
            email: Some(USERNAME.to_string()),
            password: Some(PASSWORD.to_string()),
            pow: get_solved_pow().await,
            client_id: client_id.to_string(),
            redirect_uri: REDIRECT_URI.to_string(),
            scopes: None,
            state: None,
            nonce: None,
            code_challenge: Some(challenge),
            code_challenge_method: Some("S256".to_string()),
            resource: None,
            resident_key_token: None,
            fwda: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 202);

    let (code, _) = code_state_from_headers(res)?;
    Ok(code)
}

async fn exchange_code(client_id: &str, code: String) -> reqwest::Response {
    reqwest::Client::new()
        .post(format!("{}/oidc/token", get_backend_url()))
        .form(&TokenRequest {
            grant_type: GrantType::AuthorizationCode,
            code: Some(code),
            redirect_uri: Some(REDIRECT_URI.to_string()),
            client_id: Some(client_id.to_string()),
            code_verifier: Some(CODE_VERIFIER.to_string()),
            ..Default::default()
        })
        .send()
        .await
        .unwrap()
}

/// Rotates the secret of `init_client`. Other tests rotate it as well, so `CLIENT_SECRET` cannot
/// be relied on.
async fn init_client_secret() -> Result<String, Box<dyn Error>> {
    let res = reqwest::Client::new()
        .post(format!("{}/clients/{CLIENT_ID}/secret", get_backend_url()))
        .headers(get_auth_headers().await?)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    Ok(res
        .json::<ClientSecretResponse>()
        .await?
        .secret
        .expect("a confidential client"))
}

/// A sessionless token set from the password grant for `init_client`.
async fn password_tokens(secret: &str) -> Result<TokenSet, Box<dyn Error>> {
    let res = reqwest::Client::new()
        .post(format!("{}/oidc/token", get_backend_url()))
        .form(&TokenRequest {
            grant_type: GrantType::Password,
            client_id: Some(CLIENT_ID.to_string()),
            client_secret: Some(secret.to_string()),
            username: Some(USERNAME.to_string()),
            password: Some(PASSWORD.to_string()),
            ..Default::default()
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    Ok(res.json().await?)
}

async fn refresh(client_id: &str, secret: Option<&str>, tokens: &TokenSet) -> reqwest::Response {
    reqwest::Client::new()
        .post(format!("{}/oidc/token", get_backend_url()))
        .form(&TokenRequest {
            grant_type: GrantType::RefreshToken,
            client_id: Some(client_id.to_string()),
            client_secret: secret.map(String::from),
            refresh_token: tokens.refresh_token.clone(),
            ..Default::default()
        })
        .send()
        .await
        .unwrap()
}

/// An auth code issued before the client was deleted and recreated with the same id must not be
/// exchangeable. This fails if new clients do not get a fresh generation.
#[tokio::test]
async fn test_recreated_client_rejects_old_auth_code() -> Result<(), Box<dyn Error>> {
    let id = "binding-auth-code";
    create_client(id).await?;

    // sanity check: the flow works for this client
    let code = auth_code(id).await?;
    assert_eq!(exchange_code(id, code).await.status(), 200);

    let code = auth_code(id).await?;
    delete_client(id).await?;
    create_client(id).await?;
    assert_eq!(exchange_code(id, code).await.status(), 401);

    delete_client(id).await?;
    Ok(())
}

/// Deleting a client deletes its refresh tokens, and only those.
#[tokio::test]
async fn test_client_delete_drops_its_refresh_tokens() -> Result<(), Box<dyn Error>> {
    let id = "binding-refresh-token";
    create_client(id).await?;
    let (_, tokens) = session_headers_for_client(id, None, REDIRECT_URI).await;
    let secret = init_client_secret().await?;
    let other = password_tokens(&secret).await?;

    delete_client(id).await?;
    create_client(id).await?;
    // 404 means the row is gone, a stale row of the old generation would be a 401
    assert_eq!(refresh(id, None, &tokens).await.status(), 404);
    assert_eq!(
        refresh(CLIENT_ID, Some(&secret), &other).await.status(),
        200
    );

    delete_client(id).await?;
    Ok(())
}

/// Sessionless refresh tokens are kept until their replacement is stored, but must still be
/// usable only once.
#[tokio::test]
async fn test_sessionless_refresh_is_single_use() -> Result<(), Box<dyn Error>> {
    let secret = init_client_secret().await?;
    let original = password_tokens(&secret).await?;

    let res = refresh(CLIENT_ID, Some(&secret), &original).await;
    assert_eq!(res.status(), 200);
    let rotated: TokenSet = res.json().await?;
    assert_eq!(
        refresh(CLIENT_ID, Some(&secret), &original).await.status(),
        404
    );

    let (first, second) = tokio::join!(
        refresh(CLIENT_ID, Some(&secret), &rotated),
        refresh(CLIENT_ID, Some(&secret), &rotated)
    );
    let mut statuses = [first.status().as_u16(), second.status().as_u16()];
    statuses.sort_unstable();
    assert_eq!(statuses, [200, 404]);
    assert_eq!(
        refresh(CLIENT_ID, Some(&secret), &rotated).await.status(),
        404
    );

    Ok(())
}
