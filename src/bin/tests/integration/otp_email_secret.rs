//! Regression tests for the empty OTP secret: `generate_otp` is `HMAC(secret, step)`, so a row
//! with an empty `secret` has a code that is a public function of the server time, and anyone with
//! the account password completes the ceremony without ever reading the mailbox.
//!
//! These tests need `otp.enable` on (the default `config-test.toml` keeps it disabled) plus the
//! bootstrap API key, which is also how the `mfa_assurance` suite is run, so one env block covers
//! both. The `just test*` recipes set it, by hand it is:
//!
//! ```sh
//! OTP_ENABLE=true \
//! BOOTSTRAP_API_KEY=eyJuYW1lIjoibWZhLWFzc3VyYW5jZSIsImV4cCI6bnVsbCwiYWNjZXNzIjpbeyJncm91cCI6IlVzZXJzIiwiYWNjZXNzX3JpZ2h0cyI6WyJyZWFkIiwiY3JlYXRlIiwidXBkYXRlIiwiZGVsZXRlIl19XX0= \
//! BOOTSTRAP_API_KEY_SECRET=MfaAssuranceRegressionSecret0123456789_mfaAssuranceRegressionSec \
//! just test-hiqlite otp_secret
//! ```
//!
//! Without them, every test skips itself and a plain `cargo test` run stays green. The
//! code the dev-only reveal endpoint returns is the very code the backend has emailed, which is the
//! only way a test can ever get one, because emails go nowhere during tests.
use crate::common::{
    accept_all, consent_required, cookie_csrf_headers_from_res_direct, get_backend_url,
    get_solved_pow, send_login, stale_consent,
};
use crate::{expect_status, expect_status_json};
use rauthy_api_types::generic::Language;
use rauthy_api_types::oidc::{LoginConsent, LoginRequest};
use rauthy_api_types::users::{
    MfaModTokenRequest, MfaModTokenResponse, MfaPurpose, NewUserRequest, OtpActivateRequest,
    OtpAuthFinishRequest, OtpAuthStartRequest, OtpCreateRequest, OtpGetResponse, OtpKind,
    UpdateUserRequest, UserConsentResponse, UserResponseSimple,
};
use rauthy_common::sha256;
use rauthy_common::utils::{base64_url_encode, new_store_id};
use reqwest::header::{AUTHORIZATION, COOKIE, HeaderMap, HeaderValue, SET_COOKIE};
use reqwest::{Client, StatusCode};
use ring::hmac;
use std::error::Error;
use std::sync::LazyLock;
use std::time::Duration;

const PKCE_VERIFIER: &str = "Ajk3hfdVsDu8DpYuZeYbUfNIWSYdq5sKjpDcTWgM7RBsXQEkSmTNXV6wLvHUmJYQ";
/// The password of the user these tests create. The seeded password policy has a minimum length,
/// and this satisfies it with all of its likely `include_*` rules.
const PASSWORD_ONLY: &str = "OtpSecret!Regression0123";
/// The user of these tests carries a random suffix: it is created, updated and logged in by email,
/// and two runs on one backend must not share the account.
static SUFFIX: LazyLock<String> = LazyLock::new(|| new_store_id().to_lowercase());
static OTP_USER: LazyLock<String> =
    LazyLock::new(|| format!("otp-secret-user-{}@localhost", *SUFFIX));
/// The name inside `BOOTSTRAP_API_KEY` - the very key the `mfa_assurance` suite documents.
const FIXTURE_KEY_NAME: &str = "mfa-assurance";
/// The defaults of `otp.length` and `otp.digest_len_default`, which the test config keeps at their
/// defaults (`OTP_LENGTH` / `OTP_DIGEST_LEN_DEFAULT` would change both).
const CODE_LEN: u8 = 6;
const DIGEST_LEN: u16 = 512;
static FIXTURE_LOCK: std::sync::OnceLock<tokio::sync::Mutex<()>> = std::sync::OnceLock::new();

/// The headers of the API key all fixture work runs on. An `ApiKey` is not a session, so it is not
/// affected by any of the session checks, and `Users / Read` covers the `last_used` disclosure.
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

/// The code the pre-fix derivation produced for a stored secret, for the very `step` the backend
/// validates against: `HMAC(secret, step)` with the raw bytes of the `secret` column. The empty
/// secret every `OtpKind::Email` row used to have is `&[]`, which is why these tests pass `&[]`.
fn code_from_secret(secret: &[u8], step: i64) -> String {
    let algorithm = match DIGEST_LEN {
        256 => hmac::HMAC_SHA256,
        384 => hmac::HMAC_SHA384,
        _ => hmac::HMAC_SHA512,
    };
    let key = hmac::Key::new(algorithm, secret);
    let hash = hmac::sign(&key, &step.to_be_bytes());
    let hash = hash.as_ref();
    let offset = (hash[hash.len() - 1] & 0xf) as usize;
    let binary = u32::from_be_bytes([
        hash[offset] & 0x7f,
        hash[offset + 1],
        hash[offset + 2],
        hash[offset + 3],
    ]);
    let digits = 10u32.pow(u32::from(CODE_LEN));
    format!("{:0width$}", binary % digits, width = CODE_LEN as usize)
}

/// Opens a session in `Init` state and returns its cookie + CSRF headers.
async fn init_session() -> Result<HeaderMap, Box<dyn Error>> {
    let res = Client::new()
        .post(format!("{}/oidc/session", get_backend_url()))
        .send()
        .await?;
    cookie_csrf_headers_from_res_direct(res).await
}

/// The `POST /oidc/authorize` of `email` to the `rauthy` client. Without a `password`, as a login
/// with a valid MFA cookie sends it.
async fn login_request(
    email: &str,
    password: Option<&str>,
    consent: Option<LoginConsent>,
) -> LoginRequest {
    LoginRequest {
        email: Some(email.to_string()),
        password: password.map(String::from),
        pow: get_solved_pow().await,
        client_id: "rauthy".to_string(),
        redirect_uri: format!("{}/oidc/callback", get_backend_url()),
        scopes: None,
        state: None,
        nonce: Some("OtpSecretNonce".to_string()),
        code_challenge: Some(base64_url_encode(sha256!(PKCE_VERIFIER.as_bytes()))),
        code_challenge_method: Some("S256".to_string()),
        resource: None,
        resident_key_token: None,
        fwda: None,
        consent,
    }
}

/// Password login of a user on an existing session, as the raw `POST /oidc/authorize` response.
async fn login(email: &str, password: &str, session: &HeaderMap) -> reqwest::Response {
    let backend = get_backend_url();
    let challenge = base64_url_encode(sha256!(PKCE_VERIFIER.as_bytes()));
    let url_auth = format!(
        "{backend}/oidc/authorize?client_id=rauthy&redirect_uri={backend}/oidc/callback\
        &response_type=code&code_challenge={challenge}&code_challenge_method=S256"
    );
    let req = login_request(email, Some(password), None).await;
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
                        given_name: Some("OtpSecret".to_string()),
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
            given_name: Some("OtpSecret".to_string()),
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

/// The OTP code the backend has emailed to the user right now. This is the dev-only endpoint that
/// is compiled out of release builds and gated behind `dev_mode`.
async fn dev_otp_code(user_id: &str) -> Result<String, Box<dyn Error>> {
    let res = Client::new()
        .post(format!("{}/dev/otp_code", get_backend_url()))
        .json(&serde_json::json!({ "user_id": user_id }))
        .send()
        .await?;

    Ok(expect_status!(res, 200).text().await?)
}

/// The rows of this user's OTPs, as an admin (`Users / Read`) via the fixture API key. This is the
/// `last_used` disclosure the attack uses as a convenience.
async fn otp_rows(user_id: &str) -> Result<Vec<OtpGetResponse>, Box<dyn Error>> {
    let res = Client::new()
        .get(format!("{}/users/{user_id}/otp", get_backend_url()))
        .headers(fixture_headers()?)
        .send()
        .await?;

    Ok(expect_status_json!(res, 200, Vec<OtpGetResponse>))
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
            otp_name: Some("OTP secret regression".to_string()),
            otp_kind: OtpKind::Email,
            mfa_mod_token_id: mod_token.id.clone(),
        })
        .send()
        .await?;
    // `406` means an OTP row already existed and only needs its id from the list
    let otp_id = match res.status().as_u16() {
        201 => res.json::<OtpGetResponse>().await?.id,
        406 => otp_rows(user_id)
            .await?
            .first()
            .expect("the existing OTP row")
            .id
            .clone(),
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

/// Starts the ceremony for the given challenge on this session and returns the ceremony's code.
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

/// `POST /users/otp_finish` with the given code. For a login ceremony, a `202` is the response
/// that finalizes the login: `LoggedIn` with the callback `Location`.
async fn otp_finish(
    session: &HeaderMap,
    start_code: &str,
    otp_code: &str,
) -> Result<reqwest::Response, Box<dyn Error>> {
    Ok(Client::new()
        .post(format!("{}/users/otp_finish", get_backend_url()))
        .headers(session.clone())
        .json(&OtpAuthFinishRequest {
            code: start_code.to_string(),
            otp_code: otp_code.to_string(),
        })
        .send()
        .await?)
}

/// Password login that stops at the 2nd factor and starts its ceremony. A login that needs the
/// 2nd factor answers `200` with the `AwaitMfa` body; the login itself only completes with the
/// `202` of a finished ceremony.
async fn start_ceremony(email: &str) -> Result<(HeaderMap, String), Box<dyn Error>> {
    let session = init_session().await?;
    let res = login(email, PASSWORD_ONLY, &session).await;
    let [challenge] = expect_status_json!(res, 200, [OtpChallenge; 1]);
    let start_code = otp_start(&session, &challenge).await?;

    Ok((session, start_code))
}

/// `true` when these tests can run: the fixture API key is available and `otp.enable` is on. The
/// disabled feature answers `403` on any OTP endpoint, which is the cheapest way to detect it.
async fn gated_suite_ready() -> Result<bool, Box<dyn Error>> {
    let skip = |why: &str| {
        println!("skipping the otp_secret suite: {why}");
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

/// The very attack of the finding: a password-only attacker starts a ceremony on their own
/// session, computes the code for the stored `last_used` step with an empty key - which is what
/// every `OtpKind::Email` row used to have - and must not complete the login with it. A rejected
/// code deletes the pending ceremony, so the login restarts at the password.
#[tokio::test]
async fn test_otp_secret_predictable_code_is_rejected() -> Result<(), Box<dyn Error>> {
    if !gated_suite_ready().await? {
        return Ok(());
    }
    let user_id = create_test_user(OTP_USER.as_str()).await?;
    ensure_otp(&user_id, OTP_USER.as_str()).await?;

    let (session, start_code) = start_ceremony(OTP_USER.as_str()).await?;
    let step = otp_rows(&user_id)
        .await?
        .first()
        .expect("the user's OTP row")
        .last_used;
    let predictable = code_from_secret(&[], step);

    let res = otp_finish(&session, &start_code, &predictable).await?;
    expect_status!(
        res,
        400,
        format!("predicted code {predictable} of step {step}")
    );

    Ok(())
}

/// The code the backend derived from the row's real secret, which is the one it emailed, completes
/// the login. Together with the test above this pins both sides: a predictable code is rejected,
/// a real one works.
#[tokio::test]
async fn test_otp_secret_mailed_code_completes_login() -> Result<(), Box<dyn Error>> {
    if !gated_suite_ready().await? {
        return Ok(());
    }
    let user_id = create_test_user(OTP_USER.as_str()).await?;
    ensure_otp(&user_id, OTP_USER.as_str()).await?;

    let (session, start_code) = start_ceremony(OTP_USER.as_str()).await?;
    let code = dev_otp_code(&user_id).await?;
    let res = otp_finish(&session, &start_code, &code).await?;
    expect_status!(res, 202);

    Ok(())
}

/// The user these consent tests create, apart from [`OTP_USER`] to not race its tests.
static OTP_CONSENT_USER: LazyLock<String> =
    LazyLock::new(|| format!("otp-consent-user-{}@localhost", *SUFFIX));

/// The user's consents, as the fixture API key sees them.
async fn consents(user_id: &str) -> Result<Vec<UserConsentResponse>, Box<dyn Error>> {
    let res = Client::new()
        .get(format!("{}/users/{user_id}/consents", get_backend_url()))
        .headers(fixture_headers()?)
        .send()
        .await?;
    Ok(expect_status_json!(res, 200, Vec<UserConsentResponse>))
}

/// `POST /oidc/authorize` with only the email, as a login with a valid MFA cookie sends it, and
/// the given consent.
async fn login_mfa_cookie(
    session: &HeaderMap,
    consent: Option<LoginConsent>,
) -> Result<reqwest::Response, Box<dyn Error>> {
    let req = login_request(OTP_CONSENT_USER.as_str(), None, consent).await;
    Ok(Client::new()
        .post(format!("{}/oidc/authorize", get_backend_url()))
        .headers(session.clone())
        .json(&req)
        .send()
        .await?)
}

/// A consent given with the password or with a valid MFA cookie alone is written only after the
/// OTP ceremony proved the user, and a login that has proven no factor yet sees nothing of the
/// grant.
#[tokio::test]
async fn test_otp_consent_waits_for_the_ceremony() -> Result<(), Box<dyn Error>> {
    if !gated_suite_ready().await? {
        return Ok(());
    }
    let user_id = create_test_user(OTP_CONSENT_USER.as_str()).await?;
    ensure_otp(&user_id, OTP_CONSENT_USER.as_str()).await?;
    let res = Client::new()
        .delete(format!(
            "{}/users/{user_id}/consents?client_id=rauthy",
            get_backend_url()
        ))
        .headers(fixture_headers()?)
        .send()
        .await?;
    expect_status!(res, 200);
    assert!(consents(&user_id).await?.is_empty());

    // --- the password and a consent: no grant before the OTP
    let session = init_session().await?;
    let res = login(OTP_CONSENT_USER.as_str(), PASSWORD_ONLY, &session).await;
    let mfa_cookie = res
        .headers()
        .get_all(SET_COOKIE)
        .iter()
        .filter_map(|c| c.to_str().ok()?.split_once(';').map(|(c, _)| c.to_string()))
        .find(|c| c.contains("RauthyMfa="));
    expect_status_json!(res, 200, [OtpChallenge; 1]);
    let mfa_cookie = mfa_cookie.expect("the MFA cookie of the OTP step");
    assert!(consents(&user_id).await?.is_empty());

    // --- a valid MFA cookie without a password: no grant before the OTP either
    let mut session = init_session().await?;
    let cookie = session.get(COOKIE).unwrap().to_str()?.to_string();
    session.insert(
        COOKIE,
        HeaderValue::from_str(&format!("{cookie}; {mfa_cookie}"))?,
    );
    let content = consent_required(login_mfa_cookie(&session, None).await?).await;
    let consent = accept_all(&content);
    let mut accepted = consent.accepted_scopes.clone();
    accepted.sort_unstable();
    let res = login_mfa_cookie(&session, Some(consent)).await?;
    let [challenge] = expect_status_json!(res, 200, [OtpChallenge; 1]);
    assert!(consents(&user_id).await?.is_empty());

    // --- the finished ceremony writes the grant
    let start_code = otp_start(&session, &challenge).await?;
    let code = dev_otp_code(&user_id).await?;
    expect_status!(otp_finish(&session, &start_code, &code).await?, 202);
    let grants = consents(&user_id).await?;
    assert_eq!(grants.len(), 1);
    assert_eq!(grants[0].client_id, "rauthy");
    assert_eq!(grants[0].resource, None);
    assert_eq!(grants[0].scopes, accepted);

    // --- without a proven factor, the grant is neither used nor shown
    let mut session = init_session().await?;
    let cookie = session.get(COOKIE).unwrap().to_str()?.to_string();
    session.insert(
        COOKIE,
        HeaderValue::from_str(&format!("{cookie}; {mfa_cookie}"))?,
    );
    let content = consent_required(login_mfa_cookie(&session, None).await?).await;
    assert!(content.granted_scopes.is_empty(), "{content:?}");
    // nor shown with a refused consent, which writes nothing: a write would change `updated`
    tokio::time::sleep(Duration::from_secs(1)).await;
    let stale = stale_consent(&content);
    let content = consent_required(login_mfa_cookie(&session, Some(stale)).await?).await;
    assert!(content.granted_scopes.is_empty(), "{content:?}");
    let after = consents(&user_id).await?;
    assert_eq!(after.len(), 1);
    assert_eq!(after[0].updated, grants[0].updated);

    Ok(())
}
