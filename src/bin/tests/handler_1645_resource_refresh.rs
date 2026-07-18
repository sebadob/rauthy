use crate::common::{
    PASSWORD, USERNAME, code_state_from_headers, cookie_csrf_headers_from_res_direct,
    get_auth_headers, get_backend_url, get_solved_pow,
};
use pretty_assertions::assert_eq;
use rauthy_api_types::clients::{ClientResponse, NewClientRequest, UpdateClientRequest};
use rauthy_api_types::oidc::{JwkKeyPairAlg, LoginRequest, TokenRequest};
use rauthy_common::sha256;
use rauthy_common::utils::{base64_url_encode, base64_url_no_pad_decode};
use rauthy_service::token_set::TokenSet;
use std::error::Error;

mod common;

const RID: &str = "res_refresh_test";
const RESOURCE: &str = "https://rs.example.com/mcp";

fn decode_payload(token: &str) -> serde_json::Value {
    let payload_b64 = token.split('.').nth(1).expect("a JWT payload segment");
    let bytes = base64_url_no_pad_decode(payload_b64).expect("valid base64url payload");
    serde_json::from_slice(&bytes).expect("valid JSON claims")
}

fn aud_contains(token: &str, expected: &str) -> bool {
    let claims = decode_payload(token);
    match claims.get("aud") {
        Some(serde_json::Value::String(s)) => s == expected,
        Some(serde_json::Value::Array(arr)) => arr.iter().any(|v| v.as_str() == Some(expected)),
        _ => false,
    }
}

// -----------------------------------------------------------------------------------------
// #1645 - the RFC 8707 `resource` indicator must survive the silent re-auth path
// (`POST /oidc/authorize/refresh`).
//
// When a still-valid session re-issues a code via the silent re-auth endpoint, the
// authorization request still carries the `resource`. Before the fix, `post_authorize_refresh`
// dropped it, so the re-issued auth code (and thus the exchanged access token) lost the
// audience restriction and its `aud` no longer contained the resource.
//
// This test drives the full flow end-to-end:
//   (a) create a client with `allowed_resources` = [RESOURCE]
//   (b) full authorization_code login WITH `resource`, exchange -> assert `aud` contains RESOURCE
//   (c) hit `POST /oidc/authorize/refresh` on the SAME authenticated session WITH the same
//       `resource`, exchange the new code -> assert `aud` STILL contains RESOURCE
//
// Without the fix, step (c)'s token drops the resource and the final `aud` assertion fails.
// -----------------------------------------------------------------------------------------
#[tokio::test]
async fn test_1645_resource_survives_authorize_refresh() -> Result<(), Box<dyn Error>> {
    let backend_url = get_backend_url();
    let auth_headers = get_auth_headers().await?;
    let http = reqwest::Client::new();

    let redirect_uri = "http://localhost:3000/oidc/callback".to_string();
    let challenge_plain = "oDXug9zfYqfz8ejcqMpALRPXfW8QhbKV2AVuScAt8xrLKDAmaRYQ4yRi2uqcH9ys";
    let challenge_s256 = base64_url_encode(sha256!(challenge_plain.as_bytes()));

    // (a) create a public client (PKCE, no secret) and configure `allowed_resources`
    let new_client = NewClientRequest {
        id: RID.to_string(),
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
        flows_enabled: vec![
            "authorization_code".to_string(),
            "refresh_token".to_string(),
        ],
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
        allowed_resources: Some(vec![RESOURCE.to_string()]),
        default_aud: None,
        scim: None,
    };
    let res = http
        .put(format!("{backend_url}/clients/{RID}"))
        .headers(auth_headers.clone())
        .json(&upd)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let resp = res.json::<ClientResponse>().await?;
    assert_eq!(resp.allowed_resources, Some(vec![RESOURCE.to_string()]));

    // Init session (cookie + CSRF) we authenticate then reuse for the silent re-auth call.
    let res = http
        .post(format!("{backend_url}/oidc/session"))
        .send()
        .await?;
    assert!(res.status().is_success());
    let session_headers = cookie_csrf_headers_from_res_direct(res).await?;

    // (b) full authorization_code login WITH the RFC 8707 `resource`
    let query_pkce = format!(
        "client_id={RID}&redirect_uri={redirect_uri}&response_type=code\
        &code_challenge={challenge_s256}&code_challenge_method=S256"
    );
    let url_auth = format!("{backend_url}/oidc/authorize?{query_pkce}");

    let req_login = LoginRequest {
        email: USERNAME.to_string(),
        password: Some(PASSWORD.to_string()),
        pow: get_solved_pow().await,
        client_id: RID.to_string(),
        redirect_uri: redirect_uri.clone(),
        scopes: None,
        state: None,
        nonce: Some("MySuperNonce".to_string()),
        code_challenge: Some(challenge_s256.clone()),
        code_challenge_method: Some("S256".to_string()),
        resource: Some(RESOURCE.to_string()),
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
        grant_type: "authorization_code".to_string(),
        code: Some(code),
        redirect_uri: Some(redirect_uri.clone()),
        client_id: Some(RID.to_string()),
        client_secret: None,
        code_verifier: Some(challenge_plain.to_string()),
        device_code: None,
        username: None,
        password: None,
        refresh_token: None,
        resource: Some(RESOURCE.to_string()),
    };
    let res = http
        .post(format!("{backend_url}/oidc/token"))
        .form(&token_req)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let ts = res.json::<TokenSet>().await?;
    assert!(
        aud_contains(&ts.access_token, RESOURCE),
        "initial access token `aud` must contain the requested resource"
    );

    // (c) silent re-auth via `POST /oidc/authorize/refresh` on the SAME authenticated session,
    // carrying the same `resource`. This is the path that dropped the resource before the fix.
    // `LoginRefreshRequest` is deserialize-only (no `Serialize`), so build the body as raw JSON.
    let req_refresh = serde_json::json!({
        "client_id": RID,
        "redirect_uri": redirect_uri,
        "nonce": "MySuperNonce",
        "code_challenge": challenge_s256,
        "code_challenge_method": "S256",
        "resource": RESOURCE,
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
        grant_type: "authorization_code".to_string(),
        code: Some(code_refresh),
        redirect_uri: Some(redirect_uri.clone()),
        client_id: Some(RID.to_string()),
        client_secret: None,
        code_verifier: Some(challenge_plain.to_string()),
        device_code: None,
        username: None,
        password: None,
        refresh_token: None,
        resource: Some(RESOURCE.to_string()),
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
        aud_contains(&ts_refresh.access_token, RESOURCE),
        "access token from /authorize/refresh must still contain the resource in `aud`"
    );

    Ok(())
}
