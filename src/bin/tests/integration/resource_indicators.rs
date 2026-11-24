use crate::common::{
    USER_PASSWORD, code_state_from_headers, cookie_csrf_headers_from_res_direct, create_client,
    create_login_user, decode_claims, delete_client, delete_user, get_auth_headers,
    get_backend_url, get_client, get_solved_pow, put_client, session_headers_with, update_request,
};
use crate::{expect_status, expect_status_json};
use pretty_assertions::assert_eq;
use rauthy_api_types::clients::{ClientSecretResponse, NewClientRequest, UpdateClientRequest};
use rauthy_api_types::oidc::{GrantType, LoginRequest, TokenRequest};
use rauthy_common::sha256;
use rauthy_common::utils::{base64_url_encode, base64_url_no_pad_decode, new_store_id};
use rauthy_service::token_set::TokenSet;
use reqwest::Response;
use reqwest::header::HeaderMap;
use std::error::Error;
use std::time::Duration;

const RES_A: &str = "https://rs-a.example.com/api";
const RES_B: &str = "https://rs-b.example.com/api";
const DEFAULT_AUD: &str = "https://always.example.com/";
const RES_REFRESH: &str = "https://rs.example.com/mcp";

/// The HTTP client for the device code tests. Those wait out the poll interval between requests,
/// and a pooled connection can be closed by the backend exactly during that wait, which answers
/// the next request with a connection reset. Without idle pooling every request connects fresh.
fn device_http() -> Result<reqwest::Client, Box<dyn Error>> {
    Ok(reqwest::Client::builder()
        .pool_max_idle_per_host(0)
        .build()?)
}

/// The device code poll interval of the test config, which a device must respect before
/// redeeming its code.
const DEVICE_POLL_INTERVAL: Duration = Duration::from_secs(5);

const PKCE_VERIFIER: &str = "oDXug9zfYqfz8ejcqMpALRPXfW8QhbKV2AVuScAt8xrLKDAmaRYQ4yRi2uqcH9ys";

/// Returns whether the token's `aud` (string or string[]) contains `expected`.
fn aud_contains(access_token: &str, expected: &str) -> bool {
    match decode_aud(access_token) {
        serde_json::Value::String(s) => s == expected,
        serde_json::Value::Array(arr) => arr.iter().any(|v| v.as_str() == Some(expected)),
        _ => false,
    }
}

/// Decodes the (unverified) JWT payload and returns the `aud` claim as raw JSON, so a
/// test can distinguish a single-string `aud` from a `string[]` `aud`.
fn decode_aud(access_token: &str) -> serde_json::Value {
    let payload_b64 = access_token
        .split('.')
        .nth(1)
        .expect("a JWT payload segment");
    let bytes = base64_url_no_pad_decode(payload_b64).expect("valid base64url payload");
    let claims: serde_json::Value = serde_json::from_slice(&bytes).expect("valid JSON claims");
    claims.get("aud").cloned().expect("an `aud` claim")
}

/// The device code grant has no consent step: its refresh tokens are exempt from the consent
/// check, and stay exempt through the rotation, which drops the `did`.
fn assert_consent_exempt(refresh_token: &str, did: bool) {
    let claims = decode_claims(refresh_token);
    assert_eq!(
        claims["consent_exempt"],
        serde_json::json!(true),
        "{claims}"
    );
    assert_eq!(claims.get("did").is_some(), did, "{claims}");
}

/// The client a test runs its device flow on: the id it registers as, and the request bodies
/// for its token exchange and refresh, so the client id can never be swapped with a token.
struct DeviceFlow {
    client_id: String,
}

impl DeviceFlow {
    fn token_req(&self, device_code: &str, resource: Option<&str>) -> TokenRequest {
        TokenRequest {
            grant_type: GrantType::DeviceCode,
            client_id: Some(self.client_id.clone()),
            device_code: Some(device_code.to_string()),
            resource: resource.map(String::from),
            ..Default::default()
        }
    }

    fn refresh_req(&self, refresh_token: &str, resource: Option<&str>) -> TokenRequest {
        TokenRequest {
            grant_type: GrantType::RefreshToken,
            client_id: Some(self.client_id.clone()),
            refresh_token: Some(refresh_token.to_string()),
            resource: resource.map(String::from),
            ..Default::default()
        }
    }
}

/// Creates a public client for the device code + refresh token flows, with the given
/// `allowed_resources`.
async fn create_device_client(
    http: &reqwest::Client,
    admin: &HeaderMap,
    name: &str,
    allowed_resources: Option<Vec<String>>,
) -> Result<DeviceFlow, Box<dyn Error>> {
    let id = format!("test_{}", new_store_id());
    create_client(
        http,
        admin,
        &NewClientRequest {
            id: id.clone(),
            secret: None,
            name: Some(name.to_string()),
            confidential: false,
            redirect_uris: vec![format!("http://{id}.test.example/callback")],
            post_logout_redirect_uris: None,
            consent: None,
        },
    )
    .await?;

    let req = UpdateClientRequest {
        flows_enabled: vec![GrantType::DeviceCode, GrantType::RefreshToken],
        allowed_resources,
        ..update_request(get_client(admin, &id).await?)
    };
    put_client(http, admin, &id, &req).await?;

    Ok(DeviceFlow { client_id: id })
}

/// A user that accepts the device codes of a test, and the authenticated session whose
/// `user_id` the accepted devices end up belonging to.
struct DeviceUser {
    email: String,
    headers: HeaderMap,
}

impl DeviceUser {
    async fn new(name: &str) -> Result<Self, Box<dyn Error>> {
        let http = reqwest::Client::new();
        let admin = get_auth_headers().await?;
        let email = format!("{}-{}@localhost", name, new_store_id().to_lowercase());
        create_login_user(&http, &admin, &email, name, USER_PASSWORD).await?;
        let headers = session_headers_with(&email, USER_PASSWORD).await;
        Ok(Self { email, headers })
    }

    /// Deletes the user again, which cascades to its devices and their refresh tokens.
    async fn delete(&self) -> Result<(), Box<dyn Error>> {
        delete_user(&get_auth_headers().await?, &self.email).await
    }
}

/// `POST /oidc/device`, waiting out the per-IP rate limit of the endpoint.
async fn device_auth(
    http: &reqwest::Client,
    form: &[(&str, &str)],
) -> Result<Response, Box<dyn Error>> {
    let url = format!("{}/oidc/device", get_backend_url());
    loop {
        let res = http.post(&url).form(form).send().await?;
        if res.status() != 429 {
            return Ok(res);
        }
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

/// Starts a device flow that `user` accepts and returns its `device_code`. The caller still has
/// to respect the poll interval before redeeming it.
async fn device_code_accepted(
    http: &reqwest::Client,
    user: &DeviceUser,
    form: &[(&str, &str)],
) -> Result<String, Box<dyn Error>> {
    let res = device_auth(http, form).await?;
    let resp = expect_status_json!(res, 200, serde_json::Value);
    let device_code = resp["device_code"].as_str().unwrap().to_string();
    let user_code = resp["user_code"].as_str().unwrap();

    let res = http
        .post(format!("{}/oidc/device/verify", get_backend_url()))
        .headers(user.headers.clone())
        .json(&serde_json::json!({
            "user_code": user_code,
            "pow": get_solved_pow().await,
            "device_accepted": "accept",
        }))
        .send()
        .await?;
    expect_status!(res, 202);

    Ok(device_code)
}

/// End-to-end coverage for RFC 8707 resource indicators on the `client_credentials`
/// grant: the per-client `allowed_resources` allow-list, the always-on `default_aud`,
/// the multi-valued `aud` array, the `invalid_target` error, and deny-by-default.
#[tokio::test]
async fn test_resource_indicators() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();
    let http = reqwest::Client::new();

    // create a confidential client
    let id = format!("test_{}", new_store_id());
    create_client(
        &http,
        &auth_headers,
        &NewClientRequest {
            id: id.clone(),
            secret: None,
            name: Some("Resource Test".to_string()),
            confidential: true,
            redirect_uris: vec![format!("http://{id}.test.example/callback")],
            post_logout_redirect_uris: None,
            consent: None,
        },
    )
    .await?;

    // configure `allowed_resources` + `default_aud` + the client_credentials flow
    let req = UpdateClientRequest {
        flows_enabled: vec![GrantType::ClientCredentials],
        allowed_resources: Some(vec![RES_A.to_string()]),
        default_aud: Some(vec![DEFAULT_AUD.to_string()]),
        ..update_request(get_client(&auth_headers, &id).await?)
    };
    let resp = put_client(&http, &auth_headers, &id, &req).await?;
    assert_eq!(resp.allowed_resources, Some(vec![RES_A.to_string()]));
    assert_eq!(resp.default_aud, Some(vec![DEFAULT_AUD.to_string()]));

    // fetch the generated client secret
    let res = http
        .post(format!("{backend_url}/clients/{id}/secret"))
        .headers(auth_headers.clone())
        .send()
        .await?;
    let res = expect_status!(res, 200);
    let secret = res
        .json::<ClientSecretResponse>()
        .await?
        .secret
        .expect("a confidential client secret");

    let url_token = format!("{backend_url}/oidc/token");
    let mut token_req = TokenRequest {
        grant_type: GrantType::ClientCredentials,
        client_id: Some(id.clone()),
        client_secret: Some(secret),
        ..Default::default()
    };

    // (1) no `resource` requested -> `aud` is an array containing the client and the
    // always-on `default_aud`
    let res = http.post(&url_token).form(&token_req).send().await?;
    let ts = expect_status_json!(res, 200, TokenSet);
    let aud = decode_aud(&ts.access_token);
    let arr = aud
        .as_array()
        .expect("`aud` must be an array once `default_aud` is set");
    assert!(arr.iter().any(|v| v.as_str() == Some(id.as_str())));
    assert!(arr.iter().any(|v| v.as_str() == Some(DEFAULT_AUD)));

    // (2) an allowed `resource` -> it is added to `aud`
    token_req.resource = Some(RES_A.to_string());
    let res = http.post(&url_token).form(&token_req).send().await?;
    let ts = expect_status_json!(res, 200, TokenSet);
    let aud = decode_aud(&ts.access_token);
    let arr = aud.as_array().expect("`aud` array");
    assert!(arr.iter().any(|v| v.as_str() == Some(RES_A)));
    assert!(arr.iter().any(|v| v.as_str() == Some(DEFAULT_AUD)));

    // (3) a disallowed `resource` -> `invalid_target` (HTTP 400)
    token_req.resource = Some(RES_B.to_string());
    let res = http.post(&url_token).form(&token_req).send().await?;
    let body = expect_status!(res, 400).text().await?;
    assert!(body.contains("invalid_target"), "unexpected body: {body}");

    // (4) deny-by-default: with no `allowed_resources`, any `resource` is rejected
    let req = UpdateClientRequest {
        allowed_resources: None,
        ..update_request(get_client(&auth_headers, &id).await?)
    };
    put_client(&http, &auth_headers, &id, &req).await?;

    token_req.resource = Some(RES_A.to_string());
    let res = http.post(&url_token).form(&token_req).send().await?;
    expect_status!(res, 400);

    delete_client(&auth_headers, &id).await?;

    Ok(())
}

// #1657 (issue #1645): the RFC 8707 `resource` indicator must survive the silent re-auth
// path (`POST /oidc/authorize/refresh`). When a still-valid session re-issues a code via
// that endpoint, the request still carries the `resource`; before the fix
// `post_authorize_refresh` hard-coded `resource: None`, so the re-issued code (and its
// exchanged token) silently lost the audience restriction on every login after the first.
#[tokio::test]
async fn test_resource_survives_authorize_refresh() -> Result<(), Box<dyn Error>> {
    let backend_url = get_backend_url();
    let auth_headers = get_auth_headers().await?;
    let http = reqwest::Client::new();

    let user_email = format!("refresh-{}@localhost", new_store_id().to_lowercase());
    create_login_user(
        &http,
        &auth_headers,
        &user_email,
        "Res Refresh",
        USER_PASSWORD,
    )
    .await?;

    let redirect_uri = "http://localhost:3000/oidc/callback".to_string();
    let challenge_s256 = base64_url_encode(sha256!(PKCE_VERIFIER.as_bytes()));
    let id = format!("test_{}", new_store_id());

    // public PKCE client with `allowed_resources` = [RES_REFRESH]
    create_client(
        &http,
        &auth_headers,
        &NewClientRequest {
            id: id.clone(),
            secret: None,
            name: Some("Resource Refresh Test".to_string()),
            confidential: false,
            redirect_uris: vec![redirect_uri.clone()],
            post_logout_redirect_uris: None,
            consent: None,
        },
    )
    .await?;

    let req = UpdateClientRequest {
        flows_enabled: vec![GrantType::AuthorizationCode, GrantType::RefreshToken],
        allowed_resources: Some(vec![RES_REFRESH.to_string()]),
        ..update_request(get_client(&auth_headers, &id).await?)
    };
    put_client(&http, &auth_headers, &id, &req).await?;

    // authenticated session we reuse for the silent re-auth call
    let res = http
        .post(format!("{backend_url}/oidc/session"))
        .send()
        .await?;
    assert!(res.status().is_success());
    let session_headers = cookie_csrf_headers_from_res_direct(res).await?;

    // full authorization_code login WITH the RFC 8707 `resource`
    let query_pkce = format!(
        "client_id={id}&redirect_uri={redirect_uri}&response_type=code\
        &code_challenge={challenge_s256}&code_challenge_method=S256"
    );
    let url_auth = format!("{backend_url}/oidc/authorize?{query_pkce}");
    let req_login = LoginRequest {
        email: Some(user_email.clone()),
        password: Some(USER_PASSWORD.to_string()),
        pow: get_solved_pow().await,
        client_id: id.clone(),
        redirect_uri: redirect_uri.clone(),
        scopes: None,
        state: None,
        nonce: Some("MySuperNonce".to_string()),
        code_challenge: Some(challenge_s256.clone()),
        code_challenge_method: Some("S256".to_string()),
        resource: Some(RES_REFRESH.to_string()),
        resident_key_token: None,
        fwda: None,
    };
    let res = http
        .post(&url_auth)
        .headers(session_headers.clone())
        .json(&req_login)
        .send()
        .await?;
    let res = expect_status!(res, 202);
    let (code, _state) = code_state_from_headers(&res)?;

    let token_req = TokenRequest {
        grant_type: GrantType::AuthorizationCode,
        code: Some(code),
        redirect_uri: Some(redirect_uri.clone()),
        client_id: Some(id.clone()),
        client_secret: None,
        code_verifier: Some(PKCE_VERIFIER.to_string()),
        resource: Some(RES_REFRESH.to_string()),
        ..Default::default()
    };
    let res = http
        .post(format!("{backend_url}/oidc/token"))
        .form(&token_req)
        .send()
        .await?;
    let ts = expect_status_json!(res, 200, TokenSet);
    assert!(
        aud_contains(&ts.access_token, RES_REFRESH),
        "initial access token `aud` must contain the requested resource"
    );

    // silent re-auth via `POST /oidc/authorize/refresh` on the SAME session, same `resource`.
    // `LoginRefreshRequest` is deserialize-only (no `Serialize`), so build the body as raw JSON.
    let req_refresh = serde_json::json!({
        "client_id": id,
        "redirect_uri": redirect_uri,
        "nonce": "MySuperNonce",
        "code_challenge": challenge_s256,
        "code_challenge_method": "S256",
        "resource": RES_REFRESH,
    });
    let res = http
        .post(format!("{backend_url}/oidc/authorize/refresh"))
        .headers(session_headers.clone())
        .json(&req_refresh)
        .send()
        .await?;
    let res = expect_status!(res, 202);
    let (code_refresh, _state) = code_state_from_headers(&res)?;

    let token_req = TokenRequest {
        grant_type: GrantType::AuthorizationCode,
        code: Some(code_refresh),
        redirect_uri: Some(redirect_uri.clone()),
        client_id: Some(id.clone()),
        code_verifier: Some(PKCE_VERIFIER.to_string()),
        resource: Some(RES_REFRESH.to_string()),
        ..Default::default()
    };
    let res = http
        .post(format!("{backend_url}/oidc/token"))
        .form(&token_req)
        .send()
        .await?;
    let ts_refresh = expect_status_json!(res, 200, TokenSet);
    // the crux of #1645: the re-issued token must STILL be audience-bound to the resource
    assert!(
        aud_contains(&ts_refresh.access_token, RES_REFRESH),
        "access token from /authorize/refresh must still contain the resource in `aud`"
    );

    delete_client(&auth_headers, &id).await?;
    delete_user(&auth_headers, &user_email).await?;

    Ok(())
}

/// RFC 8707 resource indicators on the device authorization grant: the `resource` is
/// validated at `POST /oidc/device`, bound to the device code, ends up in the access token
/// `aud`, and survives a refresh. Without a `resource`, nothing changes.
#[tokio::test]
async fn test_resource_indicators_device_code() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let url_token = format!("{}/oidc/token", get_backend_url());
    let http = device_http()?;
    let user = DeviceUser::new("res-device").await?;

    let flow = create_device_client(
        &http,
        &auth_headers,
        "Resource Device Test",
        Some(vec![RES_A.to_string()]),
    )
    .await?;
    let id = flow.client_id.as_str();

    // a `resource` that is not allowed for the client -> `invalid_target`
    let res = device_auth(&http, &[("client_id", id), ("resource", RES_B)]).await?;
    let body = expect_status!(res, 400).text().await?;
    assert!(body.contains("invalid_target"), "unexpected body: {body}");

    // a single `resource` is supported, like for the other grants -> more than one is
    // rejected and never silently reduced to one of them
    let res = device_auth(
        &http,
        &[("client_id", id), ("resource", RES_A), ("resource", RES_B)],
    )
    .await?;
    expect_status!(res, 400);

    // no `resource` -> unchanged behavior, `aud` is only the client
    let device_code = device_code_accepted(&http, &user, &[("client_id", id)]).await?;
    tokio::time::sleep(DEVICE_POLL_INTERVAL).await;
    let res = http
        .post(&url_token)
        .form(&flow.token_req(&device_code, None))
        .send()
        .await?;
    let ts = expect_status_json!(res, 200, TokenSet);
    assert_eq!(decode_aud(&ts.access_token), serde_json::json!(id));

    // ... a refresh keeps it that way
    let res = http
        .post(&url_token)
        .form(&flow.refresh_req(ts.refresh_token.as_deref().unwrap(), None))
        .send()
        .await?;
    let ts = expect_status_json!(res, 200, TokenSet);
    assert_eq!(decode_aud(&ts.access_token), serde_json::json!(id));

    // ... and cannot add a `resource` that was never granted
    let res = http
        .post(&url_token)
        .form(&flow.refresh_req(ts.refresh_token.as_deref().unwrap(), Some(RES_A)))
        .send()
        .await?;
    let body = expect_status!(res, 400).text().await?;
    assert!(body.contains("invalid_target"), "unexpected body: {body}");

    // an allowed `resource` is bound to the device code
    let device_code =
        device_code_accepted(&http, &user, &[("client_id", id), ("resource", RES_A)]).await?;

    // the token request cannot switch to another `resource`
    let res = http
        .post(&url_token)
        .form(&flow.token_req(&device_code, Some(RES_B)))
        .send()
        .await?;
    let body = expect_status!(res, 400).text().await?;
    assert!(body.contains("invalid_target"), "unexpected body: {body}");

    // the bound `resource` on the token request is fine, and it ends up in `aud`
    tokio::time::sleep(DEVICE_POLL_INTERVAL).await;
    let res = http
        .post(&url_token)
        .form(&flow.token_req(&device_code, Some(RES_A)))
        .send()
        .await?;
    let ts = expect_status_json!(res, 200, TokenSet);
    assert_eq!(decode_aud(&ts.access_token), serde_json::json!([id, RES_A]));

    // a refresh without `resource` keeps the audience
    let res = http
        .post(&url_token)
        .form(&flow.refresh_req(ts.refresh_token.as_deref().unwrap(), None))
        .send()
        .await?;
    let ts = expect_status_json!(res, 200, TokenSet);
    assert_eq!(decode_aud(&ts.access_token), serde_json::json!([id, RES_A]));

    // a refresh cannot switch to another `resource`, and the refresh token stays usable
    let refresh_token = ts.refresh_token.unwrap();
    let res = http
        .post(&url_token)
        .form(&flow.refresh_req(&refresh_token, Some(RES_B)))
        .send()
        .await?;
    let body = expect_status!(res, 400).text().await?;
    assert!(body.contains("invalid_target"), "unexpected body: {body}");

    // the granted `resource` is accepted on a refresh
    let res = http
        .post(&url_token)
        .form(&flow.refresh_req(&refresh_token, Some(RES_A)))
        .send()
        .await?;
    let ts = expect_status_json!(res, 200, TokenSet);
    assert_eq!(decode_aud(&ts.access_token), serde_json::json!([id, RES_A]));

    // omitting the `resource` on the token request still applies the bound one
    let device_code =
        device_code_accepted(&http, &user, &[("client_id", id), ("resource", RES_A)]).await?;
    tokio::time::sleep(DEVICE_POLL_INTERVAL).await;
    let res = http
        .post(&url_token)
        .form(&flow.token_req(&device_code, None))
        .send()
        .await?;
    let ts = expect_status_json!(res, 200, TokenSet);
    assert_eq!(decode_aud(&ts.access_token), serde_json::json!([id, RES_A]));

    delete_client(&auth_headers, id).await?;
    user.delete().await?;

    Ok(())
}

/// A device refresh rotates into a regular refresh token that stays consent-exempt and consumes
/// its source, so a replay of the source fails. Once its device is deleted, its refresh fails.
#[tokio::test]
async fn test_device_refresh_rotation() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();
    let url_token = format!("{backend_url}/oidc/token");
    let http = device_http()?;
    let user = DeviceUser::new("device-rotate").await?;

    let flow = create_device_client(&http, &auth_headers, "Device Refresh Test", None).await?;
    let id = flow.client_id.as_str();

    let device_token_set = async || -> Result<TokenSet, Box<dyn Error>> {
        let device_code = device_code_accepted(&http, &user, &[("client_id", id)]).await?;
        tokio::time::sleep(DEVICE_POLL_INTERVAL).await;
        let res = http
            .post(&url_token)
            .form(&flow.token_req(&device_code, None))
            .send()
            .await?;
        Ok(expect_status_json!(res, 200, TokenSet))
    };
    let refresh = async |refresh_token: &str| {
        http.post(&url_token)
            .form(&flow.refresh_req(refresh_token, None))
            .send()
            .await
            .unwrap()
    };

    // --- one rotation
    let device_rt = device_token_set().await?.refresh_token.unwrap();
    assert_consent_exempt(&device_rt, true);

    let res = refresh(&device_rt).await;
    let rotated_rt = expect_status_json!(res, 200, TokenSet)
        .refresh_token
        .unwrap();
    // a regular refresh token now, still exempt
    assert_consent_exempt(&rotated_rt, false);

    // the source is consumed: a replay fails on the missing row
    let res = refresh(&device_rt).await;
    let body = expect_status!(res, 404).text().await?;
    assert!(body.contains("Device Refresh Token not found"), "{body}");

    // the replacement keeps rotating
    let res = refresh(&rotated_rt).await;
    let ts = expect_status_json!(res, 200, TokenSet);
    assert_consent_exempt(&ts.refresh_token.unwrap(), false);

    // --- the device and its refresh tokens are deleted between issue and refresh
    let device_rt = device_token_set().await?.refresh_token.unwrap();
    let claims = decode_claims(&device_rt);
    let device_id = claims["did"].as_str().unwrap();
    let user_id = claims["uid"].as_str().unwrap();

    let res = http
        .delete(format!("{backend_url}/users/{user_id}/devices"))
        .headers(auth_headers.clone())
        .json(&serde_json::json!({ "device_id": device_id }))
        .send()
        .await?;
    expect_status!(res, 200);

    // the deleted device took its refresh tokens with it: the refresh fails on the missing row
    let res = refresh(&device_rt).await;
    let body = expect_status!(res, 404).text().await?;
    assert!(body.contains("Device Refresh Token not found"), "{body}");

    delete_client(&auth_headers, id).await?;
    user.delete().await?;

    Ok(())
}
