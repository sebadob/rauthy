use crate::common::{get_auth_headers, get_backend_url};
use pretty_assertions::assert_eq;
use rauthy_api_types::generic::Language;
use rauthy_api_types::groups::GroupRequest;
use rauthy_api_types::users::{NewUserRequest, UserResponse, UserResponseSimple};
use rauthy_data::entity::groups::Group;
use std::error::Error;

mod common;

#[tokio::test]
async fn test_groups() -> Result<(), Box<dyn Error>> {
    let auth_headers = get_auth_headers().await?;
    let backend_url = get_backend_url();

    let url = format!("{}/groups", backend_url);
    let res = reqwest::Client::new()
        .get(&url)
        .headers(auth_headers.clone())
        .send()
        .await?;
    let groups = expect_status_json!(res, 200, Vec<Group>);
    let len_orig = groups.len();

    // add a group
    let new_group = GroupRequest {
        group: "My Group 123".to_string(),
        meta: None,
    };
    let res = reqwest::Client::new()
        .post(&url)
        .headers(auth_headers.clone())
        .json(&new_group)
        .send()
        .await?;
    let group = expect_status_json!(res, 200, Group);
    assert_eq!(group.name, "My Group 123");

    // assign the group to a user, so the rename below must rewrite the membership
    let url_users = format!("{}/users", backend_url);
    let res = reqwest::Client::new()
        .post(&url_users)
        .headers(auth_headers.clone())
        .json(&NewUserRequest {
            given_name: Some("Group".to_string()),
            family_name: Some("Rename".to_string()),
            email: "group-rename@localhost.de".to_string(),
            preferred_username: None,
            language: Language::En,
            roles: vec!["user".to_string()],
            groups: Some(vec!["My Group 123".to_string()]),
            user_expires: None,
            tz: None,
        })
        .send()
        .await?;
    let user = expect_status_json!(res, 200, UserResponse);
    assert_eq!(user.groups, Some(vec!["My Group 123".to_string()]));

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
        group: "group456".to_string(),
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
    assert_eq!(user.groups, Some(vec!["group456".to_string()]));

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
            group: "group45".to_string(),
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
    assert_eq!(groups.len(), len_orig);

    Ok(())
}
