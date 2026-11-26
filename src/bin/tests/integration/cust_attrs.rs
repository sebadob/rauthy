use crate::common::{
    TestClient, TestClientSetup, USER_PASSWORD, create_login_user, create_test_client,
    delete_client, delete_user, get_auth_headers, get_backend_url, get_client,
    password_token_set_for, put_client, update_request,
};
use crate::{expect_status, expect_status_json};
use pretty_assertions::assert_eq;
use rauthy_api_types::clients::UpdateClientRequest;
use rauthy_api_types::oidc::GrantType;
use rauthy_api_types::scopes::{ScopeRequest, ScopeResponse};
use rauthy_api_types::users::{
    UserAttrConfigRequest, UserAttrConfigResponse, UserAttrValueRequest, UserAttrValuesResponse,
    UserAttrValuesUpdateRequest,
};
use rauthy_common::utils::{base64_url_no_pad_decode, new_store_id};
use rauthy_data::entity::user_attr::UserAttrConfigEntity;
use rauthy_jwt::claims::JwtAccessClaims;
use reqwest::header::HeaderMap;
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

const TEST_VALUE: &str = "Some String";

/// Everything the custom attribute tests need: an attribute, a scope it is mapped to, a client
/// that carries that scope, and the user the attribute value is set on. Each run of the test
/// creates its own set with unique names and removes all of it again.
struct CustAttrs {
    admin: HeaderMap,
    attr: String,
    attr_renamed: String,
    scope_name: String,
    scope_id: String,
    client: TestClient,
    user_id: String,
    user_email: String,
}

impl CustAttrs {
    async fn new() -> Result<Self, Box<dyn Error>> {
        let admin = get_auth_headers().await?;
        let http = reqwest::Client::new();
        let backend_url = get_backend_url();
        let suffix = new_store_id().to_lowercase();

        // the attribute and the scope it gets mapped to
        let attr = format!("cust_{suffix}");
        let attr_renamed = format!("{attr}2");
        let scope_name = format!("scope_{suffix}");
        let res = http
            .post(format!("{backend_url}/users/attr"))
            .headers(admin.clone())
            .json(&UserAttrConfigRequest {
                name: attr.clone(),
                desc: Some("some description".to_string()),
                default_value: None,
                typ: None,
                user_editable: None,
            })
            .send()
            .await?;
        let created = expect_status_json!(res, 200, UserAttrConfigEntity);
        assert_eq!(created.name, attr);

        let res = http
            .post(format!("{backend_url}/scopes"))
            .headers(admin.clone())
            .json(&ScopeRequest {
                scope: scope_name.clone(),
                attr_include_access: None,
                attr_include_id: None,
                claims_at_root: false,
            })
            .send()
            .await?;
        let scope = expect_status_json!(res, 200, ScopeResponse);
        assert_eq!(scope.name, scope_name);

        // the client the token is issued for, with that scope in its scopes
        let setup = TestClientSetup {
            name: format!("Cust Attrs Client {suffix}"),
            flows: vec![GrantType::Password, GrantType::RefreshToken],
            redirect_uri: "http://localhost:3000/oidc/callback".to_string(),
            post_logout_redirect_uris: Vec::new(),
            backchannel_logout_uri: None,
        };
        let client = create_test_client(&setup).await?;
        let c = get_client(&admin, &client.id).await?;
        let mut scopes = c.scopes.clone();
        scopes.push(scope_name.clone());
        let mut default_scopes = c.default_scopes.clone();
        default_scopes.push(scope_name.clone());
        let req = UpdateClientRequest {
            scopes,
            default_scopes,
            ..update_request(c)
        };
        let c = put_client(&http, &admin, &client.id, &req).await?;
        assert!(c.scopes.iter().any(|i| i == &scope_name));
        assert!(c.default_scopes.iter().any(|i| i == &scope_name));

        // the user the attribute value is set on
        let user_email = format!("cust-attr-{suffix}@localhost");
        let user =
            create_login_user(&http, &admin, &user_email, "Cust Attrs", USER_PASSWORD).await?;

        Ok(Self {
            admin,
            attr,
            attr_renamed,
            scope_name,
            scope_id: scope.id,
            client,
            user_id: user.id,
            user_email,
        })
    }

    /// The decoded access token payload of a fresh token for the test user on the test client.
    async fn claim_bytes(&self) -> Result<Vec<u8>, Box<dyn Error>> {
        let ts = password_token_set_for(
            &reqwest::Client::new(),
            &self.client,
            &self.user_email,
            USER_PASSWORD,
        )
        .await;
        Ok(extract_raw_claims(&ts.access_token))
    }

    /// The current `custom` claims of the test user's access token.
    async fn custom_claims(&self) -> Result<Option<HashMap<String, Value>>, Box<dyn Error>> {
        let bytes = self.claim_bytes().await?;
        let claims = serde_json::from_slice::<JwtAccessClaims>(&bytes)?;
        Ok(claims.custom)
    }

    /// The stored mapping of this test's scope.
    async fn scope(&self) -> Result<ScopeResponse, Box<dyn Error>> {
        let res = reqwest::Client::new()
            .get(format!("{}/scopes", get_backend_url()))
            .headers(self.admin.clone())
            .send()
            .await?;
        let scopes = expect_status_json!(res, 200, Vec<ScopeResponse>);
        let scope = scopes
            .into_iter()
            .find(|s| s.id == self.scope_id)
            .ok_or("the test scope to exist")?;
        Ok(scope)
    }

    /// PUTs this test's scope with the given attribute mapping.
    async fn put_scope(&self, req: &ScopeRequest) -> Result<ScopeResponse, Box<dyn Error>> {
        let res = reqwest::Client::new()
            .put(format!("{}/scopes/{}", get_backend_url(), self.scope_id))
            .headers(self.admin.clone())
            .json(req)
            .send()
            .await?;
        Ok(expect_status_json!(res, 200, ScopeResponse))
    }

    /// The attribute values stored for the test user.
    async fn user_attrs(&self) -> Result<UserAttrValuesResponse, Box<dyn Error>> {
        let res = reqwest::Client::new()
            .get(format!("{}/users/{}/attr", get_backend_url(), self.user_id))
            .headers(self.admin.clone())
            .send()
            .await?;
        Ok(expect_status_json!(res, 200, UserAttrValuesResponse))
    }

    async fn teardown(&self) -> Result<(), Box<dyn Error>> {
        let http = reqwest::Client::new();
        let res = http
            .delete(format!("{}/scopes/{}", get_backend_url(), self.scope_id))
            .headers(self.admin.clone())
            .send()
            .await?;
        expect_status!(res, 200, format!("delete scope {}", self.scope_name));
        delete_client(&self.admin, &self.client.id).await?;
        delete_user(&self.admin, &self.user_email).await
    }
}

fn extract_raw_claims(token: &str) -> Vec<u8> {
    let mut split = token.split('.');
    split.next().unwrap();
    let claims = split.next().unwrap();
    split.next().unwrap();
    assert!(split.next().is_none());
    base64_url_no_pad_decode(claims).unwrap()
}

#[tokio::test]
async fn test_cust_attrs() -> Result<(), Box<dyn Error>> {
    let s = CustAttrs::new().await?;
    let http = reqwest::Client::new();
    let backend_url = get_backend_url();
    let url_attrs = format!("{backend_url}/users/attr");

    // add a new attr
    let res = http.get(&url_attrs).headers(s.admin.clone()).send().await?;
    let _ = expect_status_json!(res, 200, UserAttrConfigResponse);

    // map the cust attr to the scope
    let req = ScopeRequest {
        scope: s.scope_name.clone(),
        attr_include_access: Some(vec![s.attr.clone()]),
        attr_include_id: Some(vec![s.attr.clone()]),
        claims_at_root: false,
    };
    let scope_mapped = s.put_scope(&req).await?;
    assert_eq!(scope_mapped.id, s.scope_id);
    assert_eq!(scope_mapped.name, s.scope_name);
    assert_eq!(scope_mapped.attr_include_access, Some(vec![s.attr.clone()]));
    assert_eq!(scope_mapped.attr_include_id, Some(vec![s.attr.clone()]));

    // add a value for the new attr to the user
    let test_val = Value::from(TEST_VALUE);
    let req = UserAttrValuesUpdateRequest {
        values: vec![UserAttrValueRequest {
            key: s.attr.clone(),
            value: test_val.clone(),
        }],
    };
    let url_user_attr = format!("{backend_url}/users/{}/attr", s.user_id);
    let res = http
        .put(&url_user_attr)
        .headers(s.admin.clone())
        .json(&req)
        .send()
        .await?;
    let resp = expect_status_json!(res, 200, UserAttrValuesResponse);
    assert_eq!(resp.values.len(), 1);
    let user_attr = resp.values.first().unwrap();
    assert_eq!(user_attr.key, s.attr);
    assert_eq!(user_attr.value, test_val);

    // fetch a new token and check the scope mapping
    let cust_claims = s
        .custom_claims()
        .await?
        .expect("custom claims from the mapped attribute");
    assert_eq!(cust_claims.get(&s.attr).unwrap(), &Value::from(TEST_VALUE));

    // === issue #1595: per-scope `claims_at_root` ===
    // Flip the scope so its mapped attribute is emitted at the token root (flattened)
    // instead of nested under `custom`, then assert the change end-to-end.
    let req = ScopeRequest {
        scope: s.scope_name.clone(),
        attr_include_access: Some(vec![s.attr.clone()]),
        attr_include_id: Some(vec![s.attr.clone()]),
        claims_at_root: true,
    };
    let scope_root = s.put_scope(&req).await?;
    assert!(scope_root.claims_at_root);

    // The mapped attribute must now be a top-level claim and must NOT appear nested
    // under `custom` (the only custom mapping on this token is root-promoted).
    let raw = serde_json::from_slice::<Value>(&s.claim_bytes().await?)?;
    assert_eq!(raw.get(&s.attr).unwrap(), &Value::from(TEST_VALUE));
    assert!(
        raw.get("custom").is_none(),
        "no nested `custom` object expected when the only mapping is root-promoted"
    );
    // It must also round-trip through the typed claims into `custom_flattened`.
    let bytes = s.claim_bytes().await?;
    let claims = serde_json::from_slice::<JwtAccessClaims>(&bytes)?;
    assert!(claims.custom.is_none());
    let flat = claims
        .custom_flattened
        .expect("root-promoted attribute must be present in custom_flattened");
    assert_eq!(flat.get(&s.attr).unwrap(), &Value::from(TEST_VALUE));

    // Restore nested behavior so the remainder of the test observes `custom.*` again.
    let req = ScopeRequest {
        scope: s.scope_name.clone(),
        attr_include_access: Some(vec![s.attr.clone()]),
        attr_include_id: Some(vec![s.attr.clone()]),
        claims_at_root: false,
    };
    s.put_scope(&req).await?;

    // modify the custom attr and change its name
    let cust_attr_mod = UserAttrConfigRequest {
        name: s.attr_renamed.clone(),
        desc: Some("some description 2".to_string()),
        default_value: None,
        typ: None,
        user_editable: None,
    };
    let url_attr_mod = format!("{backend_url}/users/attr/{}", s.attr);
    let res = http
        .put(&url_attr_mod)
        .headers(s.admin.clone())
        .json(&cust_attr_mod)
        .send()
        .await?;

    let attr = expect_status_json!(res, 200, UserAttrConfigEntity);
    assert_eq!(attr.name, cust_attr_mod.name);
    assert_eq!(attr.desc, cust_attr_mod.desc);

    // make sure the new name has been changed in the scope mapping
    let scope_mapped = s.scope().await?;
    assert_eq!(
        scope_mapped.attr_include_access,
        Some(vec![s.attr_renamed.clone()])
    );
    assert_eq!(
        scope_mapped.attr_include_id,
        Some(vec![s.attr_renamed.clone()])
    );

    // make sure the new name has been changed for the user
    let resp = s.user_attrs().await?;
    assert_eq!(resp.values.len(), 1);
    let user_attr_mod = resp.values.first().unwrap();
    assert_eq!(user_attr_mod.key, s.attr_renamed);
    // the value must be the same
    assert_eq!(user_attr_mod.value, user_attr.value);

    // fetch a new token and check the scope mapping
    let cust_claims = s
        .custom_claims()
        .await?
        .expect("custom claims from the renamed attribute");
    assert_eq!(
        cust_claims.get(&s.attr_renamed).unwrap(),
        &Value::from(TEST_VALUE)
    );

    // delete the custom attr and make sure its properly cleaned up everywhere
    let url_attr_del = format!("{backend_url}/users/attr/{}", s.attr_renamed);
    let res = http
        .delete(&url_attr_del)
        .headers(s.admin.clone())
        .send()
        .await?;
    expect_status!(res, 200);

    // scope mapping should have been cleaned up
    let scope_mapped = s.scope().await?;
    assert_eq!(scope_mapped.attr_include_access, None);
    assert_eq!(scope_mapped.attr_include_id, None);

    // user attr value should be gone
    let resp = s.user_attrs().await?;
    assert!(resp.values.is_empty());

    // no custom token mapping anymore
    assert!(s.custom_claims().await?.is_none());

    s.teardown().await?;

    Ok(())
}
