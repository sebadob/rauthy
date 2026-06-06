use crate::common::{get_auth_headers, get_backend_url};
use chrono::Utc;
use pretty_assertions::assert_eq;
use rauthy_api_types::api_keys::{AccessGroup, AccessRights, ApiKeyAccess, ApiKeyRequest};
use rauthy_api_types::groups::GroupRequest;
use rauthy_data::entity::groups::Group;
use reqwest::StatusCode;
use reqwest::header::AUTHORIZATION;
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

    // the groups-only API key must not manage API keys
    let res = client
        .get(&url)
        .header(AUTHORIZATION, &key_header)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::FORBIDDEN);

    // create an API-key manager key and use it to manage another key
    let manager_payload = ApiKeyRequest {
        name: "api-key-manager".to_string(),
        exp: None,
        access: vec![ApiKeyAccess {
            group: AccessGroup::ApiKeys,
            access_rights: vec![
                AccessRights::Read,
                AccessRights::Create,
                AccessRights::Update,
                AccessRights::Delete,
            ],
        }],
    };
    let res = client
        .post(&url)
        .headers(auth_headers.clone())
        .json(&manager_payload)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let manager_secret = res.text().await?;
    let manager_header = format!("API-Key {}", manager_secret);

    let managed_payload = ApiKeyRequest {
        name: "api-key-managed".to_string(),
        exp: None,
        access: vec![ApiKeyAccess {
            group: AccessGroup::Groups,
            access_rights: vec![AccessRights::Read],
        }],
    };
    let res = client
        .post(&url)
        .header(AUTHORIZATION, &manager_header)
        .json(&managed_payload)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let managed_secret = res.text().await?;
    assert!(managed_secret.starts_with(&managed_payload.name));

    let managed_secret_url = format!("{}/{}/secret", url, managed_payload.name);
    let res = client
        .put(&managed_secret_url)
        .header(AUTHORIZATION, &manager_header)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let rotated_secret = res.text().await?;
    assert!(rotated_secret.starts_with(&managed_payload.name));
    assert_ne!(managed_secret, rotated_secret);

    let managed_url = format!("{}/{}", url, managed_payload.name);
    let res = client
        .delete(&managed_url)
        .header(AUTHORIZATION, &manager_header)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    // we should NOT be able to create a new group
    let new_group = GroupRequest {
        group: "api_key_test_group".to_string(),
        meta: None,
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

    // issue #1554: an API key with `AuthProviders` access can reach the upstream
    // provider endpoints, which used to be admin-session-only.
    payload.access = vec![ApiKeyAccess {
        group: AccessGroup::AuthProviders,
        access_rights: vec![AccessRights::Read],
    }];
    let res = client
        .put(&url_put)
        .headers(auth_headers.clone())
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    // listing providers is `POST /providers` (an admin route, now API-key accessible)
    let url_providers = format!("{}/providers", get_backend_url());
    let res = client
        .post(&url_providers)
        .header(AUTHORIZATION, &key_header)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    // without `AuthProviders` access, the same call is forbidden
    payload.access = vec![ApiKeyAccess {
        group: AccessGroup::Groups,
        access_rights: vec![AccessRights::Read],
    }];
    let res = client
        .put(&url_put)
        .headers(auth_headers.clone())
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let res = client
        .post(&url_providers)
        .header(AUTHORIZATION, &key_header)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::FORBIDDEN);

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
