use rauthy_api_types::clients::{
    ClientResponse, ClientSecretResponse, NewClientRequest, UpdateClientRequest,
};
use rauthy_api_types::generic::Language;
use rauthy_api_types::oidc::{
    GrantType, JwkKeyPairAlg, LoginConsent, LoginRequest, SessionInfoResponse, TokenRequest,
};
use rauthy_api_types::users::{
    NewUserRequest, RequestResetRequest, UpdateUserRequest, UserResponse, UserResponseSimple,
};
use rauthy_common::constants::CSRF_HEADER;
use rauthy_common::sha256;
use rauthy_common::utils::{base64_url_encode, base64_url_no_pad_decode, new_store_id};
use rauthy_data::entity::consent_sources::LoginConsentContent;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_service::token_set::TokenSet;
use reqwest::header::{HeaderMap, HeaderValue, SET_COOKIE};
use reqwest::{Response, StatusCode, header};
use serde_json::json;
use spow::pow::Pow;
use std::error::Error;
use tokio::sync::OnceCell;

/// Set once the admin has given its consent to the `rauthy` client in this test binary.
static ADMIN_CONSENT: OnceCell<()> = OnceCell::const_new();

pub const CLIENT_ID: &str = "init_client";
pub const CLIENT_SECRET: &str = "LjERi0WSEz1E9OY9KFJaMjlwV1Uf3nuIuOUnJnoJQNm2i7YMjTDMy4PbAKnYRgFy";
pub const USERNAME: &str = "init_admin@localhost";
pub const PASSWORD: &str = "123SuperSafe";
/// A `content_hash` of no consent content.
pub const STALE_HASH: &str = "0000000000000000000000000000000000000000000000000000000000000000";

/// The password for every user a test creates: the test backend's password policy requires at
/// least 14 characters, which the seeded [`PASSWORD`] of `init_admin` does not satisfy.
pub const USER_PASSWORD: &str = "Login-TEST.1234!#$";

/// Asserts the status code of a response and returns it. Only use this when the body is *not*
/// needed — to get the JSON body, use [`expect_status_json!`]. The `panic!` reports this macro's
/// call site, not this definition. The optional last argument adds context to the panic, which is
/// how a test in a loop says *which* case failed.
///
/// The argument is bound exactly once, so it can be a call that consumes its arguments, and it is
/// never evaluated more than once.
#[macro_export]
macro_rules! expect_status {
    ($res:expr, $code:expr) => {
        $crate::expect_status!($res, $code, "")
    };
    ($res:expr, $code:expr, $msg:expr) => {{
        let res = $res;
        if res.status().as_u16() != $code {
            let status = res.status();
            let err = res.text().await.unwrap_or_default();
            if $msg.is_empty() {
                panic!(
                    "Status: {} / Expected: {} - Content: {:?}",
                    status, $code, err
                );
            }
            panic!(
                "Status: {} / Expected: {} - Content: {:?} - {}",
                status, $code, err, $msg
            );
        }
        res
    }};
}

/// The same as `expect_status!`, and the way to get the body of a matching response as JSON:
/// asserts the status, then deserializes and returns it. Needs a `Result` with `reqwest`'s error
/// type because of the `?` on the body read.
#[macro_export]
macro_rules! expect_status_json {
    ($res:expr, $code:expr, $ty:ty) => {{
        let res = $crate::expect_status!($res, $code);
        res.json::<$ty>().await?
    }};
    ($res:expr, $code:expr, $ty:ty, $msg:expr) => {{
        let res = $crate::expect_status!($res, $code, $msg);
        res.json::<$ty>().await?
    }};
}

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
pub async fn get_auth_headers() -> Result<HeaderMap, Box<dyn Error>> {
    Ok(session_headers().await.0)
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
    let res: Response = expect_status!(res, 200, format!("password grant for {email}"));
    res.json::<TokenSet>().await.unwrap()
}

/// A `password` grant for a test's own [`TestClient`]: the seeded `init_client` cannot be used for
/// a client-secret login, because other tests rotate its secret.
pub async fn password_token_set_for(
    http: &reqwest::Client,
    client: &TestClient,
    email: &str,
    password: &str,
) -> TokenSet {
    let res = http
        .post(format!("{}/oidc/token", get_backend_url()))
        .form(&TokenRequest {
            grant_type: GrantType::Password,
            client_id: Some(client.id.clone()),
            client_secret: Some(client.secret.clone()),
            username: Some(email.to_string()),
            password: Some(password.to_string()),
            ..Default::default()
        })
        .send()
        .await
        .expect("the test backend to be running");
    let res: Response = expect_status!(res, 200, format!("password grant for {email}"));
    res.json::<TokenSet>().await.unwrap()
}

/// The setup of a client a test owns: its name, the flows it may use, and the URIs it is allowed
/// to redirect to. Everything a client needs, and nothing a test wants to share.
pub struct TestClientSetup {
    pub name: String,
    pub flows: Vec<GrantType>,
    pub redirect_uri: String,
    pub post_logout_redirect_uris: Vec<String>,
    pub backchannel_logout_uri: Option<String>,
}

/// A confidential client of its own, with the secret a test logs in with.
#[derive(Clone)]
pub struct TestClient {
    pub id: String,
    pub secret: String,
}

/// Creates a client with [`TestClientSetup`], configures it, and rotates its secret.
pub async fn create_test_client(setup: &TestClientSetup) -> Result<TestClient, Box<dyn Error>> {
    let id = format!("test_{}", new_store_id());
    let backend_url = get_backend_url();
    let admin = get_auth_headers().await?;
    let http = reqwest::Client::new();

    let res = http
        .post(format!("{backend_url}/clients"))
        .headers(admin.clone())
        .json(&NewClientRequest {
            id: id.clone(),
            secret: None,
            name: Some(setup.name.clone()),
            confidential: true,
            redirect_uris: vec![setup.redirect_uri.clone()],
            post_logout_redirect_uris: None,
            consent: None,
        })
        .send()
        .await?;
    expect_status!(res, 200, format!("create client {id}"));

    let res = http
        .put(format!("{backend_url}/clients/{id}"))
        .headers(admin.clone())
        .json(&UpdateClientRequest {
            name: Some(setup.name.clone()),
            confidential: true,
            redirect_uris: vec![setup.redirect_uri.clone()],
            post_logout_redirect_uris: Some(setup.post_logout_redirect_uris.clone()),
            allowed_origins: None,
            enabled: true,
            flows_enabled: setup.flows.clone(),
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
            backchannel_logout_uri: setup.backchannel_logout_uri.clone(),
            restrict_group_prefix: None,
            claims: None,
            claims_at_root: false,
            allowed_resources: None,
            default_aud: None,
            actor_same_client: false,
            scim: None,
            consent: None,
        })
        .send()
        .await?;
    expect_status!(res, 200, format!("configure client {id}"));

    let secret = rotate_client_secret(&http, &admin, &id).await?;

    Ok(TestClient { id, secret })
}

/// POSTs `/clients/{id}/secret`, rotates the client's secret, and returns the new one.
pub async fn rotate_client_secret(
    http: &reqwest::Client,
    admin: &HeaderMap,
    id: &str,
) -> Result<String, Box<dyn Error>> {
    let res = http
        .post(format!("{}/clients/{}/secret", get_backend_url(), id))
        .headers(admin.clone())
        .send()
        .await?;
    let res = expect_status!(res, 200, format!("rotate secret of client {id}"));
    Ok(res.json::<ClientSecretResponse>().await?.secret.unwrap())
}

/// Deletes a client created with [`create_test_client`] again.
pub async fn delete_test_client(id: &str) -> Result<(), Box<dyn Error>> {
    let res = reqwest::Client::new()
        .delete(format!("{}/clients/{id}", get_backend_url()))
        .headers(get_auth_headers().await?)
        .send()
        .await?;
    expect_status!(res, 200, format!("delete client {id}"));
    Ok(())
}

/// POST `/clients` with the request as it is and return the created client.
pub async fn create_client(
    http: &reqwest::Client,
    admin: &HeaderMap,
    req: &NewClientRequest,
) -> Result<ClientResponse, Box<dyn Error>> {
    let res = http
        .post(format!("{}/clients", get_backend_url()))
        .headers(admin.clone())
        .json(req)
        .send()
        .await?;
    let res = expect_status!(res, 200, format!("create client {}", req.id));
    Ok(res.json::<ClientResponse>().await?)
}

/// GET `/clients/{id}`.
pub async fn get_client(admin: &HeaderMap, id: &str) -> Result<ClientResponse, Box<dyn Error>> {
    let res = reqwest::Client::new()
        .get(format!("{}/clients/{}", get_backend_url(), id))
        .headers(admin.clone())
        .send()
        .await?;
    let res = expect_status!(res, 200, format!("get client {id}"));
    Ok(res.json::<ClientResponse>().await?)
}

/// PUT `/clients/{id}` with the request as it is and return the stored client.
pub async fn put_client(
    http: &reqwest::Client,
    admin: &HeaderMap,
    id: &str,
    req: &UpdateClientRequest,
) -> Result<ClientResponse, Box<dyn Error>> {
    let res = http
        .put(format!("{}/clients/{}", get_backend_url(), id))
        .headers(admin.clone())
        .json(req)
        .send()
        .await?;
    let res = expect_status!(res, 200, format!("update client {id}"));
    Ok(res.json::<ClientResponse>().await?)
}

/// DELETE `/clients/{id}`.
pub async fn delete_client(admin: &HeaderMap, id: &str) -> Result<(), Box<dyn Error>> {
    let res = reqwest::Client::new()
        .delete(format!("{}/clients/{id}", get_backend_url()))
        .headers(admin.clone())
        .send()
        .await?;
    expect_status!(res, 200, format!("delete client {id}"));
    Ok(())
}

/// Rebuilds a full `UpdateClientRequest` from a GET response, so a test overrides a few fields and
/// PUTs the client back (and so we don't depend on `JwkKeyPairAlg` being `Clone`).
pub fn update_request(c: ClientResponse) -> UpdateClientRequest {
    UpdateClientRequest {
        name: c.name,
        confidential: c.confidential,
        redirect_uris: c.redirect_uris,
        post_logout_redirect_uris: c.post_logout_redirect_uris,
        allowed_origins: c.allowed_origins,
        enabled: c.enabled,
        flows_enabled: c.flows_enabled,
        access_token_alg: c.access_token_alg,
        id_token_alg: c.id_token_alg,
        auth_code_lifetime: c.auth_code_lifetime,
        access_token_lifetime: c.access_token_lifetime,
        scopes: c.scopes,
        default_scopes: c.default_scopes,
        challenges: c.challenges,
        force_mfa: c.force_mfa,
        client_uri: c.client_uri,
        contacts: c.contacts,
        backchannel_logout_uri: c.backchannel_logout_uri,
        restrict_group_prefix: c.restrict_group_prefix,
        claims: c.claims,
        claims_at_root: c.claims_at_root,
        allowed_resources: c.allowed_resources,
        default_aud: c.default_aud,
        consent: c.consent,
        actor_same_client: c.actor_same_client,
        scim: None,
    }
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

/// A session of the admin on the `rauthy` client.
///
/// Almost every test starts with it. On a fresh backend the admin has not consented yet, and
/// without the one login [`ADMIN_CONSENT`] waits for, all of them would be answered with
/// `ConsentRequired` at once and log in a second time: twice the password hashes while the
/// deliberate failed logins of other tests count towards the blacklist of the source address.
pub async fn session_headers() -> (HeaderMap, TokenSet) {
    let redirect_uri = format!("{}/oidc/callback", get_backend_url());
    ADMIN_CONSENT
        .get_or_init(async || {
            session_headers_for_client("rauthy", None, &redirect_uri).await;
        })
        .await;
    session_headers_for_client("rauthy", None, &redirect_uri).await
}

pub async fn session_headers_for_client(
    client_id: &str,
    client_secret: Option<&str>,
    redirect_uri: &str,
) -> (HeaderMap, TokenSet) {
    login_session(&LoginTarget {
        email: USERNAME.to_string(),
        password: PASSWORD.to_string(),
        client_id: client_id.to_string(),
        client_secret: client_secret.map(String::from),
        redirect_uri: redirect_uri.to_string(),
    })
    .await
}

/// A full login of `email` on [`CLIENT_ID`], whose backchannel logout stores the logout token
/// for the tests, returning the session headers and the issued tokens.
pub async fn login_session_init_client(email: &str, password: &str) -> (HeaderMap, TokenSet) {
    login_session(&LoginTarget {
        email: email.to_string(),
        password: password.to_string(),
        client_id: CLIENT_ID.to_string(),
        client_secret: Some(CLIENT_SECRET.to_string()),
        redirect_uri: "http://localhost:3000/oidc/callback".to_string(),
    })
    .await
}

/// The status of `GET /oidc/sessioninfo` with the session `headers`, `401` once it is deleted.
pub async fn session_status(headers: &HeaderMap) -> StatusCode {
    reqwest::Client::new()
        .get(format!("{}/oidc/sessioninfo", get_backend_url()))
        .headers(headers.clone())
        .send()
        .await
        .unwrap()
        .status()
}

/// Who logs in, and on which client: the credentials of the user, and the client with the
/// `redirect_uri` the login ends on. A test that logs in as a user of its own is not affected by
/// any other test's sessions and logouts.
pub struct LoginTarget {
    pub email: String,
    pub password: String,
    pub client_id: String,
    pub client_secret: Option<String>,
    pub redirect_uri: String,
}

/// A full authorization code login (PKCE, S256) on the target client, returning the session
/// headers and the issued tokens.
pub async fn login_session(target: &LoginTarget) -> (HeaderMap, TokenSet) {
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();

    let challenge_plain = "oDXug9zfYqfz8ejcqMpALRPXfW8QhbKV2AVuScAt8xrLKDAmaRYQ4yRi2uqcH9ys";
    let query = format!(
        "client_id={}&redirect_uri={}&response_type=code",
        target.client_id, target.redirect_uri
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
        email: Some(target.email.clone()),
        password: Some(target.password.clone()),
        pow: get_solved_pow().await,
        client_id: target.client_id.clone(),
        redirect_uri: target.redirect_uri.clone(),
        scopes: None,
        state: None,
        nonce: Some("MySuperNonce".to_string()),
        code_challenge: Some(challenge_s256),
        code_challenge_method: Some("S256".to_string()),
        resource: None,
        resident_key_token: None,
        fwda: None,
        consent: None,
    };

    let res = send_login(&client, &url_auth, &headers, req_login).await;
    let res = expect_status!(res, 202);

    let (code, _state) = code_state_from_headers(res).unwrap();
    let req_token = TokenRequest {
        grant_type: GrantType::AuthorizationCode,
        code: Some(code),
        redirect_uri: Some(target.redirect_uri.clone()),
        client_id: Some(target.client_id.clone()),
        client_secret: target.client_secret.clone(),
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
        let res = send_login(
            &reqwest::Client::new(),
            &self.url_auth,
            &self.headers,
            self.req,
        )
        .await;
        let status = res.status();
        if status != 202 {
            let body = res.text().await.unwrap_or_default();
            panic!("login for {email} failed: {status} - {body}");
        }
        headers
    }

    /// Sends the login without consent and returns its response, whatever its status.
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

/// Sends the login `req`. When it fails with `ConsentRequired`, it is sent once more with a new
/// proof of work and the consent it asks for, accepting all requested scopes.
///
/// # Panics
///
/// On any other 400, whose body is read for the check. Logins expected to fail with a 400 are
/// sent directly.
pub async fn send_login(
    client: &reqwest::Client,
    url_auth: &str,
    headers: &HeaderMap,
    mut req: LoginRequest,
) -> Response {
    let send = async |req: &LoginRequest| {
        client
            .post(url_auth)
            .headers(headers.clone())
            .json(req)
            .send()
            .await
            .unwrap()
    };

    let res = send(&req).await;
    if res.status() != 400 {
        return res;
    }
    req.consent = Some(accept_all(&consent_required(res).await));
    req.pow = get_solved_pow().await;
    send(&req).await
}

/// The [`LoginConsentContent`] of a `ConsentRequired` response.
///
/// # Panics
///
/// On any other response.
pub async fn consent_required(res: Response) -> LoginConsentContent {
    let res = expect_status!(res, 400);
    let err = res.json::<ErrorResponse>().await.unwrap();
    assert_eq!(
        err.error,
        ErrorResponseType::ConsentRequired,
        "{}",
        err.message
    );
    consent_content(&err.message)
}

/// The [`LoginConsentContent`] of a `ConsentRequired` message or a login page template.
pub fn consent_content(json: &str) -> LoginConsentContent {
    serde_json::from_str(json).expect("LoginConsentContent JSON")
}

/// The consent template of a login page, `None` if it asks for no consent.
pub fn consent_template(html: &str) -> Option<LoginConsentContent> {
    let (_, tpl) = html.split_once("<template id=\"tpl_consent_content\">")?;
    let (json, _) = tpl
        .split_once("</template>")
        .expect("closed consent template");
    Some(consent_content(json))
}

/// The consent to `content` with all requested scopes.
pub fn accept_all(content: &LoginConsentContent) -> LoginConsent {
    LoginConsent {
        content_hash: content.content.content_hash.clone(),
        accepted_scopes: content
            .content
            .scopes
            .iter()
            .map(|s| s.name.clone())
            .collect(),
    }
}

/// The consent with all requested scopes of `content`, but given for other content.
pub fn stale_consent(content: &LoginConsentContent) -> LoginConsent {
    LoginConsent {
        content_hash: STALE_HASH.to_string(),
        ..accept_all(content)
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
        redirect_uri: redirect_uri.clone(),
        scopes: None,
        state: None,
        nonce: Some("MySuperNonce".to_string()),
        code_challenge: Some(challenge_s256),
        code_challenge_method: Some("S256".to_string()),
        resource: None,
        resident_key_token: None,
        fwda: None,
        consent: None,
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
        expect_status!(res, 204);
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
    let user = expect_status_json!(res, 200, UserResponse);
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
    expect_status!(res, 200);

    Ok(user)
}

/// Deletes the user with this email, the counterpart of [`create_login_user`]. Deleting a user
/// cascades to its sessions, devices and refresh tokens.
pub async fn delete_user(admin: &HeaderMap, email: &str) -> Result<(), Box<dyn Error>> {
    let backend = get_backend_url();
    let users = reqwest::Client::new()
        .get(format!("{backend}/users"))
        .headers(admin.clone())
        .send()
        .await?
        .json::<Vec<UserResponseSimple>>()
        .await?;
    let id = users
        .iter()
        .find(|u| u.email == email)
        .ok_or("test user not found")?
        .id
        .clone();

    let res = reqwest::Client::new()
        .delete(format!("{backend}/users/{id}"))
        .headers(admin.clone())
        .send()
        .await?;
    expect_status!(res, 204, format!("delete user {email}"));
    Ok(())
}

/// The user of a test run: created with a unique email and a usable password, and deleted again by
/// the test itself. A test never logs in with the seeded bootstrap data, so its sessions, tokens
/// and logouts are always its own.
pub struct TestUser {
    pub user: UserResponse,
}

impl TestUser {
    /// Creates a user whose email is derived from `given_name` plus a random id, with `password`.
    pub async fn new(given_name: &str, password: &str) -> Result<Self, Box<dyn Error>> {
        let email = format!(
            "{}-{}@localhost",
            given_name.to_lowercase().replace(' ', "-"),
            new_store_id().to_lowercase()
        );
        let user = create_login_user(
            &reqwest::Client::new(),
            &get_auth_headers().await?,
            &email,
            given_name,
            password,
        )
        .await?;
        Ok(Self { user })
    }

    /// Removes the user again, which cascades to its sessions, devices and refresh tokens.
    pub async fn delete(&self) -> Result<(), Box<dyn Error>> {
        delete_user(&get_auth_headers().await?, &self.user.email).await
    }
}

/// Requests a password reset for `email` on the public endpoint and returns the id of the created
/// magic link: a reset link leaves the backend only via mail, so a test has no other way to ever
/// get the id of a valid one.
pub async fn create_pwd_reset_link(
    client: &reqwest::Client,
    admin: &HeaderMap,
    user_id: &str,
    email: &str,
) -> Result<String, Box<dyn Error>> {
    let backend = get_backend_url();

    let res = client
        .post(format!("{backend}/users/request_reset"))
        .json(&RequestResetRequest {
            email: email.to_string(),
            redirect_uri: None,
            pow: get_solved_pow().await,
        })
        .send()
        .await?;
    expect_status!(res, 200);

    let res = client
        .post(format!("{backend}/dev/magic_link"))
        .headers(admin.clone())
        .json(&json!({ "user_id": user_id }))
        .send()
        .await?;

    Ok(expect_status!(res, 200).text().await?)
}
