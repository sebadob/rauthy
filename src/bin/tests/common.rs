#![allow(dead_code)]
use rauthy_api_types::generic::Language;
use rauthy_api_types::oidc::{GrantType, LoginRequest, SessionInfoResponse, TokenRequest};
use rauthy_api_types::users::{
    NewUserRequest, UpdateUserRequest, UserResponse, UserResponseSimple,
};
use rauthy_common::constants::CSRF_HEADER;
use rauthy_common::sha256;
use rauthy_common::utils::{base64_url_encode, base64_url_no_pad_decode};
use rauthy_service::token_set::TokenSet;
use reqwest::header::{HeaderMap, HeaderValue, SET_COOKIE};
use reqwest::{Response, header};
use spow::pow::Pow;
use std::error::Error;
use std::sync::OnceLock;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time;

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

/// Decodes the (unverified) JWT payload, so a test can assert on any claim.
#[allow(dead_code)]
pub fn decode_claims(access_token: &str) -> serde_json::Value {
    let payload_b64 = access_token
        .split('.')
        .nth(1)
        .expect("a JWT payload segment");
    let bytes = base64_url_no_pad_decode(payload_b64).expect("valid base64url payload");
    serde_json::from_slice(&bytes).expect("valid JSON claims")
}

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

/// A password grant for `email` on the init client, whatever its status.
pub async fn password_grant(client: &reqwest::Client, email: &str, password: &str) -> Response {
    client
        .post(format!("{}/oidc/token", get_backend_url()))
        .form(&TokenRequest {
            grant_type: GrantType::Password,
            client_id: Some(CLIENT_ID.to_string()),
            client_secret: Some(CLIENT_SECRET.to_string()),
            username: Some(email.to_string()),
            password: Some(password.to_string()),
            ..Default::default()
        })
        .send()
        .await
        .unwrap()
}

/// A password grant token set for `email` on the init client.
pub async fn password_token_set(client: &reqwest::Client, email: &str, password: &str) -> TokenSet {
    let res = password_grant(client, email, password).await;
    assert_eq!(res.status(), 200, "password grant for {email}");
    res.json::<TokenSet>().await.unwrap()
}

/// A refresh token grant on the init client.
pub async fn refresh(client: &reqwest::Client, refresh_token: String) -> Response {
    client
        .post(format!("{}/oidc/token", get_backend_url()))
        .form(&TokenRequest {
            grant_type: GrantType::RefreshToken,
            client_id: Some(CLIENT_ID.to_string()),
            client_secret: Some(CLIENT_SECRET.to_string()),
            refresh_token: Some(refresh_token),
            ..Default::default()
        })
        .send()
        .await
        .unwrap()
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
    let redirect_uri = format!("{}/oidc/callback", get_backend_url());
    session_headers_for_client("rauthy", None, &redirect_uri).await
}

pub async fn session_headers_for_client(
    client_id: &str,
    client_secret: Option<&str>,
    redirect_uri: &str,
) -> (HeaderMap, TokenSet) {
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();

    let challenge_plain = "oDXug9zfYqfz8ejcqMpALRPXfW8QhbKV2AVuScAt8xrLKDAmaRYQ4yRi2uqcH9ys";
    let query = format!(
        "client_id={client_id}&redirect_uri={}&response_type=code",
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
        client_id: client_id.to_string(),
        redirect_uri: redirect_uri.to_string(),
        scopes: None,
        state: None,
        nonce: Some("MySuperNonce".to_string()),
        code_challenge: Some(challenge_s256),
        code_challenge_method: Some("S256".to_string()),
        resource: None,
        resident_key_token: None,
        fwda: None,
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
        client_id: Some(client_id.to_string()),
        client_secret: client_secret.map(String::from),
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
    if res.status() != 200 {
        panic!("Error getting session headers: {res:?}");
    }

    let ts = res.json::<TokenSet>().await.unwrap();

    (headers, ts)
}

/// Logs in as an arbitrary user (must have a usable password) and returns the
/// authenticated session headers (cookie + CSRF). Mirrors [`session_headers`] but with
/// caller-provided credentials, used to obtain a delegated group-admin session.
pub async fn session_headers_with(email: &str, password: &str) -> HeaderMap {
    prepare_login(email, password).await.send().await
}

/// A password login to the `rauthy` client with its session and proof of work done, whose
/// `POST /oidc/authorize` is only sent by [`PreparedLogin::send`].
pub struct PreparedLogin {
    url_auth: String,
    headers: HeaderMap,
    req: LoginRequest,
}

impl PreparedLogin {
    /// Sends the login and returns its authenticated session headers.
    pub async fn send(self) -> HeaderMap {
        let email = self.req.email.clone().unwrap_or_default();
        let headers = self.headers.clone();
        let res = self.response().await;
        let status = res.status();
        if status != 202 {
            let body = res.text().await.unwrap_or_default();
            panic!("login for {email} failed: {status} - {body}");
        }
        headers
    }

    /// Sends the login and returns its response, whatever its status.
    pub async fn response(self) -> Response {
        reqwest::Client::new()
            .post(&self.url_auth)
            .headers(self.headers)
            .json(&self.req)
            .send()
            .await
            .unwrap()
    }
}

/// Opens a session, solves its proof of work and builds the login without sending it, so callers
/// can send several at once through [`PreparedLogin::send`], or [`PreparedLogin::response`] for a
/// login expected to fail.
pub async fn prepare_login(email: &str, password: &str) -> PreparedLogin {
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

    let req = LoginRequest {
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
        fwda: None,
    };

    PreparedLogin {
        url_auth,
        headers,
        req,
    }
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

/// Creates `email` with the role `user`, `given_name` and `password`, deleting a left-over of a
/// prior run first.
pub async fn create_login_user(
    client: &reqwest::Client,
    admin: &HeaderMap,
    email: &str,
    given_name: &str,
    password: &str,
) -> Result<UserResponse, Box<dyn Error>> {
    let backend = get_backend_url();

    let res = client
        .get(format!("{backend}/users"))
        .headers(admin.clone())
        .send()
        .await?;
    let users = res.json::<Vec<UserResponseSimple>>().await?;
    if let Some(user) = users.iter().find(|u| u.email == email) {
        let res = client
            .delete(format!("{backend}/users/{}", user.id))
            .headers(admin.clone())
            .send()
            .await?;
        assert_eq!(res.status(), 204);
    }

    let res = client
        .post(format!("{backend}/users"))
        .headers(admin.clone())
        .json(&NewUserRequest {
            given_name: Some(given_name.to_string()),
            family_name: None,
            email: email.to_string(),
            preferred_username: None,
            language: Language::En,
            roles: vec!["user".to_string()],
            groups: None,
            user_expires: None,
            tz: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let user = res.json::<UserResponse>().await?;
    let res = client
        .put(format!("{backend}/users/{}", user.id))
        .headers(admin.clone())
        .json(&UpdateUserRequest {
            email: user.email.clone(),
            given_name: user.given_name.clone(),
            family_name: user.family_name.clone(),
            language: Some(Language::En),
            password: Some(password.to_string()),
            roles: user.roles.clone(),
            groups: user.groups.clone(),
            enabled: true,
            email_verified: true,
            user_expires: None,
            user_values: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    Ok(user)
}

/// In three rounds, starts concurrent logins of `user` through `start_logins` and, while they wait
/// for their password hash, changes the user's name and roles as an admin. Asserts after each
/// round that the admin change survived the logins, which save only their login state.
pub async fn assert_admin_update_survives_logins(
    client: &reqwest::Client,
    admin: &HeaderMap,
    user: &UserResponse,
    start_logins: impl AsyncFn() -> Vec<JoinHandle<()>>,
) -> Result<(), Box<dyn Error>> {
    let url_user = format!("{}/users/{}", get_backend_url(), user.id);

    for round in 0..3 {
        let given_name = format!("Admin round {round}");
        let roles = if round % 2 == 0 {
            vec!["admin".to_string(), "user".to_string()]
        } else {
            vec!["user".to_string()]
        };

        let logins = start_logins().await;
        // update while the logins wait for their password hash, one at a time
        time::sleep(Duration::from_millis(20)).await;
        let res = client
            .put(&url_user)
            .headers(admin.clone())
            .json(&UpdateUserRequest {
                email: user.email.clone(),
                given_name: Some(given_name.clone()),
                family_name: user.family_name.clone(),
                language: Some(Language::En),
                password: None,
                roles: roles.clone(),
                groups: user.groups.clone(),
                enabled: true,
                email_verified: true,
                user_expires: None,
                user_values: None,
            })
            .send()
            .await?;
        assert_eq!(res.status(), 200);
        for login in logins {
            login.await?;
        }

        let res = client.get(&url_user).headers(admin.clone()).send().await?;
        assert_eq!(res.status(), 200);
        let found = res.json::<UserResponse>().await?;
        assert_eq!(
            found.given_name.as_deref(),
            Some(given_name.as_str()),
            "round {round}"
        );
        let mut found_roles = found.roles.clone();
        found_roles.sort();
        assert_eq!(found_roles, roles, "round {round}");
        assert!(found.last_login.is_some());
        assert_eq!(found.failed_login_attempts, None);
    }

    Ok(())
}
