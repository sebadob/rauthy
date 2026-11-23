use crate::common::{
    CLIENT_ID, PASSWORD, TestUser, USER_PASSWORD, USERNAME, decode_claims, get_auth_headers,
    get_backend_url, get_solved_pow, get_token_set, get_token_set_init_client, password_grant,
    password_token_set, prepare_login, refresh, session_headers_with,
};
use crate::{expect_status, expect_status_json};
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

/// The picture the tests upload.
const LOGO: &str = "../../assets/logo/rauthy_dark_small.png";
/// A password that satisfies the policy and belongs to no test user.
const WRONG_PWD: &str = "Wrong-TEST.1234!#$";

#[tokio::test]
async fn test_users() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let client = reqwest::Client::new();
    let backend = get_backend_url();
    let url = format!("{backend}/users");

    // The search idx endpoints are only meaningful with names nobody else on this backend has.
    let suffix = new_store_id().to_lowercase();
    let given_name = format!("Alfred{suffix}");
    let family_name = format!("Batman{suffix}");
    let username = format!("alfredo{suffix}");

    // get all users and check the admin user
    let res = client.get(&url).headers(admin.clone()).send().await?;
    let res = expect_status!(res, 200);
    let users = res.json::<Vec<UserResponseSimple>>().await?;

    // post a new user
    let new_user = NewUserRequest {
        given_name: Some(given_name.clone()),
        family_name: Some(family_name.clone()),
        email: format!("alfred-{suffix}@batcave.io"),
        preferred_username: Some(username.clone()),
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
    // make sure this test's user does not exist
    assert!(!users.iter().any(|u| u.email == new_user.email));

    let res = client
        .post(&url)
        .headers(admin.clone())
        .json(&new_user)
        .send()
        .await?;
    let res = expect_status!(res, 200);

    let alfred = res.json::<UserResponse>().await?;
    assert_eq!(alfred.email, new_user.email);
    assert_eq!(alfred.given_name.as_deref(), Some(given_name.as_str()));
    assert_eq!(alfred.family_name.as_deref(), Some(family_name.as_str()));
    assert!(alfred.roles.iter().any(|i| i == "admin"));
    assert!(alfred.roles.iter().any(|i| i == "user"));
    assert!(
        !alfred.roles.iter().any(|i| i == "non_existent"),
        "roles sanitization"
    );
    assert!(alfred.groups.is_some());
    assert!(alfred.groups.as_ref().unwrap().iter().any(|i| i == "admin"));
    assert!(alfred.groups.as_ref().unwrap().iter().any(|i| i == "user"));
    assert!(
        !alfred
            .groups
            .as_ref()
            .unwrap()
            .iter()
            .any(|i| i == "non_existent"),
        "groups sanitization"
    );
    assert_eq!(alfred.enabled, true);
    assert_eq!(alfred.email_verified, false);

    // get the new user by id
    let url_id = format!("{backend}/users/{}", alfred.id);
    let res = client.get(&url_id).headers(admin.clone()).send().await?;
    let user_by_id = expect_status_json!(res, 200, UserResponse);
    assert_eq!(user_by_id.email, new_user.email);
    // the preferred_username given at creation must round-trip onto the created user
    assert_eq!(
        user_by_id.user_values.preferred_username.as_deref(),
        Some(username.as_str())
    );

    // get the new user by email
    let url_email = format!("{backend}/users/email/{}", alfred.email);
    let res = client.get(&url_email).headers(admin.clone()).send().await?;
    let user_by_email = expect_status_json!(res, 200, UserResponse);
    assert_eq!(user_by_email.id, alfred.id);

    // the `username` search idx lives in the `users_values` table
    let searched = search(
        &admin,
        &Search {
            idx: "username",
            q: &username,
        },
    )
    .await?;
    assert_eq!(searched.len(), 1, "search for '{username}'");
    assert_eq!(searched[0].id, alfred.id);

    // an unknown username matches nobody
    let searched = search(
        &admin,
        &Search {
            idx: "username",
            q: "nobody",
        },
    )
    .await?;
    assert!(searched.is_empty(), "search for 'nobody'");

    // the `name` search idx matches `given_name` / `family_name` and is case-insensitive on both
    // databases (`ILIKE` on Postgres)
    let searched = search(
        &admin,
        &Search {
            idx: "name",
            q: &family_name,
        },
    )
    .await?;
    assert_eq!(searched.len(), 1, "search for '{family_name}'");
    assert_eq!(searched[0].id, alfred.id);

    // the very same query in lower case
    let searched = search(
        &admin,
        &Search {
            idx: "name",
            q: &family_name.to_lowercase(),
        },
    )
    .await?;
    assert_eq!(
        searched.len(),
        1,
        "search for '{}'",
        family_name.to_lowercase()
    );

    // `given_name` is matched as well
    let searched = search(
        &admin,
        &Search {
            idx: "name",
            q: &given_name,
        },
    )
    .await?;
    assert_eq!(searched.len(), 1, "search for '{given_name}'");
    assert_eq!(searched[0].email, new_user.email);

    // a name nobody has matches nobody
    let searched = search(
        &admin,
        &Search {
            idx: "name",
            q: "nobody",
        },
    )
    .await?;
    assert!(searched.is_empty(), "search for name 'nobody'");

    // a user with non-ASCII names and a `+` in the email for the searches below
    let umlaut_name = format!("Gödel{suffix}");
    let new_umlaut_user = NewUserRequest {
        given_name: Some(format!("André{suffix}")),
        family_name: Some(umlaut_name.clone()),
        email: format!("andre+tag{suffix}@batcave.io"),
        preferred_username: None,
        language: Language::En,
        roles: vec!["user".to_string()],
        groups: None,
        user_expires: None,
        tz: None,
    };
    let res = client
        .post(&url)
        .headers(admin.clone())
        .json(&new_umlaut_user)
        .send()
        .await?;
    let umlaut_user = expect_status_json!(res, 200, UserResponse);

    // non-ASCII queries survive the `q` validation and the query string round-trip
    let searched = search(
        &admin,
        &Search {
            idx: "name",
            q: &umlaut_name,
        },
    )
    .await?;
    assert_eq!(searched.len(), 1, "search for '{umlaut_name}'");
    assert_eq!(searched[0].id, umlaut_user.id);

    // `%2B` has to arrive as a `+` - a raw `+` in a query string would decode as a space
    let searched = search(
        &admin,
        &Search {
            idx: "email",
            q: &format!("%2Btag{suffix}"),
        },
    )
    .await?;
    assert_eq!(searched.len(), 1, "search for '+tag{suffix}'");
    assert_eq!(searched[0].email, new_umlaut_user.email);

    // an idx that only exists on sessions is a BadRequest for users
    let res = client
        .get(format!("{backend}/search?ty=user&idx=ip&q=127"))
        .headers(admin.clone())
        .send()
        .await?;
    expect_status!(res, 400);

    // an admin may clear a required value (`given_name` by default) to make the user set it again
    let res = client
        .patch(&url_id)
        .headers(admin.clone())
        .json(&json!({ "put": [], "del": ["given_name"] }))
        .send()
        .await?;
    assert_eq!(res.status(), StatusCode::OK);
    let patched = res.json::<UserResponse>().await?;
    assert_eq!(patched.given_name, None);

    // delete the user again
    let res = client.delete(&url_id).headers(admin.clone()).send().await?;
    expect_status!(res, 204);

    // get all users again and check that we are back to only 1
    let res = client.get(&url).headers(admin.clone()).send().await?;
    let res = expect_status!(res, 200);

    let users = res.json::<Vec<UserResponseSimple>>().await?;
    assert!(!users.iter().any(|u| u.id == alfred.id));

    // the second user of this test goes the same way
    let res = client
        .delete(format!("{backend}/users/{}", umlaut_user.id))
        .headers(admin.clone())
        .send()
        .await?;
    expect_status!(res, 204);

    Ok(())
}

#[tokio::test]
async fn test_password_reset_always_ok() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let client = reqwest::Client::new();
    let user = TestUser::new("Reset always ok", USER_PASSWORD).await?;

    // get password reset for an existing user
    let mut payload = RequestResetRequest {
        email: user.user.email.clone(),
        redirect_uri: None,
        pow: get_solved_pow().await,
    };
    let url = format!("{}/users/request_reset", get_backend_url());
    let res = client
        .post(&url)
        .headers(admin.clone())
        .json(&payload)
        .send()
        .await?;
    expect_status!(res, 200);

    // we should always get back an HTTP 200 for username enumeration prevention
    payload.email = "idonotexist@iamaghost.io".to_string();
    payload.pow = get_solved_pow().await;
    let res = client
        .post(&url)
        .headers(admin.clone())
        .json(&payload)
        .send()
        .await?;
    expect_status!(res, 200);

    user.delete().await
}

#[tokio::test]
async fn test_userinfo() -> Result<(), Box<dyn Error>> {
    let url = format!("{}/oidc/userinfo", get_backend_url());
    let client = reqwest::Client::new();

    // Unauthorized without a Bearer
    let res = client.get(&url).send().await?;
    expect_status!(res, 401);

    // This should be good
    let ts = get_token_set().await;
    let res = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", ts.access_token))
        .send()
        .await?;
    let info = expect_status_json!(res, 200, Userinfo);
    assert_eq!(info.sub, "m4PJ3TnyP32LA8hzY23deme3");
    assert_eq!(info.name, "Admin Init");
    assert!(info.roles.iter().any(|i| i == "rauthy_admin"));

    // `init_client` has the `profile` scope, the admin has no `preferred_username`
    // -> `email_fallback` (default: true) must apply, like in the ID token
    let ts = get_token_set_init_client().await;
    let res = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", ts.access_token))
        .send()
        .await?;
    let info = expect_status_json!(res, 200, Userinfo);
    assert!(info.email.is_some());
    assert_eq!(info.preferred_username, info.email);

    Ok(())
}

#[tokio::test]
async fn test_user_picture() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let client = reqwest::Client::new();
    let backend = get_backend_url();

    // the user whose picture is read, and a second one it must never be able to read
    let user = TestUser::new("Picture", USER_PASSWORD).await?;
    let other = TestUser::new("Picture other", USER_PASSWORD).await?;
    let url_user = format!("{backend}/users/{}", user.user.id);
    let url_other = format!("{backend}/users/{}", other.user.id);

    // upload a picture
    let url_picture = format!("{url_user}/picture");
    let picture_id = upload_picture(&client, &url_user, &admin).await?;
    assert!(!picture_id.is_empty());

    let url = format!("{url_picture}/{picture_id}");

    // make sure we get 401 without authn
    let res = client.get(&url).send().await?;
    expect_status!(res, 401);

    // success with valid session
    let res = client.get(&url).headers(admin.clone()).send().await?;
    let res = expect_status!(res, 200);
    assert_eq!(
        res.headers().get("content-type").unwrap().to_str().unwrap(),
        "image/webp"
    );
    let body = res.bytes().await?;
    assert!(!body.is_empty());

    // success with proper API key
    let key = ApiKeyRequest {
        name: new_store_id(),
        exp: None,
        access: vec![ApiKeyAccess {
            group: AccessGroup::Users,
            access_rights: vec![AccessRights::Read],
        }],
    };
    let key_header = create_api_key(&client, &admin, &key).await?;
    let res = client
        .get(&url)
        .header(AUTHORIZATION, key_header)
        .send()
        .await?;
    expect_status!(res, 200);

    // not allowed with token - `rauthy` client does not have the `profile` scope
    let ts = get_token_set().await;
    let res = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", ts.access_token))
        .send()
        .await?;
    expect_status!(res, 401);

    // success with access token - `profile` is in default scopes for `init_client`
    let ts = password_token_set(&client, &user.user.email, USER_PASSWORD).await;
    let res = client
        .get(&url)
        .header(AUTHORIZATION, format!("Bearer {}", ts.access_token))
        .send()
        .await?;
    expect_status!(res, 200);

    // success with id token
    let res = client
        .get(&url)
        .header(
            AUTHORIZATION,
            format!("Bearer {}", ts.id_token.clone().unwrap()),
        )
        .send()
        .await?;
    expect_status!(res, 200);

    // make sure access with token for another user fails
    let other_picture = upload_picture(&client, &url_other, &admin).await?;
    assert!(!other_picture.is_empty());

    let url_admin = format!("{url_other}/picture/{other_picture}");
    let res = client
        .get(&url_admin)
        .header(AUTHORIZATION, format!("Bearer {}", ts.access_token))
        .send()
        .await?;
    expect_status!(res, 401);
    let res = client
        .get(&url_admin)
        .header(AUTHORIZATION, format!("Bearer {}", ts.id_token.unwrap()))
        .send()
        .await?;
    expect_status!(res, 401);

    for user in [&user, &other] {
        user.delete().await?;
    }

    Ok(())
}

#[tokio::test]
async fn test_api_key_passkey_delete_authorization() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let client = reqwest::Client::new();
    let backend = get_backend_url();
    let user = TestUser::new("Pk delete", USER_PASSWORD).await?;

    let key = ApiKeyRequest {
        name: new_store_id(),
        exp: None,
        access: vec![ApiKeyAccess {
            group: AccessGroup::Users,
            access_rights: vec![AccessRights::Read],
        }],
    };
    let key_header = create_api_key(&client, &admin, &key).await?;

    let delete_url = format!("{}/users/{}/webauthn/delete/missing", backend, user.user.id);
    let res = client
        .delete(&delete_url)
        .header(AUTHORIZATION, &key_header)
        .json(&json!({}))
        .send()
        .await?;
    expect_status!(res, 401);

    let key = ApiKeyRequest {
        access: vec![ApiKeyAccess {
            group: AccessGroup::Users,
            access_rights: vec![AccessRights::Delete],
        }],
        ..key
    };
    let res = client
        .put(format!("{backend}/api_keys/{}", key.name))
        .headers(admin.clone())
        .json(&key)
        .send()
        .await?;
    expect_status!(res, 200);

    let res = client
        .delete(&delete_url)
        .header(AUTHORIZATION, &key_header)
        .json(&json!({}))
        .send()
        .await?;
    expect_status!(res, 200);

    let res = client
        .delete(format!("{backend}/api_keys/{}", key.name))
        .headers(admin.clone())
        .send()
        .await?;
    expect_status!(res, 200);
    user.delete().await
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
            .json(
                &UserUpdate::new(user)
                    .given_name(given_name.clone())
                    .roles(roles.clone())
                    .build(),
            )
            .send()
            .await?;
        expect_status!(res, 200);
        for login in logins {
            login.await?;
        }

        let res = client.get(&url_user).headers(admin.clone()).send().await?;
        let found = expect_status_json!(res, 200, UserResponse);
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
    let admin = get_auth_headers().await?;
    let client = Client::new();
    let user = TestUser::new("Login", USER_PASSWORD).await?;
    let email = user.user.email.clone();

    assert_admin_update_survives_logins(&client, &admin, &user.user, async || {
        let mut prepared = Vec::new();
        for _ in 0..6 {
            prepared.push(prepare_login(&email, USER_PASSWORD).await);
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

    user.delete().await
}

/// An admin update of a user made while password grant logins of that user wait for their hash
/// survives the logins.
#[tokio::test]
async fn test_admin_update_survives_concurrent_password_grants() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let client = Client::new();
    let user = TestUser::new("Login", USER_PASSWORD).await?;
    let email = user.user.email.clone();

    assert_admin_update_survives_logins(&client, &admin, &user.user, async || {
        (0..6)
            .map(|_| {
                let email = email.clone();
                tokio::spawn(async move {
                    password_token_set(&Client::new(), &email, USER_PASSWORD).await;
                })
            })
            .collect()
    })
    .await?;

    user.delete().await
}

/// An admin update of a user made while PAM password logins of that user wait for their hash
/// survives the logins: a PAM login saves only its login state, not the copy of the user it read
/// before.
#[tokio::test]
async fn test_admin_update_survives_concurrent_pam_logins() -> Result<(), Box<dyn Error>> {
    let pam_username = format!("pamlogins{}", new_store_id().to_lowercase());
    let hostname = format!("pam-logins-{}", new_store_id().to_lowercase());

    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();
    let user = TestUser::new("Pam", USER_PASSWORD).await?;
    let url_user = format!("{backend}/users/{}", user.user.id);

    let res = client
        .post(format!("{backend}/pam/users"))
        .headers(admin.clone())
        .json(&json!({ "username": pam_username, "email": user.user.email }))
        .send()
        .await?;
    let pam_user = expect_status_json!(res, 200, Value);

    let res = client
        .post(format!("{backend}/pam/hosts"))
        .headers(admin.clone())
        .json(&json!({
            "hostname": hostname,
            "gid": pam_user["gid"],
            "force_mfa": false,
            "local_password_only": true,
        }))
        .send()
        .await?;
    let res = expect_status!(res, 200);
    let host_id = res.json::<Value>().await?["id"]
        .as_str()
        .unwrap()
        .to_string();
    let res = client
        .post(format!("{backend}/pam/hosts/{host_id}/secret"))
        .headers(admin.clone())
        .send()
        .await?;
    let res = expect_status!(res, 200);
    let host_secret = res.json::<Value>().await?["secret"]
        .as_str()
        .unwrap()
        .to_string();

    // A failed PAM login is counted, and a successful one resets the count. The reset is checked
    // on its own, before the race below: an admin save there writes back the count of the copy
    // it read, so a seeded count read before a login reset it could come back.
    let pam = PamLogin {
        client: client.clone(),
        backend: backend.clone(),
        host_id: host_id.clone(),
        host_secret: host_secret.clone(),
        username: pam_username.clone(),
    };
    assert_eq!(pam.login(WRONG_PWD).await, 401);
    assert_eq!(failed_logins(&client, &admin, &url_user).await, Some(1));
    assert_eq!(pam.login(USER_PASSWORD).await, 200);
    assert_eq!(failed_logins(&client, &admin, &url_user).await, None);

    assert_admin_update_survives_logins(&client, &admin, &user.user, async || {
        (0..6)
            .map(|_| {
                let pam = pam.clone();
                tokio::spawn(async move {
                    expect_status!(pam.send_login(USER_PASSWORD).await, 200);
                })
            })
            .collect()
    })
    .await?;

    let res = client
        .delete(format!("{backend}/pam/hosts/{host_id}"))
        .headers(admin.clone())
        .send()
        .await?;
    expect_status!(res, 200);
    user.delete().await
}

/// A picture uploaded while password logins of the same user wait for their hash survives the
/// logins: a login saves only its login state, not the copy of the user it read before.
#[tokio::test]
async fn test_user_picture_survives_concurrent_logins() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();
    let user = TestUser::new("Login", USER_PASSWORD).await?;
    let email = user.user.email.clone();

    let url_user = format!("{backend}/users/{}", user.user.id);
    for round in 0..3 {
        let mut prepared = Vec::new();
        for _ in 0..6 {
            prepared.push(prepare_login(&email, USER_PASSWORD).await);
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
        let found = expect_status_json!(res, 200, UserResponse);
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
        expect_status!(res, 200, "round {round}");
    }

    user.delete().await
}

/// A picture uploaded while wrong password logins of the same user, through `/oidc/authorize` or
/// the password grant, wait for their hash survives the logins, and every failure is counted.
#[tokio::test]
async fn test_user_picture_survives_concurrent_failed_logins() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();
    let user = TestUser::new("Login", USER_PASSWORD).await?;
    let email = user.user.email.clone();
    let url_user = format!("{backend}/users/{}", user.user.id);

    for (round, authorize) in [true, true, false, false].into_iter().enumerate() {
        // two failures only: from the third one of this IP or user on, each is delayed by seconds
        let logins = if authorize {
            let mut prepared = Vec::new();
            for _ in 0..2 {
                prepared.push(prepare_login(&email, WRONG_PWD).await);
            }
            prepared
                .into_iter()
                .map(|login| tokio::spawn(async move { login.response().await.status() }))
                .collect::<Vec<_>>()
        } else {
            (0..2)
                .map(|_| {
                    let email = email.clone();
                    tokio::spawn(async move {
                        password_grant(&Client::new(), &email, WRONG_PWD)
                            .await
                            .status()
                    })
                })
                .collect()
        };
        // upload while the logins wait for their password hash, one at a time
        time::sleep(Duration::from_millis(20)).await;
        let picture_id = upload_picture(&client, &url_user, &admin).await?;
        let expected = if authorize {
            StatusCode::BAD_REQUEST
        } else {
            StatusCode::UNAUTHORIZED
        };
        for login in logins {
            assert_eq!(login.await?, expected, "round {round}");
        }

        let res = client.get(&url_user).headers(admin.clone()).send().await?;
        let found = expect_status_json!(res, 200, UserResponse);
        assert_eq!(
            found.picture_id.as_deref(),
            Some(picture_id.as_str()),
            "round {round}"
        );
        assert_eq!(found.failed_login_attempts, Some(2), "round {round}");
        assert!(found.last_failed_login.is_some());

        // a successful login resets the failed logins of the user and of this IP
        password_token_set(&client, &email, USER_PASSWORD).await;
    }

    user.delete().await
}

/// An admin save of a user keeps its sessions and refresh tokens while the user stays enabled,
/// and drops them once the save disables the user: they stay invalid after it is enabled again.
/// Those of another user are kept.
#[tokio::test]
async fn test_save_disabled_user_drops_sessions_and_refresh_tokens() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();
    let user = TestUser::new("Disable", USER_PASSWORD).await?;
    let other = TestUser::new("Disable other", USER_PASSWORD).await?;
    let url_user = format!("{backend}/users/{}", user.user.id);
    let url_session = format!("{backend}/oidc/sessioninfo");

    let ts = password_token_set(&client, &user.user.email, USER_PASSWORD).await;
    let session = session_headers_with(&user.user.email, USER_PASSWORD).await;
    let other_ts = password_token_set(&client, &other.user.email, USER_PASSWORD).await;
    let other_session = session_headers_with(&other.user.email, USER_PASSWORD).await;

    let save = |enabled: bool| save_user(&client, &admin, &url_user, &user.user, enabled);

    // a save of the enabled user keeps both
    save(true).await;
    let res = client
        .get(&url_session)
        .headers(session.clone())
        .send()
        .await?;
    expect_status!(res, 200);
    let res = refresh(&client, ts.refresh_token.clone().unwrap()).await;
    let ts = expect_status_json!(res, 200, TokenSet);

    // a save that disables the user drops both, for good
    save(false).await;
    save(true).await;
    let res = client.get(&url_session).headers(session).send().await?;
    expect_status!(res, 401);
    assert_refresh_not_found(&client, ts.refresh_token.unwrap()).await;

    // the other user keeps both
    let res = client
        .get(&url_session)
        .headers(other_session)
        .send()
        .await?;
    expect_status!(res, 200);
    let res = refresh(&client, other_ts.refresh_token.unwrap()).await;
    expect_status!(res, 200);

    for user in [&user, &other] {
        user.delete().await?;
    }

    Ok(())
}

/// An admin update that sets a password, and so reads the user long before it saves it, keeps a
/// picture removed or uploaded meanwhile: a full-row save does not write `picture_id`.
#[tokio::test]
async fn test_admin_update_keeps_concurrent_picture_change() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let client = Client::new();
    let user = TestUser::new("Picture", USER_PASSWORD).await?;
    let email = user.user.email.clone();
    let url_user = format!("{}/users/{}", get_backend_url(), user.user.id);
    let uploaded = upload_picture(&client, &url_user, &admin).await?;

    // round 0 removes the picture, round 1 uploads a new one
    for round in 0_usize..2 {
        // the password the update of the previous round left behind: the logins go ahead of the
        // update in the hashing queue, so they authenticate against it
        let login_pwd = if round == 0 {
            USER_PASSWORD.to_string()
        } else {
            format!("{USER_PASSWORD}{}", round - 1)
        };
        let logins = (0..3)
            .map(|_| {
                let email = email.clone();
                let login_pwd = login_pwd.clone();
                tokio::spawn(async move {
                    password_token_set(&Client::new(), &email, &login_pwd).await;
                })
            })
            .collect::<Vec<_>>();
        let update = tokio::spawn(
            client
                .put(&url_user)
                .headers(admin.clone())
                .json(
                    &UserUpdate::new(&user.user)
                        .password(format!("{USER_PASSWORD}{round}"))
                        .build(),
                )
                .send(),
        );
        time::sleep(Duration::from_millis(20)).await;
        let picture_id = if round == 0 {
            let res = client
                .delete(format!("{url_user}/picture/{uploaded}"))
                .headers(admin.clone())
                .send()
                .await?;
            expect_status!(res, 200);
            None
        } else {
            Some(upload_picture(&client, &url_user, &admin).await?)
        };
        assert!(
            !update.is_finished(),
            "round {round}: the picture change must land while the update waits for its hash"
        );
        expect_status!(update.await??, 200, "round {round}");
        for login in logins {
            login.await?;
        }

        let res = client.get(&url_user).headers(admin.clone()).send().await?;
        let found = expect_status_json!(res, 200, UserResponse);
        assert_eq!(found.picture_id, picture_id, "round {round}");
    }

    user.delete().await
}

/// An admin update that changes the email of a user drops the user cached under the old email.
#[tokio::test]
async fn test_admin_email_change_drops_cached_old_email() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();
    let user = TestUser::new("Email", USER_PASSWORD).await?;
    let new_email = format!("email-changed-{}@localhost", new_store_id().to_lowercase());
    let url_user = format!("{backend}/users/{}", user.user.id);
    let url_by_email = |email: &str| format!("{backend}/users/email/{email}");

    // the lookup by email caches the user under it
    let res = client
        .get(url_by_email(&user.user.email))
        .headers(admin.clone())
        .send()
        .await?;
    expect_status!(res, 200);

    let res = client
        .put(&url_user)
        .headers(admin.clone())
        .json(&UserUpdate::new(&user.user).email(new_email.clone()).build())
        .send()
        .await?;
    expect_status!(res, 200);

    let res = client
        .get(url_by_email(&user.user.email))
        .headers(admin.clone())
        .send()
        .await?;
    expect_status!(res, 404);
    let res = client
        .get(url_by_email(&new_email))
        .headers(admin.clone())
        .send()
        .await?;
    let res = expect_status!(res, 200);
    assert_eq!(res.json::<UserResponse>().await?.id, user.user.id);

    let res = client
        .delete(&url_user)
        .headers(admin.clone())
        .send()
        .await?;
    expect_status!(res, 204);

    Ok(())
}

/// A picture upload, which reads the user before it receives the picture, keeps an admin update
/// made while it waits for the rest of its body, and so does a later picture removal: both write
/// only the `picture_id` of their user, and leave another user alone.
#[tokio::test]
async fn test_picture_change_keeps_concurrent_admin_update() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();
    let user = TestUser::new("Picture", USER_PASSWORD).await?;
    let other = TestUser::new("Picture other", USER_PASSWORD).await?;
    let url_user = format!("{backend}/users/{}", user.user.id);
    let url_other = format!("{backend}/users/{}", other.user.id);
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
        .json(
            &UserUpdate::new(&user.user)
                .roles(roles.clone())
                .enabled(false)
                .build(),
        )
        .send()
        .await?;
    expect_status!(res, 200);
    assert!(
        !upload.is_finished(),
        "the admin update must land while the upload waits for its body"
    );
    send_tail.send(()).unwrap();
    let res = upload.await??;
    let picture_id = expect_status!(res, 200).text().await?;

    let check = CheckUsers {
        client: &client,
        admin: &admin,
        backend: &backend,
        url_user: &url_user,
        other_id: &other.user.id,
        other_picture: &other_picture,
        roles: &roles,
    };
    check.picture(Some(&picture_id)).await?;

    let res = client
        .delete(format!("{url_user}/picture/{picture_id}"))
        .headers(admin.clone())
        .send()
        .await?;
    expect_status!(res, 200);
    check.picture(None).await?;

    for user in [&user, &other] {
        user.delete().await?;
    }

    Ok(())
}

/// One of the user search idx endpoints.
struct Search<'a> {
    idx: &'a str,
    q: &'a str,
}

/// A search for users on a single idx.
async fn search(
    admin: &HeaderMap,
    req: &Search<'_>,
) -> Result<Vec<UserResponseSimple>, Box<dyn Error>> {
    let res = reqwest::Client::new()
        .get(format!(
            "{}/search?ty=user&idx={}&q={}",
            get_backend_url(),
            req.idx,
            req.q
        ))
        .headers(admin.clone())
        .send()
        .await?;
    Ok(expect_status_json!(res, 200, Vec<UserResponseSimple>))
}

/// The consents handler: authorization, listing, validation, and that a revocation drops the
/// refresh tokens of the user for that client, for the user itself and for an admin.
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

    let user = TestUser::new("Consent", USER_PASSWORD).await?;
    let email = user.user.email.clone();

    let url = format!("{backend}/users/{}/consents", user.user.id);
    // without `resource` for a consent without one
    let url_delete = format!("{url}?client_id={CLIENT_ID}");

    // --- authorization
    let res = client.get(&url).send().await?;
    expect_status!(res, 401);
    let res = client.delete(&url_delete).send().await?;
    expect_status!(res, 401);

    let user_headers = session_headers_with(&email, USER_PASSWORD).await;
    let res = client
        .get(format!("{backend}/users/{admin_id}/consents"))
        .headers(user_headers.clone())
        .send()
        .await?;
    expect_status!(res, 403);
    let res = client
        .delete(format!(
            "{backend}/users/{admin_id}/consents?client_id={CLIENT_ID}"
        ))
        .headers(user_headers.clone())
        .send()
        .await?;
    expect_status!(res, 403);

    // --- list, the user itself and an admin
    for headers in [&user_headers, &admin] {
        let res = client.get(&url).headers(headers.clone()).send().await?;
        let res = expect_status!(res, 200);
        assert!(res.json::<Vec<UserConsentResponse>>().await?.is_empty());
    }
    let res = client
        .get(format!("{url}?resource=https%3A%2F%2Fapi.example.com"))
        .headers(user_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);
    // an empty filter selects the consents without a resource
    let res = client
        .get(format!("{url}?resource="))
        .headers(user_headers.clone())
        .send()
        .await?;
    let res = expect_status!(res, 200);
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
    expect_status!(res, 400);

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
        expect_status!(res, 400, "{query}");
    }

    // --- revocation drops the refresh tokens of the user for the client
    let ts = password_token_set(&client, &email, USER_PASSWORD).await;
    assert_consent_exempt(&ts);

    time::sleep(Duration::from_secs(2)).await;
    let res = refresh(&client, ts.refresh_token.unwrap()).await;
    let ts = expect_status_json!(res, 200, TokenSet);
    // the rotation keeps the exemption
    assert_consent_exempt(&ts);

    // another user's refresh token for the same client
    let other = password_token_set(&client, USERNAME, PASSWORD).await;

    let userinfo = UserInfo {
        client: client.clone(),
        backend: backend.clone(),
    };
    assert_eq!(userinfo.status(&ts.access_token).await, 200);

    let events_from = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
    let res = client
        .delete(&url_delete)
        .headers(user_headers.clone())
        .send()
        .await?;
    let deleted = expect_status_json!(res, 200, UserConsentDeleteResponse);
    // the password grant does not ask for consent, there is no consent row to delete
    assert!(!deleted.consent_deleted);
    // the rotated predecessor may still be there with a lowered `exp`
    assert!(deleted.refresh_tokens_deleted >= 1);

    assert_refresh_not_found(&client, ts.refresh_token.unwrap()).await;
    // the access token issued with the deleted refresh token is revoked as well
    assert_eq!(userinfo.status(&ts.access_token).await, 401);
    // the other user keeps theirs
    let res = refresh(&client, other.refresh_token.unwrap()).await;
    expect_status!(res, 200);

    // an admin, then idempotent
    let ts = password_token_set(&client, &email, USER_PASSWORD).await;
    let res = client
        .delete(&url_delete)
        .headers(admin.clone())
        .send()
        .await?;
    let deleted = expect_status_json!(res, 200, UserConsentDeleteResponse);
    assert!(!deleted.consent_deleted);
    assert!(deleted.refresh_tokens_deleted >= 1);
    assert_refresh_not_found(&client, ts.refresh_token.unwrap()).await;
    let res = client
        .delete(&url_delete)
        .headers(admin.clone())
        .send()
        .await?;
    let deleted = expect_status_json!(res, 200, UserConsentDeleteResponse);
    assert!(!deleted.consent_deleted);
    assert_eq!(deleted.refresh_tokens_deleted, 0);

    // one event for each revoke that deleted something, each naming who revoked, none for the
    // no-op
    assert_eq!(
        consent_revoked_events(&client, &admin, events_from, &email).await,
        vec![
            format!("Consent of `{email}` for client `{CLIENT_ID}` revoked by admin `{USERNAME}`"),
            format!("Consent of `{email}` for client `{CLIENT_ID}` revoked by the user"),
        ]
    );

    // --- a revoke with a `resource` drops the refresh tokens for the client as a whole
    let ts = password_token_set(&client, &email, USER_PASSWORD).await;
    let res = client
        .delete(format!(
            "{url}?client_id={CLIENT_ID}&resource=https%3A%2F%2Fapi.example.com"
        ))
        .headers(user_headers.clone())
        .send()
        .await?;
    let deleted = expect_status_json!(res, 200, UserConsentDeleteResponse);
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
    expect_status!(res, 404);

    // --- cleanup
    user.delete().await
}

/// An API key with `Users` access reads and revokes the consents of any user, each with its
/// own access right.
#[tokio::test]
async fn test_user_consents_api_key() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();
    let user = TestUser::new("Consent", USER_PASSWORD).await?;

    let url = format!("{backend}/users/{}/consents", user.user.id);
    let url_delete = format!("{url}?client_id={CLIENT_ID}");

    let mut key = ApiKeyRequest {
        name: new_store_id(),
        exp: None,
        access: vec![ApiKeyAccess {
            group: AccessGroup::Clients,
            access_rights: vec![AccessRights::Read, AccessRights::Delete],
        }],
    };
    let key_header = create_api_key(&client, &admin, &key).await?;
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
        expect_status!(res, 200);
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
    let ts = password_token_set(&client, &user.user.email, USER_PASSWORD).await;
    let events_from = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
    let (status, deleted) = delete_with_session().await;
    assert_eq!(status, StatusCode::OK);
    assert!(deleted.unwrap().refresh_tokens_deleted >= 1);
    assert_refresh_not_found(&client, ts.refresh_token.unwrap()).await;
    assert_eq!(
        consent_revoked_events(&client, &admin, events_from, &user.user.email).await,
        vec![format!(
            "Consent of `{}` for client `{CLIENT_ID}` revoked by API key `{}`",
            user.user.email, key.name
        )]
    );

    let res = client
        .delete(format!("{backend}/api_keys/{}", key.name))
        .headers(admin.clone())
        .send()
        .await?;
    expect_status!(res, 200);
    user.delete().await
}

/// An admin `PUT /users/{id}` that keeps everything of the stored user and overrides single
/// fields. Every admin save of these tests verifies the email.
struct UserUpdate {
    req: UpdateUserRequest,
}

impl UserUpdate {
    fn new(user: &UserResponse) -> Self {
        Self {
            req: UpdateUserRequest {
                email: user.email.clone(),
                given_name: user.given_name.clone(),
                family_name: user.family_name.clone(),
                language: Some(Language::En),
                password: None,
                roles: user.roles.clone(),
                groups: user.groups.clone(),
                enabled: user.enabled,
                email_verified: true,
                user_expires: user.user_expires,
                user_values: None,
            },
        }
    }

    fn password(mut self, password: String) -> Self {
        self.req.password = Some(password);
        self
    }

    fn email(mut self, email: String) -> Self {
        self.req.email = email;
        self
    }

    fn given_name(mut self, given_name: String) -> Self {
        self.req.given_name = Some(given_name);
        self
    }

    fn roles(mut self, roles: Vec<String>) -> Self {
        self.req.roles = roles;
        self
    }

    fn enabled(mut self, enabled: bool) -> Self {
        self.req.enabled = enabled;
        self
    }

    fn build(self) -> UpdateUserRequest {
        self.req
    }
}

/// Creates an API key and returns its `Authorization` header value.
async fn create_api_key(
    client: &Client,
    admin: &HeaderMap,
    req: &ApiKeyRequest,
) -> Result<String, Box<dyn Error>> {
    let res = client
        .post(format!("{}/api_keys", get_backend_url()))
        .headers(admin.clone())
        .json(req)
        .send()
        .await?;
    let res = expect_status!(res, 200, format!("create api key {}", req.name));
    // the header needs to be in format: 'API-Key my_apy_key'
    Ok(format!("API-Key {}", res.text().await?))
}

/// The state every picture change of this test has to leave behind.
struct CheckUsers<'a> {
    client: &'a Client,
    admin: &'a HeaderMap,
    backend: &'a str,
    url_user: &'a str,
    other_id: &'a str,
    other_picture: &'a str,
    roles: &'a [String],
}

impl CheckUsers<'_> {
    /// The user with the changed picture, and the untouched picture of the other one.
    async fn picture(&self, picture_id: Option<&str>) -> Result<(), Box<dyn Error>> {
        let res = self
            .client
            .get(self.url_user)
            .headers(self.admin.clone())
            .send()
            .await?;
        let found = expect_status_json!(res, 200, UserResponse);
        assert_eq!(found.picture_id.as_deref(), picture_id);
        assert!(!found.enabled);
        let mut found_roles = found.roles.clone();
        found_roles.sort();
        assert_eq!(found_roles, self.roles);

        // the list reads the stored rows, not the cached users
        let res = self
            .client
            .get(format!("{}/users", self.backend))
            .headers(self.admin.clone())
            .send()
            .await?;
        let users = expect_status_json!(res, 200, Vec<UserResponseSimple>);
        let found = users.iter().find(|u| u.id == self.other_id).unwrap();
        assert_eq!(found.picture_id.as_deref(), Some(self.other_picture));
        Ok(())
    }
}

/// An admin `PUT /users/{id}` that only toggles `enabled`.
async fn save_user(
    client: &Client,
    admin: &HeaderMap,
    url_user: &str,
    user: &UserResponse,
    enabled: bool,
) {
    let res = client
        .put(url_user)
        .headers(admin.clone())
        .json(&UserUpdate::new(user).enabled(enabled).build())
        .send()
        .await
        .unwrap();
    expect_status!(res, 200, "save with enabled: {enabled}");
}

/// `GET /oidc/userinfo` with a bearer token.
struct UserInfo {
    client: Client,
    backend: String,
}

impl UserInfo {
    async fn status(&self, access_token: &str) -> StatusCode {
        self.client
            .get(format!("{}/oidc/userinfo", self.backend))
            .header(AUTHORIZATION, format!("Bearer {access_token}"))
            .send()
            .await
            .unwrap()
            .status()
    }
}

/// The PAM login endpoint of these tests, with everything one login needs.
#[derive(Clone)]
struct PamLogin {
    client: Client,
    backend: String,
    host_id: String,
    host_secret: String,
    username: String,
}

impl PamLogin {
    async fn login(&self, password: &str) -> StatusCode {
        let res = self.send_login(password).await;
        res.status()
    }

    async fn send_login(&self, password: &str) -> reqwest::Response {
        self.client
            .post(format!("{}/pam/login", self.backend))
            .json(&json!({
                "host_id": self.host_id,
                "host_secret": self.host_secret,
                "username": self.username,
                "password": password,
            }))
            .send()
            .await
            .unwrap()
    }
}

/// The failed logins stored for the user at `url_user`.
async fn failed_logins(client: &Client, admin: &HeaderMap, url_user: &str) -> Option<i64> {
    let res = client
        .get(url_user)
        .headers(admin.clone())
        .send()
        .await
        .unwrap();
    let res = expect_status!(res, 200);
    res.json::<UserResponse>()
        .await
        .unwrap()
        .failed_login_attempts
}

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
    let res = expect_status!(res, 200);
    Ok(res.text().await?)
}

/// The refresh of a revoked refresh token fails on the missing row.
async fn assert_refresh_not_found(client: &Client, refresh_token: String) {
    let res = refresh(client, refresh_token).await;
    let res = expect_status!(res, 404);
    let body = res.text().await.unwrap();
    assert!(body.contains("Refresh Token not found"), "{body}");
}

/// The password grant has no consent step, its refresh tokens are exempt from the consent check.
fn assert_consent_exempt(ts: &TokenSet) {
    let claims = decode_claims(ts.refresh_token.as_deref().unwrap());
    assert_eq!(claims["consent_exempt"], json!(true), "{claims}");
}

/// The texts of the `ConsentRevoked` events for `email` since `from`, sorted.
async fn consent_revoked_events(
    client: &Client,
    admin: &HeaderMap,
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
    let res = expect_status!(res, 200);
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
