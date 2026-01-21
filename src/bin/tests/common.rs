#![allow(dead_code)]
use rauthy_api_types::oidc::{LoginRequest, SessionInfoResponse, TokenRequest};
use rauthy_common::constants::CSRF_HEADER;
use rauthy_common::sha256;
use rauthy_common::utils::base64_url_encode;
use rauthy_service::token_set::TokenSet;
use reqwest::header::{HeaderMap, HeaderValue, SET_COOKIE};
use reqwest::{Response, header};
use spow::pow::Pow;
use std::error::Error;
use std::sync::OnceLock;

#[macro_export]
macro_rules! aw {
    ($e:expr) => {
        tokio_test::block_on($e)
    };
}

static SESSION_HEADERS: OnceLock<HeaderMap> = OnceLock::new();

pub const CLIENT_ID: &str = "init_client";
pub const CLIENT_SECRET: &str = "LjERi0WSEz1E9OY9KFJaMjlwV1Uf3nuIuOUnJnoJQNm2i7YMjTDMy4PbAKnYRgFy";
pub const USERNAME: &str = "init_admin@localhost";
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
    "http://localhost:8081/auth/v1".to_string()
}

#[allow(dead_code)]
pub fn get_issuer() -> String {
    format!("{}", get_backend_url())
}

pub async fn get_token_set() -> TokenSet {
    let (_headers, ts) = session_headers().await;
    ts
}

pub async fn get_token_set_init_client() -> TokenSet {
    // get a token to validate
    let url_token = format!("{}/oidc/token", get_backend_url());
    let body = TokenRequest {
        grant_type: "password".to_string(),
        code: None,
        redirect_uri: None,
        client_id: Some(CLIENT_ID.to_string()),
        client_secret: Some(CLIENT_SECRET.to_string()),
        code_verifier: None,
        device_code: None,
        username: Some(USERNAME.to_string()),
        password: Some(PASSWORD.to_string()),
        refresh_token: None,
    };

    let res = reqwest::Client::new()
        .post(&url_token)
        .form(&body)
        .send()
        .await
        .expect("Is the test-backend running?");
    if !res.status().is_success() {
        let text = res.text().await.unwrap();
        panic!("Error during login to init_client:\n{text}");
    }

    res.json::<TokenSet>().await.unwrap()
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
    let challenge_s256 = base64_url_encode(sha256!(challenge_plain.as_bytes()));
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
        pow: get_solved_pow().await,
        client_id: "rauthy".to_string(),
        redirect_uri: redirect_uri.to_string(),
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
        redirect_uri: Some(redirect_uri.to_string()),
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
    for cookie in res.headers().get_all(header::SET_COOKIE) {
        let (cookie, _) = cookie.to_str()?.split_once(';').unwrap();
        if cookie.starts_with("__Host-RauthySession=") {
            println!("Extracted session cookie: {:?}", cookie);
            let mut headers = HeaderMap::new();
            headers.append(header::COOKIE, HeaderValue::from_str(&cookie)?);

            let content = res.text().await?;
            let (_, content_split) = content
                .split_once("<template id=\"tpl_csrf_token\">")
                .unwrap();
            let (csrf_token, _) = content_split.split_once("</template>").unwrap();
            println!("Extracted CSRF Token: {}", csrf_token);
            headers.append(CSRF_HEADER, HeaderValue::from_str(csrf_token)?);

            return Ok(headers);
        }
    }

    panic!("Error extracting session cookie");
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

pub fn init_client_bcl_uri() -> String {
    "http://localhost:8081/auth/v1/dev/backchannel_logout".to_string()
}

pub async fn get_solved_pow() -> String {
    let url = format!("{}/pow", get_backend_url());
    let res = reqwest::Client::new().post(&url).send().await.unwrap();
    let pow = res.text().await.unwrap();
    Pow::work(&pow).unwrap()
}
