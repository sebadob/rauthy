use crate::common::{CLIENT_ID, CLIENT_SECRET, get_auth_headers, get_backend_url};
use pretty_assertions::{assert_eq, assert_ne};
use rauthy_api_types::clients::{
    ClientResponse, ClientSecretRequest, ClientSecretResponse, NewClientRequest,
    UpdateClientRequest,
};
use rauthy_api_types::oidc::{JwkKeyPairAlg, TokenRequest};
use rauthy_common::constants::APPLICATION_JSON;
use rauthy_service::token_set::TokenSet;
use reqwest::header::CONTENT_TYPE;
use std::error::Error;

mod common;

#[tokio::test]
async fn test_clients() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();

    let url = format!("{}/clients", backend_url);
    let res = reqwest::Client::new()
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let clients = res.json::<Vec<ClientResponse>>().await?;
    assert_eq!(clients.len(), 2);
    let mut client = clients.get(0).unwrap();
    if client.id == "init_client" {
        client = clients.get(1).unwrap();
    }
    println!("{:?}", client);
    assert_eq!(client.id, "rauthy");
    assert_eq!(client.name, Some("Rauthy".to_string()));
    assert_eq!(client.enabled, true);
    assert_eq!(client.confidential, false);
    assert_eq!(client.redirect_uris.len(), 3);
    assert_eq!(client.post_logout_redirect_uris, None);
    assert_eq!(
        client.allowed_origins,
        Some(vec!["http://localhost:5173".to_string()])
    );
    assert_eq!(client.flows_enabled.len(), 1);
    assert_eq!(client.access_token_alg, JwkKeyPairAlg::EdDSA);
    assert_eq!(client.id_token_alg, JwkKeyPairAlg::EdDSA);
    assert_eq!(client.auth_code_lifetime, 10);
    assert_eq!(client.access_token_lifetime, 10);
    assert_eq!(client.scopes.len(), 2);
    assert_eq!(client.default_scopes.len(), 2);
    assert_eq!(client.challenges.as_ref().unwrap().len(), 1);

    // add a new client
    let new_client = NewClientRequest {
        id: "test123".to_string(),
        secret: None,
        name: Some("Test Client 123".to_string()),
        confidential: true,
        redirect_uris: vec!["http://test.client.io/callback".to_string()],
        post_logout_redirect_uris: Some(vec!["http://test.client.io/logout".to_string()]),
    };
    let res = reqwest::Client::new()
        .post(&url)
        .headers(auth_headers.clone())
        .json(&new_client)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let client = res.json::<ClientResponse>().await?;
    assert_eq!(client.id, new_client.id);
    assert_eq!(client.name, new_client.name);
    assert_eq!(client.confidential, new_client.confidential);
    assert_eq!(client.redirect_uris, new_client.redirect_uris);
    assert_eq!(
        client.post_logout_redirect_uris,
        new_client.post_logout_redirect_uris
    );
    assert_eq!(client.allowed_origins, None);
    // authorization_code should be the only default flow since it is secure
    assert_eq!(client.flows_enabled.get(0).unwrap(), "authorization_code");
    // S256 code challenge by default for better security
    assert_eq!(client.challenges.as_ref().unwrap().get(0).unwrap(), "S256");

    // modify the client
    let mut redirect_uris = client.redirect_uris;
    redirect_uris.push("http://test.client.io/callback123".to_string());

    let allowed_origins = Some(vec!["http://origin.test.cient.io".to_string()]);

    let mut flows_enabled = client.flows_enabled;
    flows_enabled.push("password".to_string());

    let update_client = UpdateClientRequest {
        id: client.id.clone(),
        name: None,
        confidential: false,
        redirect_uris: redirect_uris.clone(),
        post_logout_redirect_uris: None,
        allowed_origins: allowed_origins.clone(),
        enabled: false,
        flows_enabled,
        access_token_alg: JwkKeyPairAlg::RS256,
        id_token_alg: JwkKeyPairAlg::RS256,
        auth_code_lifetime: 60,
        access_token_lifetime: 900,
        scopes: vec![
            "openid".to_string(),
            "email".to_string(),
            "doestnotexist".to_string(),
        ],
        default_scopes: vec![
            "openid".to_string(),
            "email".to_string(),
            "doestnotexist".to_string(),
        ],
        challenges: Some(vec!["S256".to_string(), "plain".to_string()]),
        force_mfa: false,
        client_uri: Some("rauthy.io".to_string()),
        contacts: Some(vec![
            "batman@localhost.de".to_string(),
            "@alfred:matrix.org".to_string(),
        ]),
        backchannel_logout_uri: None,
        restrict_group_prefix: None,
        scim: None,
    };

    let url_id = format!("{}/clients/{}", backend_url, client.id);
    let res = reqwest::Client::new()
        .put(&url_id)
        .headers(auth_headers.clone())
        .json(&update_client)
        .send()
        .await?;
    if res.status() != 200 {
        let err = res.text().await?;
        // let err = aw!(res.json::<ErrorResponse>())?;
        panic!("{:?}", err);
    }
    assert_eq!(res.status(), 200);

    let client = res.json::<ClientResponse>().await?;
    assert_eq!(client.name, None);
    assert_eq!(client.confidential, false);
    assert_eq!(client.redirect_uris, redirect_uris);
    assert_eq!(client.post_logout_redirect_uris, None);
    assert_eq!(client.allowed_origins, allowed_origins);
    assert_eq!(client.enabled, false);
    assert_eq!(
        client.flows_enabled,
        vec!["authorization_code".to_string(), "password".to_string()]
    );
    assert_eq!(client.access_token_alg, JwkKeyPairAlg::RS256);
    assert_eq!(client.id_token_alg, JwkKeyPairAlg::RS256);
    assert_eq!(client.auth_code_lifetime, 60);
    assert_eq!(client.access_token_lifetime, 900);
    assert!(client.scopes.contains(&"openid".to_string()));
    assert!(client.scopes.contains(&"email".to_string()));
    assert!(client.default_scopes.contains(&"email".to_string()));
    assert!(client.default_scopes.contains(&"email".to_string()));
    let challenges = client.challenges.clone().unwrap();
    assert!(challenges.contains(&"S256".to_string()));
    assert!(challenges.contains(&"plain".to_string()));
    assert_eq!(client.client_uri, Some("rauthy.io".to_string()));
    let contacts = client.contacts.expect("contacts to exist");
    assert!(contacts.contains(&"batman@localhost.de".to_string()));
    assert!(contacts.contains(&"@alfred:matrix.org".to_string()));

    // delete the client again
    let res = reqwest::Client::new()
        .delete(&url_id)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // now verify that it is gone
    let res = reqwest::Client::new()
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let clients = res.json::<Vec<ClientResponse>>().await?;
    assert_eq!(clients.len(), 2);

    Ok(())
}

#[tokio::test]
async fn test_client_secret() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();

    // get the current client secret
    let url = format!("{}/clients/init_client/secret", backend_url);
    let res = client
        .post(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert!(res.status().is_success());
    let resp = res.json::<ClientSecretResponse>().await?;
    assert_eq!(resp.id, "init_client");
    assert_eq!(resp.confidential, true);
    assert_eq!(resp.secret.unwrap(), CLIENT_SECRET);

    // be sure that this endpoint is idempotent
    let url = format!("{}/clients/init_client/secret", backend_url);
    let res = client
        .post(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert!(res.status().is_success());
    let resp = res.json::<ClientSecretResponse>().await?;
    assert!(resp.secret.is_some());
    let secret = resp.secret.unwrap();

    // try to get a token with the credentials
    let mut token_req = TokenRequest {
        grant_type: "client_credentials".to_string(),
        code: None,
        redirect_uri: None,
        client_id: Some(CLIENT_ID.to_string()),
        client_secret: Some(secret.clone()),
        code_verifier: None,
        device_code: None,
        username: None,
        password: None,
        refresh_token: None,
    };
    let url_token = format!("{}/oidc/token", backend_url);
    let res = client.post(&url_token).form(&token_req).send().await?;
    assert_eq!(res.status(), 200);
    let ts = res.json::<TokenSet>().await?;
    assert!(!ts.access_token.is_empty());

    // generate a new secret
    let res = client
        .put(&url)
        .headers(auth_headers.clone())
        .header(CONTENT_TYPE, APPLICATION_JSON)
        .send()
        .await?;
    assert!(res.status().is_success());
    let resp = res.json::<ClientSecretResponse>().await?;
    assert!(resp.secret.is_some());
    let secret_new = resp.secret.unwrap();
    assert_ne!(secret_new, secret);

    // make sure we cannot login anymore with the old secret
    let res = client.post(&url_token).form(&token_req).send().await?;
    assert_eq!(res.status(), 401);

    // but we should be able to log in with the new one
    token_req.client_secret = Some(secret_new.clone());
    let res = client.post(&url_token).form(&token_req).send().await?;
    assert_eq!(res.status(), 200);

    // make sure we get the same secret back
    let res = client
        .post(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert!(res.status().is_success());
    let resp = res.json::<ClientSecretResponse>().await?;
    assert!(resp.secret.is_some());
    assert_eq!(resp.secret.unwrap(), secret_new);

    // rotate secret gracefully and cache the current one
    let payload = ClientSecretRequest {
        cache_current_hours: Some(1),
    };
    let res = client
        .put(&url)
        .headers(auth_headers.clone())
        .json(&payload)
        .send()
        .await?;
    assert!(res.status().is_success());
    let resp = res.json::<ClientSecretResponse>().await?;
    assert!(resp.secret.is_some());
    let secret_rotated = resp.secret.unwrap();
    assert_ne!(secret_rotated, secret_new);

    // we should be able to use both secrets (for 1 hour) to log in
    let res = client.post(&url_token).form(&token_req).send().await?;
    assert_eq!(res.status(), 200);

    token_req.client_secret = Some(secret_rotated);
    let res = client.post(&url_token).form(&token_req).send().await?;
    assert_eq!(res.status(), 200);

    Ok(())
}
