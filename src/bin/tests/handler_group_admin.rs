use crate::common::{get_auth_headers, get_backend_url, session_headers_with};
use pretty_assertions::assert_eq;
use rauthy_api_types::generic::Language;
use rauthy_api_types::groups::GroupRequest;
use rauthy_api_types::roles::RoleRequest;
use rauthy_api_types::users::{NewUserRequest, UpdateUserRequest, UserResponse};
use reqwest::Client;
use reqwest::header::HeaderMap;
use std::error::Error;

mod common;

const PWD: &str = "123SuperSafe123";

/// Builds an `UpdateUserRequest` that mirrors the user's current state, so a test can
/// change one thing at a time.
fn upd(user: &UserResponse) -> UpdateUserRequest {
    UpdateUserRequest {
        email: user.email.clone(),
        given_name: user.given_name.clone(),
        family_name: user.family_name.clone(),
        // all users in this test are created with `En`; keep it simple
        language: Some(Language::En),
        password: None,
        roles: user.roles.clone(),
        groups: user.groups.clone(),
        enabled: user.enabled,
        email_verified: user.email_verified,
        user_expires: user.user_expires,
        user_values: None,
    }
}

async fn create_user(
    client: &Client,
    backend: &str,
    admin: &HeaderMap,
    email: &str,
    roles: Vec<String>,
    groups: Vec<String>,
) -> Result<UserResponse, Box<dyn Error>> {
    let payload = NewUserRequest {
        given_name: Some("Test".to_string()),
        family_name: Some("User".to_string()),
        email: email.to_string(),
        language: Language::En,
        roles,
        groups: Some(groups),
        user_expires: None,
        tz: None,
    };
    let res = client
        .post(format!("{backend}/users"))
        .headers(admin.clone())
        .json(&payload)
        .send()
        .await?;
    assert_eq!(res.status(), 200);
    Ok(res.json::<UserResponse>().await?)
}

#[tokio::test]
async fn test_group_admin_delegation() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();

    // --- setup: a group-admin role + two groups (tolerate left-overs from a prior run)
    let res = client
        .post(format!("{backend}/roles"))
        .headers(admin.clone())
        .json(&RoleRequest {
            role: "rauthy_admin:gatest".to_string(),
            meta: None,
        })
        .send()
        .await?;
    assert!(res.status() == 200 || res.status() == 400);

    for group in ["gatest", "gaother"] {
        let res = client
            .post(format!("{backend}/groups"))
            .headers(admin.clone())
            .json(&GroupRequest {
                group: group.to_string(),
                meta: None,
            })
            .send()
            .await?;
        assert!(res.status() == 200 || res.status() == 400);
    }

    // --- setup: users
    // a normal user inside the managed group
    let member = create_user(
        &client,
        &backend,
        &admin,
        "ga-member@gatest.io",
        vec!["user".to_string()],
        vec!["gatest".to_string()],
    )
    .await?;
    // a normal user in a different, unmanaged group
    let outsider = create_user(
        &client,
        &backend,
        &admin,
        "ga-outsider@gatest.io",
        vec!["user".to_string()],
        vec!["gaother".to_string()],
    )
    .await?;
    // the group admin itself
    let ga = create_user(
        &client,
        &backend,
        &admin,
        "ga-admin@gatest.io",
        vec!["rauthy_admin:gatest".to_string()],
        vec!["gatest".to_string()],
    )
    .await?;

    // give the group admin a usable password + verified email so it can log in
    let mut set_pwd = upd(&ga);
    set_pwd.password = Some(PWD.to_string());
    set_pwd.email_verified = true;
    set_pwd.enabled = true;
    let res = client
        .put(format!("{backend}/users/{}", ga.id))
        .headers(admin.clone())
        .json(&set_pwd)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // --- log in as the group admin
    let ga_headers = session_headers_with("ga-admin@gatest.io", PWD).await;

    // 1. a group admin sees the (full) user list
    let res = client
        .get(format!("{backend}/users"))
        .headers(ga_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // 2. it can read a managed user
    let res = client
        .get(format!("{backend}/users/{}", member.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // 3. it can edit profile data of a managed user
    let mut change = upd(&member);
    change.given_name = Some("Changed".to_string());
    let res = client
        .put(format!("{backend}/users/{}", member.id))
        .headers(ga_headers.clone())
        .json(&change)
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // 4. it cannot edit a user outside its managed groups
    let mut nope = upd(&outsider);
    nope.given_name = Some("Nope".to_string());
    let res = client
        .put(format!("{backend}/users/{}", outsider.id))
        .headers(ga_headers.clone())
        .json(&nope)
        .send()
        .await?;
    assert_eq!(res.status(), 403);

    // 5. it cannot change roles (no privilege escalation)
    let mut esc = upd(&member);
    esc.roles = vec!["user".to_string(), "admin".to_string()];
    let res = client
        .put(format!("{backend}/users/{}", member.id))
        .headers(ga_headers.clone())
        .json(&esc)
        .send()
        .await?;
    assert_eq!(res.status(), 403);

    // 6. it cannot add a membership in a group it does not manage
    let mut grp = upd(&member);
    grp.groups = Some(vec!["gatest".to_string(), "gaother".to_string()]);
    let res = client
        .put(format!("{backend}/users/{}", member.id))
        .headers(ga_headers.clone())
        .json(&grp)
        .send()
        .await?;
    assert_eq!(res.status(), 403);

    // 7. it cannot set a password
    let mut pwd = upd(&member);
    pwd.password = Some("SomeOtherPwd123".to_string());
    let res = client
        .put(format!("{backend}/users/{}", member.id))
        .headers(ga_headers.clone())
        .json(&pwd)
        .send()
        .await?;
    assert_eq!(res.status(), 403);

    // 8. it cannot delete users
    let res = client
        .delete(format!("{backend}/users/{}", member.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 403);

    // 9. it can force-logout a managed user
    let res = client
        .delete(format!("{backend}/sessions/{}", member.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 200);

    // 10. but not a user outside its managed groups
    let res = client
        .delete(format!("{backend}/sessions/{}", outsider.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    assert_eq!(res.status(), 403);

    // --- cleanup (full admin can still do everything: non-breaking)
    for id in [&member.id, &outsider.id, &ga.id] {
        let res = client
            .delete(format!("{backend}/users/{id}"))
            .headers(admin.clone())
            .send()
            .await?;
        assert_eq!(res.status(), 204);
    }

    Ok(())
}
