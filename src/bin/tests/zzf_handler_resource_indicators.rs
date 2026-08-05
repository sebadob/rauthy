use crate::common::{
    PASSWORD, USERNAME, code_state_from_headers, cookie_csrf_headers_from_res_direct,
    get_auth_headers, get_backend_url, get_solved_pow,
};
use pretty_assertions::assert_eq;
use rauthy_api_types::clients::{
    ClientResponse, ClientSecretResponse, NewClientRequest, UpdateClientRequest,
};
use rauthy_api_types::oidc::{GrantType, JwkKeyPairAlg, LoginRequest, TokenRequest};
use rauthy_common::sha256;
use rauthy_common::utils::{base64_url_encode, base64_url_no_pad_decode};
use rauthy_service::token_set::TokenSet;
use std::error::Error;

mod common;

const ID: &str = "res_test";
const RES_A: &str = "https://rs-a.example.com/api";
const RES_B: &str = "https://rs-b.example.com/api";
const DEFAULT_AUD: &str = "https://always.example.com/";

const ID_REFRESH: &str = "res_refresh_test";
const RES_REFRESH: &str = "https://rs.example.com/mcp";

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

fn base_update() -> UpdateClientRequest {
    UpdateClientRequest {
        name: Some("Resource Test".to_string()),
        confidential: true,
        redirect_uris: vec!["http://localhost/callback".to_string()],
        post_logout_redirect_uris: None,
        allowed_origins: None,
        enabled: true,
        flows_enabled: vec![GrantType::ClientCredentials],
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
        backchannel_logout_uri: None,
        restrict_group_prefix: None,
        claims: None,
        claims_at_root: false,
        allowed_resources: None,
        default_aud: None,
        scim: None,
    }
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
    let new_client = NewClientRequest {
        id: ID.to_string(),
        secret: None,
        name: Some("Resource Test".to_string()),
        confidential: true,
        redirect_uris: vec!["http://localhost/callback".to_string()],
        post_logout_redirect_uris: None,
    };
    let res = http
        .post(format!("{backend_url}/clients"))
        .headers(auth_headers.clone())
        .json(&new_client)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // configure `allowed_resources` + `default_aud` + the client_credentials flow
    let mut upd = base_update();
    upd.allowed_resources = Some(vec![RES_A.to_string()]);
    upd.default_aud = Some(vec![DEFAULT_AUD.to_string()]);
    let res = http
        .put(format!("{backend_url}/clients/{ID}"))
        .headers(auth_headers.clone())
        .json(&upd)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let resp = res.json::<ClientResponse>().await?;
    assert_eq!(resp.allowed_resources, Some(vec![RES_A.to_string()]));
    assert_eq!(resp.default_aud, Some(vec![DEFAULT_AUD.to_string()]));

    // fetch the generated client secret
    let res = http
        .post(format!("{backend_url}/clients/{ID}/secret"))
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let secret = res
        .json::<ClientSecretResponse>()
        .await?
        .secret
        .expect("a confidential client secret");

    let url_token = format!("{backend_url}/oidc/token");
    let mut token_req = TokenRequest {
        grant_type: GrantType::ClientCredentials,
        client_id: Some(ID.to_string()),
        client_secret: Some(secret),
        ..Default::default()
    };

    // (1) no `resource` requested -> `aud` is an array containing the client and the
    // always-on `default_aud`
    let res = http.post(&url_token).form(&token_req).send().await?;
    assert_eq!(res.status(), 200);
    let ts = res.json::<TokenSet>().await?;
    let aud = decode_aud(&ts.access_token);
    let arr = aud
        .as_array()
        .expect("`aud` must be an array once `default_aud` is set");
    assert!(arr.iter().any(|v| v.as_str() == Some(ID)));
    assert!(arr.iter().any(|v| v.as_str() == Some(DEFAULT_AUD)));

    // (2) an allowed `resource` -> it is added to `aud`
    token_req.resource = Some(RES_A.to_string());
    let res = http.post(&url_token).form(&token_req).send().await?;
    assert_eq!(res.status(), 200);
    let ts = res.json::<TokenSet>().await?;
    let aud = decode_aud(&ts.access_token);
    let arr = aud.as_array().expect("`aud` array");
    assert!(arr.iter().any(|v| v.as_str() == Some(RES_A)));
    assert!(arr.iter().any(|v| v.as_str() == Some(DEFAULT_AUD)));

    // (3) a disallowed `resource` -> `invalid_target` (HTTP 400)
    token_req.resource = Some(RES_B.to_string());
    let res = http.post(&url_token).form(&token_req).send().await?;
    assert_eq!(res.status(), 400);
    let body = res.text().await?;
    assert!(body.contains("invalid_target"), "unexpected body: {body}");

    // (4) deny-by-default: with no `allowed_resources`, any `resource` is rejected
    let mut upd = base_update();
    upd.default_aud = Some(vec![DEFAULT_AUD.to_string()]);
    let res = http
        .put(format!("{backend_url}/clients/{ID}"))
        .headers(auth_headers.clone())
        .json(&upd)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    token_req.resource = Some(RES_A.to_string());
    let res = http.post(&url_token).form(&token_req).send().await?;
    assert_eq!(res.status(), 400);

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

    let redirect_uri = "http://localhost:3000/oidc/callback".to_string();
    let challenge_plain = "oDXug9zfYqfz8ejcqMpALRPXfW8QhbKV2AVuScAt8xrLKDAmaRYQ4yRi2uqcH9ys";
    let challenge_s256 = base64_url_encode(sha256!(challenge_plain.as_bytes()));

    // public PKCE client with `allowed_resources` = [RES_REFRESH]
    let new_client = NewClientRequest {
        id: ID_REFRESH.to_string(),
        secret: None,
        name: Some("Resource Refresh Test".to_string()),
        confidential: false,
        redirect_uris: vec![redirect_uri.clone()],
        post_logout_redirect_uris: None,
    };
    let res = http
        .post(format!("{backend_url}/clients"))
        .headers(auth_headers.clone())
        .json(&new_client)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let upd = UpdateClientRequest {
        name: Some("Resource Refresh Test".to_string()),
        confidential: false,
        redirect_uris: vec![redirect_uri.clone()],
        post_logout_redirect_uris: None,
        allowed_origins: None,
        enabled: true,
        flows_enabled: vec![GrantType::AuthorizationCode, GrantType::RefreshToken],
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
        backchannel_logout_uri: None,
        restrict_group_prefix: None,
        claims: None,
        claims_at_root: false,
        allowed_resources: Some(vec![RES_REFRESH.to_string()]),
        default_aud: None,
        scim: None,
    };
    let res = http
        .put(format!("{backend_url}/clients/{ID_REFRESH}"))
        .headers(auth_headers.clone())
        .json(&upd)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // authenticated session we reuse for the silent re-auth call
    let res = http
        .post(format!("{backend_url}/oidc/session"))
        .send()
        .await?;
    assert!(res.status().is_success());
    let session_headers = cookie_csrf_headers_from_res_direct(res).await?;

    // full authorization_code login WITH the RFC 8707 `resource`
    let query_pkce = format!(
        "client_id={ID_REFRESH}&redirect_uri={redirect_uri}&response_type=code\
        &code_challenge={challenge_s256}&code_challenge_method=S256"
    );
    let url_auth = format!("{backend_url}/oidc/authorize?{query_pkce}");
    let req_login = LoginRequest {
        email: USERNAME.to_string(),
        password: Some(PASSWORD.to_string()),
        pow: get_solved_pow().await,
        client_id: ID_REFRESH.to_string(),
        redirect_uri: redirect_uri.clone(),
        scopes: None,
        state: None,
        nonce: Some("MySuperNonce".to_string()),
        code_challenge: Some(challenge_s256.clone()),
        code_challenge_method: Some("S256".to_string()),
        resource: Some(RES_REFRESH.to_string()),
    };
    let res = http
        .post(&url_auth)
        .headers(session_headers.clone())
        .json(&req_login)
        .send()
        .await?;
    assert_eq!(res.status(), 202);
    let (code, _state) = code_state_from_headers(res)?;

    let token_req = TokenRequest {
        grant_type: GrantType::AuthorizationCode,
        code: Some(code),
        redirect_uri: Some(redirect_uri.clone()),
        client_id: Some(ID_REFRESH.to_string()),
        client_secret: None,
        code_verifier: Some(challenge_plain.to_string()),
        device_code: None,
        username: None,
        password: None,
        refresh_token: None,
        resource: Some(RES_REFRESH.to_string()),
        subject_token: None,
        subject_token_type: None,
        actor_token: None,
        actor_token_type: None,
        requested_token_type: None,
        audience: None,
        scope: None,
    };
    let res = http
        .post(format!("{backend_url}/oidc/token"))
        .form(&token_req)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let ts = res.json::<TokenSet>().await?;
    assert!(
        aud_contains(&ts.access_token, RES_REFRESH),
        "initial access token `aud` must contain the requested resource"
    );

    // silent re-auth via `POST /oidc/authorize/refresh` on the SAME session, same `resource`.
    // `LoginRefreshRequest` is deserialize-only (no `Serialize`), so build the body as raw JSON.
    let req_refresh = serde_json::json!({
        "client_id": ID_REFRESH,
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
    assert_eq!(res.status(), 202);
    let (code_refresh, _state) = code_state_from_headers(res)?;

    let token_req = TokenRequest {
        grant_type: GrantType::AuthorizationCode,
        code: Some(code_refresh),
        redirect_uri: Some(redirect_uri.clone()),
        client_id: Some(ID_REFRESH.to_string()),
        client_secret: None,
        code_verifier: Some(challenge_plain.to_string()),
        device_code: None,
        username: None,
        password: None,
        refresh_token: None,
        resource: Some(RES_REFRESH.to_string()),
        subject_token: None,
        subject_token_type: None,
        actor_token: None,
        actor_token_type: None,
        requested_token_type: None,
        audience: None,
        scope: None,
    };
    let res = http
        .post(format!("{backend_url}/oidc/token"))
        .form(&token_req)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let ts_refresh = res.json::<TokenSet>().await?;
    // the crux of #1645: the re-issued token must STILL be audience-bound to the resource
    assert!(
        aud_contains(&ts_refresh.access_token, RES_REFRESH),
        "access token from /authorize/refresh must still contain the resource in `aud`"
    );

    Ok(())
}
