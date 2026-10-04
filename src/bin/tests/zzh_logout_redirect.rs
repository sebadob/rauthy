use crate::common::{PASSWORD, USERNAME, get_auth_headers, get_backend_url, session_headers};
use pretty_assertions::assert_eq;
use rauthy_api_types::clients::{
    ClientResponse, ClientSecretResponse, NewClientRequest, UpdateClientRequest,
};
use rauthy_api_types::oidc::{GrantType, JwkKeyPairAlg, LogoutRequest, TokenRequest};
use rauthy_service::token_set::TokenSet;
use reqwest::header;
use std::error::Error;

mod common;

const ID: &str = "logout_redirect_test";
const POST_LOGOUT_URI: &str = "http://localhost:8080/bye?foo=bar";
const POST_LOGOUT_URI_WILDCARD: &str = "http://localhost:8080/wild/*";

fn client_update() -> UpdateClientRequest {
    UpdateClientRequest {
        name: Some("Logout Redirect Test".to_string()),
        confidential: true,
        redirect_uris: vec!["http://localhost:8080/callback".to_string()],
        post_logout_redirect_uris: Some(vec![
            POST_LOGOUT_URI.to_string(),
            POST_LOGOUT_URI_WILDCARD.to_string(),
        ]),
        allowed_origins: None,
        enabled: true,
        flows_enabled: vec![GrantType::Password],
        access_token_alg: JwkKeyPairAlg::EdDSA,
        id_token_alg: JwkKeyPairAlg::EdDSA,
        auth_code_lifetime: 60,
        access_token_lifetime: 300,
        scopes: vec!["openid".to_string()],
        default_scopes: vec!["openid".to_string()],
        challenges: Some(vec!["S256".to_string()]),
        force_mfa: false,
        client_uri: None,
        contacts: None,
        backchannel_logout_uri: None,
        restrict_group_prefix: None,
        claims: None,
        claims_at_root: false,
        allowed_resources: None,
        default_aud: None,
        scim: None,
    }
}

/// A fresh `id_token` for the test user, since a successful logout revokes the session tokens.
async fn id_token(http: &reqwest::Client, secret: &str) -> String {
    let res = http
        .post(format!("{}/oidc/token", get_backend_url()))
        .form(&TokenRequest {
            grant_type: GrantType::Password,
            client_id: Some(ID.to_string()),
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
        panic!("Error during password login for '{ID}':\n{text}");
    }
    res.json::<TokenSet>().await.unwrap().id_token.unwrap()
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

    let res = http
        .post(format!("{backend_url}/clients"))
        .headers(auth_headers.clone())
        .json(&NewClientRequest {
            id: ID.to_string(),
            secret: None,
            name: Some("Logout Redirect Test".to_string()),
            confidential: true,
            redirect_uris: vec!["http://localhost:8080/callback".to_string()],
            post_logout_redirect_uris: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let _: ClientResponse = res.json().await?;

    let res = http
        .put(format!("{backend_url}/clients/{ID}"))
        .headers(auth_headers.clone())
        .json(&client_update())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let res = http
        .post(format!("{backend_url}/clients/{ID}/secret"))
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let secret = res.json::<ClientSecretResponse>().await?.secret.unwrap();

    let token = id_token(&http, &secret).await;

    // These pass the wildcard prefix match, but would let a crafted URI hand the client a second
    // `state`, or swallow the appended one in a fragment. They are rejected before any logout.
    let res = logout(
        &http,
        token.clone(),
        "http://localhost:8080/wild/x?foo=bar&State%5B%5D=evil",
        Some("st4te"),
    )
    .await;
    assert_eq!(res.status(), 400);
    assert!(res.headers().get(header::LOCATION).is_none());
    let body = res.text().await?;
    assert!(body.contains("`redirect_uri` must not contain"));

    // `state` is appended once, form-urlencoded with a space as `%20`, to the existing query
    let state = "a b+c&state=evil#x";
    let res = logout(&http, token, POST_LOGOUT_URI, Some(state)).await;
    assert_eq!(res.status(), 302);
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

    // the logout ended the admin session as well
    let (auth_headers, _) = session_headers().await;
    let res = http
        .delete(format!("{backend_url}/clients/{ID}"))
        .headers(auth_headers)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    Ok(())
}
