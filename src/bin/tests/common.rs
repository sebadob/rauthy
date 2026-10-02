#![allow(dead_code)]
use rauthy_api_types::oidc::{GrantType, LoginRequest, SessionInfoResponse, TokenRequest};
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
    get_backend_url()
}

/// The RFC 9207 `iss` value as it appears on the wire: full issuer with trailing `/`,
/// form-urlencoded independently of the server-side encoder.
pub fn get_issuer_urlencoded() -> String {
    format!(
        "{}%2F",
        get_issuer().replace(':', "%3A").replace('/', "%2F")
    )
}

pub async fn get_token_set() -> TokenSet {
    let (_headers, ts) = session_headers().await;
    ts
}

pub async fn get_token_set_init_client() -> TokenSet {
    // get a token to validate
    let url_token = format!("{}/oidc/token", get_backend_url());
    let body = TokenRequest {
        grant_type: GrantType::Password,
        client_id: Some(CLIENT_ID.to_string()),
        client_secret: Some(CLIENT_SECRET.to_string()),
        username: Some(USERNAME.to_string()),
        password: Some(PASSWORD.to_string()),
        ..Default::default()
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
        email: Some(USERNAME.to_string()),
        password: Some(PASSWORD.to_string()),
        pow: get_solved_pow().await,
        client_id: "rauthy".to_string(),
        redirect_uri: redirect_uri.to_string(),
        scopes: None,
        state: None,
        nonce: Some("MySuperNonce".to_string()),
        code_challenge: Some(challenge_s256),
        code_challenge_method: Some("S256".to_string()),
        resource: None,
        resident_key_token: None,
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
        grant_type: GrantType::AuthorizationCode,
        code: Some(code),
        redirect_uri: Some(redirect_uri.to_string()),
        client_id: Some("rauthy".to_string()),
        code_verifier: Some(challenge_plain.to_string()),
        ..Default::default()
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

/// Logs in as an arbitrary user (must have a usable password) and returns the
/// authenticated session headers (cookie + CSRF). Mirrors [`session_headers`] but with
/// caller-provided credentials, used to obtain a delegated group-admin session.
pub async fn session_headers_with(email: &str, password: &str) -> HeaderMap {
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();

    let challenge_plain = "oDXug9zfYqfz8ejcqMpALRPXfW8QhbKV2AVuScAt8xrLKDAmaRYQ4yRi2uqcH9ys";
    let redirect_uri = format!("{}/oidc/callback", backend_url);
    let challenge_s256 = base64_url_encode(sha256!(challenge_plain.as_bytes()));
    let query_pkce = format!(
        "client_id=rauthy&redirect_uri={}&response_type=code&code_challenge={}\
        &code_challenge_method=S256",
        redirect_uri, challenge_s256
    );
    let url_auth = format!("{}/oidc/authorize?{}", backend_url, query_pkce);

    let url_session = format!("{}/oidc/session", backend_url);
    let res = client.post(&url_session).send().await.unwrap();
    assert!(res.status().is_success());
    let headers = cookie_csrf_headers_from_res_direct(res).await.unwrap();

    let req_login = LoginRequest {
        email: Some(email.to_string()),
        password: Some(password.to_string()),
        pow: get_solved_pow().await,
        client_id: "rauthy".to_string(),
        redirect_uri: redirect_uri.to_string(),
        scopes: None,
        state: None,
        nonce: Some("MySuperNonce".to_string()),
        code_challenge: Some(challenge_s256),
        code_challenge_method: Some("S256".to_string()),
        resource: None,
        resident_key_token: None,
    };

    let res = client
        .post(&url_auth)
        .headers(headers.clone())
        .json(&req_login)
        .send()
        .await
        .unwrap();
    let status = res.status();
    if status != 202 {
        let body = res.text().await.unwrap_or_default();
        panic!("group-admin login for {email} failed: {status} - {body}");
    }
    headers
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
    headers.append(header::COOKIE, HeaderValue::from_str(session_cookie)?);

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
            headers.append(header::COOKIE, HeaderValue::from_str(cookie)?);

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

fn location_url(res: &Response) -> Result<reqwest::Url, Box<dyn Error>> {
    let loc_header = res
        .headers()
        .get(header::LOCATION)
        .ok_or("missing Location header")?
        .to_str()?;
    println!("Location Header: {}", loc_header);

    let url = reqwest::Url::parse(loc_header)?;
    if url.fragment().is_some() {
        return Err("Location must not contain a fragment".into());
    }
    Ok(url)
}

/// Raw (undecoded) pairs of the parsed `Location` query; asserts exactly one correct `iss`.
pub fn authorization_response_params(
    res: &Response,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let url = location_url(res)?;

    for key in ["code", "state", "iss", "error"] {
        let count = url.query_pairs().filter(|(k, _)| k == key).count();
        if count > 1 {
            return Err(format!("query param '{key}' appears {count} times").into());
        }
    }
    let iss = url
        .query_pairs()
        .find(|(k, _)| k == "iss")
        .map(|(_, v)| v.into_owned());
    assert_eq!(iss, Some(format!("{}/", get_issuer())));

    let params = url
        .query()
        .ok_or("Location has no query")?
        .split('&')
        .filter(|kv| !kv.is_empty())
        .map(|kv| {
            let (k, v) = kv.split_once('=').unwrap_or((kv, ""));
            (k.to_string(), v.to_string())
        })
        .collect::<Vec<_>>();

    let iss = params
        .iter()
        .find(|(k, _)| k == "iss")
        .map(|(_, v)| v.as_str());
    assert_eq!(iss, Some(get_issuer_urlencoded().as_str()));

    Ok(params)
}

/// Like `authorization_response_params`, but percent-decoded as a client sees them.
pub fn authorization_response_params_decoded(
    res: &Response,
) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    Ok(location_url(res)?
        .query_pairs()
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect())
}

pub fn code_state_from_headers(res: Response) -> Result<(String, Option<String>), Box<dyn Error>> {
    let params = authorization_response_params(&res)?;
    let get = |key: &str| {
        params
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v.clone())
    };

    let code = get("code").ok_or("missing 'code' in Location")?;
    Ok((code, get("state")))
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
