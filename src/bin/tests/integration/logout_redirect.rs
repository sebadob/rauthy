use crate::common::{
    TestClientSetup, USER_PASSWORD, create_login_user, create_test_client, delete_test_client,
    get_auth_headers, get_backend_url, password_token_set_for,
};
use crate::expect_status;
use pretty_assertions::assert_eq;
use rauthy_api_types::oidc::{GrantType, LogoutRequest};
use reqwest::header;
use std::error::Error;

const POST_LOGOUT_URI: &str = "http://localhost:8080/bye?foo=bar";
const POST_LOGOUT_URI_WILDCARD: &str = "http://localhost:8080/wild/*";

fn setup() -> TestClientSetup {
    TestClientSetup {
        name: "Logout Redirect Test".to_string(),
        flows: vec![GrantType::Password],
        redirect_uri: "http://localhost:8080/callback".to_string(),
        post_logout_redirect_uris: vec![
            POST_LOGOUT_URI.to_string(),
            POST_LOGOUT_URI_WILDCARD.to_string(),
        ],
        backchannel_logout_uri: None,
    }
}

/// An RP-Initiated Logout as a browser sends it, so the response is a redirect.
async fn logout(
    http: &reqwest::Client,
    id_token_hint: String,
    post_logout_redirect_uri: &str,
    state: Option<&str>,
) -> reqwest::Response {
    http.post(format!("{}/oidc/logout", get_backend_url()))
        .header("sec-fetch-site", "cross-site")
        .form(&LogoutRequest {
            id_token_hint: Some(id_token_hint),
            post_logout_redirect_uri: Some(post_logout_redirect_uri.to_string()),
            state: state.map(String::from),
            logout_token: None,
        })
        .send()
        .await
        .expect("the test backend to be running")
}

#[tokio::test]
async fn test_logout_redirect() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();
    let http = reqwest::Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()?;

    let client = create_test_client(&setup()).await?;
    let email = format!("logout-redirect-{}@localhost.de", client.id);

    // A user of its own: a successful logout ends every session of that user, and the seeded
    // `init_admin` shares its sessions with every other test that logs in as admin.
    let user = create_login_user(
        &http,
        &auth_headers,
        &email,
        "Logout Redirect",
        USER_PASSWORD,
    )
    .await?;

    // A fresh `id_token` for the test user, since a successful logout revokes the session tokens.
    let token = password_token_set_for(&http, &client, &email, USER_PASSWORD)
        .await
        .id_token
        .unwrap();

    // These pass the wildcard prefix match, but would let a crafted URI hand the client a second
    // `state`, or swallow the appended one in a fragment. They are rejected before any logout.
    let res = logout(
        &http,
        token.clone(),
        "http://localhost:8080/wild/x?foo=bar&State%5B%5D=evil",
        Some("st4te"),
    )
    .await;
    let res = expect_status!(res, 400);
    assert!(res.headers().get(header::LOCATION).is_none());
    let body = res.text().await?;
    assert!(body.contains("`redirect_uri` must not contain"));

    // `state` is appended once, form-urlencoded with a space as `%20`, to the existing query
    let state = "a b+c&state=evil#x";
    let res = logout(&http, token, POST_LOGOUT_URI, Some(state)).await;
    let res = expect_status!(res, 302);
    let loc = res
        .headers()
        .get(header::LOCATION)
        .unwrap()
        .to_str()?
        .to_string();
    assert_eq!(
        loc,
        format!("{POST_LOGOUT_URI}&state=a%20b%2Bc%26state%3Devil%23x")
    );
    let url = reqwest::Url::parse(&loc)?;
    assert!(url.fragment().is_none());
    assert_eq!(
        url.query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect::<Vec<_>>(),
        vec![
            ("foo".to_string(), "bar".to_string()),
            ("state".to_string(), state.to_string()),
        ]
    );

    delete_test_client(&client.id).await?;

    let res = http
        .delete(format!("{backend_url}/users/{}", user.id))
        .headers(auth_headers)
        .send()
        .await?;
    expect_status!(res, 204);

    Ok(())
}
