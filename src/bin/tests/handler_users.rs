use crate::common::{
    get_auth_headers, get_backend_url, get_solved_pow, get_token_set, get_token_set_init_client,
};
use pretty_assertions::assert_eq;
use rauthy_api_types::api_keys::{AccessGroup, AccessRights, ApiKeyAccess, ApiKeyRequest};
use rauthy_api_types::generic::Language;
use rauthy_api_types::users::{
    NewUserRequest, RequestResetRequest, UserResponse, UserResponseSimple, Userinfo,
};
use rauthy_common::utils::new_store_id;
use reqwest::StatusCode;
use reqwest::header::AUTHORIZATION;
use std::error::Error;

mod common;

#[tokio::test]
async fn test_users() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;

    // get all users and check the admin user
    let url = format!("{}/users", get_backend_url());
    let res = reqwest::Client::new()
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let users = res.json::<Vec<UserResponseSimple>>().await?;
    assert_eq!(users.len(), 3);

    // post a new user
    let new_user = NewUserRequest {
        given_name: "Alfred".to_string(),
        family_name: Some("Batman".to_string()),
        email: "alfred@batcave.io".to_string(),
        language: Language::En,
        roles: vec![
            "admin".to_string(),
            "user".to_string(),
            // check roles sanitization
            "non_existent".to_string(),
        ],
        groups: Some(vec![
            "admin".to_string(),
            "user".to_string(),
            // check groups sanitization
            "non_existent".to_string(),
        ]),
        user_expires: None,
    };
    let res = reqwest::Client::new()
        .post(&url)
        .headers(auth_headers.clone())
        .json(&new_user)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let alfred = res.json::<UserResponse>().await?;
    assert_eq!(alfred.email, "alfred@batcave.io");
    assert_eq!(alfred.given_name, "Alfred");
    assert_eq!(alfred.family_name.as_deref(), Some("Batman"));
    assert!(alfred.roles.contains(&"admin".to_string()));
    assert!(alfred.roles.contains(&"user".to_string()));
    assert!(alfred.groups.is_some());
    assert!(
        alfred
            .groups
            .as_ref()
            .unwrap()
            .contains(&"admin".to_string())
    );
    assert!(
        alfred
            .groups
            .as_ref()
            .unwrap()
            .contains(&"user".to_string())
    );
    assert_eq!(alfred.enabled, true);
    assert_eq!(alfred.email_verified, false);

    // get the new user by id
    let url_id = format!("{}/users/{}", get_backend_url(), alfred.id);
    let res = reqwest::Client::new()
        .get(&url_id)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let user_by_id = res.json::<UserResponse>().await?;
    assert_eq!(user_by_id.email, "alfred@batcave.io");

    // get the new user by email
    let url_email = format!("{}/users/email/{}", get_backend_url(), alfred.email);
    let res = reqwest::Client::new()
        .get(&url_email)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let user_by_email = res.json::<UserResponse>().await?;
    assert_eq!(user_by_email.id, alfred.id);

    // delete the user again
    let res = reqwest::Client::new()
        .delete(&url_id)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 204);

    // get all users again and check that we are back to only 1
    let res = reqwest::Client::new()
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let users = res.json::<Vec<UserResponseSimple>>().await?;
    assert_eq!(users.len(), 3);

    Ok(())
}

#[tokio::test]
async fn test_password_reset_always_ok() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let client = reqwest::Client::new();

    // get password reset for an existing user
    let mut payload = RequestResetRequest {
        email: "admin@localhost".to_string(),
        redirect_uri: None,
        pow: get_solved_pow().await,
    };
    let url = format!("{}/users/request_reset", get_backend_url());
    let res = client
        .post(&url)
        .headers(auth_headers.clone())
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // we should always get back an HTTP 200 for username enumeration prevention
    payload.email = "idonotexist@iamaghost.io".to_string();
    let res = client
        .post(&url)
        .headers(auth_headers.clone())
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    Ok(())
}

#[tokio::test]
async fn test_userinfo() -> Result<(), Box<dyn Error>> {
    let url = format!("{}/oidc/userinfo", get_backend_url());
    let client = reqwest::Client::new();

    // Unauthorized without a Bearer
    let res = client.get(&url).send().await?;
    assert_eq!(res.status(), 401);

    // This should be good
    let ts = get_token_set().await;
    let res = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", ts.access_token))
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let info = res.json::<Userinfo>().await?;
    assert_eq!(info.sub, "m4PJ3TnyP32LA8hzY23deme3");
    assert_eq!(info.name, "Admin Init");
    assert!(info.roles.contains(&"rauthy_admin".to_string()));

    Ok(())
}

#[tokio::test]
async fn test_user_picture() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let client = reqwest::Client::new();

    // find our test user
    let res = client
        .get(&format!("{}/users", get_backend_url()))
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let users = res.json::<Vec<UserResponseSimple>>().await?;
    let user_id_admin = users
        .iter()
        .find(|u| u.email == "admin@localhost")
        .unwrap()
        .id
        .clone();
    let user_id = users
        .into_iter()
        .find(|u| u.email == "init_admin@localhost")
        .unwrap()
        .id;
    eprintln!("user id for picture tests: {}", user_id);

    // upload a picture
    let url_picture = format!("{}/users/{}/picture", get_backend_url(), user_id);

    let path = "../../assets/logo/rauthy_dark_small.png";
    let part = reqwest::multipart::Part::file(path).await.unwrap();
    let form = reqwest::multipart::Form::new();
    let form = form.part("image.png", part);

    let res = client
        .put(&url_picture)
        .headers(auth_headers.clone())
        .multipart(form)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let picture_id = res.text().await?;
    assert!(!picture_id.is_empty());

    let url = format!("{}/{}", url_picture, picture_id);

    // make sure we get 401 without authn
    let res = client.get(&url).send().await?;
    assert_eq!(res.status(), 401);

    // success with valid session
    let res = client
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    assert_eq!(
        res.headers().get("content-type").unwrap().to_str().unwrap(),
        "image/webp"
    );
    let body = res.bytes().await?;
    assert!(!body.is_empty());

    // success with proper API key
    let payload = ApiKeyRequest {
        name: new_store_id(),
        exp: None,
        access: vec![ApiKeyAccess {
            group: AccessGroup::Users,
            access_rights: vec![AccessRights::Read],
        }],
    };
    let res = client
        .post(&format!("{}/api_keys", get_backend_url()))
        .headers(auth_headers.clone())
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let secret = res.text().await.unwrap();
    // the header needs to be in format: 'API-Key my_apy_key'
    let key_header = format!("API-Key {}", secret);
    let res = client
        .get(&url)
        .header(AUTHORIZATION, key_header)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // not allowed with token - `rauthy` client does not have the `profile` scope
    let ts = get_token_set().await;
    let res = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", ts.access_token))
        .send()
        .await?;
    assert_eq!(res.status(), 401);

    // success with access token - `profile` is in default scopes for `init_client`
    let ts = get_token_set_init_client().await;
    let res = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", ts.access_token))
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // success with id token
    let res = client
        .get(&url)
        .header(
            AUTHORIZATION,
            format!("Bearer {}", ts.id_token.clone().unwrap()),
        )
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // make sure access with token for another user fails
    let url_picture = format!("{}/users/{}/picture", get_backend_url(), user_id_admin);

    let path = "../../assets/logo/rauthy_dark_small.png";
    let part = reqwest::multipart::Part::file(path).await.unwrap();
    let form = reqwest::multipart::Form::new();
    let form = form.part("image.png", part);

    let res = client
        .put(&url_picture)
        .headers(auth_headers.clone())
        .multipart(form)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let picture_id = res.text().await?;
    assert!(!picture_id.is_empty());

    let url_admin = format!(
        "{}/users/{}/picture/{}",
        get_backend_url(),
        user_id_admin,
        picture_id
    );
    let res = client
        .get(&url_admin)
        .header(AUTHORIZATION, format!("Bearer {}", ts.access_token))
        .send()
        .await?;
    assert_eq!(res.status(), 401);
    let res = client
        .get(&url_admin)
        .header(AUTHORIZATION, format!("Bearer {}", ts.id_token.unwrap()))
        .send()
        .await?;
    assert_eq!(res.status(), 401);

    Ok(())
}
