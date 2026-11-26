use crate::common::{get_auth_headers, get_backend_url};
use crate::{expect_status, expect_status_json};
use pretty_assertions::assert_eq;
use rauthy_api_types::generic::Language;
use rauthy_api_types::roles::RoleRequest;
use rauthy_api_types::users::{NewUserRequest, UserResponse, UserResponseSimple};
use rauthy_common::utils::new_store_id;
use rauthy_data::entity::roles::Role;
use std::error::Error;

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
    // the roles that existed before this test, by id: concurrent tests create and delete roles of
    // their own, so the list is verified by membership, never by its length
    let roles_orig = roles.iter().map(|r| r.id.clone()).collect::<Vec<_>>();

    // every name carries a unique suffix, and the rename target and the substring role extend it,
    // so the substring check keeps its meaning and no two runs ever use the same names
    let suffix = new_store_id();
    let name = format!("role{suffix}");
    let name_upd = format!("{name}456");
    let name_sub = format!("{name}45");
    let email = format!("role-rename-{suffix}@localhost.de");

    // add a role
    let new_role = RoleRequest {
        role: name.clone(),
        meta: None,
    };
    let res = reqwest::Client::new()
        .post(&url)
        .headers(auth_headers.clone())
        .json(&new_role)
        .send()
        .await?;
    let role = expect_status_json!(res, 200, Role);
    assert_eq!(role.name, name);

    // assign the role to a user, so the rename below must rewrite the membership
    let url_users = format!("{}/users", backend_url);
    let res = reqwest::Client::new()
        .post(&url_users)
        .headers(auth_headers.clone())
        .json(&NewUserRequest {
            given_name: Some("Role".to_string()),
            family_name: Some("Rename".to_string()),
            email: email.clone(),
            preferred_username: None,
            language: Language::En,
            roles: vec![name.clone()],
            groups: None,
            user_expires: None,
            tz: None,
        })
        .send()
        .await?;
    let user = expect_status_json!(res, 200, UserResponse);
    assert_eq!(
        user.roles,
        vec![name.clone()],
        "the new user has the new role"
    );

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
        role: name_upd.clone(),
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
    assert_eq!(
        user.roles,
        vec![name_upd.clone()],
        "the renamed role stays assigned to the user"
    );

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
            role: name_sub.clone(),
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
    let ids = roles.iter().map(|r| r.id.as_str()).collect::<Vec<_>>();
    for id in &roles_orig {
        assert!(
            ids.contains(&id.as_str()),
            "the pre-existing role {id} disappeared"
        );
    }
    assert!(
        !ids.contains(&role.id.as_str()) && !ids.contains(&sub_role.id.as_str()),
        "a role this test created was not deleted"
    );

    Ok(())
}
