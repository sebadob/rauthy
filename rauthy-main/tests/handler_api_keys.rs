use crate::common::{get_auth_headers, get_backend_url};
use chrono::Utc;
use pretty_assertions::assert_eq;
use rauthy_models::entity::api_keys::{AccessGroup, AccessRights, ApiKeyAccess};
use rauthy_models::entity::groups::Group;
use rauthy_models::request::{ApiKeyRequest, NewGroupRequest};
use reqwest::header::AUTHORIZATION;
use reqwest::StatusCode;
use std::error::Error;
use std::ops::Sub;

mod common;

#[tokio::test]
async fn test_api_keys() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let auth_headers = get_auth_headers().await?;

    let url = format!("{}/api_keys", get_backend_url());
    let res = client.get(&url).send().await?;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    // create a new api key
    let mut payload = ApiKeyRequest {
        name: "test123".to_string(),
        exp: None,
        access: vec![ApiKeyAccess {
            group: AccessGroup::Groups,
            access_rights: vec![AccessRights::Read],
        }],
    };
    let res = client
        .post(&url)
        .headers(auth_headers.clone())
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let secret = res.text().await.unwrap();
    eprintln!("api key secret: {}", secret);
    assert!(secret.starts_with(&payload.name));
    // the header needs to be in format: 'API-Key my_apy_key'
    let key_header = format!("API-Key {}", secret);

    // creating the same key fails -> duplicate name
    let res = client
        .post(&url)
        .headers(auth_headers.clone())
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::BAD_REQUEST);

    // we should be able to read groups now
    let url_groups = format!("{}/groups", get_backend_url());
    let res = client
        .get(&url_groups)
        .header(AUTHORIZATION, &key_header)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let body = res.json::<Vec<Group>>().await?;
    assert!(!body.is_empty());

    // we should NOT be able to create a new group
    let new_group = NewGroupRequest {
        group: "api_key_test_group".to_string(),
    };
    let res = client
        .post(&url_groups)
        .header(AUTHORIZATION, &key_header)
        .json(&new_group)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    // modify the API key and give permissions to create a new group
    payload.access = vec![ApiKeyAccess {
        group: AccessGroup::Groups,
        access_rights: vec![AccessRights::Read, AccessRights::Create],
    }];
    let url_put = format!("{}/{}", url, payload.name);
    let res = client
        .put(&url_put)
        .headers(auth_headers.clone())
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    // try again to create a group
    let res = client
        .post(&url_groups)
        .header(AUTHORIZATION, &key_header)
        .json(&new_group)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    // let our key expire
    let exp_ts = Utc::now().sub(chrono::Duration::seconds(1)).timestamp();
    payload.exp = Some(exp_ts);
    let res = client
        .put(&url_put)
        .headers(auth_headers.clone())
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    // we should not be able to access the API with the now expired key -> try reading groups again
    let res = client
        .get(&url_groups)
        .header(AUTHORIZATION, &key_header)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    Ok(())
}
