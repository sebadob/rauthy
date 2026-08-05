use crate::common::{PASSWORD, USERNAME, get_auth_headers, get_backend_url};
use pretty_assertions::assert_eq;
use rauthy_api_types::clients::{
    ClientResponse, ClientSecretResponse, NewClientRequest, UpdateClientRequest,
};
use rauthy_api_types::oidc::{GrantType, JwkKeyPairAlg, TokenRequest, TokenType};
use rauthy_common::utils::base64_url_no_pad_decode;
use rauthy_service::token_set::TokenSet;
use std::error::Error;

mod common;

const ID: &str = "exchange_test";
const TARGET: &str = "https://rs-downstream.example.com/api";
const TARGET_FORBIDDEN: &str = "https://rs-forbidden.example.com/api";

/// Decodes the (unverified) JWT payload, so a test can assert on any claim.
fn decode_claims(access_token: &str) -> serde_json::Value {
    let payload_b64 = access_token
        .split('.')
        .nth(1)
        .expect("a JWT payload segment");
    let bytes = base64_url_no_pad_decode(payload_b64).expect("valid base64url payload");
    serde_json::from_slice(&bytes).expect("valid JSON claims")
}

fn base_update(flows: Vec<GrantType>) -> UpdateClientRequest {
    UpdateClientRequest {
        name: Some("Exchange Test".to_string()),
        confidential: true,
        redirect_uris: vec!["http://localhost/callback".to_string()],
        post_logout_redirect_uris: None,
        allowed_origins: None,
        enabled: true,
        flows_enabled: flows,
        access_token_alg: JwkKeyPairAlg::EdDSA,
        id_token_alg: JwkKeyPairAlg::EdDSA,
        auth_code_lifetime: 60,
        access_token_lifetime: 300,
        scopes: vec!["openid".to_string(), "email".to_string()],
        default_scopes: vec!["openid".to_string()],
        challenges: Some(vec!["S256".to_string()]),
        force_mfa: false,
        client_uri: None,
        contacts: None,
        backchannel_logout_uri: None,
        restrict_group_prefix: None,
        claims: None,
        claims_at_root: false,
        allowed_resources: Some(vec![TARGET.to_string()]),
        default_aud: None,
        scim: None,
    }
}

/// Mints a real access token for the test user via the `password` grant on the given client.
/// This test deliberately does not use `common::get_token_set_init_client`, because
/// `zzd_handler_clients` rotates the `init_client` secret and this test runs after it.
async fn password_token(http: &reqwest::Client, id: &str, secret: &str) -> TokenSet {
    let res = http
        .post(format!("{}/oidc/token", get_backend_url()))
        .form(&TokenRequest {
            grant_type: GrantType::Password,
            client_id: Some(id.to_string()),
            client_secret: Some(secret.to_string()),
            username: Some(USERNAME.to_string()),
            password: Some(PASSWORD.to_string()),
            ..Default::default()
        })
        .send()
        .await
        .expect("the test backend to be running");
    if !res.status().is_success() {
        let text = res.text().await.unwrap();
        panic!("Error during password login for '{id}':\n{text}");
    }
    res.json::<TokenSet>().await.unwrap()
}

/// End-to-end coverage for the RFC 8693 token exchange: deny-by-default via
/// `flows_enabled`, the `allowed_resources` target allow-list, impersonation vs
/// delegation (`act`), scope narrowing, and the guarantee that an exchanged token never
/// carries a refresh or id token.
#[tokio::test]
async fn test_token_exchange() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();

    // (0) create the exchanging client, WITHOUT the token exchange flow at first
    let new_client = NewClientRequest {
        id: ID.to_string(),
        secret: None,
        name: Some("Exchange Test".to_string()),
        confidential: true,
        redirect_uris: vec!["http://localhost/callback".to_string()],
        post_logout_redirect_uris: None,
    };
    let res = client
        .post(format!("{backend_url}/clients"))
        .headers(auth_headers.clone())
        .json(&new_client)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let _: ClientResponse = res.json().await?;

    let res = client
        .put(format!("{backend_url}/clients/{ID}"))
        .headers(auth_headers.clone())
        .json(&base_update(vec![
            GrantType::ClientCredentials,
            GrantType::Password,
            GrantType::RefreshToken,
        ]))
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let res = client
        .post(format!("{backend_url}/clients/{ID}/secret"))
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let secret: ClientSecretResponse = res.json().await?;
    let secret = secret.secret.unwrap();

    // a real access token for the test user, used as the subject token
    let subject_ts = password_token(&client, ID, &secret).await;
    let subject_token = subject_ts.access_token.clone();

    let url_token = format!("{backend_url}/oidc/token");

    // (1) deny-by-default: the flow is not enabled for this client yet
    let res = client
        .post(&url_token)
        .form(&TokenRequest {
            grant_type: GrantType::TokenExchange,
            client_id: Some(ID.to_string()),
            client_secret: Some(secret.clone()),
            subject_token: Some(subject_token.clone()),
            subject_token_type: Some(TokenType::AccessToken.to_string()),
            audience: Some(TARGET.to_string()),
            ..Default::default()
        })
        .send()
        .await?;
    assert_eq!(res.status(), 400);
    let body = res.text().await?;
    assert!(
        body.contains("token_exchange") || body.contains("token-exchange"),
        "unexpected body: {body}"
    );

    // now enable the flow
    let res = client
        .put(format!("{backend_url}/clients/{ID}"))
        .headers(auth_headers.clone())
        .json(&base_update(vec![
            GrantType::ClientCredentials,
            GrantType::Password,
            GrantType::RefreshToken,
            GrantType::TokenExchange,
        ]))
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // (2) impersonation: no actor token -> no `act` claim, `sub` stays the subject's
    let res = client
        .post(&url_token)
        .form(&TokenRequest {
            grant_type: GrantType::TokenExchange,
            client_id: Some(ID.to_string()),
            client_secret: Some(secret.clone()),
            subject_token: Some(subject_token.clone()),
            subject_token_type: Some(TokenType::AccessToken.to_string()),
            audience: Some(TARGET.to_string()),
            ..Default::default()
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let ts: TokenSet = res.json().await?;

    // an exchanged token never carries a refresh or id token
    assert!(ts.refresh_token.is_none());
    assert!(ts.id_token.is_none());

    let claims = decode_claims(&ts.access_token);
    let subject_claims = decode_claims(&subject_token);
    assert_eq!(claims["sub"], subject_claims["sub"]);
    assert!(
        claims.get("act").is_none(),
        "impersonation must not set `act`"
    );
    // the target ends up in the audience, and the exchanging client is the `azp`
    assert!(
        claims["aud"].to_string().contains(TARGET),
        "unexpected aud: {}",
        claims["aud"]
    );
    assert_eq!(claims["azp"], serde_json::json!(ID));

    // (3) a target outside `allowed_resources` -> `invalid_target`
    let res = client
        .post(&url_token)
        .form(&TokenRequest {
            grant_type: GrantType::TokenExchange,
            client_id: Some(ID.to_string()),
            client_secret: Some(secret.clone()),
            subject_token: Some(subject_token.clone()),
            subject_token_type: Some(TokenType::AccessToken.to_string()),
            audience: Some(TARGET_FORBIDDEN.to_string()),
            ..Default::default()
        })
        .send()
        .await?;
    assert_eq!(res.status(), 400);
    let body = res.text().await?;
    assert!(body.contains("invalid_target"), "unexpected body: {body}");

    // (4) delegation: an actor token adds the `act` claim
    let actor_ts = password_token(&client, ID, &secret).await;
    let res = client
        .post(&url_token)
        .form(&TokenRequest {
            grant_type: GrantType::TokenExchange,
            client_id: Some(ID.to_string()),
            client_secret: Some(secret.clone()),
            subject_token: Some(subject_token.clone()),
            subject_token_type: Some(TokenType::AccessToken.to_string()),
            actor_token: Some(actor_ts.access_token.clone()),
            actor_token_type: Some(TokenType::AccessToken.to_string()),
            audience: Some(TARGET.to_string()),
            ..Default::default()
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let ts: TokenSet = res.json().await?;
    let claims = decode_claims(&ts.access_token);
    let actor_claims = decode_claims(&actor_ts.access_token);
    assert_eq!(claims["sub"], subject_claims["sub"]);
    assert_eq!(claims["act"]["sub"], actor_claims["sub"]);

    // (5) an `actor_token` without its `actor_token_type` is rejected
    let res = client
        .post(&url_token)
        .form(&TokenRequest {
            grant_type: GrantType::TokenExchange,
            client_id: Some(ID.to_string()),
            client_secret: Some(secret.clone()),
            subject_token: Some(subject_token.clone()),
            subject_token_type: Some(TokenType::AccessToken.to_string()),
            actor_token: Some(actor_ts.access_token.clone()),
            audience: Some(TARGET.to_string()),
            ..Default::default()
        })
        .send()
        .await?;
    assert_eq!(res.status(), 400);

    // (6) a refresh token as the subject token is rejected: access tokens only
    let refresh = subject_ts.refresh_token.clone().unwrap();
    let res = client
        .post(&url_token)
        .form(&TokenRequest {
            grant_type: GrantType::TokenExchange,
            client_id: Some(ID.to_string()),
            client_secret: Some(secret.clone()),
            subject_token: Some(refresh),
            subject_token_type: Some(TokenType::AccessToken.to_string()),
            audience: Some(TARGET.to_string()),
            ..Default::default()
        })
        .send()
        .await?;
    assert_eq!(res.status(), 400);
    let body = res.text().await?;
    assert!(body.contains("invalid_grant"), "unexpected body: {body}");

    // (7) a garbage subject token is rejected
    let res = client
        .post(&url_token)
        .form(&TokenRequest {
            grant_type: GrantType::TokenExchange,
            client_id: Some(ID.to_string()),
            client_secret: Some(secret.clone()),
            subject_token: Some("not.a.token".to_string()),
            subject_token_type: Some(TokenType::AccessToken.to_string()),
            audience: Some(TARGET.to_string()),
            ..Default::default()
        })
        .send()
        .await?;
    assert_eq!(res.status(), 400);

    // (8) an unsupported `subject_token_type` is rejected
    let res = client
        .post(&url_token)
        .form(&TokenRequest {
            grant_type: GrantType::TokenExchange,
            client_id: Some(ID.to_string()),
            client_secret: Some(secret.clone()),
            subject_token: Some(subject_token.clone()),
            subject_token_type: Some("urn:ietf:params:oauth:token-type:id_token".to_string()),
            audience: Some(TARGET.to_string()),
            ..Default::default()
        })
        .send()
        .await?;
    assert_eq!(res.status(), 400);

    // (9) scope narrowing: a scope the subject token does not have is rejected
    let res = client
        .post(&url_token)
        .form(&TokenRequest {
            grant_type: GrantType::TokenExchange,
            client_id: Some(ID.to_string()),
            client_secret: Some(secret.clone()),
            subject_token: Some(subject_token.clone()),
            subject_token_type: Some(TokenType::AccessToken.to_string()),
            audience: Some(TARGET.to_string()),
            scope: Some("openid some_scope_the_subject_never_had".to_string()),
            ..Default::default()
        })
        .send()
        .await?;
    assert_eq!(res.status(), 400);

    // (10) narrowing to a subset of the subject's scopes works
    let subject_scope = subject_claims["scope"].as_str().unwrap_or_default();
    let first_scope = subject_scope.split_whitespace().next().unwrap().to_string();
    let res = client
        .post(&url_token)
        .form(&TokenRequest {
            grant_type: GrantType::TokenExchange,
            client_id: Some(ID.to_string()),
            client_secret: Some(secret.clone()),
            subject_token: Some(subject_token.clone()),
            subject_token_type: Some(TokenType::AccessToken.to_string()),
            audience: Some(TARGET.to_string()),
            scope: Some(first_scope.clone()),
            ..Default::default()
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let ts: TokenSet = res.json().await?;
    let claims = decode_claims(&ts.access_token);
    assert_eq!(claims["scope"], serde_json::json!(first_scope));

    // cleanup
    let res = client
        .delete(format!("{backend_url}/clients/{ID}"))
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    Ok(())
}
