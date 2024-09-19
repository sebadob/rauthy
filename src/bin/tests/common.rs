#![allow(dead_code)]
use rauthy_api_types::oidc::{LoginRequest, SessionInfoResponse, TokenRequest};
use rauthy_common::constants::CSRF_HEADER;
use rauthy_common::utils::{base64_url_encode, base64_url_no_pad_decode};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_service::token_set::TokenSet;
use reqwest::header::{HeaderMap, HeaderValue, SET_COOKIE};
use reqwest::{header, Response};
use ring::digest;
use std::env;
use std::error::Error;
use std::sync::OnceLock;
use tracing::error;

#[macro_export]
macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

static SESSION_HEADERS: OnceLock<HeaderMap> = OnceLock::new();

pub const CLIENT_ID: &str = "init_client";
pub const CLIENT_SECRET: &str = "LjERi0WSEz1E9OY9KFJaMjlwV1Uf3nuIuOUnJnoJQNm2i7YMjTDMy4PbAKnYRgFy";
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
    if let Some(headers) = SESSION_HEADERS.get() {
        Ok(headers.clone())
    } else {
        let (headers, _ts) = session_headers().await;
        let _ = SESSION_HEADERS.set(headers.clone());
        Ok(headers)
    }
}

pub fn get_backend_url() -> String {
    dotenvy::from_filename_override("rauthy.test.cfg").ok();
    let scheme = match env::var("LISTEN_SCHEME")
        .unwrap_or_else(|_| String::from("http"))
        .to_lowercase()
    {
        x if x == "https" || x == "http" || x == "http_https" => x,
        x if x == "unix_http" => "http".to_string(),
        x if x == "unix_https" => "https".to_string(),
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

pub async fn get_token_set() -> TokenSet {
    let (_headers, ts) = session_headers().await;
    ts
}

pub async fn session_headers() -> (HeaderMap, TokenSet) {
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();

    let challenge_plain = "oDXug9zfYqfz8ejcqMpALRPXfW8QhbKV2AVuScAt8xrLKDAmaRYQ4yRi2uqcH9ys";
    let redirect_uri = format!("{}/oidc/callback", backend_url);
    let query = format!(
        "client_id=rauthy&redirect_uri={}&response_type=code",
        redirect_uri
    );
    let hash = digest::digest(&digest::SHA256, challenge_plain.as_bytes());
    let challenge_s256 = base64_url_encode(hash.as_ref());
    let query_pkce = format!(
        "{}&code_challenge={}&code_challenge_method=S256",
        query, challenge_s256
    );
    let url_auth = format!("{}/oidc/authorize?{}", backend_url, query_pkce);

    // we need a session in Init state
    let url_session = format!("{}/oidc/session", backend_url);
    let res = client.post(&url_session).send().await.unwrap();
    assert!(res.status().is_success());
    let headers = cookie_csrf_headers_from_res_direct(res).await.unwrap();

    let req_login = LoginRequest {
        email: USERNAME.to_string(),
        password: Some(PASSWORD.to_string()),
        client_id: "rauthy".to_string(),
        redirect_uri: redirect_uri.to_owned(),
        scopes: None,
        state: None,
        nonce: Some("MySuperNonce".to_string()),
        code_challenge: Some(challenge_s256),
        code_challenge_method: Some("S256".to_string()),
    };

    let res = client
        .post(&url_auth)
        .headers(headers.clone())
        .json(&req_login)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 202);

    let (code, _state) = code_state_from_headers(res).unwrap();
    let req_token = TokenRequest {
        grant_type: "authorization_code".to_string(),
        code: Some(code),
        redirect_uri: None,
        client_id: Some("rauthy".to_string()),
        client_secret: None,
        code_verifier: Some(challenge_plain.to_string()),
        device_code: None,
        username: None,
        password: None,
        refresh_token: None,
    };

    let url_token = format!("{}/oidc/token", backend_url);
    let res = client
        .post(&url_token)
        .form(&req_token)
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);

    let ts = res.json::<TokenSet>().await.unwrap();

    (headers, ts)
}

/// extractor for the POST `/oidc/session` endpoint
pub async fn cookie_csrf_headers_from_res_direct(
    res: Response,
) -> Result<HeaderMap, Box<dyn Error>> {
    assert!(res.status().is_success());

    let cookie = res
        .headers()
        .get(SET_COOKIE)
        .expect("Set-Cookie header to exist");
    let (session_cookie, _) = cookie.to_str()?.split_once(';').unwrap();

    let mut headers = HeaderMap::new();
    headers.append(header::COOKIE, HeaderValue::from_str(&session_cookie)?);

    let session_info = res.json::<SessionInfoResponse>().await.unwrap();
    headers.append(
        CSRF_HEADER,
        HeaderValue::from_str(&session_info.csrf_token.unwrap())?,
    );

    Ok(headers)
}

/// extractor from the `/oidc/authorize` html
pub async fn cookie_csrf_headers_from_res(res: Response) -> Result<HeaderMap, Box<dyn Error>> {
    let cookie = res.headers().get(header::SET_COOKIE).unwrap();
    let (session_cookie, _) = cookie.to_str()?.split_once(';').unwrap();
    println!("Extracted session cookie: {:?}", session_cookie);
    let mut headers = HeaderMap::new();
    headers.append(header::COOKIE, HeaderValue::from_str(&session_cookie)?);

    let content = res.text().await?;
    let csrf_find = "name=\"rauthy-csrf-token\" id=\"";
    let (_, content_split) = content.split_once(csrf_find).unwrap();
    let (csrf_token, _) = content_split.split_once('"').unwrap();
    println!("Extracted CSRF Token: {}", csrf_token);
    headers.append(CSRF_HEADER, HeaderValue::from_str(csrf_token)?);

    Ok(headers)
}

pub fn code_state_from_headers(res: Response) -> Result<(String, Option<String>), Box<dyn Error>> {
    let loc_header = res
        .headers()
        .get(header::LOCATION)
        .unwrap()
        .to_str()
        .unwrap();
    println!("Location Header: {}", loc_header);

    let code: String;
    let mut state = None;
    let (_, code_str) = loc_header.split_once("code=").unwrap();
    if let Some((c, state_str)) = code_str.split_once('&') {
        code = c.to_string();
        let (_, s) = state_str.split_once("state=").unwrap();
        state = Some(s.to_string());
    } else {
        code = code_str.to_string();
    }

    Ok((code, state))
}

// Extracts the claims from a given token into a HashMap.
// Returns an empty HashMap if no values could be extracted at all.
// CAUTION: Does not validate the token!
pub fn extract_token_claims_unverified<T>(token: &str) -> Result<T, ErrorResponse>
where
    T: for<'a> serde::Deserialize<'a>,
{
    let body = match token.split_once('.') {
        None => None,
        Some((_metadata, rest)) => rest.split_once('.').map(|(body, _validation_str)| body),
    };
    if body.is_none() {
        return Err(ErrorResponse::new(
            ErrorResponseType::Unauthorized,
            "Invalid or malformed JWT Token",
        ));
    }
    let body = body.unwrap();
    let b64 = base64_url_no_pad_decode(body).unwrap();

    let s = String::from_utf8_lossy(b64.as_slice());
    let claims = match serde_json::from_str::<T>(s.as_ref()) {
        Ok(claims) => claims,
        Err(err) => {
            error!("Error deserializing JWT Token claims: {}", err);
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Invalid JWT Token claims",
            ));
        }
    };

    Ok(claims)
}
