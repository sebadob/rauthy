//! Regression tests for the MFA assurance of admin sessions: `Session::is_mfa` may only ever be set
//! by a completed 2nd factor ceremony, never by the *request* of one.
//!
//! These tests need both feature flags that the default `config-test.toml` keeps disabled, plus the
//! bootstrap API key every fixture request runs on: an API key is not a session, and with
//! `admin_force_mfa` on, no session of the MFA-less bootstrap admin can create users at all.
//!
//! ```sh
//! ADMIN_FORCE_MFA=true OTP_ENABLE=true \
//! BOOTSTRAP_API_KEY=eyJuYW1lIjoibWZhLWFzc3VyYW5jZSIsImV4cCI6bnVsbCwiYWNjZXNzIjpbeyJncm91cCI6IlVzZXJzIiwiYWNjZXNzX3JpZ2h0cyI6WyJyZWFkIiwiY3JlYXRlIiwidXBkYXRlIiwiZGVsZXRlIl19XX0= \
//! BOOTSTRAP_API_KEY_SECRET=MfaAssuranceRegressionSecret0123456789_mfaAssuranceRegressionSec \
//! just test-hiqlite mfa_assurance
//! ```
//!
//! The bootstrap admin is used exactly as the bootstrap seeds it, without any MFA, and every user
//! with an OTP is created by these tests, so nothing here interferes with the rest of the suite.
//! Without those variables, the gate under test is off and every test skips itself, so the plain
//! `just test-hiqlite` run stays green.
mod common;

use crate::common::{
    PASSWORD, USERNAME, cookie_csrf_headers_from_res_direct, get_backend_url, get_solved_pow,
};
use pretty_assertions::assert_eq;
use rauthy_api_types::generic::Language;
use rauthy_api_types::oidc::LoginRequest;
use rauthy_api_types::users::{
    MfaModTokenRequest, MfaModTokenResponse, MfaPurpose, NewUserRequest, OtpActivateRequest,
    OtpAuthFinishRequest, OtpAuthStartRequest, OtpCreateRequest, OtpGetResponse, OtpKind,
    UpdateUserRequest, UserResponseSimple,
};
use rauthy_common::sha256;
use rauthy_common::utils::base64_url_encode;
use reqwest::header::{AUTHORIZATION, HeaderMap, HeaderValue};
use reqwest::{Client, StatusCode};
use std::error::Error;

const PKCE_VERIFIER: &str = "Ajk3hfdVsDu8DpYuZeYbUfNIWSYdq5sKjpDcTWgM7RBsXQEkSmTNXV6wLvHUmJYQ";
/// The password of every user these tests create: the seeded password policy has a minimum length,
/// and this satisfies it with all of its likely `include_*` rules.
const PASSWORD_ONLY: &str = "MfaAssurance!Regression0123";
const MFA_USER: &str = "mfa-user@localhost";
const ADMIN_WITH_OTP: &str = "admin-with-otp@localhost";
const MFA_REQUIRED: u16 = 406;
const FIXTURE_KEY_NAME: &str = "mfa-assurance";

/// Tests run in parallel in one process, and several of them build the very same fixture user, so
/// fixture creation is serialized: two parallel creates of one email is a `UNIQUE` violation.
static FIXTURE_LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();

/// The headers of the API key all fixture work runs on. `BOOTSTRAP_API_KEY` and
/// `BOOTSTRAP_API_KEY_SECRET` are given to the backend and to this test, and an ApiKey is not a
/// session, so it is the only way to create users at all when `admin_force_mfa` is on: no session of
/// the MFA-less bootstrap admin can touch any admin endpoint in that mode.
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

/// The `AwaitOtpCode` body of a `POST /oidc/authorize`, down to what a ceremony needs.
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

/// Password login of a user on an existing session, as the raw `POST /oidc/authorize` response.
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
        nonce: Some("MfaAssuranceNonce".to_string()),
        code_challenge: Some(challenge.clone()),
        code_challenge_method: Some("S256".to_string()),
        resource: None,
        resident_key_token: None,
        fwda: None,
    };

    Client::new()
        .post(format!(
            "{backend}/oidc/authorize?client_id=rauthy&redirect_uri={backend}/oidc/callback\
            &response_type=code&code_challenge={challenge}&code_challenge_method=S256"
        ))
        .headers(session.clone())
        .json(&req)
        .send()
        .await
        .unwrap()
}

/// Creates a user with the given roles and password and returns its id, re-using and re-setting a
/// left-over of a prior run, because a delete would tear down the sessions of any test running in
/// parallel on that user.
async fn create_test_user(email: &str, roles: &[&str]) -> Result<String, Box<dyn Error>> {
    let _fixture = FIXTURE_LOCK.get_or_init(Default::default).lock().await;
    let backend = get_backend_url();
    let admin = fixture_headers()?;
    let roles = roles.iter().map(|r| r.to_string()).collect::<Vec<String>>();

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
                        given_name: Some("MfaAssurance".to_string()),
                        family_name: None,
                        email: email.to_string(),
                        preferred_username: None,
                        language: Language::En,
                        roles: roles.clone(),
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
            given_name: Some("MfaAssurance".to_string()),
            family_name: None,
            language: Some(Language::En),
            // only ever set on creation: the policy rejects a repeat of a recent password, and a
            // left-over of a parallel test already has exactly this one
            password: set_password.then(|| PASSWORD_ONLY.to_string()),
            roles,
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

/// The OTP code the backend has emailed to the user right now. This is the dev-only endpoint that
/// is compiled out of release builds and gated behind `dev_mode`, and it is the only way for a test
/// to ever get a valid code, because emails go nowhere during tests.
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
            otp_name: Some("MFA assurance regression".to_string()),
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
                .headers(session.clone())
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

/// Starts the ceremony for the given challenge on this session.
async fn otp_start(
    session: &HeaderMap,
    challenge: &OtpChallenge,
) -> Result<String, Box<dyn Error>> {
    let res = Client::new()
        .post(format!("{}/users/otp_start", get_backend_url()))
        .headers(session.clone())
        .json(&OtpAuthStartRequest {
            otp_id: challenge.active_otps[0].otp_id.clone(),
            purpose: MfaPurpose::Login(challenge.code.clone()),
        })
        .send()
        .await?;
    let start = expect_status_json!(res, 200, serde_json::Value);

    Ok(start["code"].as_str().unwrap_or_default().to_string())
}

/// Completes a started ceremony with the code from the dev-only reveal. For a login ceremony, this
/// is the response that finalizes the login: `LoggedIn` is a `202` with the callback `Location`.
async fn complete_ceremony(
    session: &HeaderMap,
    user_id: &str,
    start_code: &str,
) -> Result<(), Box<dyn Error>> {
    let res = Client::new()
        .post(format!("{}/users/otp_finish", get_backend_url()))
        .headers(session.clone())
        .json(&OtpAuthFinishRequest {
            code: start_code.to_string(),
            otp_code: dev_otp_code(user_id).await?,
        })
        .send()
        .await?;
    expect_status!(res, 202);

    Ok(())
}

/// The admin login of an account with an active OTP, through its ceremony, ending with an `Auth`
/// session that has a completed 2nd factor.
async fn assured_admin_session(email: &str) -> Result<(String, HeaderMap), Box<dyn Error>> {
    let admin_id = create_test_user(email, &["rauthy_admin"]).await?;
    ensure_otp(&admin_id, email).await?;

    let session = init_session().await?;
    let challenge = expect_status_json!(
        login(email, PASSWORD_ONLY, &session).await,
        200,
        OtpChallenge
    );
    let start_code = otp_start(&session, &challenge).await?;
    complete_ceremony(&session, &admin_id, &start_code).await?;

    Ok((admin_id, session))
}

/// `GET /users` is the probe for the admin gate: it is admin-only and `admin_force_mfa` adds the
/// MFA assurance check on top, which is the invariant these tests are about.
async fn admin_api_status(session: &HeaderMap) -> Result<StatusCode, Box<dyn Error>> {
    let res = Client::new()
        .get(format!("{}/users", get_backend_url()))
        .headers(session.clone())
        .send()
        .await?;

    Ok(res.status())
}

/// These tests assert on a gated config only: `admin_force_mfa` and `otp.enable` both on, and the
/// bootstrap API key of the header docblock. On the default `config-test.toml`, the gate they test
/// is off, so they skip themselves instead of failing the plain `just test-hiqlite` run.
async fn gated_suite_ready() -> Result<bool, Box<dyn Error>> {
    let skip = |why: &str| {
        println!("skipping the mfa_assurance suite: {why}");
        false
    };

    if std::env::var_os("BOOTSTRAP_API_KEY_SECRET").is_none() {
        return Ok(skip("BOOTSTRAP_API_KEY_SECRET is not set"));
    }

    // The gate is observable from the outside: the login of the MFA-less bootstrap admin succeeds,
    // and every admin endpoint answers `406` on that session.
    let session = init_session().await?;
    if login(USERNAME, PASSWORD, &session).await.status() != 202 {
        return Ok(skip(
            "the bootstrap admin login is not a plain password login",
        ));
    }

    Ok(
        admin_api_status(&session).await? == StatusCode::from_u16(MFA_REQUIRED)?
            || skip("admin_force_mfa is off on this backend"),
    )
}

/// `admin_force_mfa` gates sessions only: the fixture's bootstrap ApiKey keeps working, which is the
/// control for all other tests in this file.
#[tokio::test]
async fn test_mfa_assurance_api_key_is_not_a_gated_session() -> Result<(), Box<dyn Error>> {
    if !gated_suite_ready().await? {
        return Ok(());
    }
    let res = Client::new()
        .get(format!("{}/users", get_backend_url()))
        .headers(fixture_headers()?)
        .send()
        .await?;
    expect_status!(res, 200);

    Ok(())
}

/// The reported attack, shape 1: a 2nd factor is requested for some other user and the ceremony is
/// abandoned. The admin, which has no MFA at all, then logs in with a phished password on that very
/// session. That login is fully valid, but it may never inherit an assurance it never completed.
#[tokio::test]
async fn test_mfa_assurance_abandoned_ceremony_never_grants_access() -> Result<(), Box<dyn Error>> {
    if !gated_suite_ready().await? {
        return Ok(());
    }

    let mfa_user_id = create_test_user(MFA_USER, &["user"]).await?;
    ensure_otp(&mfa_user_id, MFA_USER).await?;

    let session = init_session().await?;
    let res = login(MFA_USER, PASSWORD_ONLY, &session).await;
    // the 2nd factor is requested here and it is never completed
    expect_status!(res, 200);

    // the admin login on the same session is a complete success: the session ends up `Auth` for the
    // admin, and the admin has no MFA, so the gate below is the only thing that can stop it
    let res = login(USERNAME, PASSWORD, &session).await;
    expect_status!(res, 202);

    assert_eq!(admin_api_status(&session).await?, MFA_REQUIRED);

    Ok(())
}

/// The positive control: a fully completed OTP ceremony does provide the assurance that
/// `admin_force_mfa` demands.
#[tokio::test]
async fn test_mfa_assurance_completed_ceremony_grants_access() -> Result<(), Box<dyn Error>> {
    if !gated_suite_ready().await? {
        return Ok(());
    }

    let (_admin_id, session) = assured_admin_session(ADMIN_WITH_OTP).await?;

    assert_eq!(admin_api_status(&session).await?, 200);

    Ok(())
}

/// The reported attack, shape 2: an admin completes its own OTP ceremony, and then logs in as a
/// second admin without any MFA on that same session. The session becomes fully authenticated for
/// that admin, so only the MFA assurance can and must keep it out of the admin API.
#[tokio::test]
async fn test_mfa_assurance_relogin_as_other_admin_resets() -> Result<(), Box<dyn Error>> {
    if !gated_suite_ready().await? {
        return Ok(());
    }

    let (_admin_id, session) = assured_admin_session(ADMIN_WITH_OTP).await?;
    assert_eq!(admin_api_status(&session).await?, 200);

    // a login for the MFA-less admin on the very same session: fully logged in and admin
    let res = login(USERNAME, PASSWORD, &session).await;
    expect_status!(res, 202);

    // the assurance of the ceremony belongs to the other principal and is gone with this login
    assert_eq!(admin_api_status(&session).await?, MFA_REQUIRED);

    Ok(())
}

/// The ceremony of a login may only ever complete on the session it was started on.
#[tokio::test]
async fn test_mfa_assurance_ceremony_is_bound_to_its_session() -> Result<(), Box<dyn Error>> {
    if !gated_suite_ready().await? {
        return Ok(());
    }

    let admin_id = create_test_user(ADMIN_WITH_OTP, &["rauthy_admin"]).await?;
    ensure_otp(&admin_id, ADMIN_WITH_OTP).await?;

    let session = init_session().await?;
    let challenge = expect_status_json!(
        login(ADMIN_WITH_OTP, PASSWORD_ONLY, &session).await,
        200,
        OtpChallenge
    );
    let start_code = otp_start(&session, &challenge).await?;

    // an unrelated session tries to finish the ceremony of the first one
    let other = init_session().await?;
    let res = Client::new()
        .post(format!("{}/users/otp_finish", get_backend_url()))
        .headers(other.clone())
        .json(&OtpAuthFinishRequest {
            code: start_code,
            otp_code: dev_otp_code(&admin_id).await?,
        })
        .send()
        .await?;
    expect_status!(res, 403);

    // and neither of the two sessions has any assurance: both are still `Init`
    assert_eq!(admin_api_status(&session).await?, 401);
    assert_eq!(admin_api_status(&other).await?, 401);

    Ok(())
}
