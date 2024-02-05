use crate::common::{get_auth_headers, get_backend_url, CLIENT_SECRET};
use pretty_assertions::{assert_eq, assert_ne};
use rauthy_models::entity::jwk::JwkKeyPairAlg;
use rauthy_models::request::{DynamicClientRequest, NewClientRequest, UpdateClientRequest};
use rauthy_models::response::{ClientResponse, ClientSecretResponse, DynamicClientResponse};
use reqwest::header::AUTHORIZATION;
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
    assert_eq!(client.id, "rauthy");
    assert_eq!(client.name, Some("Rauthy".to_string()));
    assert_eq!(client.enabled, true);
    assert_eq!(client.confidential, false);
    assert_eq!(client.redirect_uris.len(), 2);
    assert_eq!(client.post_logout_redirect_uris.as_ref().unwrap().len(), 2);
    assert_eq!(
        client.allowed_origins,
        Some(vec!["http://localhost:5173".to_string()])
    );
    assert_eq!(client.flows_enabled.len(), 2);
    assert_eq!(client.access_token_alg, "EdDSA");
    assert_eq!(client.id_token_alg, "EdDSA");
    assert_eq!(client.refresh_token, false);
    assert_eq!(client.auth_code_lifetime, 60);
    assert_eq!(client.access_token_lifetime, 1800);
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

    let allowed_origins = Some(vec!["http://origin.test.cient.io/*".to_string()]);

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
        refresh_token: true,
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
    assert_eq!(client.access_token_alg, "RS256");
    assert_eq!(client.id_token_alg, "RS256");
    assert_eq!(client.refresh_token, true);
    assert_eq!(client.auth_code_lifetime, 60);
    assert_eq!(client.access_token_lifetime, 900);
    assert!(client.scopes.contains(&"openid".to_string()));
    assert!(client.scopes.contains(&"email".to_string()));
    assert!(client.default_scopes.contains(&"email".to_string()));
    assert!(client.default_scopes.contains(&"email".to_string()));
    let challenges = client.challenges.clone().unwrap();
    assert!(challenges.contains(&"S256".to_string()));
    assert!(challenges.contains(&"plain".to_string()));

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
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert!(res.status().is_success());
    let resp = res.json::<ClientSecretResponse>().await?;
    assert_eq!(resp.id, "init_client");
    assert_eq!(resp.confidential, true);
    assert_eq!(resp.secret.unwrap(), CLIENT_SECRET);

    let url = format!("{}/clients/init_client/secret", backend_url);
    let res = client
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert!(res.status().is_success());
    let resp = res.json::<ClientSecretResponse>().await?;
    assert!(resp.secret.is_some());
    let secret = resp.secret.unwrap();

    // generate a new secret
    let res = client
        .put(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert!(res.status().is_success());
    let resp = res.json::<ClientSecretResponse>().await?;
    assert!(resp.secret.is_some());
    let new_secret = resp.secret.unwrap();
    assert_ne!(new_secret, secret);

    let url = format!("{}/clients/init_client/secret", backend_url);
    let res = client
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert!(res.status().is_success());
    let resp = res.json::<ClientSecretResponse>().await?;
    assert!(resp.secret.is_some());
    assert_eq!(resp.secret.unwrap(), new_secret);

    Ok(())
}

#[tokio::test]
async fn test_dynamic_client() -> Result<(), Box<dyn Error>> {
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();

    let url = format!("{}/clients_dyn", backend_url);
    let mut payload = DynamicClientRequest {
        redirect_uris: vec!["http://localhost:8080/*".to_string()],
        grant_types: vec!["authorization_code".to_string()],
        client_name: Some("Dyn Test Client 123".to_string()),
        id_token_signed_response_alg: None,
        token_endpoint_auth_method: Some("none".to_string()),
        token_endpoint_auth_signing_alg: None,
        post_logout_redirect_uri: None,
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
    assert_eq!(res.status(), 400);

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
