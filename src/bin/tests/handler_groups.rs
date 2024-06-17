use crate::common::{get_auth_headers, get_backend_url};
use pretty_assertions::assert_eq;
use rauthy_models::entity::groups::Group;
use rauthy_models::request::NewGroupRequest;
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
    assert_eq!(res.status(), 200);
    let groups = res.json::<Vec<Group>>().await?;
    assert_eq!(groups.len(), 3);

    // add a group
    let new_group = NewGroupRequest {
        group: "group123".to_string(),
    };
    let res = reqwest::Client::new()
        .post(&url)
        .headers(auth_headers.clone())
        .json(&new_group)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let group = res.json::<Group>().await?;
    assert_eq!(group.name, "group123");

    // modify the group
    let upd_group = NewGroupRequest {
        group: "group456".to_string(),
    };
    let url_name = format!("{}/{}", url, group.id);
    let res = reqwest::Client::new()
        .put(&url_name)
        .headers(auth_headers.clone())
        .json(&upd_group)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    let upd_grp = res.json::<Group>().await?;
    assert_eq!(upd_group.group, upd_grp.name);

    // delete the group
    let url_del = format!("{}/{}", url, upd_grp.id);
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

    let groups = res.json::<Vec<Group>>().await?;
    assert_eq!(groups.len(), 3);

    Ok(())
}
