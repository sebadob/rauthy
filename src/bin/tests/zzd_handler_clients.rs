use crate::common::{CLIENT_ID, CLIENT_SECRET, get_auth_headers, get_backend_url};
use pretty_assertions::{assert_eq, assert_ne};
use rauthy_api_types::clients::{
    ClientResponse, ClientSecretRequest, ClientSecretResponse, NewClientRequest,
    UpdateClientRequest,
};
use rauthy_api_types::oidc::{GrantType, JwkKeyPairAlg, TokenRequest};
use rauthy_common::constants::APPLICATION_JSON;
use rauthy_common::utils::base64_url_no_pad_decode;
use rauthy_jwt::claims::JwtAccessClaims;
use rauthy_service::token_set::TokenSet;
use reqwest::header::CONTENT_TYPE;
use serde_json::{Value, json};
use std::error::Error;

mod common;

fn extract_raw_claims(token: &str) -> Vec<u8> {
    let mut split = token.split('.');
    split.next().unwrap();
    let claims = split.next().unwrap();
    base64_url_no_pad_decode(claims).unwrap()
}

fn image_form(
    bytes: &'static [u8],
    file_name: &'static str,
    mime_type: &'static str,
) -> reqwest::multipart::Form {
    let part = reqwest::multipart::Part::bytes(bytes)
        .file_name(file_name)
        .mime_str(mime_type)
        .unwrap();
    reqwest::multipart::Form::new().part(file_name, part)
}

fn template_value<'a>(html: &'a str, id: &str) -> &'a str {
    let start = format!(r#"<template id="{id}">"#);
    html.split_once(&start)
        .and_then(|(_, rest)| rest.split_once("</template>"))
        .map(|(value, _)| value)
        .unwrap_or_else(|| panic!("template {id} missing from authorize HTML"))
}

/// GETs the client, rebuilds the full `UpdateClientRequest` (so we don't depend
/// on `JwkKeyPairAlg` being `Clone`), overrides `flows_enabled` + `claims`, PUTs
/// it back and returns the response.
async fn put_client_claims(
    auth_headers: &reqwest::header::HeaderMap,
    id: &str,
    flows_enabled: Vec<GrantType>,
    claims: Option<Value>,
    claims_at_root: bool,
) -> Result<ClientResponse, Box<dyn Error>> {
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();
    let url_id = format!("{}/clients/{}", backend_url, id);

    let c = client
        .get(&url_id)
        .headers(auth_headers.clone())
        .send()
        .await?
        .json::<ClientResponse>()
        .await?;

    let req = UpdateClientRequest {
        name: c.name,
        confidential: c.confidential,
        redirect_uris: c.redirect_uris,
        post_logout_redirect_uris: c.post_logout_redirect_uris,
        allowed_origins: c.allowed_origins,
        enabled: c.enabled,
        flows_enabled,
        access_token_alg: c.access_token_alg,
        id_token_alg: c.id_token_alg,
        auth_code_lifetime: c.auth_code_lifetime,
        access_token_lifetime: c.access_token_lifetime,
        scopes: c.scopes,
        default_scopes: c.default_scopes,
        challenges: c.challenges,
        force_mfa: c.force_mfa,
        client_uri: c.client_uri,
        contacts: c.contacts,
        backchannel_logout_uri: c.backchannel_logout_uri,
        restrict_group_prefix: c.restrict_group_prefix,
        claims,
        claims_at_root,
        allowed_resources: None,
        default_aud: None,
        scim: None,
    };

    let res = client
        .put(&url_id)
        .headers(auth_headers.clone())
        .json(&req)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    Ok(res.json::<ClientResponse>().await?)
}

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
    let len_orig = clients.len();
    let client = clients.iter().find(|c| c.id == "rauthy").unwrap();
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
    assert_eq!(
        client.flows_enabled.get(0).unwrap(),
        &GrantType::AuthorizationCode
    );
    // S256 code challenge by default for better security
    assert_eq!(client.challenges.as_ref().unwrap().get(0).unwrap(), "S256");

    // a URL-shaped id must be rejected for manually managed (non-ephemeral) clients:
    // these are restricted to `RE_CLIENT_ID_STRICT`, only ephemeral clients may use a URI id.
    let url_id_client = NewClientRequest {
        id: "https://connector.example.com/oauth/client.json".to_string(),
        secret: None,
        name: Some("URL Id Client".to_string()),
        confidential: true,
        redirect_uris: vec!["http://test.client.io/callback".to_string()],
        post_logout_redirect_uris: None,
    };
    let res = reqwest::Client::new()
        .post(&url)
        .headers(auth_headers.clone())
        .json(&url_id_client)
        .send()
        .await?;
    assert_eq!(res.status(), 400);

    // modify the client
    let mut redirect_uris = client.redirect_uris;
    redirect_uris.push("http://test.client.io/callback123".to_string());

    let allowed_origins = Some(vec!["http://origin.test.client.io".to_string()]);

    let mut flows_enabled = client.flows_enabled;
    flows_enabled.push(GrantType::Password);

    let update_client = UpdateClientRequest {
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
        claims: None,
        claims_at_root: false,
        allowed_resources: None,
        default_aud: None,
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
        vec![GrantType::AuthorizationCode, GrantType::Password]
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
    assert_eq!(clients.len(), len_orig);

    Ok(())
}

#[tokio::test]
async fn test_client_favicon_is_independent_from_logo() -> Result<(), Box<dyn Error>> {
    const LOGO: &[u8] = include_bytes!("../../../assets/logo/rauthy_dark_small.png");
    const FAVICON: &[u8] = include_bytes!("../../../assets/logo/rauthy_light_small.png");
    const FAVICON_SVG: &[u8] = include_bytes!("../../../assets/logo/rauthy_light.svg");

    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();
    let client_id = "favicon_client";
    let clients_url = format!("{backend_url}/clients");
    let client_url = format!("{clients_url}/{client_id}");
    let logo_url = format!("{client_url}/logo");
    let favicon_url = format!("{client_url}/favicon");
    let authorize_url = format!(
        "{backend_url}/oidc/authorize?client_id={client_id}\
         &redirect_uri=http://favicon.client.io/callback\
         &response_type=code&code_challenge=test&code_challenge_method=S256"
    );

    // Allow a rerun after an interrupted test.
    let _ = client
        .delete(&favicon_url)
        .headers(auth_headers.clone())
        .send()
        .await;
    let _ = client
        .delete(&client_url)
        .headers(auth_headers.clone())
        .send()
        .await;

    let new_client = NewClientRequest {
        id: client_id.to_string(),
        secret: None,
        name: Some("Favicon Client".to_string()),
        confidential: false,
        redirect_uris: vec!["http://favicon.client.io/callback".to_string()],
        post_logout_redirect_uris: None,
    };
    let res = client
        .post(&clients_url)
        .headers(auth_headers.clone())
        .json(&new_client)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let res = client
        .put(&favicon_url)
        .multipart(image_form(FAVICON, "favicon.png", "image/png"))
        .send()
        .await?;
    assert_eq!(res.status(), 401);

    let res = client
        .put(&logo_url)
        .headers(auth_headers.clone())
        .multipart(image_form(LOGO, "logo.png", "image/png"))
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let res = client.get(&authorize_url).send().await?;
    assert_eq!(res.status(), 200);
    let html = res.text().await?;
    assert!(!template_value(&html, "tpl_client_logo_updated").is_empty());
    assert!(template_value(&html, "tpl_client_favicon_updated").is_empty());

    let res = client
        .put(&favicon_url)
        .headers(auth_headers.clone())
        .multipart(image_form(FAVICON, "favicon.png", "image/png"))
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let res = client.get(&logo_url).send().await?;
    assert_eq!(res.status(), 200);
    let logo = res.bytes().await?;
    assert!(!logo.is_empty());

    let res = client.get(&favicon_url).send().await?;
    assert_eq!(res.status(), 200);
    assert_eq!(
        res.headers().get(CONTENT_TYPE).unwrap().to_str().unwrap(),
        "image/webp"
    );
    let favicon = res.bytes().await?;
    assert!(!favicon.is_empty());
    assert_ne!(logo, favicon);

    let res = client
        .put(&favicon_url)
        .headers(auth_headers.clone())
        .multipart(image_form(b"not a png", "invalid.png", "image/png"))
        .send()
        .await?;
    assert_eq!(res.status(), 400);

    let res = client.get(&favicon_url).send().await?;
    assert_eq!(res.status(), 200);
    assert_eq!(res.bytes().await?, favicon);

    let res = client
        .put(&favicon_url)
        .headers(auth_headers.clone())
        .multipart(reqwest::multipart::Form::new())
        .send()
        .await?;
    assert_eq!(res.status(), 400);

    let res = client.get(&authorize_url).send().await?;
    assert_eq!(res.status(), 200);
    let html = res.text().await?;
    assert!(!template_value(&html, "tpl_client_logo_updated").is_empty());
    assert!(!template_value(&html, "tpl_client_favicon_updated").is_empty());

    let res = client.delete(&favicon_url).send().await?;
    assert_eq!(res.status(), 401);

    let res = client.get(&favicon_url).send().await?;
    assert_eq!(res.status(), 200);
    assert_eq!(res.bytes().await?, favicon);

    let res = client
        .put(&favicon_url)
        .headers(auth_headers.clone())
        .multipart(image_form(FAVICON_SVG, "favicon.svg", "image/svg+xml"))
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let res = client.get(&logo_url).send().await?;
    assert_eq!(res.status(), 200);
    assert_eq!(res.bytes().await?, logo);

    let res = client
        .delete(&favicon_url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let res = client.get(&favicon_url).send().await?;
    assert_eq!(res.status(), 404);

    let res = client.get(&logo_url).send().await?;
    assert_eq!(res.status(), 200);
    assert_eq!(res.bytes().await?, logo);

    let res = client.get(&authorize_url).send().await?;
    assert_eq!(res.status(), 200);
    let html = res.text().await?;
    assert!(!template_value(&html, "tpl_client_logo_updated").is_empty());
    assert!(template_value(&html, "tpl_client_favicon_updated").is_empty());

    let res = client
        .delete(&client_url)
        .headers(auth_headers)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

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
        grant_type: GrantType::ClientCredentials,
        client_id: Some(CLIENT_ID.to_string()),
        client_secret: Some(secret.clone()),
        ..Default::default()
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

#[tokio::test]
async fn test_client_credentials_custom_claims() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();

    let url = format!("{}/clients", backend_url);
    let client_id = "claims_client";
    let url_id = format!("{}/clients/{}", backend_url, client_id);
    let url_secret = format!("{}/clients/{}/secret", backend_url, client_id);
    let url_token = format!("{}/oidc/token", backend_url);

    // pre-cleanup so a failed earlier run does not block creation
    let _ = client
        .delete(&url_id)
        .headers(auth_headers.clone())
        .send()
        .await;

    // create a dedicated confidential client for the `client_credentials` flow
    let new_client = NewClientRequest {
        id: client_id.to_string(),
        secret: None,
        name: Some("Claims Client".to_string()),
        confidential: true,
        redirect_uris: vec!["http://claims.client.io/callback".to_string()],
        post_logout_redirect_uris: None,
    };
    let res = client
        .post(&url)
        .headers(auth_headers.clone())
        .json(&new_client)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // set custom claims + enable the client_credentials flow
    let claims = json!({
        "tenant": "acme",
        "tier": "gold",
        "level": 3,
    });
    let resp = put_client_claims(
        &auth_headers,
        client_id,
        vec![GrantType::ClientCredentials],
        Some(claims.clone()),
        false,
    )
    .await?;
    // the value round-trips through the response
    assert_eq!(resp.claims.as_ref(), Some(&claims));
    assert!(!resp.claims_at_root);

    // grab the client secret in cleartext
    let res = client
        .post(&url_secret)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert!(res.status().is_success());
    let secret = res
        .json::<ClientSecretResponse>()
        .await?
        .secret
        .expect("a confidential client to have a secret");

    let token_req = TokenRequest {
        grant_type: GrantType::ClientCredentials,
        client_id: Some(client_id.to_string()),
        client_secret: Some(secret),
        ..Default::default()
    };
    let res = client.post(&url_token).form(&token_req).send().await?;
    assert_eq!(res.status(), 200);
    let ts = res.json::<TokenSet>().await?;

    // the access token carries the client's custom claims nested under `custom`
    let bytes = extract_raw_claims(&ts.access_token);
    let access = serde_json::from_slice::<JwtAccessClaims>(&bytes).unwrap();
    let custom = access.custom.expect("client claims emitted under `custom`");
    assert_eq!(custom.get("tenant").unwrap(), &Value::from("acme"));
    assert_eq!(custom.get("tier").unwrap(), &Value::from("gold"));
    assert_eq!(custom.get("level").unwrap(), &Value::from(3));
    // nested-only: the claims live under `custom`, never promoted to the token root
    let raw = serde_json::from_slice::<Value>(&bytes).unwrap();
    assert!(raw.get("custom").is_some());
    assert!(raw.get("tenant").is_none());
    assert!(raw.get("tier").is_none());
    assert!(raw.get("level").is_none());

    // `claims_at_root`: the same claims are now emitted flattened at the token
    // root instead of nested under `custom`
    let resp = put_client_claims(
        &auth_headers,
        client_id,
        vec![GrantType::ClientCredentials],
        Some(claims.clone()),
        true,
    )
    .await?;
    assert!(resp.claims_at_root);

    let res = client.post(&url_token).form(&token_req).send().await?;
    assert_eq!(res.status(), 200);
    let ts = res.json::<TokenSet>().await?;
    let bytes = extract_raw_claims(&ts.access_token);
    let raw = serde_json::from_slice::<Value>(&bytes).unwrap();
    assert_eq!(raw.get("tenant").unwrap(), &Value::from("acme"));
    assert_eq!(raw.get("tier").unwrap(), &Value::from("gold"));
    assert_eq!(raw.get("level").unwrap(), &Value::from(3));
    assert!(
        raw.get("custom").is_none(),
        "no nested `custom` expected when claims are promoted to the root"
    );

    // a root-promoted claim that collides with a reserved claim must fail issuance
    put_client_claims(
        &auth_headers,
        client_id,
        vec![GrantType::ClientCredentials],
        Some(json!({ "iss": "evil" })),
        true,
    )
    .await?;
    let res = client.post(&url_token).form(&token_req).send().await?;
    assert_ne!(
        res.status(),
        200,
        "issuance must fail when a root-promoted claim shadows a reserved claim"
    );

    // back-compat / clearing: a client without claims emits no `custom` at all
    let resp = put_client_claims(
        &auth_headers,
        client_id,
        vec![GrantType::ClientCredentials],
        None,
        false,
    )
    .await?;
    assert!(resp.claims.is_none());

    let res = client.post(&url_token).form(&token_req).send().await?;
    assert_eq!(res.status(), 200);
    let ts = res.json::<TokenSet>().await?;
    let bytes = extract_raw_claims(&ts.access_token);
    let access = serde_json::from_slice::<JwtAccessClaims>(&bytes).unwrap();
    assert!(
        access.custom.is_none(),
        "no `custom` expected when the client has no claims"
    );

    // cleanup
    let res = client
        .delete(&url_id)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    Ok(())
}
