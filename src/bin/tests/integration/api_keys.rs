use crate::common::{get_auth_headers, get_backend_url};
use crate::expect_status;
use crate::expect_status_json;
use chrono::Utc;
use rauthy_api_types::api_keys::{AccessGroup, AccessRights, ApiKeyAccess, ApiKeyRequest};
use rauthy_api_types::groups::GroupRequest;
use rauthy_common::utils::get_rand;
use rauthy_data::entity::groups::Group;
use reqwest::header::AUTHORIZATION;
use std::error::Error;
use std::ops::Sub;

/// API keys are unique by name, live on for the whole backend, and their name is validated with
/// `^[a-zA-Z0-9_-/]{2,24}$`, so each run gets short, unique ones and deletes them again.
fn key_name(prefix: &str) -> String {
    format!("ak-{prefix}-{}", get_rand(12).to_lowercase())
}

#[tokio::test]
async fn test_api_keys() -> Result<(), Box<dyn Error>> {
    let client = reqwest::Client::new();
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();
    let url = format!("{backend_url}/api_keys");

    let res = client.get(&url).send().await?;
    expect_status!(res, 401);

    // create a new api key
    let mut payload = ApiKeyRequest {
        name: key_name("read"),
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
    let res = expect_status!(res, 200);
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
    expect_status!(res, 400);

    // we should be able to read groups now
    let url_groups = format!("{backend_url}/groups");
    let res = client
        .get(&url_groups)
        .header(AUTHORIZATION, &key_header)
        .send()
        .await?;
    let body = expect_status_json!(res, 200, Vec<Group>);
    assert!(!body.is_empty());

    // the groups-only API key must not manage API keys
    let res = client
        .get(&url)
        .header(AUTHORIZATION, &key_header)
        .send()
        .await?;
    expect_status!(res, 403);

    // create an API-key manager key and use it to manage another key
    let manager_payload = ApiKeyRequest {
        name: key_name("manager"),
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
    let manager_secret = expect_status!(res, 200).text().await?;
    let manager_header = format!("API-Key {}", manager_secret);

    let managed_payload = ApiKeyRequest {
        name: key_name("managed"),
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
    let managed_secret = expect_status!(res, 200).text().await?;
    assert!(managed_secret.starts_with(&managed_payload.name));

    let managed_secret_url = format!("{}/{}/secret", url, managed_payload.name);
    let res = client
        .put(&managed_secret_url)
        .header(AUTHORIZATION, &manager_header)
        .send()
        .await?;
    let rotated_secret = expect_status!(res, 200).text().await?;
    assert!(rotated_secret.starts_with(&managed_payload.name));
    assert_ne!(managed_secret, rotated_secret);

    let managed_url = format!("{}/{}", url, managed_payload.name);
    let res = client
        .delete(&managed_url)
        .header(AUTHORIZATION, &manager_header)
        .send()
        .await?;
    expect_status!(res, 200);

    // we should NOT be able to create a new group
    let new_group = GroupRequest {
        group: key_name("group"),
        meta: None,
    };
    let res = client
        .post(&url_groups)
        .header(AUTHORIZATION, &key_header)
        .json(&new_group)
        .send()
        .await?;
    expect_status!(res, 403);

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
    expect_status!(res, 200);

    // try again to create a group
    let res = client
        .post(&url_groups)
        .header(AUTHORIZATION, &key_header)
        .json(&new_group)
        .send()
        .await?;
    let res = expect_status!(res, 200);
    let created_group = res.json::<Group>().await?;

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
    expect_status!(res, 200);

    // listing providers is `POST /providers` (an admin route, now API-key accessible)
    let url_providers = format!("{backend_url}/providers");
    let res = client
        .post(&url_providers)
        .header(AUTHORIZATION, &key_header)
        .send()
        .await?;
    expect_status!(res, 200);

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
    expect_status!(res, 200);
    let res = client
        .post(&url_providers)
        .header(AUTHORIZATION, &key_header)
        .send()
        .await?;
    expect_status!(res, 403);

    // let our key expire
    let exp_ts = Utc::now().sub(chrono::Duration::seconds(1)).timestamp();
    payload.exp = Some(exp_ts);
    let res = client
        .put(&url_put)
        .headers(auth_headers.clone())
        .json(&payload)
        .send()
        .await?;
    expect_status!(res, 200);

    // we should not be able to access the API with the now expired key -> try reading groups again
    let res = client
        .get(&url_groups)
        .header(AUTHORIZATION, &key_header)
        .send()
        .await?;
    expect_status!(res, 401);

    // Everything this test created is removed again, an expired key included.
    let res = client
        .delete(format!("{url_groups}/{}", created_group.id))
        .headers(auth_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200, "delete the group the API key created");
    for name in [&payload.name, &manager_payload.name] {
        let res = client
            .delete(format!("{url}/{name}"))
            .headers(auth_headers.clone())
            .send()
            .await?;
        expect_status!(res, 200);
    }

    Ok(())
}
