//! The login requests of all MFA methods offered for one login share the same code. With
//! `optional_select_mfa` on, starting any of them consumes the whole login, so the user cannot
//! start a second method after the first one. With it off, only one method is offered anyway, and
//! a start only removes its own login request.
//!
//! A passkey cannot be registered from a test, so the user here only has an email OTP: the login
//! offers OTP only, and a webauthn start with that code removes the OTP login request as well, when
//! the feature is on. The webauthn start fails either way, because the login request for webauthn
//! was never created.
//!
//! These tests need `otp.enable` and the bootstrap API key, both of which `config-test.toml` sets;
//! the key's secret stays outside the config, because the test process needs it to use the key.
//! `OPTIONAL_SELECT_MFA` must be given to both the backend and the test, since it is not
//! observable from the outside for a user without a passkey:
//!
//! ```sh
//! OPTIONAL_SELECT_MFA=true \
//! BOOTSTRAP_API_KEY_SECRET=MfaAssuranceRegressionSecret0123456789_mfaAssuranceRegressionSec \
//! just test-hiqlite mfa_select
//! ```
//!
//! Without the API key secret or with `otp.enable` off, every test skips itself and the plain
//! `just test-hiqlite` run stays green.
use crate::common::{
    cookie_csrf_headers_from_res_direct, get_backend_url, get_solved_pow, send_login,
};
use crate::{expect_status, expect_status_json};
use rauthy_api_types::generic::Language;
use rauthy_api_types::oidc::LoginRequest;
use rauthy_api_types::users::{
    MfaModTokenRequest, MfaModTokenResponse, MfaPurpose, NewUserRequest, OtpActivateRequest,
    OtpAuthStartRequest, OtpCreateRequest, OtpGetResponse, OtpKind, UpdateUserRequest,
    UserResponseSimple, WebauthnAuthStartRequest,
};
use rauthy_common::sha256;
use rauthy_common::utils::{base64_url_encode, new_store_id};
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use reqwest::{Client, StatusCode};
use std::error::Error;
use std::sync::LazyLock;

const PKCE_VERIFIER: &str = "Ajk3hfdVsDu8DpYuZeYbUfNIWSYdq5sKjpDcTWgM7RBsXQEkSmTNXV6wLvHUmJYQ";
/// The password of the user these tests create. The seeded password policy has a minimum length,
/// and this satisfies it with all of its likely `include_*` rules.
const PASSWORD_ONLY: &str = "MfaSelect!Regression0123";
/// The user of these tests carries a random suffix: it is created, updated and logged in by email,
/// and two runs on one backend must not share the account.
static SUFFIX: LazyLock<String> = LazyLock::new(|| new_store_id().to_lowercase());
static OTP_USER: LazyLock<String> =
    LazyLock::new(|| format!("mfa-select-user-{}@localhost", *SUFFIX));
/// The name inside `BOOTSTRAP_API_KEY` - the very key the `mfa_assurance` suite documents.
const FIXTURE_KEY_NAME: &str = "mfa-assurance";
static FIXTURE_LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();

/// The headers of the API key all fixture work runs on.
fn fixture_headers() -> Result<HeaderMap, Box<dyn Error>> {
    let secret = std::env::var("BOOTSTRAP_API_KEY_SECRET").map_err(
        |_| "BOOTSTRAP_API_KEY_SECRET must be set for both the test backend and these tests",
    )?;

    let mut headers = HeaderMap::new();
    headers.insert(
        AUTHORIZATION,
        HeaderValue::from_str(&format!("API-Key {FIXTURE_KEY_NAME}${secret}"))?,
    );

    Ok(headers)
}

/// `optional_select_mfa` of the backend, which must be given to this test via the same env var.
fn select_mfa_enabled() -> bool {
    std::env::var("OPTIONAL_SELECT_MFA").is_ok_and(|v| v == "true")
}

/// The OTP entry of the `AwaitMfa` body of a `POST /oidc/authorize`, as far as a ceremony needs.
#[derive(serde::Deserialize)]
struct OtpChallenge {
    code: String,
    active_otps: Vec<ActiveOtp>,
}

#[derive(serde::Deserialize)]
struct ActiveOtp {
    otp_id: String,
}

/// Opens a session in `Init` state and returns its cookie + CSRF headers.
async fn init_session() -> Result<HeaderMap, Box<dyn Error>> {
    let res = Client::new()
        .post(format!("{}/oidc/session", get_backend_url()))
        .send()
        .await?;
    cookie_csrf_headers_from_res_direct(res).await
}

/// Password login of a user on an existing session, as the `POST /oidc/authorize` response. The
/// suite runs with `consent.enable`, so the login is sent again with the consent it asks for.
async fn login(email: &str, password: &str, session: &HeaderMap) -> reqwest::Response {
    let backend = get_backend_url();
    let challenge = base64_url_encode(sha256!(PKCE_VERIFIER.as_bytes()));
    let req = LoginRequest {
        email: Some(email.to_string()),
        password: Some(password.to_string()),
        pow: get_solved_pow().await,
        client_id: "rauthy".to_string(),
        redirect_uri: format!("{backend}/oidc/callback"),
        scopes: None,
        state: None,
        nonce: Some("MfaSelectNonce".to_string()),
        code_challenge: Some(challenge.clone()),
        code_challenge_method: Some("S256".to_string()),
        resource: None,
        resident_key_token: None,
        fwda: None,
        consent: None,
    };

    let url_auth = format!(
        "{backend}/oidc/authorize?client_id=rauthy&redirect_uri={backend}/oidc/callback\
        &response_type=code&code_challenge={challenge}&code_challenge_method=S256"
    );
    send_login(&Client::new(), &url_auth, session, req).await
}

/// Creates the test user with a password, re-using and re-setting a left-over of a prior run,
/// because a delete would tear down the sessions of any test running in parallel on that user.
async fn create_test_user(email: &str) -> Result<String, Box<dyn Error>> {
    let _fixture = FIXTURE_LOCK.get_or_init(Default::default).lock().await;
    let backend = get_backend_url();
    let admin = fixture_headers()?;

    let res = Client::new()
        .get(format!("{backend}/users"))
        .headers(admin.clone())
        .send()
        .await?;
    let users = expect_status_json!(res, 200, Vec<UserResponseSimple>);
    let (id, set_password) = match users.iter().find(|u| u.email == email) {
        Some(user) => (user.id.clone(), false),
        None => (
            expect_status_json!(
                Client::new()
                    .post(format!("{backend}/users"))
                    .headers(admin.clone())
                    .json(&NewUserRequest {
                        given_name: Some("MfaSelect".to_string()),
                        family_name: None,
                        email: email.to_string(),
                        preferred_username: None,
                        language: Language::En,
                        roles: vec!["rauthy_user".to_string()],
                        groups: None,
                        user_expires: None,
                        tz: None,
                    })
                    .send()
                    .await?,
                200,
                serde_json::Value
            )["id"]
                .as_str()
                .expect("the new user's id")
                .to_string(),
            true,
        ),
    };

    let res = Client::new()
        .put(format!("{backend}/users/{id}"))
        .headers(admin)
        .json(&UpdateUserRequest {
            email: email.to_string(),
            given_name: Some("MfaSelect".to_string()),
            family_name: None,
            language: Some(Language::En),
            // only ever set on creation: the policy rejects a repeat of a recent password, and a
            // left-over of a parallel test already has exactly this one
            password: set_password.then(|| PASSWORD_ONLY.to_string()),
            roles: vec!["rauthy_user".to_string()],
            groups: None,
            enabled: true,
            email_verified: true,
            user_expires: None,
            user_values: None,
        })
        .send()
        .await?;
    expect_status!(res, 200);

    Ok(id)
}

/// The OTP code the backend has emailed to the user right now, via the dev-only reveal.
async fn dev_otp_code(user_id: &str) -> Result<String, Box<dyn Error>> {
    let res = Client::new()
        .post(format!("{}/dev/otp_code", get_backend_url()))
        .json(&serde_json::json!({ "user_id": user_id }))
        .send()
        .await?;

    Ok(expect_status!(res, 200).text().await?)
}

/// Makes sure the user has an active email OTP, enrolling one over its own login if needed.
async fn ensure_otp(user_id: &str, email: &str) -> Result<(), Box<dyn Error>> {
    let _fixture = FIXTURE_LOCK.get_or_init(Default::default).lock().await;
    let backend = get_backend_url();
    let session = init_session().await?;
    if login(email, PASSWORD_ONLY, &session).await.status() == 200 {
        // the login stops at the 2nd factor -> there already is an active OTP
        return Ok(());
    }

    let res = Client::new()
        .post(format!("{backend}/users/{user_id}/mfa_token"))
        .headers(session.clone())
        .json(&MfaModTokenRequest {
            password: Some(PASSWORD_ONLY.to_string()),
            mfa_code: None,
        })
        .send()
        .await?;
    let mod_token = expect_status_json!(res, 200, MfaModTokenResponse);

    let res = Client::new()
        .post(format!("{backend}/users/{user_id}/otp"))
        .headers(session.clone())
        .json(&OtpCreateRequest {
            otp_name: Some("MFA select".to_string()),
            otp_kind: OtpKind::Email,
            mfa_mod_token_id: mod_token.id.clone(),
        })
        .send()
        .await?;
    // `406` means an OTP row already existed and only needs its id from the list
    let otp_id = match res.status().as_u16() {
        201 => res.json::<OtpGetResponse>().await?.id,
        406 => {
            let res = Client::new()
                .get(format!("{backend}/users/{user_id}/otp"))
                .headers(fixture_headers()?)
                .send()
                .await?;
            expect_status_json!(res, 200, Vec<OtpGetResponse>)
                .first()
                .expect("the existing OTP row")
                .id
                .clone()
        }
        s => panic!("unexpected OTP create response: {s}"),
    };

    let res = Client::new()
        .put(format!("{backend}/users/{user_id}/otp"))
        .headers(session)
        .json(&OtpActivateRequest {
            otp_id,
            otp_code: dev_otp_code(user_id).await?,
            mfa_mod_token_id: mod_token.id,
        })
        .send()
        .await?;
    expect_status!(res, 200);

    Ok(())
}

/// `POST /users/otp_start` for the login of the given challenge.
async fn otp_start(
    session: &HeaderMap,
    challenge: &OtpChallenge,
) -> Result<reqwest::Response, Box<dyn Error>> {
    Ok(Client::new()
        .post(format!("{}/users/otp_start", get_backend_url()))
        .headers(session.clone())
        .json(&OtpAuthStartRequest {
            otp_id: challenge.active_otps[0].otp_id.clone(),
            purpose: MfaPurpose::Login(challenge.code.clone()),
        })
        .send()
        .await?)
}

/// `POST /users/webauthn_start` for the login of the given challenge.
async fn webauthn_start(
    session: &HeaderMap,
    challenge: &OtpChallenge,
) -> Result<reqwest::Response, Box<dyn Error>> {
    Ok(Client::new()
        .post(format!("{}/users/webauthn_start", get_backend_url()))
        .headers(session.clone())
        .json(&WebauthnAuthStartRequest {
            purpose: MfaPurpose::Login(challenge.code.clone()),
        })
        .send()
        .await?)
}

/// Password login of the OTP user that stops at the 2nd factor.
async fn await_mfa(email: &str) -> Result<(HeaderMap, OtpChallenge), Box<dyn Error>> {
    let session = init_session().await?;
    let res = login(email, PASSWORD_ONLY, &session).await;
    let [challenge] = expect_status_json!(res, 200, [OtpChallenge; 1]);

    Ok((session, challenge))
}

/// `true` when these tests can run: the fixture API key is available and `otp.enable` is on. The
/// disabled feature answers `403` on any OTP endpoint, which is the cheapest way to detect it.
async fn gated_suite_ready() -> Result<bool, Box<dyn Error>> {
    let skip = |why: &str| {
        println!("skipping the mfa_select suite: {why}");
        false
    };
    if std::env::var_os("BOOTSTRAP_API_KEY_SECRET").is_none() {
        return Ok(skip("BOOTSTRAP_API_KEY_SECRET is not set"));
    }

    let res = Client::new()
        .get(format!("{}/users/0/otp", get_backend_url()))
        .headers(fixture_headers()?)
        .send()
        .await?;
    if res.status() == StatusCode::FORBIDDEN {
        return Ok(skip("otp.enable is false on this backend"));
    }
    expect_status!(res, 200);

    Ok(true)
}

/// A webauthn start for a login that only offers OTP fails, and with `optional_select_mfa` on it
/// consumes the whole login, so OTP cannot be started afterward. With it off, the OTP login request
/// is left alone and OTP can still be started.
#[tokio::test]
async fn test_mfa_select_webauthn_start_removes_otp_login() -> Result<(), Box<dyn Error>> {
    if !gated_suite_ready().await? {
        return Ok(());
    }
    let user_id = create_test_user(OTP_USER.as_str()).await?;
    ensure_otp(&user_id, OTP_USER.as_str()).await?;

    let (session, challenge) = await_mfa(OTP_USER.as_str()).await?;
    let res = webauthn_start(&session, &challenge).await?;
    expect_status!(res, 401);

    let res = otp_start(&session, &challenge).await?;
    if select_mfa_enabled() {
        expect_status!(res, 404);
    } else {
        expect_status!(res, 200);
    }

    Ok(())
}

/// The OTP start consumes its own login request in any case: a second start with the same code
/// fails.
#[tokio::test]
async fn test_mfa_select_otp_start_consumes_login() -> Result<(), Box<dyn Error>> {
    if !gated_suite_ready().await? {
        return Ok(());
    }
    let user_id = create_test_user(OTP_USER.as_str()).await?;
    ensure_otp(&user_id, OTP_USER.as_str()).await?;

    let (session, challenge) = await_mfa(OTP_USER.as_str()).await?;
    let res = otp_start(&session, &challenge).await?;
    expect_status!(res, 200);

    let res = otp_start(&session, &challenge).await?;
    expect_status!(res, 404);

    Ok(())
}
