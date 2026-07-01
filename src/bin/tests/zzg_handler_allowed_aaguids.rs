use crate::common::{get_auth_headers, get_backend_url};
use pretty_assertions::assert_eq;
use rauthy_api_types::clients::{ClientResponse, NewClientRequest, UpdateClientRequest};
use rauthy_api_types::oidc::JwkKeyPairAlg;
use std::error::Error;

mod common;

const ID: &str = "aaguid_test";
// Two well-formed canonical (dashed) AAGUIDs. Any valid UUID round-trips here: this slice
// only stores and surfaces the allow-list, enforcement at the WebAuthn ceremony is a
// planned follow-up, so the values need not map to a real authenticator.
const AAGUID_A: &str = "ee882879-721c-4913-9775-3dfcce97072a";
const AAGUID_B: &str = "d8522d9f-575b-4866-88a9-ba99fa02f35b";

fn base_update() -> UpdateClientRequest {
    UpdateClientRequest {
        name: Some("AAGUID Test".to_string()),
        confidential: true,
        redirect_uris: vec!["http://localhost/callback".to_string()],
        post_logout_redirect_uris: None,
        allowed_origins: None,
        enabled: true,
        flows_enabled: vec!["authorization_code".to_string()],
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
        allowed_aaguids: None,
        scim: None,
    }
}

/// End-to-end coverage for the per-client `allowed_aaguids` allow-list: the CSV round-trip
/// through create -> update -> read (both the `PUT` response and a fresh `GET`), that
/// clearing the list persists as absent, and that a malformed AAGUID is rejected with 400
/// by `validate_vec_aaguid`.
#[tokio::test]
async fn test_allowed_aaguids() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();
    let http = reqwest::Client::new();

    // create a confidential client
    let new_client = NewClientRequest {
        id: ID.to_string(),
        secret: None,
        name: Some("AAGUID Test".to_string()),
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

    // (1) set a two-entry allow-list; the PUT response echoes it back in order
    let mut upd = base_update();
    upd.allowed_aaguids = Some(vec![AAGUID_A.to_string(), AAGUID_B.to_string()]);
    let res = http
        .put(format!("{backend_url}/clients/{ID}"))
        .headers(auth_headers.clone())
        .json(&upd)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let resp = res.json::<ClientResponse>().await?;
    assert_eq!(
        resp.allowed_aaguids,
        Some(vec![AAGUID_A.to_string(), AAGUID_B.to_string()])
    );

    // (2) a fresh GET returns the same persisted allow-list (proves it survived storage)
    let res = http
        .get(format!("{backend_url}/clients/{ID}"))
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let resp = res.json::<ClientResponse>().await?;
    assert_eq!(
        resp.allowed_aaguids,
        Some(vec![AAGUID_A.to_string(), AAGUID_B.to_string()])
    );

    // (3) clearing the list round-trips as absent
    let res = http
        .put(format!("{backend_url}/clients/{ID}"))
        .headers(auth_headers.clone())
        .json(&base_update())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let resp = res.json::<ClientResponse>().await?;
    assert_eq!(resp.allowed_aaguids, None);

    // (4) a malformed AAGUID is rejected with 400 by `validate_vec_aaguid`
    let mut upd = base_update();
    upd.allowed_aaguids = Some(vec!["not-a-uuid".to_string()]);
    let res = http
        .put(format!("{backend_url}/clients/{ID}"))
        .headers(auth_headers.clone())
        .json(&upd)
        .send()
        .await?;
    assert_eq!(res.status(), 400);

    Ok(())
}
