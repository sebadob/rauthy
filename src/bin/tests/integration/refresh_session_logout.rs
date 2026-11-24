use crate::common::{
    LoginTarget, TestClient, TestClientSetup, USER_PASSWORD, create_login_user, create_test_client,
    decode_claims, delete_test_client, delete_user, get_auth_headers, get_backend_url,
    login_session,
};
use crate::expect_status;
use rauthy_api_types::oidc::{GrantType, LogoutRequest, TokenRequest};
use rauthy_common::utils::new_store_id;
use rauthy_service::token_set::TokenSet;
use reqwest::header::HeaderMap;
use std::error::Error;

/// The only `redirect_uri` of the client these tests create.
const REDIRECT_URI: &str = "http://localhost:3000/oidc/callback";

/// The user of a test run: the login of these tests is its own, so no other test's sessions and
/// logouts are ever touched.
struct TestUser {
    email: String,
}

impl TestUser {
    async fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        let email = format!("{}-{}@localhost", name, new_store_id().to_lowercase());
        create_login_user(
            &reqwest::Client::new(),
            &get_auth_headers().await?,
            &email,
            name,
            USER_PASSWORD,
        )
        .await?;
        Ok(Self { email })
    }

    async fn delete(&self) -> Result<(), Box<dyn Error>> {
        delete_user(&get_auth_headers().await?, &self.email).await
    }

    /// A full login of this user on the given client: its own session and tokens.
    async fn login(&self, client: &TestClient) -> (HeaderMap, TokenSet) {
        login_session(&LoginTarget {
            email: self.email.clone(),
            password: USER_PASSWORD.to_string(),
            client_id: client.id.clone(),
            client_secret: Some(client.secret.clone()),
            redirect_uri: REDIRECT_URI.to_string(),
        })
        .await
    }
}

fn setup() -> TestClientSetup {
    TestClientSetup {
        name: "Refresh Logout Test".to_string(),
        flows: vec![GrantType::AuthorizationCode, GrantType::RefreshToken],
        redirect_uri: REDIRECT_URI.to_string(),
        post_logout_redirect_uris: Vec::new(),
        backchannel_logout_uri: None,
    }
}

async fn refresh(
    http: &reqwest::Client,
    tokens: &TokenSet,
    client: &TestClient,
) -> reqwest::Response {
    http.post(format!("{}/oidc/token", get_backend_url()))
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

async fn test_refresh_preserves_session_id(
    http: &reqwest::Client,
    client: &TestClient,
    user: &TestUser,
) -> Result<(), Box<dyn Error>> {
    let (_, original) = user.login(client).await;
    let sid = decode_claims(original.id_token.as_ref().unwrap())["sid"].clone();
    assert!(sid.is_string(), "the login issued a session id");

    let response = refresh(http, &original, client).await;
    let response = expect_status!(response, 200, "refresh_preserves_session_id: first refresh");
    let first: TokenSet = response.json().await?;
    let response = refresh(http, &first, client).await;
    let response = expect_status!(
        response,
        200,
        "refresh_preserves_session_id: second refresh"
    );
    let rotated: TokenSet = response.json().await?;
    assert_eq!(
        decode_claims(rotated.id_token.as_ref().unwrap())["sid"],
        sid,
        "refresh_preserves_session_id: the session id survived both refreshes"
    );
    Ok(())
}

async fn test_logout_before_refresh_rejects_rotation(
    client: &TestClient,
    user: &TestUser,
) -> Result<(), Box<dyn Error>> {
    let http = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let (_, original) = user.login(client).await;
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
    expect_status!(
        response,
        200,
        "logout_before_refresh_rejects_rotation: logout"
    );
    expect_status!(
        refresh(&http, &original, client).await,
        404,
        "logout_before_refresh_rejects_rotation: refresh after logout"
    );
    Ok(())
}

async fn test_session_logout_revokes_rotated_tokens(
    client: &TestClient,
    user: &TestUser,
) -> Result<(), Box<dyn Error>> {
    let http = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;
    let (_, original) = user.login(client).await;
    let (_, independent) = user.login(client).await;
    let response = refresh(&http, &original, client).await;
    let response = expect_status!(
        response,
        200,
        "session_logout_revokes_rotated_tokens: first refresh"
    );
    let first: TokenSet = response.json().await?;
    let response = refresh(&http, &first, client).await;
    let response = expect_status!(
        response,
        200,
        "session_logout_revokes_rotated_tokens: second refresh"
    );
    let rotated: TokenSet = response.json().await?;
    let inspect_url = format!("{}/oidc/introspect", get_backend_url());
    let before = http
        .post(&inspect_url)
        .basic_auth(client.id.as_str(), Some(client.secret.as_str()))
        .form(&[("token", &rotated.access_token)])
        .send()
        .await?;
    let before = expect_status!(
        before,
        200,
        "session_logout_revokes_rotated_tokens: introspect"
    );
    assert_eq!(
        before.json::<serde_json::Value>().await?["active"],
        true,
        "session_logout_revokes_rotated_tokens: the rotated token is active before the logout"
    );

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
    expect_status!(logout, 200, "session_logout_revokes_rotated_tokens: logout");

    let after = http
        .post(&inspect_url)
        .basic_auth(client.id.as_str(), Some(client.secret.as_str()))
        .form(&[("token", &rotated.access_token)])
        .send()
        .await?;
    expect_status!(
        after,
        401,
        "session_logout_revokes_rotated_tokens: introspect after logout"
    );
    expect_status!(
        refresh(&http, &rotated, client).await,
        404,
        "session_logout_revokes_rotated_tokens: the rotated chain is revoked"
    );
    expect_status!(
        refresh(&http, &independent, client).await,
        200,
        "session_logout_revokes_rotated_tokens: the independent session still refreshes"
    );
    Ok(())
}

async fn test_concurrent_logout_revokes_returned_refresh(
    client: &TestClient,
    user: &TestUser,
) -> Result<(), Box<dyn Error>> {
    let http = reqwest::Client::new();
    let (_, original) = user.login(client).await;
    let logout = http
        .post(format!("{}/oidc/logout", get_backend_url()))
        .form(&LogoutRequest {
            id_token_hint: original.id_token.clone(),
            post_logout_redirect_uri: None,
            state: None,
            logout_token: None,
        });
    let (rotated, logout) = tokio::join!(refresh(&http, &original, client), logout.send());
    expect_status!(
        logout?,
        200,
        "concurrent_logout_revokes_returned_refresh: logout"
    );
    if rotated.status().is_success() {
        let rotated: TokenSet = rotated.json().await?;
        let inspected = http
            .post(format!("{}/oidc/introspect", get_backend_url()))
            .basic_auth(client.id.as_str(), Some(client.secret.as_str()))
            .form(&[("token", &rotated.access_token)])
            .send()
            .await?;
        expect_status!(
            inspected,
            401,
            "concurrent_logout_revokes_returned_refresh: introspect"
        );
        expect_status!(
            refresh(&http, &rotated, client).await,
            404,
            "concurrent_logout_revokes_returned_refresh: refresh won the race, its token is revoked"
        );
    } else {
        expect_status!(
            rotated,
            404,
            "concurrent_logout_revokes_returned_refresh: logout won the race"
        );
    }
    Ok(())
}

/// The refresh → logout state machine: these four steps touch the same session, refresh token and
/// rotation chain, so they run in one runtime, in a fixed order, after another.
#[tokio::test]
async fn test_refresh_and_logout_lifecycle() -> Result<(), Box<dyn Error>> {
    let client = create_test_client(&setup()).await?;
    let user = TestUser::new("refresh-logout").await?;

    test_refresh_preserves_session_id(&reqwest::Client::new(), &client, &user).await?;
    test_logout_before_refresh_rejects_rotation(&client, &user).await?;
    test_session_logout_revokes_rotated_tokens(&client, &user).await?;
    test_concurrent_logout_revokes_returned_refresh(&client, &user).await?;

    delete_test_client(&client.id).await?;
    user.delete().await
}

#[tokio::test]
async fn test_concurrent_refresh_has_one_winner() -> Result<(), Box<dyn Error>> {
    let client = create_test_client(&setup()).await?;
    let user = TestUser::new("refresh-race").await?;
    let http = reqwest::Client::new();
    let (_, original) = user.login(&client).await;
    let (first, second) = tokio::join!(
        refresh(&http, &original, &client),
        refresh(&http, &original, &client)
    );
    assert_ne!(
        first.status().is_success(),
        second.status().is_success(),
        "exactly one of two concurrent refreshes of the same token may win"
    );
    let (winner, loser) = if first.status().is_success() {
        (first, second)
    } else {
        (second, first)
    };
    let winner = expect_status!(winner, 200, "concurrent_refresh_has_one_winner: winner");
    expect_status!(loser, 404, "concurrent_refresh_has_one_winner: loser");
    expect_status!(
        refresh(&http, &original, &client).await,
        404,
        "concurrent_refresh_has_one_winner: the original token is spent"
    );
    let rotated: TokenSet = winner.json().await?;
    let inspected = http
        .post(format!("{}/oidc/introspect", get_backend_url()))
        .basic_auth(client.id.as_str(), Some(client.secret.as_str()))
        .form(&[("token", &rotated.access_token)])
        .send()
        .await?;
    let inspected = expect_status!(
        inspected,
        200,
        "concurrent_refresh_has_one_winner: introspect"
    );
    assert_eq!(
        inspected.json::<serde_json::Value>().await?["active"],
        true,
        "concurrent_refresh_has_one_winner: the winner's token is active"
    );
    expect_status!(
        refresh(&http, &rotated, &client).await,
        200,
        "concurrent_refresh_has_one_winner: the winner's chain refreshes"
    );

    delete_test_client(&client.id).await?;
    user.delete().await
}
