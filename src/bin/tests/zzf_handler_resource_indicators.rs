use crate::common::{get_auth_headers, get_backend_url};
use pretty_assertions::assert_eq;
use rauthy_api_types::clients::{
    ClientResponse, ClientSecretResponse, NewClientRequest, UpdateClientRequest,
};
use rauthy_api_types::oidc::{JwkKeyPairAlg, TokenRequest};
use rauthy_common::utils::base64_url_no_pad_decode;
use rauthy_service::token_set::TokenSet;
use std::error::Error;

mod common;

const ID: &str = "res_test";
const RES_A: &str = "https://rs-a.example.com/api";
const RES_B: &str = "https://rs-b.example.com/api";
const DEFAULT_AUD: &str = "https://always.example.com/";

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
        id: ID.to_string(),
        name: Some("Resource Test".to_string()),
        confidential: true,
        redirect_uris: vec!["http://localhost/callback".to_string()],
        post_logout_redirect_uris: None,
        allowed_origins: None,
        enabled: true,
        flows_enabled: vec!["client_credentials".to_string()],
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
        grant_type: "client_credentials".to_string(),
        code: None,
        redirect_uri: None,
        client_id: Some(ID.to_string()),
        client_secret: Some(secret),
        code_verifier: None,
        device_code: None,
        username: None,
        password: None,
        refresh_token: None,
        resource: None,
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
