use crate::common::{
    extract_token_claims_unverified, get_auth_headers, get_backend_url, get_token_set,
};
use rauthy_api_types::clients::{ClientResponse, UpdateClientRequest};
use rauthy_api_types::oidc::JwkKeyPairAlg;
use rauthy_api_types::scopes::{ScopeRequest, ScopeResponse};
use rauthy_api_types::users::{
    UserAttrConfigRequest, UserAttrConfigResponse, UserAttrValueRequest, UserAttrValuesResponse,
    UserAttrValuesUpdateRequest,
};
use rauthy_models::JwtAccessClaims;
use rauthy_models::entity::user_attr::UserAttrConfigEntity;
use serde_json::Value;
use std::error::Error;

mod common;

#[tokio::test]
async fn test_cust_attrs() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();
    let client = reqwest::Client::new();

    // make sure there are currently no cust attrs defined
    let url_attrs = format!("{}/users/attr", backend_url);
    let res = client
        .get(&url_attrs)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let attrs = res.json::<UserAttrConfigResponse>().await?;
    assert!(attrs.values.is_empty());

    // add a new attr
    let cust_attr = UserAttrConfigRequest {
        name: "cust1".to_string(),
        desc: Some("some description".to_string()),
    };
    let res = client
        .post(&url_attrs)
        .headers(auth_headers.clone())
        .json(&cust_attr)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let attr = res.json::<UserAttrConfigEntity>().await?;
    assert_eq!(attr.name, "cust1");
    assert_eq!(attr.desc, Some("some description".to_string()));

    // add a new custom scope to map the attr to (no mapping)
    let req = ScopeRequest {
        scope: "cust_scope".to_string(),
        attr_include_access: None,
        attr_include_id: None,
    };
    let url_scopes = format!("{}/scopes", backend_url);
    let res = client
        .post(&url_scopes)
        .headers(auth_headers.clone())
        .json(&req)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let scope = res.json::<ScopeResponse>().await?;
    assert_eq!(scope.name, "cust_scope");
    assert_eq!(scope.attr_include_access, None);
    assert_eq!(scope.attr_include_id, None);

    // map the cust attr to the scope
    let req = ScopeRequest {
        scope: "cust_scope".to_string(),
        attr_include_access: Some(vec!["cust1".to_string()]),
        attr_include_id: Some(vec!["cust1".to_string()]),
    };
    let url_scope = format!("{}/{}", url_scopes, scope.id);
    let res = client
        .put(&url_scope)
        .headers(auth_headers.clone())
        .json(&req)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let scope_mapped = res.json::<ScopeResponse>().await?;
    assert_eq!(scope.id, scope_mapped.id);
    assert_eq!(scope.name, scope_mapped.name);
    assert_eq!(
        scope_mapped.attr_include_access,
        Some(vec!["cust1".to_string()])
    );
    assert_eq!(
        scope_mapped.attr_include_id,
        Some(vec!["cust1".to_string()])
    );

    // get the current client config and then add a default mapping
    // we do these changes for the rauthy client to be able to re-use the original login data
    let url_client = format!("{}/clients/rauthy", backend_url);
    let res = client
        .get(&url_client)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let c = res.json::<ClientResponse>().await?;
    assert_eq!(c.id, "rauthy".to_string());
    let mut scopes = c.scopes;
    scopes.push("cust_scope".to_string());
    let mut default_scopes = c.default_scopes;
    default_scopes.push("cust_scope".to_string());
    let client_req = UpdateClientRequest {
        id: c.id,
        name: c.name,
        confidential: c.confidential,
        redirect_uris: c.redirect_uris,
        post_logout_redirect_uris: c.post_logout_redirect_uris,
        allowed_origins: c.allowed_origins,
        enabled: c.enabled,
        flows_enabled: c.flows_enabled,
        access_token_alg: JwkKeyPairAlg::from(c.access_token_alg),
        id_token_alg: JwkKeyPairAlg::from(c.id_token_alg),
        auth_code_lifetime: c.auth_code_lifetime,
        access_token_lifetime: c.access_token_lifetime,
        scopes,
        default_scopes,
        challenges: c.challenges,
        force_mfa: c.force_mfa,
        client_uri: None,
        contacts: None,
        backchannel_logout_uri: None,
    };
    let res = client
        .put(&url_client)
        .headers(auth_headers.clone())
        .json(&client_req)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let c = res.json::<ClientResponse>().await?;
    assert_eq!(c.id, "rauthy".to_string());
    assert!(c.scopes.contains(&"cust_scope".to_string()));
    assert!(c.default_scopes.contains(&"cust_scope".to_string()));

    // add a value for the new attr to a user
    let test_val = serde_json::Value::from("Some String");
    let req = UserAttrValuesUpdateRequest {
        values: vec![UserAttrValueRequest {
            key: cust_attr.name.clone(),
            value: test_val.clone(),
        }],
    };
    // init_admin id: m4PJ3TnyP32LA8hzY23deme3
    let url_user_attr = format!("{}/users/m4PJ3TnyP32LA8hzY23deme3/attr", backend_url);
    let res = client
        .put(&url_user_attr)
        .headers(auth_headers.clone())
        .json(&req)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let resp = res.json::<UserAttrValuesResponse>().await?;
    assert_eq!(resp.values.len(), 1);
    let user_attr = resp.values.get(0).unwrap();
    assert_eq!(user_attr.key, cust_attr.name);
    assert_eq!(user_attr.value, test_val);

    // fetch a new token and check the scope mapping
    let token = get_token_set().await;
    let claims = extract_token_claims_unverified::<JwtAccessClaims>(&token.access_token).unwrap();
    assert!(claims.custom.is_some());
    let cust_claims = claims.custom.unwrap();
    assert_eq!(
        cust_claims.get("cust1").unwrap(),
        &Value::String("Some String".to_string())
    );

    // modify the custom attr and change its name
    let cust_attr_mod = UserAttrConfigRequest {
        name: "cust2".to_string(),
        desc: Some("some description 2".to_string()),
    };
    let url_attr_mod = format!("{}/users/attr/{}", backend_url, cust_attr.name);
    let res = client
        .put(&url_attr_mod)
        .headers(auth_headers.clone())
        .json(&cust_attr_mod)
        .send()
        .await?;

    assert_eq!(res.status(), 200);
    let attr = res.json::<UserAttrConfigEntity>().await?;
    assert_eq!(attr.name, cust_attr_mod.name);
    assert_eq!(attr.desc, cust_attr_mod.desc);

    // make sure the new name has been changed in the scope mapping
    let res = client
        .get(&url_scopes)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let scopes = res.json::<Vec<ScopeResponse>>().await?;
    let scopes_reduced = scopes
        .into_iter()
        .filter(|s| s.id == scope.id)
        .take(1)
        .collect::<Vec<ScopeResponse>>();
    let scope_mapped = scopes_reduced.get(0).unwrap();
    assert_eq!(
        scope_mapped.attr_include_access,
        Some(vec!["cust2".to_string()])
    );
    assert_eq!(
        scope_mapped.attr_include_id,
        Some(vec!["cust2".to_string()])
    );

    // make sure the new name has been changed for the user
    let res = client
        .get(&url_user_attr)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let resp = res.json::<UserAttrValuesResponse>().await?;
    assert_eq!(resp.values.len(), 1);
    let user_attr_mod = resp.values.get(0).unwrap();
    assert_eq!(&user_attr_mod.key, "cust2");
    // the value must be the same
    assert_eq!(user_attr_mod.value, user_attr.value);

    // fetch a new token and check the scope mapping
    let token = get_token_set().await;
    let claims = extract_token_claims_unverified::<JwtAccessClaims>(&token.access_token).unwrap();
    assert!(claims.custom.is_some());
    let cust_claims = claims.custom.unwrap();
    assert_eq!(
        cust_claims.get("cust2").unwrap(),
        &Value::String("Some String".to_string())
    );

    // delete the custom attr and make sure its properly cleaned up everywhere
    // add a new attr
    let url_attr_del = format!("{}/users/attr/cust2", backend_url);
    let res = client
        .delete(&url_attr_del)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // scope mapping should have been cleaned up
    let res = client
        .get(&url_scopes)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let scopes = res.json::<Vec<ScopeResponse>>().await?;
    let scopes_reduced = scopes
        .into_iter()
        .filter(|s| s.id == scope.id)
        .take(1)
        .collect::<Vec<ScopeResponse>>();
    let scope_mapped = scopes_reduced.get(0).unwrap();
    assert_eq!(scope_mapped.attr_include_access, None);
    assert_eq!(scope_mapped.attr_include_id, None);

    // user attr value should be gone
    let res = client
        .get(&url_user_attr)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let resp = res.json::<UserAttrValuesResponse>().await?;
    assert!(resp.values.is_empty());

    // no custom token mapping anymore
    let token = get_token_set().await;
    let claims = extract_token_claims_unverified::<JwtAccessClaims>(&token.access_token).unwrap();
    assert!(claims.custom.is_none());

    Ok(())
}
