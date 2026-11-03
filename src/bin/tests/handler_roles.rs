use crate::common::{get_auth_headers, get_backend_url};
use pretty_assertions::assert_eq;
use rauthy_api_types::generic::Language;
use rauthy_api_types::roles::RoleRequest;
use rauthy_api_types::users::{NewUserRequest, UserResponse};
use rauthy_data::entity::roles::Role;
use std::error::Error;

mod common;

#[tokio::test]
async fn test_roles() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();

    let url = format!("{}/roles", backend_url);
    let res = reqwest::Client::new()
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let roles = res.json::<Vec<Role>>().await?;
    let len_orig = roles.len();

    // add a role
    let new_role = RoleRequest {
        role: "role123".to_string(),
        meta: None,
    };
    let res = reqwest::Client::new()
        .post(&url)
        .headers(auth_headers.clone())
        .json(&new_role)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let role = res.json::<Role>().await?;
    assert_eq!(role.name, "role123");

    // assign the role to a user, so the rename below must rewrite the membership
    let url_users = format!("{}/users", backend_url);
    let res = reqwest::Client::new()
        .post(&url_users)
        .headers(auth_headers.clone())
        .json(&NewUserRequest {
            given_name: Some("Role".to_string()),
            family_name: Some("Rename".to_string()),
            email: "role-rename@localhost.de".to_string(),
            preferred_username: None,
            language: Language::En,
            roles: vec!["role123".to_string()],
            groups: None,
            user_expires: None,
            tz: None,
        })
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let user = res.json::<UserResponse>().await?;
    assert_eq!(user.roles, vec!["role123".to_string()]);

    // modify the role
    let upd_role = RoleRequest {
        role: "role456".to_string(),
        meta: None,
    };
    let url_name = format!("{}/{}", url, role.id);
    let res = reqwest::Client::new()
        .put(&url_name)
        .headers(auth_headers.clone())
        .json(&upd_role)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let upd_rl = res.json::<Role>().await?;
    assert_eq!(upd_role.role, upd_rl.name);

    // the user must have the renamed role, not lose it
    let url_user = format!("{}/{}", url_users, user.id);
    let res = reqwest::Client::new()
        .get(&url_user)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let user = res.json::<UserResponse>().await?;
    assert_eq!(user.roles, vec!["role456".to_string()]);

    let res = reqwest::Client::new()
        .delete(&url_user)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 204);

    // delete the role
    let url_del = format!("{}/{}", url, upd_rl.id);
    let res = reqwest::Client::new()
        .delete(&url_del)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // verify it is gone
    let res = reqwest::Client::new()
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    let roles = res.json::<Vec<Role>>().await?;
    assert_eq!(roles.len(), len_orig);

    Ok(())
}
