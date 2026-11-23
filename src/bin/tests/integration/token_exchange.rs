use crate::common::{
    LoginTarget, TestClient, USER_PASSWORD, create_client, create_login_user, decode_claims,
    delete_client, delete_user, get_auth_headers, get_backend_url, login_session,
    password_token_set_for, put_client, rotate_client_secret,
};
use crate::expect_status;
use pretty_assertions::assert_eq;
use rauthy_api_types::clients::{ClientResponse, NewClientRequest, UpdateClientRequest};
use rauthy_api_types::oidc::{GrantType, JwkKeyPairAlg, LogoutRequest, TokenRequest, TokenType};
use rauthy_common::utils::new_store_id;
use rauthy_service::token_set::TokenSet;
use reqwest::Response;
use reqwest::header::HeaderMap;
use std::error::Error;

/// The only `redirect_uri` of the clients these tests create.
const REDIRECT_URI: &str = "http://localhost/callback";
/// The only `allowed_resources` entry of the exchanging client.
const TARGET: &str = "https://rs-downstream.example.com/api";
const TARGET_FORBIDDEN: &str = "https://rs-forbidden.example.com/api";
/// The backend's hard limit for the length of a nested `act` delegation chain.
const ACT_CHAIN_MAX: usize = 32;

/// The base configuration of the exchanging client, with `allowed_resources` set to [`TARGET`].
fn base_update(flows: Vec<GrantType>) -> UpdateClientRequest {
    UpdateClientRequest {
        name: Some("Exchange Test".to_string()),
        confidential: true,
        redirect_uris: vec![REDIRECT_URI.to_string()],
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
        actor_same_client: false,
        scim: None,
    }
}

/// A user of these tests with a usable password, to have an acting party with its own `sub`.
struct Actor {
    id: String,
    email: String,
}

impl Actor {
    async fn new(given_name: &str) -> Result<Self, Box<dyn Error>> {
        let email = format!(
            "{}-{}@localhost",
            given_name.to_lowercase().replace(' ', "-"),
            new_store_id().to_lowercase()
        );
        let user = create_login_user(
            &reqwest::Client::new(),
            &get_auth_headers().await?,
            &email,
            given_name,
            USER_PASSWORD,
        )
        .await?;
        Ok(Self { id: user.id, email })
    }

    async fn delete(&self) -> Result<(), Box<dyn Error>> {
        delete_user(&get_auth_headers().await?, &self.email).await
    }

    /// A real access token of this actor, minted via the `password` grant on the given client.
    async fn token(&self, client: &TestClient) -> TokenSet {
        password_token_set_for(&reqwest::Client::new(), client, &self.email, USER_PASSWORD).await
    }
}

/// The exchanging client of a test run with everything it created for itself.
struct Exchange {
    admin: HeaderMap,
    http: reqwest::Client,
    client: TestClient,
}

impl Exchange {
    /// Creates a confidential exchanging client of its own with the given flows and its secret.
    async fn new(flows: Vec<GrantType>) -> Result<Self, Box<dyn Error>> {
        let admin = get_auth_headers().await?;
        let http = reqwest::Client::new();
        let client = Self::create_client(&admin, "Exchange Test", flows).await?;
        Ok(Self {
            admin,
            http,
            client,
        })
    }

    /// A client with the base configuration of this test, with a secret a test can use.
    async fn create_client(
        admin: &HeaderMap,
        name: &str,
        flows: Vec<GrantType>,
    ) -> Result<TestClient, Box<dyn Error>> {
        let id = format!("test_{}", new_store_id());
        create_client(
            &reqwest::Client::new(),
            admin,
            &NewClientRequest {
                id: id.clone(),
                secret: None,
                name: Some(name.to_string()),
                confidential: true,
                redirect_uris: vec![REDIRECT_URI.to_string()],
                post_logout_redirect_uris: None,
                consent: None,
            },
        )
        .await?;
        put_client(&reqwest::Client::new(), admin, &id, &base_update(flows)).await?;
        let secret = rotate_client_secret(&reqwest::Client::new(), admin, &id).await?;
        Ok(TestClient { id, secret })
    }

    /// PUTs the exchanging client's configuration and returns the stored client.
    async fn put(&self, req: &UpdateClientRequest) -> Result<ClientResponse, Box<dyn Error>> {
        put_client(&self.http, &self.admin, &self.client.id, req).await
    }

    /// PUTs the `actor_same_client` flag on top of the given flows and returns the stored client.
    async fn set_actor_same_client(
        &self,
        enable: bool,
        flows: &[GrantType],
    ) -> Result<ClientResponse, Box<dyn Error>> {
        self.put(&UpdateClientRequest {
            scopes: vec!["openid".to_string()],
            default_scopes: vec!["openid".to_string(), "email".to_string()],
            access_token_lifetime: 600,
            actor_same_client: enable,
            ..base_update(flows.to_vec())
        })
        .await
    }

    /// `POST /oidc/token` with an exchange grant for the given tokens.
    async fn delegate(
        &self,
        subject: &str,
        actor: Option<&str>,
    ) -> Result<Response, Box<dyn Error>> {
        let res = self
            .http
            .post(format!("{}/oidc/token", get_backend_url()))
            .form(&self.exchange_with(subject, actor))
            .send()
            .await?;
        Ok(res)
    }

    /// `POST /oidc/token` with the exchange grant for the given subject token.
    async fn exchange(&self, subject_token: &str) -> Result<Response, Box<dyn Error>> {
        self.delegate(subject_token, None).await
    }

    /// `POST /oidc/introspect` for the exchanging client.
    async fn introspect(&self, token: &str) -> Result<Response, Box<dyn Error>> {
        let res = self
            .http
            .post(format!("{}/oidc/introspect", get_backend_url()))
            .basic_auth(self.client.id.as_str(), Some(self.client.secret.as_str()))
            .form(&[("token", token)])
            .send()
            .await?;
        Ok(res)
    }

    /// A session login of the given user on the exchanging client: the session headers and the
    /// issued tokens.
    async fn login(&self, actor: &Actor) -> (HeaderMap, TokenSet) {
        login_session(&LoginTarget {
            email: actor.email.clone(),
            password: USER_PASSWORD.to_string(),
            client_id: self.client.id.clone(),
            client_secret: Some(self.client.secret.clone()),
            redirect_uri: REDIRECT_URI.to_string(),
        })
        .await
    }

    /// The exchange grant form with the given subject and actor tokens.
    fn exchange_with(&self, subject_token: &str, actor_token: Option<&str>) -> TokenRequest {
        TokenRequest {
            grant_type: GrantType::TokenExchange,
            client_id: Some(self.client.id.clone()),
            client_secret: Some(self.client.secret.clone()),
            subject_token: Some(subject_token.to_string()),
            subject_token_type: Some(TokenType::AccessToken.to_string()),
            actor_token: actor_token.map(String::from),
            actor_token_type: actor_token.map(|_| TokenType::AccessToken.to_string()),
            audience: Some(TARGET.to_string()),
            ..Default::default()
        }
    }

    /// Deletes the exchanging client again.
    async fn teardown(&self) -> Result<(), Box<dyn Error>> {
        delete_client(&self.admin, &self.client.id).await
    }
}

/// End-to-end coverage for the RFC 8693 token exchange: deny-by-default via `flows_enabled`, the
/// `allowed_resources` target allow-list, impersonation vs delegation (`act`), scope narrowing,
/// and the guarantee that an exchanged token never carries a refresh or id token. On top of that:
/// the nested `act` delegation chain, the per-client binding of the `actor_token` to the
/// exchanging client, the inherited session and `act` in the introspection and the limit of the
/// `act` chain.
#[tokio::test]
async fn test_token_exchange() -> Result<(), Box<dyn Error>> {
    let url_token = format!("{}/oidc/token", get_backend_url());

    // (0) create the exchanging client, WITHOUT the token exchange flow at first
    let ex = Exchange::new(vec![
        GrantType::ClientCredentials,
        GrantType::Password,
        GrantType::RefreshToken,
    ])
    .await?;
    let subject = Actor::new("Subject").await?;

    // a real access token for the test user, used as the subject token
    let subject_ts = subject.token(&ex.client).await;
    let subject_token = subject_ts.access_token.clone();

    // (1) deny-by-default: the flow is not enabled for this client yet
    let res = ex.exchange(&subject_token).await?;
    let body = expect_status!(res, 400).text().await?;
    assert!(
        body.contains("token_exchange") || body.contains("token-exchange"),
        "unexpected body: {body}"
    );

    // now enable the flow
    ex.put(&base_update(vec![
        GrantType::ClientCredentials,
        GrantType::Password,
        GrantType::RefreshToken,
        GrantType::TokenExchange,
    ]))
    .await?;

    // (2) impersonation: no actor token -> no `act` claim, `sub` stays the subject's
    let res = ex.exchange(&subject_token).await?;
    let res = expect_status!(res, 200);
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
    assert_eq!(claims["azp"], serde_json::json!(&ex.client.id));

    // (3) a target outside `allowed_resources` -> `invalid_target`
    let res = ex
        .http
        .post(&url_token)
        .form(&TokenRequest {
            audience: Some(TARGET_FORBIDDEN.to_string()),
            ..ex.exchange_with(&subject_token, None)
        })
        .send()
        .await?;
    let body = expect_status!(res, 400).text().await?;
    assert!(body.contains("invalid_target"), "unexpected body: {body}");

    // (4) delegation: an actor token adds the `act` claim
    let actor_ts = subject.token(&ex.client).await;
    let res = ex
        .delegate(&subject_token, Some(&actor_ts.access_token))
        .await?;
    let res = expect_status!(res, 200);
    let ts: TokenSet = res.json().await?;
    let claims = decode_claims(&ts.access_token);
    let actor_claims = decode_claims(&actor_ts.access_token);
    assert_eq!(claims["sub"], subject_claims["sub"]);
    assert_eq!(claims["act"]["sub"], actor_claims["sub"]);
    assert_eq!(ts.scope.as_deref(), subject_claims["scope"].as_str());
    assert_eq!(ts.scope.as_deref(), claims["scope"].as_str());

    // (5) an `actor_token` without its `actor_token_type` is rejected
    let res = ex
        .http
        .post(&url_token)
        .form(&TokenRequest {
            actor_token: Some(actor_ts.access_token.clone()),
            actor_token_type: None,
            ..ex.exchange_with(&subject_token, None)
        })
        .send()
        .await?;
    expect_status!(res, 400);

    // (6) a refresh token as the subject token is rejected: access tokens only
    let refresh = subject_ts.refresh_token.clone().unwrap();
    let res = ex.exchange(&refresh).await?;
    let body = expect_status!(res, 400).text().await?;
    assert!(body.contains("invalid_grant"), "unexpected body: {body}");

    // (7) a garbage subject token is rejected
    let res = ex.exchange("not.a.token").await?;
    expect_status!(res, 400);

    // (8) an unsupported `subject_token_type` is rejected
    let res = ex
        .http
        .post(&url_token)
        .form(&TokenRequest {
            subject_token_type: Some("urn:ietf:params:oauth:token-type:id_token".to_string()),
            ..ex.exchange_with(&subject_token, None)
        })
        .send()
        .await?;
    expect_status!(res, 400);

    // (9) scope narrowing: a scope the subject token does not have is rejected
    let res = ex
        .http
        .post(&url_token)
        .form(&TokenRequest {
            scope: Some("openid some_scope_the_subject_never_had".to_string()),
            ..ex.exchange_with(&subject_token, None)
        })
        .send()
        .await?;
    expect_status!(res, 400);

    // (10) narrowing to a subset of the subject's scopes works
    let subject_scope = subject_claims["scope"].as_str().unwrap_or_default();
    let first_scope = subject_scope.split_whitespace().next().unwrap().to_string();
    let res = ex
        .http
        .post(&url_token)
        .form(&TokenRequest {
            scope: Some(first_scope.clone()),
            ..ex.exchange_with(&subject_token, None)
        })
        .send()
        .await?;
    let res = expect_status!(res, 200);
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
    ex.put(&UpdateClientRequest {
        default_scopes: vec!["openid".to_string(), "email".to_string()],
        ..base_update(flows.clone())
    })
    .await?;
    let wide_subject = subject.token(&ex.client).await;
    let wide_claims = decode_claims(&wide_subject.access_token);
    assert!(
        wide_claims["scope"].as_str().unwrap().contains("email"),
        "{}",
        wide_claims["scope"]
    );

    // the client loses `email` and gets a longer lifetime than the subject token has left
    ex.put(&UpdateClientRequest {
        scopes: vec!["openid".to_string()],
        access_token_lifetime: 600,
        ..base_update(flows.clone())
    })
    .await?;

    // inherited scopes the client is not allowed are dropped
    let res = ex.exchange(&wide_subject.access_token).await?;
    let res = expect_status!(res, 200);
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
    let res = ex
        .http
        .post(&url_token)
        .form(&TokenRequest {
            scope: Some("email".to_string()),
            ..ex.exchange_with(&wide_subject.access_token, None)
        })
        .send()
        .await?;
    expect_status!(res, 400);

    // (12) the client's `default_scopes` are allowed as well, same as on login
    ex.put(&UpdateClientRequest {
        scopes: vec!["openid".to_string()],
        default_scopes: vec!["openid".to_string(), "email".to_string()],
        access_token_lifetime: 600,
        ..base_update(flows.clone())
    })
    .await?;
    let res = ex.exchange(&wide_subject.access_token).await?;
    let res = expect_status!(res, 200);
    let ts: TokenSet = res.json().await?;
    assert!(
        ts.scope.as_deref().unwrap().contains("email"),
        "{:?}",
        ts.scope
    );

    // (13) the exchanged token never outlives its user
    let user_expires = chrono::Utc::now().timestamp() + 60;
    let patch_user = |body: serde_json::Value| {
        ex.http
            .patch(format!("{}/users/{}", get_backend_url(), subject.id))
            .headers(ex.admin.clone())
            .json(&body)
            .send()
    };
    let res = patch_user(serde_json::json!({
        "put": [{ "key": "user_expires", "value": user_expires }],
        "del": [],
    }))
    .await?;
    expect_status!(res, 200);
    let res = ex.exchange(&wide_subject.access_token).await;
    // restore the user before asserting anything, so a failure cannot leave it expiring
    let restore = serde_json::json!({ "put": [], "del": ["user_expires"] });
    expect_status!(patch_user(restore).await?, 200);
    let res = res?;
    let res = expect_status!(res, 200);
    let ts: TokenSet = res.json().await?;
    let claims = decode_claims(&ts.access_token);
    assert!(
        claims["exp"].as_i64().unwrap() <= user_expires,
        "exchanged exp {} outlives the user expiry {user_expires}",
        claims["exp"]
    );
    assert!(ts.expires_in <= 60, "expires_in {}", ts.expires_in);

    // (14) a two-hop delegation chain: the subject's `act` is nested below the new actor
    let actor_a = Actor::new("Actor A").await?;
    let actor_b = Actor::new("Actor B").await?;
    let token_a = actor_a.token(&ex.client).await.access_token;
    let token_b = actor_b.token(&ex.client).await.access_token;
    assert_ne!(actor_a.id, actor_b.id);
    assert_ne!(subject_claims["sub"], serde_json::json!(actor_a.id));

    let res = ex.delegate(&subject_token, Some(&token_a)).await?;
    assert_eq!(res.status(), 200);
    let hop_1: TokenSet = res.json().await?;
    let claims = decode_claims(&hop_1.access_token);
    assert_eq!(claims["sub"], subject_claims["sub"]);
    assert_eq!(claims["act"], serde_json::json!({ "sub": actor_a.id }));

    let res = ex.delegate(&hop_1.access_token, Some(&token_b)).await?;
    assert_eq!(res.status(), 200);
    let hop_2: TokenSet = res.json().await?;
    let claims = decode_claims(&hop_2.access_token);
    assert_eq!(claims["sub"], subject_claims["sub"]);
    // the current actor is the outermost one, the prior actor is nested
    let chain = serde_json::json!({ "sub": actor_b.id, "act": { "sub": actor_a.id } });
    assert_eq!(claims["act"], chain);

    // an actor that is acting for someone itself brings its own chain along
    let res = ex.delegate(&token_a, Some(&hop_1.access_token)).await?;
    assert_eq!(res.status(), 200);
    let ts: TokenSet = res.json().await?;
    let claims = decode_claims(&ts.access_token);
    assert_eq!(claims["sub"], serde_json::json!(actor_a.id));
    assert_eq!(
        claims["act"],
        serde_json::json!({ "sub": subject_claims["sub"], "act": { "sub": actor_a.id } })
    );

    // (15) impersonation keeps the chain: no actor is added, but none is hidden either
    let res = ex.exchange(&hop_2.access_token).await?;
    assert_eq!(res.status(), 200);
    let ts: TokenSet = res.json().await?;
    let claims = decode_claims(&ts.access_token);
    assert_eq!(claims["sub"], subject_claims["sub"]);
    assert_eq!(claims["act"], chain);

    // (16) a conflict is rejected: both tokens carry an `act`, which cannot be merged
    let res = ex
        .delegate(&hop_2.access_token, Some(&hop_1.access_token))
        .await?;
    assert_eq!(res.status(), 400);
    let body: serde_json::Value = res.json().await?;
    assert_eq!(body["error"], "invalid_request", "unexpected body: {body}");
    assert!(
        body["message"].to_string().contains("both carry an 'act'"),
        "unexpected body: {body}"
    );

    // (17) `actor_same_client`: an actor token of a foreign client
    let foreign = Exchange::create_client(
        &ex.admin,
        "Exchange Test Foreign",
        vec![GrantType::Password],
    )
    .await?;
    let token_foreign = actor_a.token(&foreign).await.access_token;
    assert_eq!(
        decode_claims(&token_foreign)["azp"],
        serde_json::json!(&foreign.id)
    );

    // the flag is off by default, and the foreign actor token is accepted
    let res = ex.delegate(&subject_token, Some(&token_foreign)).await?;
    assert_eq!(res.status(), 200);
    let ts: TokenSet = res.json().await?;
    let claims = decode_claims(&ts.access_token);
    assert_eq!(claims["act"], serde_json::json!({ "sub": actor_a.id }));

    assert!(
        ex.set_actor_same_client(true, &flows)
            .await?
            .actor_same_client
    );

    // with the flag on, the foreign actor token is rejected ...
    let res = ex.delegate(&subject_token, Some(&token_foreign)).await?;
    assert_eq!(res.status(), 400);
    let body: serde_json::Value = res.json().await?;
    assert_eq!(body["error"], "invalid_request", "unexpected body: {body}");
    assert!(
        body["message"]
            .to_string()
            .contains("not issued to this client"),
        "unexpected body: {body}"
    );
    // ... while an actor token issued to the exchanging client itself still works, ...
    let res = ex.delegate(&subject_token, Some(&token_a)).await?;
    assert_eq!(res.status(), 200);
    // ... and the flag only is about the actor: a foreign `subject_token` is not affected
    let res = ex.delegate(&token_foreign, Some(&token_a)).await?;
    assert_eq!(res.status(), 200);

    assert!(
        !ex.set_actor_same_client(false, &flows)
            .await?
            .actor_same_client
    );
    let res = ex.delegate(&subject_token, Some(&token_foreign)).await?;
    assert_eq!(res.status(), 200);

    delete_client(&ex.admin, &foreign.id).await?;

    // (18) a session logout revokes the exchanged token (needs `token_revoke_on_logout = true`)
    ex.put(&UpdateClientRequest {
        scopes: vec!["openid".to_string()],
        default_scopes: vec!["openid".to_string(), "email".to_string()],
        access_token_lifetime: 600,
        ..base_update(vec![
            GrantType::AuthorizationCode,
            GrantType::ClientCredentials,
            GrantType::Password,
            GrantType::RefreshToken,
            GrantType::TokenExchange,
        ])
    })
    .await?;

    let (_, session_ts) = ex.login(&subject).await;
    let res = ex
        .delegate(&session_ts.access_token, Some(&token_a))
        .await?;
    assert_eq!(res.status(), 200);
    let from_session: TokenSet = res.json().await?;
    // derived from a token that has no session: must survive the logout below
    let sessionless = ex.exchange(&subject_token).await?;
    assert_eq!(sessionless.status(), 200);
    let sessionless: TokenSet = sessionless.json().await?;
    // a second hop keeps the session as well
    let res = ex.exchange(&from_session.access_token).await?;
    assert_eq!(res.status(), 200);
    let second_hop: TokenSet = res.json().await?;

    for token in [
        &from_session.access_token,
        &second_hop.access_token,
        &sessionless.access_token,
    ] {
        let res = ex.introspect(token).await?;
        assert_eq!(res.status(), 200);
        assert_eq!(res.json::<serde_json::Value>().await?["active"], true);
    }

    let res = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?
        .post(format!("{}/oidc/logout", get_backend_url()))
        .form(&LogoutRequest {
            id_token_hint: session_ts.id_token.clone(),
            post_logout_redirect_uri: None,
            state: None,
            logout_token: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    assert_eq!(
        ex.introspect(&from_session.access_token).await?.status(),
        401
    );
    assert_eq!(ex.introspect(&second_hop.access_token).await?.status(), 401);
    // a revoked token cannot be exchanged any further
    let res = ex.exchange(&from_session.access_token).await?;
    assert_ne!(res.status(), 200);
    // other sessions and session-less tokens of the same user are not affected
    let res = ex.introspect(&sessionless.access_token).await?;
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<serde_json::Value>().await?["active"], true);

    // (19) an admin deleting the session revokes the exchanged token, too
    let (_, session_ts) = ex.login(&subject).await;
    let sid = decode_claims(session_ts.id_token.as_ref().unwrap())["sid"]
        .as_str()
        .expect("the id token to have a `sid`")
        .to_string();
    let res = ex
        .delegate(&session_ts.access_token, Some(&token_a))
        .await?;
    assert_eq!(res.status(), 200);
    let from_session: TokenSet = res.json().await?;
    let res = ex.introspect(&from_session.access_token).await?;
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<serde_json::Value>().await?["active"], true);

    let res = ex
        .http
        .delete(format!("{}/sessions/id/{}", get_backend_url(), sid))
        .headers(ex.admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    assert_eq!(
        ex.introspect(&from_session.access_token).await?.status(),
        401
    );
    let res = ex.introspect(&sessionless.access_token).await?;
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<serde_json::Value>().await?["active"], true);

    // (20) the introspection returns the nested `act`
    let res = ex.introspect(&hop_2.access_token).await?;
    assert_eq!(res.status(), 200);
    let info: serde_json::Value = res.json().await?;
    assert_eq!(info["active"], true);
    assert_eq!(info["sub"], subject_claims["sub"]);
    assert_eq!(info["act"], chain);
    // no `act` for a token without one
    let res = ex.introspect(&subject_token).await?;
    assert_eq!(res.status(), 200);
    let info: serde_json::Value = res.json().await?;
    assert_eq!(info["active"], true);
    assert!(info.get("act").is_none(), "unexpected `act` in: {info}");

    // (21) the `act` chain is limited
    let mut token = hop_2.access_token.clone();
    for _ in 2..ACT_CHAIN_MAX {
        let res = ex.delegate(&token, Some(&token_a)).await?;
        assert_eq!(res.status(), 200);
        token = res.json::<TokenSet>().await?.access_token;
    }
    let res = ex.delegate(&token, Some(&token_a)).await?;
    assert_eq!(res.status(), 400);
    let body = res.text().await?;
    assert!(body.contains("invalid_request"), "unexpected body: {body}");
    assert!(body.contains("limited to"), "unexpected body: {body}");
    // a chain at the limit can still be passed on without an actor
    let res = ex.exchange(&token).await?;
    assert_eq!(res.status(), 200);

    // cleanup
    actor_a.delete().await?;
    actor_b.delete().await?;
    subject.delete().await?;
    ex.teardown().await
}
