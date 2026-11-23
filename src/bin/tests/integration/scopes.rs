use crate::common::{
    TestClientSetup, create_test_client, delete_test_client, get_auth_headers, get_backend_url,
    init_client_bcl_uri,
};
use crate::expect_status;
use crate::expect_status_json;
use pretty_assertions::assert_eq;
use rauthy_api_types::clients::{ClientResponse, UpdateClientRequest};
use rauthy_api_types::oidc::GrantType;
use rauthy_api_types::scopes::{ScopeRequest, ScopeResponse};
use rauthy_common::utils::new_store_id;
use std::error::Error;

fn setup() -> TestClientSetup {
    TestClientSetup {
        name: "Scopes Test".to_string(),
        flows: vec![GrantType::Password],
        redirect_uri: "http://localhost:8080/callback".to_string(),
        post_logout_redirect_uris: Vec::new(),
        backchannel_logout_uri: Some(init_client_bcl_uri()),
    }
}

/// The `scope` name validation accepts lowercase alphanumerics only, which [`new_store_id`] mixes.
fn scope_name(prefix: &str) -> String {
    format!("{}{}", prefix, new_store_id().to_lowercase())
}

/// Scopes are global, so this test asserts membership: every scope that existed before it is
/// there afterwards, and only the ones it created are gone.
#[tokio::test]
async fn test_scopes() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();
    let test_client = create_test_client(&setup()).await?;
    let url_client = format!("{backend_url}/clients/{}", test_client.id);

    let url = format!("{backend_url}/scopes");
    let res = client
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    let res = expect_status!(res, 200);

    let scopes = res.json::<Vec<ScopeResponse>>().await?;
    let scopes_orig = scopes.iter().map(|s| s.id.clone()).collect::<Vec<_>>();

    // add a scope
    let new_scope = ScopeRequest {
        scope: scope_name("scope"),
        attr_include_access: None,
        attr_include_id: None,
        claims_at_root: false,
    };
    let res = client
        .post(&url)
        .headers(auth_headers.clone())
        .json(&new_scope)
        .send()
        .await?;
    let res = expect_status!(res, 200);

    let scope = res.json::<ScopeResponse>().await?;
    assert_eq!(scope.name, new_scope.scope);

    // link the scope to a client and check
    let res = client
        .get(&url_client)
        .headers(auth_headers.clone())
        .send()
        .await?;
    let linked_client = expect_status_json!(res, 200, ClientResponse);
    assert!(!linked_client.scopes.contains(&new_scope.scope));
    assert!(!linked_client.default_scopes.contains(&new_scope.scope));

    let mut scopes = linked_client.scopes;
    scopes.push(new_scope.scope.clone());
    let mut default_scopes = linked_client.default_scopes;
    default_scopes.push(new_scope.scope.clone());

    let update_client = UpdateClientRequest {
        name: linked_client.name,
        confidential: linked_client.confidential,
        redirect_uris: linked_client.redirect_uris,
        post_logout_redirect_uris: linked_client.post_logout_redirect_uris,
        allowed_origins: Some(vec!["http://localhost:8080".to_string()]),
        enabled: linked_client.enabled,
        flows_enabled: linked_client.flows_enabled,
        access_token_alg: linked_client.access_token_alg,
        id_token_alg: linked_client.id_token_alg,
        auth_code_lifetime: linked_client.auth_code_lifetime,
        access_token_lifetime: linked_client.access_token_lifetime,
        scopes,
        default_scopes,
        challenges: linked_client.challenges,
        force_mfa: linked_client.force_mfa,
        client_uri: linked_client.client_uri,
        contacts: linked_client.contacts,
        backchannel_logout_uri: Some(init_client_bcl_uri()),
        restrict_group_prefix: None,
        claims: None,
        claims_at_root: false,
        allowed_resources: None,
        default_aud: None,
        consent: None,
        actor_same_client: false,
        scim: None,
    };
    let res = client
        .put(&url_client)
        .headers(auth_headers.clone())
        .json(&update_client)
        .send()
        .await?;
    let linked_client = expect_status_json!(res, 200, ClientResponse);
    assert!(linked_client.scopes.contains(&new_scope.scope));
    assert!(linked_client.default_scopes.contains(&new_scope.scope));

    // modify the scope
    let upd_scope = ScopeRequest {
        scope: scope_name("scopeupd"),
        attr_include_access: None,
        attr_include_id: None,
        claims_at_root: false,
    };
    let url_name = format!("{url}/{}", scope.id);
    let res = client
        .put(&url_name)
        .headers(auth_headers.clone())
        .json(&upd_scope)
        .send()
        .await?;
    let res = expect_status!(res, 200);

    let upd_scp = res.json::<ScopeResponse>().await?;
    assert_eq!(upd_scope.scope, upd_scp.name);

    // check the linked client update
    let res = client
        .get(&url_client)
        .headers(auth_headers.clone())
        .send()
        .await?;
    let linked_client = expect_status_json!(res, 200, ClientResponse);
    assert!(linked_client.scopes.contains(&upd_scp.name));
    assert!(linked_client.default_scopes.contains(&upd_scp.name));

    // delete the scope
    let url_del = format!("{url}/{}", upd_scp.id);
    let res = client
        .delete(&url_del)
        .headers(auth_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);

    // verify it is gone
    let res = client
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    let res = expect_status!(res, 200);

    let scopes = res.json::<Vec<ScopeResponse>>().await?;
    let ids = scopes.iter().map(|s| s.id.as_str()).collect::<Vec<_>>();
    for id in &scopes_orig {
        assert!(
            ids.contains(&id.as_str()),
            "the pre-existing scope {id} disappeared"
        );
    }
    assert!(
        !ids.contains(&scope.id.as_str()) && !ids.contains(&upd_scp.id.as_str()),
        "a scope this test created was not deleted"
    );

    // check the linked client deletion
    let res = client
        .get(&url_client)
        .headers(auth_headers.clone())
        .send()
        .await?;
    let linked_client = expect_status_json!(res, 200, ClientResponse);
    assert!(!linked_client.scopes.contains(&upd_scp.name));
    assert!(!linked_client.default_scopes.contains(&upd_scp.name));

    delete_test_client(&test_client.id).await?;

    Ok(())
}
