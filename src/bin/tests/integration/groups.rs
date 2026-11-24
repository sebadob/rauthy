use crate::common::{get_auth_headers, get_backend_url};
use crate::{expect_status, expect_status_json};
use pretty_assertions::assert_eq;
use rauthy_api_types::generic::Language;
use rauthy_api_types::groups::GroupRequest;
use rauthy_api_types::users::{NewUserRequest, UserResponse, UserResponseSimple};
use rauthy_common::utils::new_store_id;
use rauthy_data::entity::groups::Group;
use std::error::Error;

#[tokio::test]
async fn test_groups() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();

    let url = format!("{}/groups", backend_url);

    // every name carries a unique suffix, and the rename target and the substring group extend
    // it, so the substring check keeps its meaning and no two runs ever use the same names
    let suffix = new_store_id();
    let name = format!("My Group {suffix}");
    let name_upd = format!("{name}456");
    let name_sub = format!("{name}45");
    let email = format!("group-rename-{suffix}@localhost.de");

    // add a group
    let new_group = GroupRequest {
        group: name.clone(),
        meta: None,
    };
    let res = reqwest::Client::new()
        .post(&url)
        .headers(auth_headers.clone())
        .json(&new_group)
        .send()
        .await?;
    let group = expect_status_json!(res, 200, Group);
    assert_eq!(group.name, name);

    // assign the group to a user, so the rename below must rewrite the membership
    let url_users = format!("{}/users", backend_url);
    let res = reqwest::Client::new()
        .post(&url_users)
        .headers(auth_headers.clone())
        .json(&NewUserRequest {
            given_name: Some("Group".to_string()),
            family_name: Some("Rename".to_string()),
            email: email.clone(),
            preferred_username: None,
            language: Language::En,
            roles: vec!["user".to_string()],
            groups: Some(vec![name.clone()]),
            user_expires: None,
            tz: None,
        })
        .send()
        .await?;
    let user = expect_status_json!(res, 200, UserResponse);
    assert_eq!(
        user.groups,
        Some(vec![name.clone()]),
        "the new user has the new group"
    );

    // the group's member list must contain exactly that user
    let url_members = format!("{}/groups/{}/users", backend_url, group.id);
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

    // modify the group
    let upd_group = GroupRequest {
        group: name_upd.clone(),
        meta: None,
    };
    let url_name = format!("{}/{}", url, group.id);
    let res = reqwest::Client::new()
        .put(&url_name)
        .headers(auth_headers.clone())
        .json(&upd_group)
        .send()
        .await?;
    let upd_grp = expect_status_json!(res, 200, Group);
    assert_eq!(upd_group.group, upd_grp.name);

    // the user must have the renamed group, not lose it
    let url_user = format!("{}/{}", url_users, user.id);
    let res = reqwest::Client::new()
        .get(&url_user)
        .headers(auth_headers.clone())
        .send()
        .await?;
    let user = expect_status_json!(res, 200, UserResponse);
    assert_eq!(
        user.groups,
        Some(vec![name_upd.clone()]),
        "the renamed group stays assigned to the user"
    );

    // the member list follows the renamed group
    let res = reqwest::Client::new()
        .get(url_members.clone())
        .headers(auth_headers.clone())
        .send()
        .await?;
    let members = expect_status_json!(res, 200, Vec<UserResponseSimple>);
    assert_eq!(
        members.iter().map(|u| u.id.as_str()).collect::<Vec<_>>(),
        vec![user.id.as_str()],
    );

    // the member list must not contain users that are only a member of a group whose name
    // contains this one as a substring
    let res = reqwest::Client::new()
        .post(&url)
        .headers(auth_headers.clone())
        .json(&GroupRequest {
            group: name_sub.clone(),
            meta: None,
        })
        .send()
        .await?;
    let sub_group = expect_status_json!(res, 200, Group);
    let res = reqwest::Client::new()
        .get(format!("{}/groups/{}/users", backend_url, sub_group.id))
        .headers(auth_headers.clone())
        .send()
        .await?;
    let members = expect_status_json!(res, 200, Vec<UserResponseSimple>);
    assert!(
        members.is_empty(),
        "a substring must not match a group member"
    );

    // the sub-group is not needed anymore
    let res = reqwest::Client::new()
        .delete(format!("{}/{}", url, sub_group.id))
        .headers(auth_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);

    // removing the user from the group via the same PATCH op the Admin UI sends empties the list
    let res = reqwest::Client::new()
        .patch(format!("{}/users/{}", backend_url, user.id))
        .headers(auth_headers.clone())
        .json(&serde_json::json!({
            "put": [],
            "del": ["groups"],
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

    let res = reqwest::Client::new()
        .delete(&url_user)
        .headers(auth_headers.clone())
        .send()
        .await?;
    expect_status!(res, 204);

    // delete the group
    let url_del = format!("{}/{}", url, upd_grp.id);
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

    let groups = res.json::<Vec<Group>>().await?;
    let ids = groups.iter().map(|g| g.id.as_str()).collect::<Vec<_>>();
    // Only the groups this test created have to be gone. Which other groups exist is not this
    // test's invariant: other areas delete groups, and their cache invalidation is delayed, so a
    // group from the snapshot can legitimately disappear while this test runs.
    assert!(
        !ids.contains(&group.id.as_str()) && !ids.contains(&sub_group.id.as_str()),
        "a group this test created was not deleted"
    );

    Ok(())
}
