use crate::common::{get_auth_headers, get_backend_url};
use pretty_assertions::assert_eq;
use rauthy_models::language::Language;
use rauthy_models::request::NewUserRequest;
use rauthy_models::response::UserResponse;
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

    let users = res.json::<Vec<UserResponse>>().await?;
    assert_eq!(users.len(), 3);

    // post a new user
    let new_user = NewUserRequest {
        given_name: "Alfred".to_string(),
        family_name: "Batman".to_string(),
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
    assert_eq!(alfred.family_name, "Batman");
    assert!(alfred.roles.contains(&"admin".to_string()));
    assert!(alfred.roles.contains(&"user".to_string()));
    assert!(alfred.groups.is_some());
    assert!(alfred
        .groups
        .as_ref()
        .unwrap()
        .contains(&"admin".to_string()));
    assert!(alfred
        .groups
        .as_ref()
        .unwrap()
        .contains(&"user".to_string()));
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

    let users = res.json::<Vec<UserResponse>>().await?;
    assert_eq!(users.len(), 3);

    Ok(())
}
