use crate::common::{
    CLIENT_ID, PASSWORD, USERNAME, create_login_user, get_auth_headers, get_backend_url,
    get_solved_pow, get_token_set, get_token_set_init_client, password_grant, password_token_set,
    prepare_login, refresh, session_headers_with,
};
use futures::{StreamExt, stream};
use pretty_assertions::assert_eq;
use rauthy_api_types::api_keys::{AccessGroup, AccessRights, ApiKeyAccess, ApiKeyRequest};
use rauthy_api_types::generic::Language;
use rauthy_api_types::users::{
    NewUserRequest, RequestResetRequest, UpdateUserRequest, UserConsentDeleteResponse,
    UserConsentResponse, UserResponse, UserResponseSimple, Userinfo,
};
use rauthy_common::utils::new_store_id;
use rauthy_service::token_set::TokenSet;
use reqwest::header::{AUTHORIZATION, HeaderMap};
use reqwest::multipart::{Form, Part};
use reqwest::{Body, Client, StatusCode};
use serde_json::{Value, json};
use std::error::Error;
use std::io;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
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

    // `init_client` has the `profile` scope, the admin has no `preferred_username`
    // -> `email_fallback` (default: true) must apply, like in the ID token
    let ts = get_token_set_init_client().await;
    let res = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", ts.access_token))
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let info = res.json::<Userinfo>().await?;
    assert!(info.email.is_some());
    assert_eq!(info.preferred_username, info.email);

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
    let url_user = format!("{}/users/{}", get_backend_url(), user_id);
    let url_picture = format!("{url_user}/picture");
    let picture_id = upload_picture(&client, &url_user, &auth_headers).await?;
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
    let url_user_admin = format!("{}/users/{}", get_backend_url(), user_id_admin);
    let picture_id = upload_picture(&client, &url_user_admin, &auth_headers).await?;
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

/// An admin update of `user` that keeps its state, as `f` changes it. All users of this file are
/// created with `En`.
fn update_request(
    user: &UserResponse,
    f: impl FnOnce(&mut UpdateUserRequest),
) -> UpdateUserRequest {
    let mut upd = UpdateUserRequest {
        email: user.email.clone(),
        given_name: user.given_name.clone(),
        family_name: user.family_name.clone(),
        language: Some(Language::En),
        password: None,
        roles: user.roles.clone(),
        groups: user.groups.clone(),
        enabled: user.enabled,
        email_verified: user.email_verified,
        user_expires: user.user_expires,
        user_values: None,
    };
    f(&mut upd);
    upd
}

const EMAIL: &str = "consent-user@consenttest.io";
/// The password of the users these tests log in with. It satisfies the stricter policy of
/// `zzb_handler_password_policy` as well.
const LOGIN_PWD: &str = "Login-TEST.1234!#$";

/// The refresh of a revoked refresh token fails on the missing row.
async fn assert_refresh_not_found(client: &Client, refresh_token: String) {
    let res = refresh(client, refresh_token).await;
    assert_eq!(res.status(), 404);
    let body = res.text().await.unwrap();
    assert!(body.contains("Refresh Token not found"), "{body}");
}

/// The texts of the `ConsentRevoked` events for `email` since `from`, sorted.
async fn consent_revoked_events(
    client: &Client,
    admin: &reqwest::header::HeaderMap,
    from: i64,
    email: &str,
) -> Vec<String> {
    // the event is sent after the response
    time::sleep(Duration::from_secs(1)).await;
    let res = client
        .post(format!("{}/events", get_backend_url()))
        .headers(admin.clone())
        .json(&json!({
            "from": from,
            "level": "info",
            "typ": "ConsentRevoked",
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(res.status(), 200);
    let mut texts = res
        .json::<Vec<Value>>()
        .await
        .unwrap()
        .iter()
        .filter_map(|e| e["text"].as_str())
        .filter(|t| t.contains(&format!("`{email}`")))
        .map(String::from)
        .collect::<Vec<_>>();
    texts.sort();
    texts
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
        .json(&update_request(&user, |upd| {
            upd.password = Some(LOGIN_PWD.to_string());
            upd.email_verified = true;
        }))
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let url = format!("{backend}/users/{}/consents", user.id);
    // without `resource` for a consent without one
    let url_delete = format!("{url}?client_id={CLIENT_ID}");

    // --- authorization
    let res = client.get(&url).send().await?;
    assert_eq!(res.status(), 401);
    let res = client.delete(&url_delete).send().await?;
    assert_eq!(res.status(), 401);

    let user_headers = session_headers_with(EMAIL, LOGIN_PWD).await;
    let res = client
        .get(format!("{backend}/users/{admin_id}/consents"))
        .headers(user_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 403);
    let res = client
        .delete(format!(
            "{backend}/users/{admin_id}/consents?client_id={CLIENT_ID}"
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
    // an empty filter selects the consents without a resource
    let res = client
        .get(format!("{url}?resource="))
        .headers(user_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    assert!(
        res.json::<Vec<UserConsentResponse>>()
            .await?
            .iter()
            .all(|c| c.resource.is_none())
    );
    let res = client
        .get(format!(
            "{url}?resource=https%3A%2F%2Fapi.example.com%23frag"
        ))
        .headers(user_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 400);

    // --- validation: `client_id` is required, `resource` is not empty and has no fragment
    for query in [
        format!("client_id={CLIENT_ID}&resource="),
        "resource=https%3A%2F%2Fapi.example.com".to_string(),
        format!("client_id={CLIENT_ID}&resource=https%3A%2F%2Fapi.example.com%23frag"),
        "client_id=bad%20id".to_string(),
    ] {
        let res = client
            .delete(format!("{url}?{query}"))
            .headers(user_headers.clone())
            .send()
            .await?;
        assert_eq!(res.status(), 400, "{query}");
    }

    // --- revocation drops the refresh tokens of the user for the client
    let ts = password_token_set(&client, EMAIL, LOGIN_PWD).await;

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

    // an admin, then idempotent
    let ts = password_token_set(&client, EMAIL, LOGIN_PWD).await;
    let res = client
        .delete(&url_delete)
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let deleted = res.json::<UserConsentDeleteResponse>().await?;
    assert!(!deleted.consent_deleted);
    assert!(deleted.refresh_tokens_deleted >= 1);
    assert_refresh_not_found(&client, ts.refresh_token.unwrap()).await;
    let res = client
        .delete(&url_delete)
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let deleted = res.json::<UserConsentDeleteResponse>().await?;
    assert!(!deleted.consent_deleted);
    assert_eq!(deleted.refresh_tokens_deleted, 0);

    // one event for each revoke that deleted something, each naming who revoked, none for the
    // no-op
    assert_eq!(
        consent_revoked_events(&client, &admin, events_from, EMAIL).await,
        vec![
            format!("Consent of `{EMAIL}` for client `{CLIENT_ID}` revoked by admin `{USERNAME}`"),
            format!("Consent of `{EMAIL}` for client `{CLIENT_ID}` revoked by the user"),
        ]
    );

    // --- a revoke with a `resource` drops the refresh tokens for the client as a whole
    let ts = password_token_set(&client, EMAIL, LOGIN_PWD).await;
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
            "{backend}/users/UnknownUserId1234/consents?client_id={CLIENT_ID}"
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

/// An API key with `Users` access reads and revokes the consents of any user, each with its
/// own access right.
#[tokio::test]
async fn test_user_consents_api_key() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();

    let res = client
        .post(format!("{backend}/users"))
        .headers(admin.clone())
        .json(&NewUserRequest {
            given_name: Some("Consent".to_string()),
            family_name: Some("ApiKey".to_string()),
            email: format!("{}@consenttest.io", new_store_id()),
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
        .json(&update_request(&user, |upd| {
            upd.password = Some(LOGIN_PWD.to_string());
            upd.email_verified = true;
        }))
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let url = format!("{backend}/users/{}/consents", user.id);
    let url_delete = format!("{url}?client_id={CLIENT_ID}");

    let mut key = ApiKeyRequest {
        name: new_store_id(),
        exp: None,
        access: vec![ApiKeyAccess {
            group: AccessGroup::Clients,
            access_rights: vec![AccessRights::Read, AccessRights::Delete],
        }],
    };
    let res = client
        .post(format!("{backend}/api_keys"))
        .headers(admin.clone())
        .json(&key)
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let key_header = format!("API-Key {}", res.text().await?);
    let get = async || {
        client
            .get(&url)
            .header(AUTHORIZATION, &key_header)
            .send()
            .await
            .unwrap()
            .status()
    };
    let delete = async || {
        client
            .delete(&url_delete)
            .header(AUTHORIZATION, &key_header)
            .send()
            .await
            .unwrap()
            .status()
    };
    let mut set_rights = async |rights: Vec<AccessRights>| {
        key.access = vec![ApiKeyAccess {
            group: AccessGroup::Users,
            access_rights: rights,
        }];
        let res = client
            .put(format!("{backend}/api_keys/{}", key.name))
            .headers(admin.clone())
            .json(&key)
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    };

    // the key and an admin session together
    let get_with_session = async || {
        client
            .get(&url)
            .headers(admin.clone())
            .header(AUTHORIZATION, &key_header)
            .send()
            .await
            .unwrap()
            .status()
    };
    let delete_with_session = async || {
        let res = client
            .delete(&url_delete)
            .headers(admin.clone())
            .header(AUTHORIZATION, &key_header)
            .send()
            .await
            .unwrap();
        (
            res.status(),
            res.json::<UserConsentDeleteResponse>().await.ok(),
        )
    };

    // another access group, rejected even with an admin session
    assert_eq!(get().await, StatusCode::FORBIDDEN);
    assert_eq!(delete().await, StatusCode::FORBIDDEN);
    assert_eq!(get_with_session().await, StatusCode::FORBIDDEN);
    assert_eq!(delete_with_session().await.0, StatusCode::FORBIDDEN);

    set_rights(vec![AccessRights::Read]).await;
    assert_eq!(get().await, StatusCode::OK);
    assert_eq!(delete().await, StatusCode::FORBIDDEN);

    set_rights(vec![AccessRights::Delete]).await;
    assert_eq!(get().await, StatusCode::FORBIDDEN);
    assert_eq!(delete().await, StatusCode::OK);

    // with an admin session as well, the revoke is the key's
    let ts = password_token_set(&client, &user.email, LOGIN_PWD).await;
    let events_from = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
    let (status, deleted) = delete_with_session().await;
    assert_eq!(status, StatusCode::OK);
    assert!(deleted.unwrap().refresh_tokens_deleted >= 1);
    assert_refresh_not_found(&client, ts.refresh_token.unwrap()).await;
    assert_eq!(
        consent_revoked_events(&client, &admin, events_from, &user.email).await,
        vec![format!(
            "Consent of `{}` for client `{CLIENT_ID}` revoked by API key `{}`",
            user.email, key.name
        )]
    );

    let res = client
        .delete(format!("{backend}/api_keys/{}", key.name))
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let res = client
        .delete(format!("{backend}/users/{}", user.id))
        .headers(admin)
        .send()
        .await?;
    assert_eq!(res.status(), 204);

    Ok(())
}

/// In three rounds, starts concurrent logins of `user` through `start_logins` and, while they wait
/// for their password hash, changes the user's name and roles as an admin. Asserts after each
/// round that the admin change survived the logins, which save only their login state.
async fn assert_admin_update_survives_logins(
    client: &reqwest::Client,
    admin: &HeaderMap,
    user: &UserResponse,
    start_logins: impl AsyncFn() -> Vec<JoinHandle<()>>,
) -> Result<(), Box<dyn Error>> {
    let url_user = format!("{}/users/{}", get_backend_url(), user.id);

    for round in 0..3 {
        let given_name = format!("Admin round {round}");
        let roles = if round % 2 == 0 {
            vec!["admin".to_string(), "user".to_string()]
        } else {
            vec!["user".to_string()]
        };

        let logins = start_logins().await;
        // update while the logins wait for their password hash, one at a time
        time::sleep(Duration::from_millis(20)).await;
        let res = client
            .put(&url_user)
            .headers(admin.clone())
            .json(&update_request(user, |upd| {
                upd.given_name = Some(given_name.clone());
                upd.roles = roles.clone();
                upd.email_verified = true;
            }))
            .send()
            .await?;
        assert_eq!(res.status(), 200);
        for login in logins {
            login.await?;
        }

        let res = client.get(&url_user).headers(admin.clone()).send().await?;
        assert_eq!(res.status(), 200);
        let found = res.json::<UserResponse>().await?;
        assert_eq!(
            found.given_name.as_deref(),
            Some(given_name.as_str()),
            "round {round}"
        );
        let mut found_roles = found.roles.clone();
        found_roles.sort();
        assert_eq!(found_roles, roles, "round {round}");
        assert!(found.last_login.is_some());
        assert_eq!(found.failed_login_attempts, None);
    }

    Ok(())
}

/// An admin update of a user made while `/oidc/authorize` logins of that user wait for their hash
/// survives the logins.
#[tokio::test]
async fn test_admin_update_survives_concurrent_logins() -> Result<(), Box<dyn Error>> {
    const EMAIL: &str = "update-logins@userstest.io";

    let admin = get_auth_headers().await?;
    let client = Client::new();
    let user = create_login_user(&client, &admin, EMAIL, "Login", LOGIN_PWD).await?;

    assert_admin_update_survives_logins(&client, &admin, &user, async || {
        let mut prepared = Vec::new();
        for _ in 0..6 {
            prepared.push(prepare_login(EMAIL, LOGIN_PWD).await);
        }
        prepared
            .into_iter()
            .map(|login| {
                tokio::spawn(async move {
                    login.send().await;
                })
            })
            .collect()
    })
    .await?;

    let res = client
        .delete(format!("{}/users/{}", get_backend_url(), user.id))
        .headers(admin)
        .send()
        .await?;
    assert_eq!(res.status(), 204);

    Ok(())
}

/// An admin update of a user made while password grant logins of that user wait for their hash
/// survives the logins.
#[tokio::test]
async fn test_admin_update_survives_concurrent_password_grants() -> Result<(), Box<dyn Error>> {
    const EMAIL: &str = "update-grants@userstest.io";

    let admin = get_auth_headers().await?;
    let client = Client::new();
    let user = create_login_user(&client, &admin, EMAIL, "Login", LOGIN_PWD).await?;

    assert_admin_update_survives_logins(&client, &admin, &user, async || {
        (0..6)
            .map(|_| {
                tokio::spawn(async {
                    password_token_set(&Client::new(), EMAIL, LOGIN_PWD).await;
                })
            })
            .collect()
    })
    .await?;

    let res = client
        .delete(format!("{}/users/{}", get_backend_url(), user.id))
        .headers(admin)
        .send()
        .await?;
    assert_eq!(res.status(), 204);

    Ok(())
}

/// An admin update of a user made while PAM password logins of that user wait for their hash
/// survives the logins: a PAM login saves only its login state, not the copy of the user it read
/// before.
#[tokio::test]
async fn test_admin_update_survives_concurrent_pam_logins() -> Result<(), Box<dyn Error>> {
    const EMAIL: &str = "pam-logins@userstest.io";
    const PAM_USERNAME: &str = "pamlogins";
    const HOSTNAME: &str = "pam-logins-host";

    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();

    // a left-over host of a prior run first, as it refers to the group of the PAM user
    let res = client
        .get(format!("{backend}/pam/hosts"))
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    for host in res.json::<Vec<Value>>().await? {
        if host["name"] == HOSTNAME {
            let res = client
                .delete(format!(
                    "{backend}/pam/hosts/{}",
                    host["id"].as_str().unwrap()
                ))
                .headers(admin.clone())
                .send()
                .await?;
            assert_eq!(res.status(), 200);
        }
    }
    let user = create_login_user(&client, &admin, EMAIL, "Pam", LOGIN_PWD).await?;
    let url_user = format!("{backend}/users/{}", user.id);

    let res = client
        .post(format!("{backend}/pam/users"))
        .headers(admin.clone())
        .json(&json!({ "username": PAM_USERNAME, "email": EMAIL }))
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let pam_user = res.json::<Value>().await?;

    let res = client
        .post(format!("{backend}/pam/hosts"))
        .headers(admin.clone())
        .json(&json!({
            "hostname": HOSTNAME,
            "gid": pam_user["gid"],
            "force_mfa": false,
            "local_password_only": true,
        }))
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let host_id = res.json::<Value>().await?["id"]
        .as_str()
        .unwrap()
        .to_string();
    let res = client
        .post(format!("{backend}/pam/hosts/{host_id}/secret"))
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let host_secret = res.json::<Value>().await?["secret"]
        .as_str()
        .unwrap()
        .to_string();

    // a failed PAM login is counted, and the successful ones below reset the count
    let res = client
        .post(format!("{backend}/pam/login"))
        .json(&json!({
            "host_id": host_id,
            "host_secret": host_secret,
            "username": PAM_USERNAME,
            "password": "Wrong-TEST.1234!#$",
        }))
        .send()
        .await?;
    assert_eq!(res.status(), 401);
    let res = client.get(&url_user).headers(admin.clone()).send().await?;
    assert_eq!(res.status(), 200);
    let found = res.json::<UserResponse>().await?;
    assert_eq!(found.failed_login_attempts, Some(1));

    assert_admin_update_survives_logins(&client, &admin, &user, async || {
        (0..6)
            .map(|_| {
                let req = client.post(format!("{backend}/pam/login")).json(&json!({
                    "host_id": host_id,
                    "host_secret": host_secret,
                    "username": PAM_USERNAME,
                    "password": LOGIN_PWD,
                }));
                tokio::spawn(async move { assert_eq!(req.send().await.unwrap().status(), 200) })
            })
            .collect()
    })
    .await?;

    let res = client
        .delete(format!("{backend}/pam/hosts/{host_id}"))
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let res = client.delete(&url_user).headers(admin).send().await?;
    assert_eq!(res.status(), 204);

    Ok(())
}

/// The picture the tests upload.
const LOGO: &str = "../../assets/logo/rauthy_dark_small.png";

/// Uploads the test logo as the picture of the user at `url_user` and returns its id.
async fn upload_picture(
    client: &Client,
    url_user: &str,
    headers: &HeaderMap,
) -> Result<String, Box<dyn Error>> {
    let part = Part::file(LOGO).await?;
    let form = Form::new().part("image.png", part);
    let res = client
        .put(format!("{url_user}/picture"))
        .headers(headers.clone())
        .multipart(form)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    Ok(res.text().await?)
}

/// A picture uploaded while password logins of the same user wait for their hash survives the
/// logins: a login saves only its login state, not the copy of the user it read before.
#[tokio::test]
async fn test_user_picture_survives_concurrent_logins() -> Result<(), Box<dyn Error>> {
    const EMAIL: &str = "picture-logins@userstest.io";

    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();
    let user = create_login_user(&client, &admin, EMAIL, "Login", LOGIN_PWD).await?;

    let url_user = format!("{backend}/users/{}", user.id);
    for round in 0..3 {
        let mut prepared = Vec::new();
        for _ in 0..6 {
            prepared.push(prepare_login(EMAIL, LOGIN_PWD).await);
        }
        let logins = prepared
            .into_iter()
            .map(|login| tokio::spawn(login.send()))
            .collect::<Vec<_>>();
        // upload while the logins wait for their password hash, one at a time
        time::sleep(Duration::from_millis(20)).await;
        let picture_id = upload_picture(&client, &url_user, &admin).await?;
        for login in logins {
            login.await?;
        }

        let res = client.get(&url_user).headers(admin.clone()).send().await?;
        assert_eq!(res.status(), 200);
        let found = res.json::<UserResponse>().await?;
        assert_eq!(
            found.picture_id.as_deref(),
            Some(picture_id.as_str()),
            "round {round}"
        );
        assert!(found.last_login.is_some());
        let res = client
            .get(format!("{url_user}/picture/{picture_id}"))
            .headers(admin.clone())
            .send()
            .await?;
        assert_eq!(res.status(), 200, "round {round}");
    }

    let res = client
        .delete(&url_user)
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 204);

    Ok(())
}

/// A picture uploaded while wrong password logins of the same user, through `/oidc/authorize` or
/// the password grant, wait for their hash survives the logins, and every failure is counted.
#[tokio::test]
async fn test_user_picture_survives_concurrent_failed_logins() -> Result<(), Box<dyn Error>> {
    const EMAIL: &str = "picture-failed-logins@userstest.io";
    const WRONG_PWD: &str = "Wrong-TEST.1234!#$";

    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();
    let user = create_login_user(&client, &admin, EMAIL, "Login", LOGIN_PWD).await?;
    let url_user = format!("{backend}/users/{}", user.id);

    for (round, authorize) in [true, true, false, false].into_iter().enumerate() {
        // two failures only: from the third one of this IP or user on, each is delayed by seconds
        let logins = if authorize {
            let mut prepared = Vec::new();
            for _ in 0..2 {
                prepared.push(prepare_login(EMAIL, WRONG_PWD).await);
            }
            prepared
                .into_iter()
                .map(|login| tokio::spawn(async move { login.response().await.status() }))
                .collect::<Vec<_>>()
        } else {
            (0..2)
                .map(|_| {
                    tokio::spawn(async {
                        password_grant(&Client::new(), EMAIL, WRONG_PWD)
                            .await
                            .status()
                    })
                })
                .collect()
        };
        // upload while the logins wait for their password hash, one at a time
        time::sleep(Duration::from_millis(20)).await;
        let picture_id = upload_picture(&client, &url_user, &admin).await?;
        for login in logins {
            assert_eq!(login.await?, StatusCode::UNAUTHORIZED, "round {round}");
        }

        let res = client.get(&url_user).headers(admin.clone()).send().await?;
        assert_eq!(res.status(), 200);
        let found = res.json::<UserResponse>().await?;
        assert_eq!(
            found.picture_id.as_deref(),
            Some(picture_id.as_str()),
            "round {round}"
        );
        assert_eq!(found.failed_login_attempts, Some(2), "round {round}");
        assert!(found.last_failed_login.is_some());

        // a successful login resets the failed logins of the user and of this IP
        password_token_set(&client, EMAIL, LOGIN_PWD).await;
    }

    let res = client.delete(&url_user).headers(admin).send().await?;
    assert_eq!(res.status(), 204);

    Ok(())
}

/// An admin save of a user keeps its sessions and refresh tokens while the user stays enabled,
/// and drops them once the save disables the user: they stay invalid after it is enabled again.
/// Those of another user are kept.
#[tokio::test]
async fn test_save_disabled_user_drops_sessions_and_refresh_tokens() -> Result<(), Box<dyn Error>> {
    const EMAIL: &str = "disable-drops@userstest.io";
    const OTHER_EMAIL: &str = "disable-drops-other@userstest.io";

    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();
    let user = create_login_user(&client, &admin, EMAIL, "Disable", LOGIN_PWD).await?;
    let other = create_login_user(&client, &admin, OTHER_EMAIL, "Other", LOGIN_PWD).await?;
    let url_user = format!("{backend}/users/{}", user.id);
    let url_session = format!("{backend}/oidc/sessioninfo");

    let ts = password_token_set(&client, EMAIL, LOGIN_PWD).await;
    let session = session_headers_with(EMAIL, LOGIN_PWD).await;
    let other_ts = password_token_set(&client, OTHER_EMAIL, LOGIN_PWD).await;
    let other_session = session_headers_with(OTHER_EMAIL, LOGIN_PWD).await;

    let save = async |enabled: bool| {
        let res = client
            .put(&url_user)
            .headers(admin.clone())
            .json(&update_request(&user, |upd| {
                upd.enabled = enabled;
                upd.email_verified = true;
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), 200, "save with enabled: {enabled}");
    };

    // a save of the enabled user keeps both
    save(true).await;
    let res = client
        .get(&url_session)
        .headers(session.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let res = refresh(&client, ts.refresh_token.clone().unwrap()).await;
    assert_eq!(res.status(), 200);
    let ts = res.json::<TokenSet>().await?;

    // a save that disables the user drops both, for good
    save(false).await;
    save(true).await;
    let res = client.get(&url_session).headers(session).send().await?;
    assert_eq!(res.status(), 401);
    assert_refresh_not_found(&client, ts.refresh_token.unwrap()).await;

    // the other user keeps both
    let res = client
        .get(&url_session)
        .headers(other_session)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let res = refresh(&client, other_ts.refresh_token.unwrap()).await;
    assert_eq!(res.status(), 200);

    for id in [&user.id, &other.id] {
        let res = client
            .delete(format!("{backend}/users/{id}"))
            .headers(admin.clone())
            .send()
            .await?;
        assert_eq!(res.status(), 204);
    }

    Ok(())
}

/// An admin update that sets a password, and so reads the user long before it saves it, keeps a
/// picture removed or uploaded meanwhile: a full-row save does not write `picture_id`.
#[tokio::test]
async fn test_admin_update_keeps_concurrent_picture_change() -> Result<(), Box<dyn Error>> {
    const EMAIL: &str = "update-picture@userstest.io";

    let admin = get_auth_headers().await?;
    let client = Client::new();
    let user = create_login_user(&client, &admin, EMAIL, "Picture", LOGIN_PWD).await?;
    let url_user = format!("{}/users/{}", get_backend_url(), user.id);
    let uploaded = upload_picture(&client, &url_user, &admin).await?;

    // round 0 removes the picture, round 1 uploads a new one
    for round in 0..2 {
        // logins ahead of the update in the hashing queue, so the change lands while it waits
        let logins = (0..3)
            .map(|_| {
                tokio::spawn(async {
                    password_token_set(&Client::new(), USERNAME, PASSWORD).await;
                })
            })
            .collect::<Vec<_>>();
        let update = tokio::spawn(
            client
                .put(&url_user)
                .headers(admin.clone())
                .json(&update_request(&user, |upd| {
                    upd.password = Some(format!("{LOGIN_PWD}{round}"));
                    upd.email_verified = true;
                }))
                .send(),
        );
        time::sleep(Duration::from_millis(20)).await;
        let picture_id = if round == 0 {
            let res = client
                .delete(format!("{url_user}/picture/{uploaded}"))
                .headers(admin.clone())
                .send()
                .await?;
            assert_eq!(res.status(), 200);
            None
        } else {
            Some(upload_picture(&client, &url_user, &admin).await?)
        };
        assert!(
            !update.is_finished(),
            "round {round}: the picture change must land while the update waits for its hash"
        );
        assert_eq!(update.await??.status(), 200, "round {round}");
        for login in logins {
            login.await?;
        }

        let res = client.get(&url_user).headers(admin.clone()).send().await?;
        assert_eq!(res.status(), 200);
        let found = res.json::<UserResponse>().await?;
        assert_eq!(found.picture_id, picture_id, "round {round}");
    }

    let res = client.delete(&url_user).headers(admin).send().await?;
    assert_eq!(res.status(), 204);

    Ok(())
}

/// An admin update that changes the email of a user drops the user cached under the old email.
#[tokio::test]
async fn test_admin_email_change_drops_cached_old_email() -> Result<(), Box<dyn Error>> {
    const EMAIL: &str = "email-change@userstest.io";
    const NEW_EMAIL: &str = "email-changed@userstest.io";

    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();
    let url_by_email = |email: &str| format!("{backend}/users/email/{email}");

    // a left-over of a prior run under the new email
    let res = client
        .get(url_by_email(NEW_EMAIL))
        .headers(admin.clone())
        .send()
        .await?;
    if res.status() == 200 {
        let id = res.json::<UserResponse>().await?.id;
        let res = client
            .delete(format!("{backend}/users/{id}"))
            .headers(admin.clone())
            .send()
            .await?;
        assert_eq!(res.status(), 204);
    }
    let user = create_login_user(&client, &admin, EMAIL, "Email", LOGIN_PWD).await?;
    let url_user = format!("{backend}/users/{}", user.id);

    // the lookup by email caches the user under it
    let res = client
        .get(url_by_email(EMAIL))
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let res = client
        .put(&url_user)
        .headers(admin.clone())
        .json(&update_request(&user, |upd| {
            upd.email = NEW_EMAIL.to_string();
            upd.email_verified = true;
        }))
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let res = client
        .get(url_by_email(EMAIL))
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 404);
    let res = client
        .get(url_by_email(NEW_EMAIL))
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    assert_eq!(res.json::<UserResponse>().await?.id, user.id);

    let res = client.delete(&url_user).headers(admin).send().await?;
    assert_eq!(res.status(), 204);

    Ok(())
}

/// A picture upload, which reads the user before it receives the picture, keeps an admin update
/// made while it waits for the rest of its body, and so does a later picture removal: both write
/// only the `picture_id` of their user, and leave another user alone.
#[tokio::test]
async fn test_picture_change_keeps_concurrent_admin_update() -> Result<(), Box<dyn Error>> {
    const EMAIL: &str = "picture-update@userstest.io";
    const OTHER_EMAIL: &str = "picture-update-other@userstest.io";

    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();
    let user = create_login_user(&client, &admin, EMAIL, "Picture", LOGIN_PWD).await?;
    let other = create_login_user(&client, &admin, OTHER_EMAIL, "Other", LOGIN_PWD).await?;
    let url_user = format!("{backend}/users/{}", user.id);
    let url_other = format!("{backend}/users/{}", other.id);
    let other_picture = upload_picture(&client, &url_other, &admin).await?;

    // the upload sends the second half of its picture only once the admin update is done
    let png = tokio::fs::read(LOGO).await?;
    let (head, tail) = png.split_at(png.len() / 2);
    let (head, tail) = (head.to_vec(), tail.to_vec());
    let (send_tail, tail_sent) = oneshot::channel::<()>();
    let body =
        stream::once(async move { Ok::<_, io::Error>(head) }).chain(stream::once(async move {
            tail_sent.await.ok();
            Ok(tail)
        }));
    let part = Part::stream_with_length(Body::wrap_stream(body), png.len() as u64)
        .file_name("image.png")
        .mime_str("image/png")?;
    let upload = tokio::spawn(
        client
            .put(format!("{url_user}/picture"))
            .headers(admin.clone())
            .multipart(Form::new().part("image.png", part))
            .send(),
    );
    time::sleep(Duration::from_millis(200)).await;

    let roles = vec!["admin".to_string(), "user".to_string()];
    let res = client
        .put(&url_user)
        .headers(admin.clone())
        .json(&update_request(&user, |upd| {
            upd.roles = roles.clone();
            upd.enabled = false;
            upd.email_verified = true;
        }))
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    assert!(
        !upload.is_finished(),
        "the admin update must land while the upload waits for its body"
    );
    send_tail.send(()).unwrap();
    let res = upload.await??;
    assert_eq!(res.status(), 200);
    let picture_id = res.text().await?;

    let assert_users = async |picture_id: Option<&str>| {
        let res = client.get(&url_user).headers(admin.clone()).send().await?;
        assert_eq!(res.status(), 200);
        let found = res.json::<UserResponse>().await?;
        assert_eq!(found.picture_id.as_deref(), picture_id);
        assert!(!found.enabled);
        let mut found_roles = found.roles.clone();
        found_roles.sort();
        assert_eq!(found_roles, roles);

        // the list reads the stored rows, not the cached users
        let res = client
            .get(format!("{backend}/users"))
            .headers(admin.clone())
            .send()
            .await?;
        assert_eq!(res.status(), 200);
        let users = res.json::<Vec<UserResponseSimple>>().await?;
        let found = users.iter().find(|u| u.id == other.id).unwrap();
        assert_eq!(found.picture_id.as_deref(), Some(other_picture.as_str()));
        Ok::<_, Box<dyn Error>>(())
    };
    assert_users(Some(&picture_id)).await?;

    let res = client
        .delete(format!("{url_user}/picture/{picture_id}"))
        .headers(admin.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    assert_users(None).await?;

    for url in [&url_user, &url_other] {
        let res = client.delete(url).headers(admin.clone()).send().await?;
        assert_eq!(res.status(), 204);
    }

    Ok(())
}
