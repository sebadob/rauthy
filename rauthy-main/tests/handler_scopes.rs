use crate::common::{get_auth_headers, get_backend_url};
use pretty_assertions::assert_eq;
use rauthy_models::entity::scopes::Scope;
use rauthy_models::request::ScopeRequest;
use std::error::Error;

mod common;

#[tokio::test]
async fn test_scopes() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();

    let url = format!("{}/scopes", backend_url);
    let res = reqwest::Client::new()
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let scopes = res.json::<Vec<Scope>>().await?;
    assert_eq!(scopes.len(), 4);

    // add a scope
    let new_scope = ScopeRequest {
        scope: "scope123".to_string(),
        attr_include_access: None,
        attr_include_id: None,
    };
    let res = reqwest::Client::new()
        .post(&url)
        .headers(auth_headers.clone())
        .json(&new_scope)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let scope = res.json::<Scope>().await?;
    assert_eq!(scope.name, "scope123");

    // modify the scope
    let upd_scope = ScopeRequest {
        scope: "scope456".to_string(),
        attr_include_access: None,
        attr_include_id: None,
    };
    let url_name = format!("{}/{}", url, scope.id);
    let res = reqwest::Client::new()
        .put(&url_name)
        .headers(auth_headers.clone())
        .json(&upd_scope)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let upd_scp = res.json::<Scope>().await?;
    assert_eq!(upd_scope.scope, upd_scp.name);

    // delete the scope
    let url_del = format!("{}/{}", url, upd_scp.id);
    let res = reqwest::Client::new()
        .delete(&url_del)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // verify it is gone
    let res = reqwest::Client::new()
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let scopes = res.json::<Vec<Scope>>().await?;
    assert_eq!(scopes.len(), 4);

    Ok(())
}
