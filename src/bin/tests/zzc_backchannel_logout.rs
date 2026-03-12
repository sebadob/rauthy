use crate::common::{get_backend_url, get_token_set_init_client};
use chrono::Utc;
use rauthy_api_types::oidc::LogoutRequest;
use rauthy_service::oidc::bcl_logout_token::LogoutToken;
use std::error::Error;
use tokio::fs;

mod common;

#[tokio::test]
async fn test_backchannel_logout() -> Result<(), Box<dyn Error>> {
    // These tests are set up a bit "weird" to reduce complexity.
    // Rauthy exposes `/auth/v1/dev/` endpoints during test and dev.
    // It has a `backchannel_logout` endpoint which just accepts a `logout_token` as speecified in
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

    let ts = get_token_set_init_client().await;

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
    pretty_assertions::assert_eq!(token.aud, "init_client");
    let now = Utc::now().timestamp();
    assert!(token.iat <= now);
    assert!(token.exp > now);
    pretty_assertions::assert_eq!(token.sub.as_deref(), Some("m4PJ3TnyP32LA8hzY23deme3"));
    // Usually, the `sid` would be given, but we did as `password` flow login here, which cannot
    // be linked to a session.
    assert!(token.sid.is_none());

    Ok(())
}
