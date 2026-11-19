use crate::common::{
    PASSWORD, USERNAME, decode_claims, get_auth_headers, get_backend_url, session_headers,
    session_headers_for_client,
};
use chrono::Utc;
use ed25519_compact::Noise;
use pretty_assertions::assert_eq;
use rauthy_api_types::clients::{
    ClientResponse, ClientSecretResponse, NewClientRequest, UpdateClientRequest,
};
use rauthy_api_types::oidc::{GrantType, JwkKeyPairAlg, LogoutRequest, TokenRequest, TokenType};
use rauthy_api_types::users::UserResponse;
use rauthy_common::constants::{HEADER_DPOP_NONCE, TOKEN_DPOP};
use rauthy_common::utils::{base64_url_encode, base64_url_no_pad_decode, base64_url_no_pad_encode};
use rauthy_data::entity::dpop_proof::{DPoPClaims, DPoPHeader};
use rauthy_data::entity::jwk::{JWKSPublicKey, JwkKeyPairType};
use rauthy_jwt::claims::JwtTokenType;
use rauthy_service::token_set::TokenSet;
use std::error::Error;

mod common;

const ID: &str = "exchange_test";
const TARGET: &str = "https://rs-downstream.example.com/api";
const TARGET_FORBIDDEN: &str = "https://rs-forbidden.example.com/api";

const ID_JAG: &str = "exchange_id_jag_test";
const ID_JAG_OTHER: &str = "exchange_id_jag_other";
const JAG_AUDIENCE: &str = "https://as.downstream.example.com/";
const JAG_RESOURCE: &str = "https://mcp.downstream.example.com/mcp";

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
        consent: None,
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
        consent: None,
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
    assert_eq!(ts.issued_token_type, Some(TokenType::AccessToken));
    assert!(subject_ts.issued_token_type.is_none());

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

    // cleanup
    let res = client
        .delete(format!("{backend_url}/clients/{ID}"))
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    Ok(())
}

fn id_jag_update(access_token_lifetime: i32) -> UpdateClientRequest {
    UpdateClientRequest {
        scopes: vec!["openid".to_string(), "email".to_string()],
        default_scopes: vec!["openid".to_string(), "email".to_string()],
        access_token_lifetime,
        ..base_update(vec![
            GrantType::AuthorizationCode,
            GrantType::Password,
            GrantType::TokenExchange,
        ])
    }
}

async fn id_jag_client(http: &reqwest::Client, id: &str) -> Result<String, Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();

    let res = http
        .post(format!("{backend_url}/clients"))
        .headers(auth_headers.clone())
        .json(&NewClientRequest {
            id: id.to_string(),
            secret: None,
            name: Some("ID-JAG Test".to_string()),
            confidential: true,
            redirect_uris: vec!["http://localhost/callback".to_string()],
            post_logout_redirect_uris: None,
            consent: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let res = http
        .put(format!("{backend_url}/clients/{id}"))
        .headers(auth_headers.clone())
        .json(&id_jag_update(300))
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let res = http
        .post(format!("{backend_url}/clients/{id}/secret"))
        .headers(auth_headers)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let secret: ClientSecretResponse = res.json().await?;
    Ok(secret.secret.unwrap())
}

/// A token set from the authorization code flow, so its id token has a session.
async fn id_jag_login(id: &str, secret: &str) -> TokenSet {
    session_headers_for_client(id, Some(secret), "http://localhost/callback")
        .await
        .1
}

fn decode_header(token: &str) -> serde_json::Value {
    let header = token.split('.').next().unwrap();
    serde_json::from_slice(&base64_url_no_pad_decode(header).unwrap()).unwrap()
}

/// Returns the DPoP proof and the JWK thumbprint of its key.
fn dpop_proof(kp: &ed25519_compact::KeyPair, nonce: Option<String>) -> (String, String) {
    let header = DPoPHeader {
        typ: "dpop+jwt".to_string(),
        alg: rauthy_data::entity::jwk::JwkKeyPairAlg::EdDSA,
        jwk: JWKSPublicKey {
            kty: JwkKeyPairType::OKP,
            alg: None,
            crv: Some("Ed25519".to_string()),
            kid: None,
            n: None,
            e: None,
            x: Some(base64_url_encode(kp.pk.as_slice())),
        },
        kid: None,
    };
    let claims = DPoPClaims {
        jti: base64_url_encode(Noise::generate().as_slice()),
        htm: "POST".to_string(),
        htu: format!("{}/oidc/token", get_backend_url()),
        iat: Utc::now().timestamp(),
        nonce,
    };
    let mut proof = format!(
        "{}.{}",
        base64_url_no_pad_encode(serde_json::to_string(&header).unwrap().as_bytes()),
        base64_url_no_pad_encode(serde_json::to_string(&claims).unwrap().as_bytes()),
    );
    let sig = kp.sk.sign(&proof, Some(Noise::generate()));
    proof = format!("{proof}.{}", base64_url_no_pad_encode(sig.as_ref()));
    (proof, header.jwk.fingerprint().unwrap())
}

#[tokio::test]
async fn test_token_exchange_id_jag() -> Result<(), Box<dyn Error>> {
    let http = reqwest::Client::new();
    let secret = id_jag_client(&http, ID_JAG).await?;
    let secret_other = id_jag_client(&http, ID_JAG_OTHER).await?;
    let url_token = format!("{}/oidc/token", get_backend_url());

    let login = id_jag_login(ID_JAG, &secret).await;
    let id_token = login.id_token.clone().unwrap();

    // the ID-JAG must not outlive the id token, even with a longer client lifetime
    let res = http
        .put(format!("{}/clients/{ID_JAG}", get_backend_url()))
        .headers(get_auth_headers().await?)
        .json(&id_jag_update(600))
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let id_jag_req = |subject_token: &str| TokenRequest {
        grant_type: GrantType::TokenExchange,
        client_id: Some(ID_JAG.to_string()),
        client_secret: Some(secret.clone()),
        subject_token: Some(subject_token.to_string()),
        subject_token_type: Some(TokenType::IdToken.to_string()),
        requested_token_type: Some(TokenType::IdJag.to_string()),
        audience: Some(JAG_AUDIENCE.to_string()),
        resource: Some(JAG_RESOURCE.to_string()),
        scope: Some("chat.read chat.history".to_string()),
        ..Default::default()
    };
    let rejected = async |req: TokenRequest, error: &str| {
        let res = http.post(&url_token).form(&req).send().await.unwrap();
        assert_eq!(res.status(), 400);
        let body = res.text().await.unwrap();
        assert!(body.contains(error), "expected '{error}', got: {body}");
    };

    let res = http
        .post(&url_token)
        .form(&id_jag_req(&id_token))
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let body: serde_json::Value = res.json().await?;
    assert_eq!(
        body["issued_token_type"],
        "urn:ietf:params:oauth:token-type:id-jag"
    );
    assert_eq!(body["token_type"], "N_A");
    assert_eq!(body["scope"], "chat.read chat.history");
    assert!(body.get("refresh_token").is_none());
    assert!(body.get("id_token").is_none());
    let ts: TokenSet = serde_json::from_value(body)?;
    assert_eq!(ts.token_type, JwtTokenType::NA);
    let id_jag = ts.access_token;

    let header = decode_header(&id_jag);
    assert_eq!(header["typ"], "oauth-id-jag+jwt");
    assert_eq!(header["alg"], "RS256");
    let claims = decode_claims(&id_jag);
    let id_claims = decode_claims(&id_token);
    assert_eq!(claims["iss"], id_claims["iss"]);
    assert_eq!(claims["sub"], id_claims["sub"]);
    assert_eq!(claims["aud"], JAG_AUDIENCE);
    assert_eq!(claims["client_id"], ID_JAG);
    assert_eq!(claims["resource"], JAG_RESOURCE);
    assert_eq!(claims["scope"], "chat.read chat.history");
    assert_eq!(claims["auth_time"], id_claims["auth_time"]);
    assert_eq!(claims["amr"], id_claims["amr"]);
    assert_eq!(claims["email"], USERNAME);
    assert!(claims["jti"].is_string());
    assert!(claims.get("typ").is_none());
    assert!(claims.get("cnf").is_none());
    assert!(claims["exp"].as_i64().unwrap() <= id_claims["exp"].as_i64().unwrap());

    // the ID-JAG is never accepted by Rauthy itself
    rejected(id_jag_req(&id_jag), "invalid_grant").await;
    let res = http
        .get(format!("{}/oidc/userinfo", get_backend_url()))
        .bearer_auth(&id_jag)
        .send()
        .await?;
    assert_eq!(res.status(), 400);

    // only a valid id token with a session, issued to this client
    rejected(id_jag_req(&login.access_token), "invalid_grant").await;
    rejected(
        TokenRequest {
            subject_token_type: Some(TokenType::AccessToken.to_string()),
            ..id_jag_req(&id_token)
        },
        "BadRequest",
    )
    .await;
    let (_, other) = session_headers().await;
    rejected(
        id_jag_req(other.id_token.as_ref().unwrap()),
        "invalid_grant",
    )
    .await;
    let res = http
        .post(&url_token)
        .form(&TokenRequest {
            grant_type: GrantType::Password,
            client_id: Some(ID_JAG.to_string()),
            client_secret: Some(secret.clone()),
            username: Some(USERNAME.to_string()),
            password: Some(PASSWORD.to_string()),
            ..Default::default()
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let password_ts: TokenSet = res.json().await?;
    rejected(
        id_jag_req(password_ts.id_token.as_ref().unwrap()),
        "no valid session",
    )
    .await;

    // `[[id_jag.audiences]]` decides about the audience, client, resource and scope
    rejected(
        TokenRequest {
            audience: None,
            ..id_jag_req(&id_token)
        },
        "BadRequest",
    )
    .await;
    rejected(
        TokenRequest {
            audience: Some(TARGET.to_string()),
            ..id_jag_req(&id_token)
        },
        "invalid_target",
    )
    .await;
    rejected(
        TokenRequest {
            resource: Some(TARGET.to_string()),
            ..id_jag_req(&id_token)
        },
        "invalid_target",
    )
    .await;
    rejected(
        TokenRequest {
            scope: Some("chat.read chat.write".to_string()),
            ..id_jag_req(&id_token)
        },
        "access_denied",
    )
    .await;
    let login_other = id_jag_login(ID_JAG_OTHER, &secret_other).await;
    rejected(
        TokenRequest {
            client_id: Some(ID_JAG_OTHER.to_string()),
            client_secret: Some(secret_other.clone()),
            ..id_jag_req(login_other.id_token.as_ref().unwrap())
        },
        "access_denied",
    )
    .await;

    // without `resource` and `scope`, both claims are left out
    let res = http
        .post(&url_token)
        .form(&TokenRequest {
            resource: None,
            scope: None,
            ..id_jag_req(&id_token)
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let ts: TokenSet = res.json().await?;
    assert!(ts.scope.is_none());
    let claims = decode_claims(&ts.access_token);
    assert!(claims.get("resource").is_none());
    assert!(claims.get("scope").is_none());

    rejected(
        TokenRequest {
            actor_token: Some(login.access_token.clone()),
            actor_token_type: Some(TokenType::AccessToken.to_string()),
            ..id_jag_req(&id_token)
        },
        "BadRequest",
    )
    .await;
    rejected(
        TokenRequest {
            requested_token_type: Some(TokenType::RefreshToken.to_string()),
            ..id_jag_req(&id_token)
        },
        "BadRequest",
    )
    .await;

    // a DPoP proof binds the ID-JAG to its key
    let kp = ed25519_compact::KeyPair::generate();
    let (proof, _) = dpop_proof(&kp, None);
    let res = http
        .post(&url_token)
        .header(TOKEN_DPOP, proof)
        .form(&id_jag_req(&id_token))
        .send()
        .await?;
    assert_eq!(res.status(), 400);
    let nonce = res.headers().get(HEADER_DPOP_NONCE).unwrap().to_str()?;
    let nonce = nonce.to_string();
    let (proof, jkt) = dpop_proof(&kp, Some(nonce.clone()));
    let res = http
        .post(&url_token)
        .header(TOKEN_DPOP, proof)
        .form(&id_jag_req(&id_token))
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let ts: TokenSet = res.json().await?;
    assert_eq!(ts.token_type, JwtTokenType::NA);
    assert_eq!(decode_claims(&ts.access_token)["cnf"]["jkt"], jkt);

    // a DPoP-bound id token needs a proof with the same key
    let (proof, _) = dpop_proof(&kp, Some(nonce.clone()));
    let res = http
        .post(&url_token)
        .header(TOKEN_DPOP, proof)
        .form(&TokenRequest {
            grant_type: GrantType::Password,
            client_id: Some(ID_JAG.to_string()),
            client_secret: Some(secret.clone()),
            username: Some(USERNAME.to_string()),
            password: Some(PASSWORD.to_string()),
            ..Default::default()
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let bound: TokenSet = res.json().await?;
    let bound_id_token = bound.id_token.unwrap();
    assert_eq!(decode_claims(&bound_id_token)["cnf"]["jkt"], jkt);
    rejected(id_jag_req(&bound_id_token), "DPoP-bound").await;
    let (proof, _) = dpop_proof(&ed25519_compact::KeyPair::generate(), Some(nonce));
    let res = http
        .post(&url_token)
        .header(TOKEN_DPOP, proof)
        .form(&id_jag_req(&bound_id_token))
        .send()
        .await?;
    assert_eq!(res.status(), 400);
    assert!(res.text().await?.contains("DPoP-bound"));

    // a logout ends the id token's session
    let session_id_token = id_jag_login(ID_JAG, &secret).await.id_token.unwrap();
    let res = http
        .post(&url_token)
        .form(&id_jag_req(&session_id_token))
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let res = http
        .post(format!("{}/oidc/logout", get_backend_url()))
        .form(&LogoutRequest {
            id_token_hint: Some(session_id_token.clone()),
            post_logout_redirect_uri: None,
            state: None,
            logout_token: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    rejected(id_jag_req(&session_id_token), "no valid session").await;

    // the flow is still required
    let res = http
        .put(format!("{}/clients/{ID_JAG}", get_backend_url()))
        .headers(get_auth_headers().await?)
        .json(&base_update(vec![GrantType::Password]))
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    rejected(id_jag_req(&id_token), "token-exchange").await;

    for id in [ID_JAG, ID_JAG_OTHER] {
        let res = http
            .delete(format!("{}/clients/{id}", get_backend_url()))
            .headers(get_auth_headers().await?)
            .send()
            .await?;
        assert_eq!(res.status(), 200);
    }

    Ok(())
}
