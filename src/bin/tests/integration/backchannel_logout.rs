use crate::common::{
    TestClientSetup, USER_PASSWORD, create_login_user, create_test_client, delete_test_client,
    get_backend_url, init_client_bcl_uri, password_token_set_for, session_headers,
};
use crate::expect_status;
use chrono::Utc;
use rauthy_api_types::oidc::LogoutRequest;
use rauthy_common::utils::new_store_id;

/// The dev endpoint that receives a backchannel logout and stores the token it got. The seeded
/// `init_client` has the very same one configured (`migration/db_migrate_dev.rs`).
fn backchannel_logout_uri() -> String {
    init_client_bcl_uri()
}

fn setup() -> TestClientSetup {
    TestClientSetup {
        name: "Backchannel Logout Test".to_string(),
        flows: vec![rauthy_api_types::oidc::GrantType::Password],
        redirect_uri: "http://localhost:8080/callback".to_string(),
        post_logout_redirect_uris: Vec::new(),
        backchannel_logout_uri: Some(backchannel_logout_uri()),
    }
}
use rauthy_service::oidc::bcl_logout_token::LogoutToken;
use std::error::Error;
use tokio::fs;

#[tokio::test]
async fn test_backchannel_logout() -> Result<(), Box<dyn Error>> {
    // These tests are set up a bit "weird" to reduce complexity.
    // Rauthy exposes `/auth/v1/dev/` endpoints during test and dev.
    // It has a `backchannel_logout` endpoint which just accepts a `logout_token` as specified in
    // the RFC. It won't do anything though apart from receiving that token and saving it into
    // a directory on disk. This test will read in the token and make sure everything is fine.
    // This will test the whole flow from POST to the logout and that the backchannel logout
    // internally would be triggered and correctly forwarded.

    let target_file = "tests/test_data/logout_token";
    let _ = fs::remove_file(target_file).await;

    let backend_url = get_backend_url();
    let client = reqwest::Client::new();
    let url = format!("{}/dev/logout", backend_url);

    // We can trigger a backchannel logout in quite a few ways.
    // The default logout endpoint with a `LogoutToken` has a restriction though, that the issuer
    // exists as a valid and reachable upstream provider, which we cannot set up easily in
    // integration tests.
    // We could trigger a logout via `id_token_hint` for instance.

    // A user of its own: a logout ends every session of that user, which the seeded `init_admin`
    // shares with every other test that uses its admin session.
    let email = format!("backchannel-logout-{}@localhost.de", new_store_id());
    let (admin, _) = session_headers().await;
    let test_client = create_test_client(&setup()).await?;
    let user =
        create_login_user(&client, &admin, &email, "Backchannel Logout", USER_PASSWORD).await?;
    let ts = password_token_set_for(&client, &test_client, &email, USER_PASSWORD).await;

    let res = client
        .post(&url)
        .form(&LogoutRequest {
            id_token_hint: Some(ts.id_token.unwrap()),
            post_logout_redirect_uri: None,
            state: None,
            logout_token: None,
        })
        .send()
        .await?;
    assert!(res.status().is_success());

    let token_str = fs::read_to_string(target_file).await.unwrap();
    let mut buf = Vec::with_capacity(256);
    let (_, token) = LogoutToken::build_from_str(&token_str, &mut buf).unwrap();
    eprintln!("token: {token:?}");
    pretty_assertions::assert_eq!(token.iss, "http://localhost:8081/auth/v1/");
    pretty_assertions::assert_eq!(token.aud, test_client.id);
    let now = Utc::now().timestamp();
    assert!(token.iat <= now);
    assert!(token.exp > now);
    pretty_assertions::assert_eq!(
        token.sub,
        Some(user.id.as_str()),
        "the logout token names the logged-in user"
    );
    // Usually, the `sid` would be given, but we did as `password` flow login here, which cannot
    // be linked to a session.
    assert!(token.sid.is_none());

    let res = client
        .delete(format!("{}/users/{}", backend_url, user.id))
        .headers(admin)
        .send()
        .await?;
    expect_status!(res, 204);

    delete_test_client(&test_client.id).await?;

    Ok(())
}
