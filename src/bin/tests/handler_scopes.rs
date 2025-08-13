use crate::common::{get_auth_headers, get_backend_url, init_client_bcl_uri};
use pretty_assertions::assert_eq;
use rauthy_api_types::clients::{ClientResponse, UpdateClientRequest};
use rauthy_api_types::scopes::ScopeRequest;
use rauthy_data::entity::scopes::Scope;
use std::error::Error;

mod common;

#[tokio::test]
async fn test_scopes() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();

    let url = format!("{}/scopes", backend_url);
    let res = client
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let scopes = res.json::<Vec<Scope>>().await?;
    assert_eq!(scopes.len(), 6);

    // add a scope
    let new_scope = ScopeRequest {
        scope: "scope123".to_string(),
        attr_include_access: None,
        attr_include_id: None,
    };
    let res = client
        .post(&url)
        .headers(auth_headers.clone())
        .json(&new_scope)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let scope = res.json::<Scope>().await?;
    assert_eq!(scope.name, new_scope.scope);

    // link the scope to a client and check
    let url_client = format!("{}/clients/init_client", backend_url);
    let res = client
        .get(&url_client)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let init_client = res.json::<ClientResponse>().await?;
    assert!(!init_client.scopes.contains(&new_scope.scope));
    assert!(!init_client.default_scopes.contains(&new_scope.scope));

    let mut scopes = init_client.scopes;
    scopes.push(new_scope.scope.clone());
    let mut default_scopes = init_client.default_scopes;
    default_scopes.push(new_scope.scope.clone());

    let update_client = UpdateClientRequest {
        id: init_client.id,
        name: init_client.name,
        confidential: init_client.confidential,
        redirect_uris: init_client.redirect_uris,
        post_logout_redirect_uris: init_client.post_logout_redirect_uris,
        allowed_origins: Some(vec!["http://localhost:8080".to_string()]),
        enabled: init_client.enabled,
        flows_enabled: init_client.flows_enabled,
        access_token_alg: init_client.access_token_alg,
        id_token_alg: init_client.id_token_alg,
        auth_code_lifetime: init_client.auth_code_lifetime,
        access_token_lifetime: init_client.access_token_lifetime,
        scopes,
        default_scopes,
        challenges: init_client.challenges,
        force_mfa: init_client.force_mfa,
        client_uri: init_client.client_uri,
        contacts: init_client.contacts,
        backchannel_logout_uri: Some(init_client_bcl_uri()),
        restrict_group_prefix: None,
        scim: None,
    };
    let res = client
        .put(&url_client)
        .headers(auth_headers.clone())
        .json(&update_client)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let init_client = res.json::<ClientResponse>().await?;
    assert!(init_client.scopes.contains(&new_scope.scope));
    assert!(init_client.default_scopes.contains(&new_scope.scope));

    // modify the scope
    let upd_scope = ScopeRequest {
        scope: "scope456".to_string(),
        attr_include_access: None,
        attr_include_id: None,
    };
    let url_name = format!("{}/{}", url, scope.id);
    let res = client
        .put(&url_name)
        .headers(auth_headers.clone())
        .json(&upd_scope)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let upd_scp = res.json::<Scope>().await?;
    assert_eq!(upd_scope.scope, upd_scp.name);

    // check the linked client update
    let res = client
        .get(&url_client)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let init_client = res.json::<ClientResponse>().await?;
    assert!(init_client.scopes.contains(&upd_scp.name));
    assert!(init_client.default_scopes.contains(&upd_scp.name));

    // delete the scope
    let url_del = format!("{}/{}", url, upd_scp.id);
    let res = client
        .delete(&url_del)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // verify it is gone
    let res = client
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let scopes = res.json::<Vec<Scope>>().await?;
    assert_eq!(scopes.len(), 6);

    // check the linked client deletion
    let res = client
        .get(&url_client)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let init_client = res.json::<ClientResponse>().await?;
    assert!(!init_client.scopes.contains(&upd_scp.name));
    assert!(!init_client.default_scopes.contains(&upd_scp.name));

    Ok(())
}
