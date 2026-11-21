use crate::common::{get_auth_headers, get_backend_url};
use pretty_assertions::assert_eq;
use rauthy_api_types::generic::Language;
use rauthy_api_types::roles::RoleRequest;
use rauthy_api_types::users::{NewUserRequest, UserResponse, UserResponseSimple};
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
    let roles = expect_status_json!(res, 200, Vec<Role>);
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
    let role = expect_status_json!(res, 200, Role);
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
    let user = expect_status_json!(res, 200, UserResponse);
    assert_eq!(user.roles, vec!["role123".to_string()]);

    // the role's member list must contain exactly that user
    let url_members = format!("{}/roles/{}/users", backend_url, role.id);
    let res = reqwest::Client::new()
        .get(&url_members)
        .headers(auth_headers.clone())
        .send()
        .await?;
    let members = expect_status_json!(res, 200, Vec<UserResponseSimple>);
    assert_eq!(
        members.iter().map(|u| u.id.as_str()).collect::<Vec<_>>(),
        vec![user.id.as_str()],
    );

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
    let upd_rl = expect_status_json!(res, 200, Role);
    assert_eq!(upd_role.role, upd_rl.name);

    // the user must have the renamed role, not lose it
    let url_user = format!("{}/{}", url_users, user.id);
    let res = reqwest::Client::new()
        .get(&url_user)
        .headers(auth_headers.clone())
        .send()
        .await?;
    let user = expect_status_json!(res, 200, UserResponse);
    assert_eq!(user.roles, vec!["role456".to_string()]);

    // the member list follows the renamed role
    let res = reqwest::Client::new()
        .get(&url_members)
        .headers(auth_headers.clone())
        .send()
        .await?;
    let members = expect_status_json!(res, 200, Vec<UserResponseSimple>);
    assert_eq!(
        members.iter().map(|u| u.id.as_str()).collect::<Vec<_>>(),
        vec![user.id.as_str()],
    );

    // the member list must not contain users that are only a member of a role whose name
    // contains this one as a substring
    let res = reqwest::Client::new()
        .post(&url)
        .headers(auth_headers.clone())
        .json(&RoleRequest {
            role: "role4".to_string(),
            meta: None,
        })
        .send()
        .await?;
    let sub_role = expect_status_json!(res, 200, Role);
    let res = reqwest::Client::new()
        .get(format!("{}/roles/{}/users", backend_url, sub_role.id))
        .headers(auth_headers.clone())
        .send()
        .await?;
    let members = expect_status_json!(res, 200, Vec<UserResponseSimple>);
    assert!(
        members.is_empty(),
        "a substring must not match a role member"
    );

    // removing the user from the role via the same PATCH op the Admin UI sends empties the list
    let res = reqwest::Client::new()
        .patch(format!("{}/users/{}", backend_url, user.id))
        .headers(auth_headers.clone())
        .json(&serde_json::json!({
            "put": [],
            "del": ["roles"],
        }))
        .send()
        .await?;
    expect_status!(res, 200);
    let res = reqwest::Client::new()
        .get(&url_members)
        .headers(auth_headers.clone())
        .send()
        .await?;
    let members = expect_status_json!(res, 200, Vec<UserResponseSimple>);
    assert!(members.is_empty());

    // the sub-role is not needed anymore
    let res = reqwest::Client::new()
        .delete(format!("{}/{}", url, sub_role.id))
        .headers(auth_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);

    let res = reqwest::Client::new()
        .delete(&url_user)
        .headers(auth_headers.clone())
        .send()
        .await?;
    expect_status!(res, 204);

    // delete the role
    let url_del = format!("{}/{}", url, upd_rl.id);
    let res = reqwest::Client::new()
        .delete(&url_del)
        .headers(auth_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);

    // verify it is gone
    let res = reqwest::Client::new()
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    let res = expect_status!(res, 200);

    let roles = res.json::<Vec<Role>>().await?;
    assert_eq!(roles.len(), len_orig);

    Ok(())
}
