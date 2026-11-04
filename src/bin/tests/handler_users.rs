use crate::common::{
    CLIENT_ID, PASSWORD, USERNAME, get_auth_headers, get_backend_url, get_solved_pow,
    get_token_set, get_token_set_init_client, password_token_set, refresh, session_headers_with,
};
use pretty_assertions::assert_eq;
use rauthy_api_types::api_keys::{AccessGroup, AccessRights, ApiKeyAccess, ApiKeyRequest};
use rauthy_api_types::generic::Language;
use rauthy_api_types::users::{
    NewUserRequest, RequestResetRequest, UpdateUserRequest, UserConsentDeleteResponse,
    UserConsentResponse, UserResponse, UserResponseSimple, Userinfo,
};
use rauthy_common::utils::new_store_id;
use rauthy_service::token_set::TokenSet;
use reqwest::header::AUTHORIZATION;
use reqwest::{Client, StatusCode};
use serde_json::{Value, json};
use std::error::Error;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::time;

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

    // post a new user
    let new_user = NewUserRequest {
        given_name: Some("Alfred".to_string()),
        family_name: Some("Batman".to_string()),
        email: "alfred@batcave.io".to_string(),
        preferred_username: Some("alfredo".to_string()),
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
        tz: None,
    };
    // make sure Alfred does not exist
    assert!(!users.iter().any(|u| u.email == new_user.email));

    let res = reqwest::Client::new()
        .post(&url)
        .headers(auth_headers.clone())
        .json(&new_user)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let alfred = res.json::<UserResponse>().await?;
    assert_eq!(alfred.email, "alfred@batcave.io");
    assert_eq!(alfred.given_name.as_deref(), Some("Alfred"));
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
    // the preferred_username given at creation must round-trip onto the created user
    assert_eq!(
        user_by_id.user_values.preferred_username.as_deref(),
        Some("alfredo")
    );

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
    assert!(!users.iter().any(|u| u.id == alfred.id));

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
    payload.pow = get_solved_pow().await;
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
        .get(format!("{}/users", get_backend_url()))
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
        .post(format!("{}/api_keys", get_backend_url()))
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

#[tokio::test]
async fn test_api_key_passkey_delete_authorization() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let client = reqwest::Client::new();

    let new_user = NewUserRequest {
        given_name: Some("Pk".to_string()),
        family_name: Some("Delete".to_string()),
        email: format!("{}@batcave.io", new_store_id()),
        preferred_username: None,
        language: Language::En,
        roles: vec!["user".to_string()],
        groups: None,
        user_expires: None,
        tz: None,
    };
    let res = client
        .post(format!("{}/users", get_backend_url()))
        .headers(auth_headers.clone())
        .json(&new_user)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let user = res.json::<UserResponse>().await?;

    let mut key = ApiKeyRequest {
        name: new_store_id(),
        exp: None,
        access: vec![ApiKeyAccess {
            group: AccessGroup::Users,
            access_rights: vec![AccessRights::Read],
        }],
    };
    let res = client
        .post(format!("{}/api_keys", get_backend_url()))
        .headers(auth_headers.clone())
        .json(&key)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let key_header = format!("API-Key {}", res.text().await.unwrap());

    let delete_url = format!(
        "{}/users/{}/webauthn/delete/missing",
        get_backend_url(),
        user.id
    );
    let res = client
        .delete(&delete_url)
        .header(AUTHORIZATION, &key_header)
        .json(&serde_json::json!({}))
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::UNAUTHORIZED);

    key.access = vec![ApiKeyAccess {
        group: AccessGroup::Users,
        access_rights: vec![AccessRights::Delete],
    }];
    let res = client
        .put(format!("{}/api_keys/{}", get_backend_url(), key.name))
        .headers(auth_headers.clone())
        .json(&key)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    let res = client
        .delete(&delete_url)
        .header(AUTHORIZATION, &key_header)
        .json(&serde_json::json!({}))
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);

    let res = client
        .delete(format!("{}/users/{}", get_backend_url(), user.id))
        .headers(auth_headers)
        .send()
        .await?;
    assert_eq!(res.status(), 204);

    Ok(())
}

const EMAIL: &str = "consent-user@consenttest.io";
// satisfies the stricter policy of `zzb_handler_password_policy` as well
const PWD: &str = "Consent-TEST.1234!#$";

/// The refresh of a revoked refresh token fails on the missing row.
async fn assert_refresh_not_found(client: &Client, refresh_token: String) {
    let res = refresh(client, refresh_token).await;
    assert_eq!(res.status(), 404);
    let body = res.text().await.unwrap();
    assert!(body.contains("Refresh Token not found"), "{body}");
}

#[tokio::test]
async fn test_user_consents() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();

    let res = client
        .get(format!("{backend}/users"))
        .headers(admin.clone())
        .send()
        .await?;
    let users = res.json::<Vec<UserResponseSimple>>().await?;
    let admin_id = users
        .iter()
        .find(|u| u.email == USERNAME)
        .unwrap()
        .id
        .clone();
    // left-over from a prior run
    if let Some(user) = users.iter().find(|u| u.email == EMAIL) {
        let res = client
            .delete(format!("{backend}/users/{}", user.id))
            .headers(admin.clone())
            .send()
            .await?;
        assert_eq!(res.status(), 204);
    }

    // --- setup: a user with a usable password
    let res = client
        .post(format!("{backend}/users"))
        .headers(admin.clone())
        .json(&NewUserRequest {
            given_name: Some("Consent".to_string()),
            family_name: None,
            email: EMAIL.to_string(),
            preferred_username: None,
            language: Language::En,
            roles: vec!["user".to_string()],
            groups: None,
            user_expires: None,
            tz: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let user = res.json::<UserResponse>().await?;

    let res = client
        .put(format!("{backend}/users/{}", user.id))
        .headers(admin.clone())
        .json(&UpdateUserRequest {
            email: user.email.clone(),
            given_name: user.given_name.clone(),
            family_name: user.family_name.clone(),
            language: Some(Language::En),
            password: Some(PWD.to_string()),
            roles: user.roles.clone(),
            groups: user.groups.clone(),
            enabled: true,
            email_verified: true,
            user_expires: None,
            user_values: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let url = format!("{backend}/users/{}/consents", user.id);
    let url_delete = format!("{url}?client_id={CLIENT_ID}&resource=");

    // --- authorization
    let res = client.get(&url).send().await?;
    assert_eq!(res.status(), 401);
    let res = client.delete(&url_delete).send().await?;
    assert_eq!(res.status(), 401);

    let user_headers = session_headers_with(EMAIL, PWD).await;
    let res = client
        .get(format!("{backend}/users/{admin_id}/consents"))
        .headers(user_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 403);
    let res = client
        .delete(format!(
            "{backend}/users/{admin_id}/consents?client_id={CLIENT_ID}&resource="
        ))
        .headers(user_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 403);

    // --- list, the user itself and an admin
    for headers in [&user_headers, &admin] {
        let res = client.get(&url).headers(headers.clone()).send().await?;
        assert_eq!(res.status(), 200);
        assert!(res.json::<Vec<UserConsentResponse>>().await?.is_empty());
    }
    let res = client
        .get(format!("{url}?resource=https%3A%2F%2Fapi.example.com"))
        .headers(user_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let res = client
        .get(format!(
            "{url}?resource=https%3A%2F%2Fapi.example.com%23frag"
        ))
        .headers(user_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 400);

    // --- validation: both params are required, `resource` without a fragment
    for query in [
        format!("client_id={CLIENT_ID}"),
        "resource=".to_string(),
        format!("client_id={CLIENT_ID}&resource=https%3A%2F%2Fapi.example.com%23frag"),
        "client_id=bad%20id&resource=".to_string(),
    ] {
        let res = client
            .delete(format!("{url}?{query}"))
            .headers(user_headers.clone())
            .send()
            .await?;
        assert_eq!(res.status(), 400, "{query}");
    }

    // --- revocation drops the refresh tokens of the user for the client
    let ts = password_token_set(&client, EMAIL, PWD).await;

    time::sleep(Duration::from_secs(2)).await;
    let res = refresh(&client, ts.refresh_token.unwrap()).await;
    assert_eq!(res.status(), 200);
    let ts = res.json::<TokenSet>().await?;

    // another user's refresh token for the same client
    let other = password_token_set(&client, USERNAME, PASSWORD).await;

    let userinfo = async |access_token: &str| {
        client
            .get(format!("{backend}/oidc/userinfo"))
            .header(AUTHORIZATION, format!("Bearer {access_token}"))
            .send()
            .await
            .unwrap()
            .status()
    };
    assert_eq!(userinfo(&ts.access_token).await, 200);

    let events_from = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
    let res = client
        .delete(&url_delete)
        .headers(user_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let deleted = res.json::<UserConsentDeleteResponse>().await?;
    // the password grant does not ask for consent, there is no consent row to delete
    assert!(!deleted.consent_deleted);
    // the rotated predecessor may still be there with a lowered `exp`
    assert!(deleted.refresh_tokens_deleted >= 1);

    assert_refresh_not_found(&client, ts.refresh_token.unwrap()).await;
    // the access token issued with the deleted refresh token is revoked as well
    assert_eq!(userinfo(&ts.access_token).await, 401);
    // the other user keeps theirs
    let res = refresh(&client, other.refresh_token.unwrap()).await;
    assert_eq!(res.status(), 200);

    // idempotent, for an admin as well
    let res = client
        .delete(&url_delete)
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let deleted = res.json::<UserConsentDeleteResponse>().await?;
    assert!(!deleted.consent_deleted);
    assert_eq!(deleted.refresh_tokens_deleted, 0);

    // one event for the revoke that deleted something, none for the no-op
    time::sleep(Duration::from_secs(1)).await;
    let res = client
        .post(format!("{backend}/events"))
        .headers(admin.clone())
        .json(&json!({
            "from": events_from,
            "level": "info",
            "typ": "ConsentRevoked",
        }))
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let events = res.json::<Vec<Value>>().await?;
    let revoked = events
        .iter()
        .filter(|e| e["text"].as_str().is_some_and(|t| t.contains(EMAIL)))
        .count();
    assert_eq!(revoked, 1, "{events:?}");

    // --- a revoke with a `resource` drops the refresh tokens for the client as a whole
    let ts = password_token_set(&client, EMAIL, PWD).await;
    let res = client
        .delete(format!(
            "{url}?client_id={CLIENT_ID}&resource=https%3A%2F%2Fapi.example.com"
        ))
        .headers(user_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let deleted = res.json::<UserConsentDeleteResponse>().await?;
    assert!(!deleted.consent_deleted);
    assert!(deleted.refresh_tokens_deleted >= 1);
    assert_refresh_not_found(&client, ts.refresh_token.unwrap()).await;

    // --- an unknown user
    let res = client
        .delete(format!(
            "{backend}/users/UnknownUserId1234/consents?client_id={CLIENT_ID}&resource="
        ))
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 404);

    // --- cleanup
    let res = client
        .delete(format!("{backend}/users/{}", user.id))
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 204);

    Ok(())
}
