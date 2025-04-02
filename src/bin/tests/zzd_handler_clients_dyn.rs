use crate::common::get_backend_url;
use pretty_assertions::{assert_eq, assert_ne};
use rauthy_api_types::clients::{DynamicClientRequest, DynamicClientResponse};
use reqwest::header::AUTHORIZATION;
use std::error::Error;

mod common;

#[tokio::test]
async fn test_dynamic_client() -> Result<(), Box<dyn Error>> {
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();

    let url = format!("{}/clients_dyn", backend_url);
    let mut payload = DynamicClientRequest {
        redirect_uris: vec!["http://localhost:8080/*".to_string()],
        grant_types: vec!["authorization_code".to_string()],
        client_name: Some("Dyn Test Client 123".to_string()),
        client_uri: None,
        contacts: None,
        id_token_signed_response_alg: None,
        token_endpoint_auth_method: Some("none".to_string()),
        token_endpoint_auth_signing_alg: None,
        post_logout_redirect_uri: None,
        backchannel_logout_uri: None,
    };
    let res = client.post(&url).json(&payload).send().await?;
    assert_eq!(res.status(), 201);
    let resp = res.json::<DynamicClientResponse>().await?;
    assert_eq!(resp.client_name, payload.client_name);
    // currently, we don't have a secret expiration
    assert_eq!(resp.client_secret_expires_at, 0);
    assert!(resp.grant_types.contains(&"authorization_code".to_string()));
    // with token_endpoint_auth_method == "none", the client must be public
    assert!(resp.client_secret.is_none());

    // test the rate limiting -> another registration that fast should be rejected
    let res = client.post(&url).json(&payload).send().await?;
    assert_eq!(res.status(), 429);

    // the registration token header for future self-modifications
    let token = format!(
        "Bearer {}",
        resp.registration_access_token.as_ref().unwrap()
    );

    // get our own metadata back with the registration token
    let url_check = format!("{}/{}", url, resp.client_id);
    let url = resp.registration_client_uri.unwrap();
    assert_eq!(url, url_check);

    let res = client.get(&url).send().await?;
    // we did not add any registration token.
    assert_eq!(res.status(), 401);

    let res = client
        .get(&url)
        .header(AUTHORIZATION, "Bearer IAmSoWrong1337")
        .send()
        .await?;
    // we did not add the correct registration token
    assert_eq!(res.status(), 401);

    let res = client
        .get(&url)
        .header(AUTHORIZATION, &token)
        .send()
        .await?;

    assert_eq!(res.status(), 200);
    let resp_get = res.json::<DynamicClientResponse>().await?;
    // We should get back the exact same response as from the registration, except for the
    // registration token and url, which should only be included when it has been changed.
    assert_eq!(resp.client_id, resp_get.client_id);
    assert_eq!(resp.client_name, resp_get.client_name);
    assert_eq!(resp.client_secret, resp_get.client_secret);
    assert_eq!(resp.client_secret, resp_get.client_secret);
    assert_eq!(resp.redirect_uris, resp_get.redirect_uris);
    assert_eq!(
        resp.post_logout_redirect_uri,
        resp_get.post_logout_redirect_uri
    );
    assert_eq!(resp.grant_types, resp_get.grant_types);
    assert_eq!(
        resp.id_token_signed_response_alg,
        resp_get.id_token_signed_response_alg
    );
    assert_eq!(
        resp.token_endpoint_auth_method,
        resp_get.token_endpoint_auth_method
    );
    assert_eq!(
        resp.token_endpoint_auth_signing_alg,
        resp_get.token_endpoint_auth_signing_alg
    );
    // These must be None for GET
    assert!(resp_get.registration_access_token.is_none());
    assert!(resp_get.registration_client_uri.is_none());

    // make sure GET is idempotent
    let res = client
        .get(&url)
        .header(AUTHORIZATION, &token)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let resp_get_new = res.json::<DynamicClientResponse>().await?;
    assert_eq!(resp_get_new, resp_get);

    // self-modify
    payload.client_name = Some("Dyn Test Client 12345".to_string());
    payload.grant_types.push("client_credentials".to_string());
    payload.grant_types.push("refresh_token".to_string());
    payload.token_endpoint_auth_method = Some("client_secret_post".to_string());
    payload.contacts = Some(vec![
        "batman@localhost.de".to_string(),
        "@alfred:matrix.org".to_string(),
    ]);
    payload.client_uri = Some("dyn.rauthy.io".to_string());
    let res = client
        .put(&url)
        .header(AUTHORIZATION, &token)
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let token_old = resp.registration_access_token;
    let resp = res.json::<DynamicClientResponse>().await?;
    assert_ne!(resp.registration_access_token, token_old);
    // we changed token_endpoint_auth_method -> should be a confidential client now
    assert!(resp.client_secret.is_some());
    assert_eq!(resp.client_name, payload.client_name);
    assert!(resp.grant_types.contains(&"client_credentials".to_string()));
    assert!(resp.grant_types.contains(&"refresh_token".to_string()));
    let contacts = resp.contacts.expect("contacts to be set");
    assert!(contacts.contains(&"batman@localhost.de".to_string()));
    assert!(contacts.contains(&"@alfred:matrix.org".to_string()));
    assert_eq!(
        &resp.client_uri.expect("client_uri to be set"),
        "dyn.rauthy.io"
    );

    // make sure the old registration token does not work anymore
    let res = client
        .get(&url)
        .header(AUTHORIZATION, &token)
        .send()
        .await?;
    assert_eq!(res.status(), 401);

    // self-modify again and make sure secrets are being rotated
    let token = format!(
        "Bearer {}",
        resp.registration_access_token.as_ref().unwrap()
    );
    let token_old = resp.registration_access_token;
    let secret_old = resp.client_secret;
    let res = client
        .put(&url)
        .header(AUTHORIZATION, &token)
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let resp = res.json::<DynamicClientResponse>().await?;
    assert_ne!(resp.registration_access_token, token_old);
    assert_ne!(resp.client_secret, secret_old);

    Ok(())
}
