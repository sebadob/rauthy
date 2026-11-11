use crate::common::{
    assert_admin_update_survives_logins, create_login_user, get_auth_headers, get_backend_url,
};
use pretty_assertions::assert_eq;
use reqwest::Client;
use serde_json::{Value, json};
use std::error::Error;

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
    let user = create_login_user(&client, &admin, EMAIL, "Pam", PWD).await?;
    let url_user = format!("{backend}/users/{}", user.id);

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

    assert_admin_update_survives_logins(&client, &admin, &user, async || {
        (0..6)
            .map(|_| {
                let req = client.post(format!("{backend}/pam/login")).json(&json!({
                    "host_id": host_id,
                    "host_secret": host_secret,
                    "username": USERNAME,
                    "password": PWD,
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
