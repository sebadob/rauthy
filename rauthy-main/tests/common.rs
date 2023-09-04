use rauthy_models::request::TokenRequest;
use rauthy_service::token_set::TokenSet;
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::{header, Response};
use std::env;
use std::error::Error;

#[macro_export]
macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

pub const CLIENT_ID: &str = "init_client";
pub const CLIENT_SECRET: &str = "UBDkjwRc1ogGYXZzvB8bPzqSsIuS4QwYUb4F0vD1Lca2Mhq9Aqz9KWokCSbMq4q3";
pub const USERNAME: &str = "init_admin@localhost.de";
pub const PASSWORD: &str = "123SuperSafe";

#[allow(dead_code)]
pub async fn check_status(res: Response, code: u16) -> Result<Response, Box<dyn Error>> {
    if res.status() != code {
        let status = res.status();
        let err = res.text().await?;
        panic!("Status: {} - Content: {:?}", status, err);
    }
    assert_eq!(res.status(), code);
    Ok(res)
}

#[allow(dead_code)]
pub async fn get_auth_headers() -> Result<HeaderMap, Box<dyn Error>> {
    let token = get_token_set().await?.access_token;
    let mut headers = HeaderMap::new();
    let bearer = format!("Bearer {}", token);
    headers.append(header::AUTHORIZATION, HeaderValue::from_str(&bearer)?);
    Ok(headers)
}

pub fn get_backend_url() -> String {
    dotenvy::from_filename("rauthy.test.cfg").ok();
    let scheme = match env::var("LISTEN_SCHEME")
        .unwrap_or_else(|_| String::from("http"))
        .to_lowercase()
    {
        x if x == "https" || x == "http" || x == "http_https" => x,
        _ => panic!("Bad 'LISTEN_SCHEME'"),
    };
    let host = env::var("PUB_URL").expect("PUB_URL env var is not set");
    let public_url = format!("{}://{}", scheme, host);
    format!("{}/auth/v1", public_url)
}

#[allow(dead_code)]
pub fn get_issuer() -> String {
    format!("{}", get_backend_url())
}

pub async fn get_token_set() -> Result<TokenSet, Box<dyn Error>> {
    let url = format!("{}/oidc/token", get_backend_url());
    let body = TokenRequest {
        grant_type: "password".to_string(),
        code: None,
        redirect_uri: None,
        client_id: Some(CLIENT_ID.to_string()),
        client_secret: Some(CLIENT_SECRET.to_string()),
        code_verifier: None,
        username: Some(USERNAME.to_string()),
        password: Some(PASSWORD.to_string()),
        refresh_token: None,
    };
    let client = reqwest::Client::new();
    let res = client.post(&url).form(&body).send().await?;
    assert_eq!(res.status(), 200);
    let ts = res.json::<TokenSet>().await?;
    Ok(ts)
}
