use crate::common::{
    TestClient, TestClientSetup, USER_PASSWORD, code_state_from_headers,
    cookie_csrf_headers_from_res_direct, create_client, create_login_user, create_test_client,
    delete_client, delete_test_client, delete_user, get_auth_headers, get_backend_url, get_client,
    get_solved_pow, password_token_set_for, put_client, update_request,
};
use crate::{expect_status, expect_status_json};
use pretty_assertions::assert_eq;
use rauthy_api_types::clients::{NewClientRequest, UpdateClientRequest};
use rauthy_api_types::oidc::{GrantType, LoginRequest, TokenRequest};
use rauthy_common::sha256;
use rauthy_common::utils::{base64_url_encode, new_store_id};
use rauthy_service::token_set::TokenSet;
use reqwest::Response;
use reqwest::header::HeaderMap;
use std::error::Error;

const REDIRECT_URI: &str = "http://localhost:3000/oidc/callback";
const CODE_VERIFIER: &str = "oDXug9zfYqfz8ejcqMpALRPXfW8QhbKV2AVuScAt8xrLKDAmaRYQ4yRi2uqcH9ys";

/// A public client with the given flows and its own redirect URI: the client a token of these
/// tests is bound to.
async fn create_public_client(
    http: &reqwest::Client,
    admin: &HeaderMap,
    id: &str,
    name: &str,
    flows: &[GrantType],
) -> Result<(), Box<dyn Error>> {
    create_client(
        http,
        admin,
        &NewClientRequest {
            id: id.to_string(),
            secret: None,
            name: Some(name.to_string()),
            confidential: false,
            redirect_uris: vec![REDIRECT_URI.to_string()],
            post_logout_redirect_uris: None,
            consent: None,
        },
    )
    .await?;

    let req = UpdateClientRequest {
        flows_enabled: flows.to_vec(),
        ..update_request(get_client(admin, id).await?)
    };
    put_client(http, admin, id, &req).await?;

    Ok(())
}

/// The setup of a client binding test: the public client its tokens are bound to, a login user of
/// its own, and, for the `password` grant tests, a second confidential client that is never
/// deleted. None of it is shared with any other test, and everything is removed at teardown.
struct Binding {
    pub_client: String,
    name: String,
    flows: Vec<GrantType>,
    password_client: Option<TestClient>,
    user_email: String,
}

impl Binding {
    async fn new(
        name: &str,
        flows: Vec<GrantType>,
        with_password_client: bool,
    ) -> Result<Self, Box<dyn Error>> {
        let admin = get_auth_headers().await?;
        let http = reqwest::Client::new();

        let pub_client = format!("test_{}", new_store_id());
        create_public_client(&http, &admin, &pub_client, name, &flows).await?;

        let user_email = format!("{}-{}@localhost", name, new_store_id().to_lowercase());
        create_login_user(&http, &admin, &user_email, name, USER_PASSWORD).await?;

        let password_client = if with_password_client {
            // a sessionless `password` grant only gets a refresh token when the client has the
            // `refresh_token` flow enabled as well
            let setup = TestClientSetup {
                name: name.to_string(),
                flows: vec![GrantType::Password, GrantType::RefreshToken],
                redirect_uri: REDIRECT_URI.to_string(),
                post_logout_redirect_uris: Vec::new(),
                backchannel_logout_uri: None,
            };
            Some(create_test_client(&setup).await?)
        } else {
            None
        };

        Ok(Self {
            pub_client,
            name: name.to_string(),
            flows,
            password_client,
            user_email,
        })
    }

    /// Deletes the public client and creates it again with the same id: a client that looks
    /// identical, down to its id, but with a fresh `created_micros`.
    async fn recreate_public_client(&self) -> Result<(), Box<dyn Error>> {
        let admin = get_auth_headers().await?;
        let http = reqwest::Client::new();
        delete_client(&admin, &self.pub_client).await?;
        create_public_client(&http, &admin, &self.pub_client, &self.name, &self.flows).await
    }

    /// Deletes everything [`Binding::new`] created.
    async fn teardown(&self) -> Result<(), Box<dyn Error>> {
        let admin = get_auth_headers().await?;
        delete_client(&admin, &self.pub_client).await?;
        delete_user(&admin, &self.user_email).await?;
        if let Some(client) = &self.password_client {
            delete_test_client(&client.id).await?;
        }
        Ok(())
    }
}

/// Logs in as the test's own user on the given client and returns the authorization code without
/// exchanging it.
async fn auth_code(client_id: &str, user_email: &str) -> Result<String, Box<dyn Error>> {
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
            email: Some(user_email.to_string()),
            password: Some(USER_PASSWORD.to_string()),
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
    let res = expect_status!(res, 202);

    let (code, _) = code_state_from_headers(&res)?;
    Ok(code)
}

async fn exchange_code(client_id: &str, code: String) -> Response {
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

/// A refresh token grant on a client a test owns.
async fn refresh(client: &TestClient, tokens: &TokenSet) -> Response {
    reqwest::Client::new()
        .post(format!("{}/oidc/token", get_backend_url()))
        .form(&TokenRequest {
            grant_type: GrantType::RefreshToken,
            client_id: Some(client.id.clone()),
            client_secret: Some(client.secret.clone()),
            refresh_token: tokens.refresh_token.clone(),
            ..Default::default()
        })
        .send()
        .await
        .unwrap()
}

/// An auth code issued before the client was deleted and recreated with the same id must not be
/// exchangeable. This fails if new clients do not get a fresh `created_micros`.
#[tokio::test]
async fn test_recreated_client_rejects_old_auth_code() -> Result<(), Box<dyn Error>> {
    let b = Binding::new(
        "binding-auth-code",
        vec![GrantType::AuthorizationCode],
        false,
    )
    .await?;
    let id = b.pub_client.as_str();

    // sanity check: the flow works for this client
    let code = auth_code(id, &b.user_email).await?;
    expect_status!(exchange_code(id, code).await, 200);

    let code = auth_code(id, &b.user_email).await?;
    b.recreate_public_client().await?;
    expect_status!(exchange_code(id, code).await, 401);

    b.teardown().await?;
    Ok(())
}

/// Deleting a client deletes its refresh tokens, and only those.
#[tokio::test]
async fn test_client_delete_drops_its_refresh_tokens() -> Result<(), Box<dyn Error>> {
    let b = Binding::new(
        "binding-refresh-token",
        vec![GrantType::AuthorizationCode, GrantType::RefreshToken],
        true,
    )
    .await?;
    let id = b.pub_client.as_str();
    let other = b
        .password_client
        .as_ref()
        .expect("the password grant client");

    let tokens = expect_status_json!(
        exchange_code(id, auth_code(id, &b.user_email).await?).await,
        200,
        TokenSet
    );
    let other_tokens =
        password_token_set_for(&reqwest::Client::new(), other, &b.user_email, USER_PASSWORD).await;

    b.recreate_public_client().await?;

    // 404 means the row is gone, a stale row for the deleted client would be a 401
    let res = reqwest::Client::new()
        .post(format!("{}/oidc/token", get_backend_url()))
        .form(&TokenRequest {
            grant_type: GrantType::RefreshToken,
            client_id: Some(id.to_string()),
            refresh_token: tokens.refresh_token.clone(),
            ..Default::default()
        })
        .send()
        .await?;
    expect_status!(res, 404);
    assert_eq!(
        refresh(other, &other_tokens).await.status(),
        200,
        "the other client's refresh token must survive"
    );

    b.teardown().await?;
    Ok(())
}

/// Sessionless refresh tokens are kept until their replacement is stored, but must still be
/// usable only once.
#[tokio::test]
async fn test_sessionless_refresh_is_single_use() -> Result<(), Box<dyn Error>> {
    let b = Binding::new(
        "binding-sessionless",
        vec![GrantType::AuthorizationCode],
        true,
    )
    .await?;
    let client = b
        .password_client
        .as_ref()
        .expect("the password grant client");

    let original = password_token_set_for(
        &reqwest::Client::new(),
        client,
        &b.user_email,
        USER_PASSWORD,
    )
    .await;

    let res = refresh(client, &original).await;
    let res = expect_status!(res, 200);
    let rotated: TokenSet = res.json().await?;
    assert_eq!(
        refresh(client, &original).await.status(),
        404,
        "the consumed refresh token must be gone"
    );

    let (first, second) = tokio::join!(refresh(client, &rotated), refresh(client, &rotated));
    let mut statuses = [first.status().as_u16(), second.status().as_u16()];
    statuses.sort_unstable();
    assert_eq!(
        statuses,
        [200, 404],
        "a rotation must be usable exactly once"
    );
    assert_eq!(
        refresh(client, &rotated).await.status(),
        404,
        "the rotated refresh token must be gone after its use"
    );

    b.teardown().await?;
    Ok(())
}
