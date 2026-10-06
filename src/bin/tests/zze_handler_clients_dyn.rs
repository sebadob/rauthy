use crate::common::{get_auth_headers, get_backend_url};
use pretty_assertions::{assert_eq, assert_ne};
use rauthy_api_types::clients::{
    ClientResponse, DynamicClientRequest, DynamicClientResponse, UpdateClientRequest,
};
use rauthy_api_types::oidc::GrantType;
use reqwest::header::AUTHORIZATION;
use std::error::Error;

mod common;

const GOOD_REDIRECT_URI: &str = "http://localhost:8080/cb";
const BAD_REDIRECT_URIS: [&str; 4] = [
    "http://localhost:8080/#/cb",
    "http://localhost:8080/cb?code=x",
    "http://localhost:8080/cb?foo=bar&iss=x",
    // stored comma-joined, so this would become a second, unchecked redirect URI
    "http://localhost:8080/cb?x=,https://evil.example/cb",
];

/// A `#` or `,` is already rejected by the payload validation (`RE_CLIENT_URI`), anything else by
/// `validate_redirect_uri_shape()`.
fn expected_redirect_uri_err(bad_uri: &str) -> &'static str {
    if bad_uri.contains(['#', ',']) {
        "Payload validation error"
    } else {
        "`redirect_uri` must not contain"
    }
}

const BAD_POST_LOGOUT_REDIRECT_URIS: [&str; 3] = [
    "http://localhost:8080/#/bye",
    "http://localhost:8080/bye?state=x",
    "http://localhost:8080/bye?foo=bar&State=x",
];

async fn admin_get_client(id: &str) -> Result<ClientResponse, Box<dyn Error>> {
    let res = reqwest::Client::new()
        .get(format!("{}/clients/{}", get_backend_url(), id))
        .headers(get_auth_headers().await?)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    Ok(res.json::<ClientResponse>().await?)
}

/// Updates the client via the admin API: GETs it, applies `modify` to the full
/// `UpdateClientRequest` and PUTs it back.
async fn admin_update_client(
    id: &str,
    modify: impl FnOnce(&mut UpdateClientRequest),
) -> Result<(), Box<dyn Error>> {
    let c = admin_get_client(id).await?;
    let mut req = UpdateClientRequest {
        name: c.name,
        confidential: c.confidential,
        redirect_uris: c.redirect_uris,
        post_logout_redirect_uris: c.post_logout_redirect_uris,
        allowed_origins: c.allowed_origins,
        enabled: c.enabled,
        flows_enabled: c.flows_enabled,
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
        claims: c.claims,
        claims_at_root: c.claims_at_root,
        allowed_resources: c.allowed_resources,
        default_aud: c.default_aud,
        scim: c.scim,
    };
    modify(&mut req);

    let res = reqwest::Client::new()
        .put(format!("{}/clients/{}", get_backend_url(), id))
        .headers(get_auth_headers().await?)
        .json(&req)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    Ok(())
}

#[tokio::test]
async fn test_dynamic_client() -> Result<(), Box<dyn Error>> {
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();

    let url = format!("{}/clients_dyn", backend_url);
    let mut payload = DynamicClientRequest {
        redirect_uris: vec![GOOD_REDIRECT_URI.to_string()],
        grant_types: vec![GrantType::AuthorizationCode, GrantType::RefreshToken],
        client_name: Some("Dyn Test Client 123".to_string()),
        client_uri: None,
        contacts: None,
        id_token_signed_response_alg: None,
        token_endpoint_auth_method: Some("none".to_string()),
        token_endpoint_auth_signing_alg: None,
        post_logout_redirect_uri: None,
        backchannel_logout_uri: None,
    };

    // a fragment, a reserved query key or a comma is rejected - before it would consume the rate limit
    for bad_uri in BAD_REDIRECT_URIS {
        payload.redirect_uris = vec![bad_uri.to_string()];
        let res = client.post(&url).json(&payload).send().await?;
        assert_eq!(res.status(), 400, "{bad_uri}");
        let body = res.text().await?;
        assert!(body.contains(expected_redirect_uri_err(bad_uri)), "{body}");
    }
    payload.redirect_uris = vec![GOOD_REDIRECT_URI.to_string()];

    // the same for a post_logout_redirect_uri with a fragment or a `state` query key
    for bad_uri in BAD_POST_LOGOUT_REDIRECT_URIS {
        payload.post_logout_redirect_uri = Some(bad_uri.to_string());
        let res = client.post(&url).json(&payload).send().await?;
        assert_eq!(res.status(), 400, "{bad_uri}");
        let body = res.text().await?;
        assert!(body.contains(expected_redirect_uri_err(bad_uri)), "{body}");
    }
    payload.post_logout_redirect_uri = None;

    let res = client.post(&url).json(&payload).send().await?;
    assert_eq!(res.status(), 201);
    let resp = res.json::<DynamicClientResponse>().await?;
    assert_eq!(resp.client_name, payload.client_name);
    // currently, we don't have a secret expiration
    assert_eq!(resp.client_secret_expires_at, 0);
    assert!(resp.grant_types.contains(&GrantType::AuthorizationCode));
    assert!(resp.grant_types.contains(&GrantType::RefreshToken));
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

    // self-modify with a fragment, a reserved query key or a comma is rejected and changes nothing
    for bad_uri in BAD_REDIRECT_URIS {
        payload.redirect_uris = vec![bad_uri.to_string()];
        let res = client
            .put(&url)
            .header(AUTHORIZATION, &token)
            .json(&payload)
            .send()
            .await?;
        assert_eq!(res.status(), 400, "{bad_uri}");
    }
    payload.redirect_uris = vec![GOOD_REDIRECT_URI.to_string()];
    for bad_uri in BAD_POST_LOGOUT_REDIRECT_URIS {
        payload.post_logout_redirect_uri = Some(bad_uri.to_string());
        let res = client
            .put(&url)
            .header(AUTHORIZATION, &token)
            .json(&payload)
            .send()
            .await?;
        assert_eq!(res.status(), 400, "{bad_uri}");
        let body = res.text().await?;
        assert!(body.contains(expected_redirect_uri_err(bad_uri)), "{body}");
    }
    payload.post_logout_redirect_uri = None;
    let res = client
        .get(&url)
        .header(AUTHORIZATION, &token)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<DynamicClientResponse>().await?, resp_get);

    // a self-update must not add a grant type the client does not have
    let grant_types = payload.grant_types.clone();
    payload.grant_types.push(GrantType::ClientCredentials);
    let res = client
        .put(&url)
        .header(AUTHORIZATION, &token)
        .json(&payload)
        .send()
        .await?;
    payload.grant_types = grant_types;
    assert_eq!(res.status(), 400);
    let err = res.json::<serde_json::Value>().await?;
    assert_eq!(err["error"], "invalid_client_metadata");
    let after = admin_get_client(&resp.client_id).await?;
    assert_eq!(after.flows_enabled, resp.grant_types);
    assert!(!after.flows_enabled.contains(&GrantType::ClientCredentials));

    // self-modify
    payload.client_name = Some("Dyn Test Client 12345".to_string());
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
    assert!(resp.grant_types.contains(&GrantType::AuthorizationCode));
    assert!(resp.grant_types.contains(&GrantType::RefreshToken));
    assert!(!resp.grant_types.contains(&GrantType::ClientCredentials));
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

    // values only an admin can set must survive a self-update
    let client_id = resp.client_id.clone();
    admin_update_client(&client_id, |req| {
        req.auth_code_lifetime = 17;
        req.access_token_lifetime = 42;
        req.force_mfa = true;
        req.restrict_group_prefix = Some("dyn_test".to_string());
        req.claims = Some(serde_json::json!({ "tenant": "dyn" }));
        req.default_aud = Some(vec!["https://aud.dyn.rauthy.io".to_string()]);
    })
    .await?;
    let before = admin_get_client(&client_id).await?;

    let token = format!(
        "Bearer {}",
        resp.registration_access_token.as_ref().unwrap()
    );

    // With the admin's `default_aud` and `claims` kept, adding `client_credentials` would let
    // the client mint machine tokens carrying them -> rejected, the flows stay unchanged.
    payload.grant_types = vec![GrantType::ClientCredentials];
    let res = client
        .put(&url)
        .header(AUTHORIZATION, &token)
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), 400);
    let err = res.json::<serde_json::Value>().await?;
    assert_eq!(err["error"], "invalid_client_metadata");
    let after = admin_get_client(&client_id).await?;
    assert_eq!(after.flows_enabled, before.flows_enabled);
    assert!(!after.flows_enabled.contains(&GrantType::ClientCredentials));

    // narrowing the grant types is allowed
    payload.grant_types = vec![GrantType::AuthorizationCode];
    payload.client_name = Some("Dyn Test Client 1234567".to_string());
    let res = client
        .put(&url)
        .header(AUTHORIZATION, &token)
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let resp = res.json::<DynamicClientResponse>().await?;
    assert_eq!(resp.client_name, payload.client_name);

    let after = admin_get_client(&client_id).await?;
    assert_eq!(after.name, payload.client_name);
    assert_eq!(after.flows_enabled, vec![GrantType::AuthorizationCode]);
    assert!(after.enabled);
    assert_eq!(after.auth_code_lifetime, 17);
    assert_eq!(after.access_token_lifetime, 42);
    assert!(after.force_mfa);
    assert_eq!(after.restrict_group_prefix.as_deref(), Some("dyn_test"));
    assert_eq!(after.claims, before.claims);
    assert!(after.claims.is_some());
    assert_eq!(after.default_aud, before.default_aud);
    assert_eq!(after.scopes, before.scopes);
    assert_eq!(after.default_scopes, before.default_scopes);

    // an admin-disabled client must not be able to modify (and re-enable) itself
    admin_update_client(&client_id, |req| req.enabled = false).await?;
    let token = format!(
        "Bearer {}",
        resp.registration_access_token.as_ref().unwrap()
    );
    payload.client_name = Some("Dyn Test Client re-enabled".to_string());
    let res = client
        .put(&url)
        .header(AUTHORIZATION, &token)
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), 403);

    let after = admin_get_client(&client_id).await?;
    assert!(!after.enabled);
    assert_eq!(after.name.as_deref(), Some("Dyn Test Client 1234567"));
    assert_eq!(after.access_token_lifetime, 42);

    Ok(())
}
