use crate::common::{get_auth_headers, get_backend_url};
use pretty_assertions::assert_eq;
use rauthy_api_types::generic::Language;
use rauthy_api_types::users::{
    NewUserRequest, UpdateUserRequest, UserResponse, UserResponseSimple,
};
use reqwest::Client;
use serde_json::{Value, json};
use std::error::Error;
use std::time::Duration;
use tokio::time;

mod common;

const EMAIL: &str = "pam-logins@consenttest.io";
const PWD: &str = "Pam-TEST.1234!#$";
const USERNAME: &str = "pamlogins";
const HOSTNAME: &str = "pam-logins-host";

/// An admin update of a user made while PAM password logins of that user wait for their hash
/// survives the logins: a PAM login saves only its login state, not the copy of the user it read
/// before.
#[tokio::test]
async fn test_admin_update_survives_concurrent_pam_logins() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();

    // left-overs from a prior run, the host first, as it refers to the group of the PAM user
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
    let res = client
        .get(format!("{backend}/users"))
        .headers(admin.clone())
        .send()
        .await?;
    let users = res.json::<Vec<UserResponseSimple>>().await?;
    if let Some(user) = users.iter().find(|u| u.email == EMAIL) {
        let res = client
            .delete(format!("{backend}/users/{}", user.id))
            .headers(admin.clone())
            .send()
            .await?;
        assert_eq!(res.status(), 204);
    }

    // a user with a password, its PAM user and a host for the PAM user's own group
    let res = client
        .post(format!("{backend}/users"))
        .headers(admin.clone())
        .json(&NewUserRequest {
            given_name: Some("Pam".to_string()),
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
    let url_user = format!("{backend}/users/{}", user.id);
    let update = |given_name: &str, roles: &[&str], password: Option<&str>| UpdateUserRequest {
        email: user.email.clone(),
        given_name: Some(given_name.to_string()),
        family_name: None,
        language: Some(Language::En),
        password: password.map(String::from),
        roles: roles.iter().map(|r| r.to_string()).collect(),
        groups: None,
        enabled: true,
        email_verified: true,
        user_expires: None,
        user_values: None,
    };
    let res = client
        .put(&url_user)
        .headers(admin.clone())
        .json(&update("Pam", &["user"], Some(PWD)))
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let res = client
        .post(format!("{backend}/pam/users"))
        .headers(admin.clone())
        .json(&json!({ "username": USERNAME, "email": EMAIL }))
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

    for round in 0..3 {
        let given_name = format!("Admin round {round}");
        let roles: &[&str] = if round % 2 == 0 {
            &["admin", "user"]
        } else {
            &["user"]
        };

        let logins = (0..6)
            .map(|_| {
                let req = client.post(format!("{backend}/pam/login")).json(&json!({
                    "host_id": host_id,
                    "host_secret": host_secret,
                    "username": USERNAME,
                    "password": PWD,
                }));
                tokio::spawn(async move { req.send().await.unwrap().status() })
            })
            .collect::<Vec<_>>();
        // update while the logins wait for their password hash, one at a time
        time::sleep(Duration::from_millis(20)).await;
        let res = client
            .put(&url_user)
            .headers(admin.clone())
            .json(&update(&given_name, roles, None))
            .send()
            .await?;
        assert_eq!(res.status(), 200);
        for login in logins {
            assert_eq!(login.await?, 200, "round {round}");
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
