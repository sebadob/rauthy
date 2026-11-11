use crate::common::{PASSWORD, USERNAME, decode_claims, get_auth_headers, get_backend_url};
use pretty_assertions::assert_eq;
use rauthy_api_types::clients::{
    ClientResponse, ClientSecretResponse, NewClientRequest, UpdateClientRequest,
};
use rauthy_api_types::generic::Language;
use rauthy_api_types::oidc::{GrantType, JwkKeyPairAlg, TokenRequest, TokenType};
use rauthy_api_types::users::{NewUserRequest, UpdateUserRequest, UserResponse};
use rauthy_service::token_set::TokenSet;
use reqwest::header::HeaderMap;
use std::error::Error;

mod common;

const ID: &str = "exchange_test";
const TARGET: &str = "https://rs-downstream.example.com/api";
const TARGET_FORBIDDEN: &str = "https://rs-forbidden.example.com/api";
// `zzb_handler_password_policy` runs before this test and leaves a strict policy behind:
// at least 2 lower case, 3 upper case, 4 digits and 5 special characters
const ACTOR_PASSWORD: &str = "Exch4nge!Act0r#2026$%&XYZ";

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
    password_token_for(http, id, secret, USERNAME, PASSWORD).await
}

async fn password_token_for(
    http: &reqwest::Client,
    id: &str,
    secret: &str,
    email: &str,
    password: &str,
) -> TokenSet {
    let res = http
        .post(format!("{}/oidc/token", get_backend_url()))
        .form(&TokenRequest {
            grant_type: GrantType::Password,
            client_id: Some(id.to_string()),
            client_secret: Some(secret.to_string()),
            username: Some(email.to_string()),
            password: Some(password.to_string()),
            ..Default::default()
        })
        .send()
        .await
        .expect("the test backend to be running");
    if !res.status().is_success() {
        let text = res.text().await.unwrap();
        panic!("Error during password login of '{email}' for '{id}':\n{text}");
    }
    res.json::<TokenSet>().await.unwrap()
}

/// Creates a user that can log in via the `password` grant, to have an acting party with a
/// `sub` different from the test user's. A left-over from an aborted run is removed first,
/// because the password policy would refuse to set the same password for it again.
async fn create_actor(
    http: &reqwest::Client,
    admin: &HeaderMap,
    email: &str,
) -> Result<UserResponse, Box<dyn Error>> {
    let backend_url = get_backend_url();
    let res = http
        .get(format!("{backend_url}/users/email/{email}"))
        .headers(admin.clone())
        .send()
        .await?;
    if res.status().is_success() {
        let left_over: UserResponse = res.json().await?;
        let res = http
            .delete(format!("{backend_url}/users/{}", left_over.id))
            .headers(admin.clone())
            .send()
            .await?;
        assert_eq!(res.status(), 204);
    }

    let res = http
        .post(format!("{backend_url}/users"))
        .headers(admin.clone())
        .json(&NewUserRequest {
            given_name: Some("Exchange".to_string()),
            family_name: Some("Actor".to_string()),
            email: email.to_string(),
            preferred_username: None,
            language: Language::En,
            roles: vec!["user".to_string()],
            groups: None,
            user_expires: None,
            tz: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let user: UserResponse = res.json().await?;

    let res = http
        .put(format!("{backend_url}/users/{}", user.id))
        .headers(admin.clone())
        .json(&UpdateUserRequest {
            email: user.email.clone(),
            given_name: user.given_name.clone(),
            family_name: user.family_name.clone(),
            language: Some(Language::En),
            password: Some(ACTOR_PASSWORD.to_string()),
            roles: user.roles.clone(),
            groups: user.groups.clone(),
            enabled: true,
            email_verified: true,
            user_expires: user.user_expires,
            user_values: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    Ok(user)
}

/// End-to-end coverage for the RFC 8693 token exchange: deny-by-default via
/// `flows_enabled`, the `allowed_resources` target allow-list, impersonation vs
/// delegation (`act`), scope narrowing, and the guarantee that an exchanged token never
/// carries a refresh or id token. On top of that: the nested `act` delegation chain.
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
    // no requested `scope`: the exchanged token inherits the subject's scope
    assert_eq!(ts.scope.as_deref(), subject_claims["scope"].as_str());
    assert_eq!(ts.scope.as_deref(), claims["scope"].as_str());
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
    assert_eq!(ts.scope.as_deref(), subject_claims["scope"].as_str());
    assert_eq!(ts.scope.as_deref(), claims["scope"].as_str());

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
    assert_eq!(ts.scope.as_deref(), Some(first_scope.as_str()));

    // (11) the exchanging client only gets its own scopes, and never outlives the subject token
    let flows = vec![
        GrantType::ClientCredentials,
        GrantType::Password,
        GrantType::RefreshToken,
        GrantType::TokenExchange,
    ];
    // the password grant always issues the `default_scopes`, so `email` must be one of them
    let res = client
        .put(format!("{backend_url}/clients/{ID}"))
        .headers(auth_headers.clone())
        .json(&UpdateClientRequest {
            default_scopes: vec!["openid".to_string(), "email".to_string()],
            ..base_update(flows.clone())
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let res = client
        .post(&url_token)
        .form(&TokenRequest {
            grant_type: GrantType::Password,
            client_id: Some(ID.to_string()),
            client_secret: Some(secret.clone()),
            username: Some(USERNAME.to_string()),
            password: Some(PASSWORD.to_string()),
            ..Default::default()
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let wide_subject: TokenSet = res.json().await?;
    let wide_claims = decode_claims(&wide_subject.access_token);
    assert!(
        wide_claims["scope"].as_str().unwrap().contains("email"),
        "{}",
        wide_claims["scope"]
    );

    // the client loses `email` and gets a longer lifetime than the subject token has left
    let res = client
        .put(format!("{backend_url}/clients/{ID}"))
        .headers(auth_headers.clone())
        .json(&UpdateClientRequest {
            scopes: vec!["openid".to_string()],
            access_token_lifetime: 600,
            ..base_update(flows.clone())
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let exchange = || TokenRequest {
        grant_type: GrantType::TokenExchange,
        client_id: Some(ID.to_string()),
        client_secret: Some(secret.clone()),
        subject_token: Some(wide_subject.access_token.clone()),
        subject_token_type: Some(TokenType::AccessToken.to_string()),
        audience: Some(TARGET.to_string()),
        ..Default::default()
    };

    // inherited scopes the client is not allowed are dropped
    let res = client.post(&url_token).form(&exchange()).send().await?;
    assert_eq!(res.status(), 200);
    let ts: TokenSet = res.json().await?;
    let claims = decode_claims(&ts.access_token);
    assert_eq!(ts.scope.as_deref(), Some("openid"));
    assert_eq!(claims["scope"], serde_json::json!("openid"));
    assert!(
        claims["exp"].as_i64().unwrap() <= wide_claims["exp"].as_i64().unwrap(),
        "exchanged exp {} outlives the subject exp {}",
        claims["exp"],
        wide_claims["exp"]
    );

    // explicitly requesting a scope the client is not allowed is rejected
    let res = client
        .post(&url_token)
        .form(&TokenRequest {
            scope: Some("email".to_string()),
            ..exchange()
        })
        .send()
        .await?;
    assert_eq!(res.status(), 400);

    // (12) the client's `default_scopes` are allowed as well, same as on login
    let res = client
        .put(format!("{backend_url}/clients/{ID}"))
        .headers(auth_headers.clone())
        .json(&UpdateClientRequest {
            scopes: vec!["openid".to_string()],
            default_scopes: vec!["openid".to_string(), "email".to_string()],
            access_token_lifetime: 600,
            ..base_update(flows)
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let res = client.post(&url_token).form(&exchange()).send().await?;
    assert_eq!(res.status(), 200);
    let ts: TokenSet = res.json().await?;
    assert!(
        ts.scope.as_deref().unwrap().contains("email"),
        "{:?}",
        ts.scope
    );

    // (13) the exchanged token never outlives its user
    let res = client
        .get(format!("{backend_url}/users/email/{USERNAME}"))
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let user: UserResponse = res.json().await?;
    let user_expires = chrono::Utc::now().timestamp() + 60;
    let patch_user = |body: serde_json::Value| {
        client
            .patch(format!("{backend_url}/users/{}", user.id))
            .headers(auth_headers.clone())
            .json(&body)
            .send()
    };
    let res = patch_user(serde_json::json!({
        "put": [{ "key": "user_expires", "value": user_expires }],
        "del": [],
    }))
    .await?;
    assert_eq!(res.status(), 200);
    let res = client.post(&url_token).form(&exchange()).send().await;
    // restore the user before asserting anything, so a failure cannot leave it expiring
    let restore = match user.user_expires {
        Some(ts) => {
            serde_json::json!({ "put": [{ "key": "user_expires", "value": ts }], "del": [] })
        }
        None => serde_json::json!({ "put": [], "del": ["user_expires"] }),
    };
    assert_eq!(patch_user(restore).await?.status(), 200);
    let res = res?;
    assert_eq!(res.status(), 200);
    let ts: TokenSet = res.json().await?;
    let claims = decode_claims(&ts.access_token);
    assert!(
        claims["exp"].as_i64().unwrap() <= user_expires,
        "exchanged exp {} outlives the user expiry {user_expires}",
        claims["exp"]
    );
    assert!(ts.expires_in <= 60, "expires_in {}", ts.expires_in);

    // (14) a two-hop delegation chain: the subject's `act` is nested below the new actor
    let actor_a = create_actor(&client, &auth_headers, "exchange_actor_a@localhost").await?;
    let actor_b = create_actor(&client, &auth_headers, "exchange_actor_b@localhost").await?;
    let token_a = password_token_for(&client, ID, &secret, &actor_a.email, ACTOR_PASSWORD)
        .await
        .access_token;
    let token_b = password_token_for(&client, ID, &secret, &actor_b.email, ACTOR_PASSWORD)
        .await
        .access_token;
    assert_ne!(actor_a.id, actor_b.id);
    assert_ne!(subject_claims["sub"], serde_json::json!(actor_a.id));

    let delegate = |subject: &str, actor: Option<&str>| {
        client
            .post(&url_token)
            .form(&TokenRequest {
                grant_type: GrantType::TokenExchange,
                client_id: Some(ID.to_string()),
                client_secret: Some(secret.clone()),
                subject_token: Some(subject.to_string()),
                subject_token_type: Some(TokenType::AccessToken.to_string()),
                actor_token: actor.map(String::from),
                actor_token_type: actor.map(|_| TokenType::AccessToken.to_string()),
                audience: Some(TARGET.to_string()),
                ..Default::default()
            })
            .send()
    };

    let res = delegate(&subject_token, Some(&token_a)).await?;
    assert_eq!(res.status(), 200);
    let hop_1: TokenSet = res.json().await?;
    let claims = decode_claims(&hop_1.access_token);
    assert_eq!(claims["sub"], subject_claims["sub"]);
    assert_eq!(claims["act"], serde_json::json!({ "sub": actor_a.id }));

    let res = delegate(&hop_1.access_token, Some(&token_b)).await?;
    assert_eq!(res.status(), 200);
    let hop_2: TokenSet = res.json().await?;
    let claims = decode_claims(&hop_2.access_token);
    assert_eq!(claims["sub"], subject_claims["sub"]);
    // the current actor is the outermost one, the prior actor is nested
    let chain = serde_json::json!({ "sub": actor_b.id, "act": { "sub": actor_a.id } });
    assert_eq!(claims["act"], chain);

    // an actor that is acting for someone itself brings its own chain along
    let res = delegate(&token_a, Some(&hop_1.access_token)).await?;
    assert_eq!(res.status(), 200);
    let ts: TokenSet = res.json().await?;
    let claims = decode_claims(&ts.access_token);
    assert_eq!(claims["sub"], serde_json::json!(actor_a.id));
    assert_eq!(
        claims["act"],
        serde_json::json!({ "sub": subject_claims["sub"], "act": { "sub": actor_a.id } })
    );

    // (15) impersonation keeps the chain: no actor is added, but none is hidden either
    let res = delegate(&hop_2.access_token, None).await?;
    assert_eq!(res.status(), 200);
    let ts: TokenSet = res.json().await?;
    let claims = decode_claims(&ts.access_token);
    assert_eq!(claims["sub"], subject_claims["sub"]);
    assert_eq!(claims["act"], chain);

    // (16) a conflict is rejected: both tokens carry an `act`, which cannot be merged
    let res = delegate(&hop_2.access_token, Some(&hop_1.access_token)).await?;
    assert_eq!(res.status(), 400);
    let body = res.text().await?;
    assert!(
        body.contains("both carry an 'act'"),
        "unexpected body: {body}"
    );

    // cleanup
    for id in [&actor_a.id, &actor_b.id] {
        let res = client
            .delete(format!("{backend_url}/users/{id}"))
            .headers(auth_headers.clone())
            .send()
            .await?;
        assert_eq!(res.status(), 204);
    }
    let res = client
        .delete(format!("{backend_url}/clients/{ID}"))
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    Ok(())
}
