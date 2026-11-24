use crate::common::{USER_PASSWORD, get_auth_headers, get_backend_url, session_headers_with};
use crate::expect_status;
use crate::expect_status_json;
use pretty_assertions::assert_eq;
use rauthy_api_types::api_keys::{AccessGroup, AccessRights, ApiKeyAccess, ApiKeyRequest};
use rauthy_api_types::generic::Language;
use rauthy_api_types::groups::GroupRequest;
use rauthy_api_types::roles::RoleRequest;
use rauthy_api_types::users::{
    NewUserRequest, UpdateUserRequest, UserResponse, UserResponseSimple,
};
use rauthy_common::utils::{get_rand, new_store_id};
use rauthy_data::entity::{groups::Group, roles::Role};
use reqwest::Client;
use reqwest::header::{AUTHORIZATION, HeaderMap};
use std::error::Error;

/// Password of the group-admin user this test creates and logs in with.
const PWD: &str = USER_PASSWORD;

/// The password of step 7. It satisfies the same rules as [`PWD`], because the shared password
/// policy can be swapped while this runs, and it differs from it, because `not_recently_used`
/// rejects a password the user has used recently.
const PWD_OTHER: &str = "GroupAdmin-TEST.5678%&@";

/// Group-admin delegation is defined by the group prefix in the role name
/// (`rauthy_admin:<group>`), so every run works on a prefix of its own and its groups, users, and
/// role are deleted again at the end.
struct GaSetup {
    role: String,
    group: String,
    other: String,
    member: String,
    outsider: String,
    ga: String,
    coadmin: String,
    outsider2: String,
}

fn ga_setup() -> GaSetup {
    let suffix = get_rand(12).to_lowercase();
    let group = format!("ga{suffix}");
    GaSetup {
        role: format!("rauthy_admin:{group}"),
        other: format!("{group}x"),
        member: format!("member-{suffix}@gatest.io"),
        outsider: format!("outsider-{suffix}@gatest.io"),
        ga: format!("ga-{suffix}@gatest.io"),
        coadmin: format!("coadmin-{suffix}@gatest.io"),
        outsider2: format!("outsider2-{suffix}@gatest.io"),
        group,
    }
}

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
        preferred_username: None,
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
    let res = expect_status!(res, 200);
    Ok(res.json::<UserResponse>().await?)
}

#[tokio::test]
async fn test_group_admin_delegation() -> Result<(), Box<dyn Error>> {
    let admin = get_auth_headers().await?;
    let backend = get_backend_url();
    let client = Client::new();
    let setup = ga_setup();

    // --- setup: a group-admin role + two groups
    let res = client
        .post(format!("{backend}/roles"))
        .headers(admin.clone())
        .json(&RoleRequest {
            role: setup.role.clone(),
            meta: None,
        })
        .send()
        .await?;
    expect_status!(res, 200, "create the group-admin role");

    for group in [&setup.group, &setup.other] {
        let res = client
            .post(format!("{backend}/groups"))
            .headers(admin.clone())
            .json(&GroupRequest {
                group: group.clone(),
                meta: None,
            })
            .send()
            .await?;
        expect_status!(res, 200, "create group {group}");
    }

    // --- setup: users
    // a normal user inside the managed group
    let member = create_user(
        &client,
        &backend,
        &admin,
        &setup.member,
        vec!["user".to_string()],
        vec![setup.group.clone()],
    )
    .await?;
    // a normal user in a different, unmanaged group
    let outsider = create_user(
        &client,
        &backend,
        &admin,
        &setup.outsider,
        vec!["user".to_string()],
        vec![setup.other.clone()],
    )
    .await?;
    // the group admin itself
    let ga = create_user(
        &client,
        &backend,
        &admin,
        &setup.ga,
        vec![setup.role.clone()],
        vec![setup.group.clone()],
    )
    .await?;
    // a second admin in the same managed group, to test the "cannot manage another admin" path
    let coadmin = create_user(
        &client,
        &backend,
        &admin,
        &setup.coadmin,
        vec![setup.role.clone()],
        vec![setup.group.clone()],
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
    expect_status!(res, 200);

    // --- log in as the group admin
    let ga_headers = session_headers_with(&setup.ga, PWD).await;

    // 1. a group admin sees the (full) user list
    let res = client
        .get(format!("{backend}/users"))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);

    // 2. it can read the full details of a managed user
    let res = client
        .get(format!("{backend}/users/{}", member.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);

    // 2b. for an unmanaged but ordinary user it gets a `428`, not the details: the Admin UI
    // uses this to offer adding the user to a managed group
    let res = client
        .get(format!("{backend}/users/{}", outsider.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 428);

    // 2b-2. trying to view another admin is a plain `403`, never a `428`
    let res = client
        .get(format!("{backend}/users/{}", coadmin.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 403);

    // 2b-3. but a group admin may always view its OWN account (it is an admin, so the gate
    // would otherwise reject it). The account dashboard relies on this for self-management.
    let res = client
        .get(format!("{backend}/users/{}", ga.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);

    // 2c. it can read the roles + groups lists (needed for the management UI to work)
    let res = client
        .get(format!("{backend}/roles"))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);
    let res = client
        .get(format!("{backend}/groups"))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);

    // 2c-2. the member list of a group (the Groups -> Users tab): it answers with the minified
    // user list, which any group admin may read for any group, like the user list above
    let res = client
        .get(format!("{backend}/groups"))
        .headers(ga_headers.clone())
        .send()
        .await?;
    let groups = expect_status_json!(res, 200, Vec<Group>);
    let managed = groups
        .iter()
        .find(|g| g.name == setup.group)
        .unwrap_or_else(|| panic!("the group {} exists", setup.group));
    let res = client
        .get(format!("{backend}/groups/{}/users", managed.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    let members = expect_status_json!(res, 200, Vec<UserResponseSimple>);
    assert!(
        members.iter().any(|u| u.id == member.id),
        "the managed user is a member of its group",
    );

    // 2d. it can read the MFA devices (webauthn) of a managed user (the MFA tab)
    let res = client
        .get(format!("{backend}/users/{}/webauthn", member.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);
    // ... but not of an unmanaged user
    let res = client
        .get(format!("{backend}/users/{}/webauthn", outsider.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 403);

    // 3. it can edit profile data of a managed user
    let mut change = upd(&member);
    change.given_name = Some("Changed".to_string());
    let res = client
        .put(format!("{backend}/users/{}", member.id))
        .headers(ga_headers.clone())
        .json(&change)
        .send()
        .await?;
    expect_status!(res, 200);

    // 4. it cannot edit a user outside its managed groups
    let mut nope = upd(&outsider);
    nope.given_name = Some("Nope".to_string());
    let res = client
        .put(format!("{backend}/users/{}", outsider.id))
        .headers(ga_headers.clone())
        .json(&nope)
        .send()
        .await?;
    expect_status!(res, 403);

    // 5. it cannot change roles (no privilege escalation)
    let mut esc = upd(&member);
    esc.roles = vec!["user".to_string(), "admin".to_string()];
    let res = client
        .put(format!("{backend}/users/{}", member.id))
        .headers(ga_headers.clone())
        .json(&esc)
        .send()
        .await?;
    expect_status!(res, 403);

    // 6. it cannot add a membership in a group it does not manage
    let mut grp = upd(&member);
    grp.groups = Some(vec![setup.group.clone(), setup.other.clone()]);
    let res = client
        .put(format!("{backend}/users/{}", member.id))
        .headers(ga_headers.clone())
        .json(&grp)
        .send()
        .await?;
    expect_status!(res, 403);

    // 7. it CAN set a password for a managed user (they cannot priv-esc anyway)
    let mut pwd = upd(&member);
    pwd.password = Some(PWD_OTHER.to_string());
    let res = client
        .put(format!("{backend}/users/{}", member.id))
        .headers(ga_headers.clone())
        .json(&pwd)
        .send()
        .await?;
    expect_status!(res, 200);

    // 8. it cannot delete users
    let res = client
        .delete(format!("{backend}/users/{}", member.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 403);

    // 9. it can force-logout a managed user
    let res = client
        .delete(format!("{backend}/sessions/{}", member.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);

    // 10. but not a user outside its managed groups
    let res = client
        .delete(format!("{backend}/sessions/{}", outsider.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 403);

    // 10b. it is authorized to list and revoke the consents of a managed user, the effect of
    // a revoke is covered in `handler_users.rs`
    let res = client
        .get(format!("{backend}/users/{}/consents", member.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);
    let res = client
        .delete(format!(
            "{backend}/users/{}/consents?client_id=init_client",
            member.id
        ))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);

    // its own consents, before the group admin check that would refuse an admin target
    let res = client
        .get(format!("{backend}/users/{}/consents", ga.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);
    let res = client
        .delete(format!(
            "{backend}/users/{}/consents?client_id=init_client",
            ga.id
        ))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 200);
    // a revoke deletes all sessions of the user, here the one of the group admin itself
    let res = client
        .get(format!("{backend}/users/{}/consents", ga.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 401);
    let ga_headers = session_headers_with(&setup.ga, PWD).await;

    // an API key of another access group is checked first and rejected, even with the group
    // admin session
    let key = ApiKeyRequest {
        name: new_store_id(),
        exp: None,
        access: vec![ApiKeyAccess {
            group: AccessGroup::Clients,
            access_rights: vec![AccessRights::Read, AccessRights::Delete],
        }],
    };
    let res = client
        .post(format!("{backend}/api_keys"))
        .headers(admin.clone())
        .json(&key)
        .send()
        .await?;
    let res = expect_status!(res, 200);
    let key_header = format!("API-Key {}", res.text().await?);
    let res = client
        .get(format!("{backend}/users/{}/consents", member.id))
        .headers(ga_headers.clone())
        .header(AUTHORIZATION, &key_header)
        .send()
        .await?;
    expect_status!(res, 403);
    let res = client
        .delete(format!(
            "{backend}/users/{}/consents?client_id=init_client",
            member.id
        ))
        .headers(ga_headers.clone())
        .header(AUTHORIZATION, &key_header)
        .send()
        .await?;
    expect_status!(res, 403);
    let res = client
        .delete(format!("{backend}/api_keys/{}", key.name))
        .headers(admin.clone())
        .send()
        .await?;
    expect_status!(res, 200);

    // 10c. but not those of a user outside its managed groups, nor of another admin
    let res = client
        .get(format!("{backend}/users/{}/consents", coadmin.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 403);
    let res = client
        .delete(format!(
            "{backend}/users/{}/consents?client_id=init_client",
            coadmin.id
        ))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 403);
    let res = client
        .get(format!("{backend}/users/{}/consents", outsider.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 403);
    let res = client
        .delete(format!(
            "{backend}/users/{}/consents?client_id=init_client",
            outsider.id
        ))
        .headers(ga_headers.clone())
        .send()
        .await?;
    expect_status!(res, 403);

    // 11. the "add an unmanaged user to my groups" flow: a groups-only PATCH that brings the
    // outsider into a managed group is allowed, even though the admin could not see it before.
    // The PATCH only sends the in-scope group; the unmanaged membership of the other group the admin
    // cannot see must be preserved.
    let patch = serde_json::json!({
        "put": [{ "key": "groups", "value": [&setup.group] }],
        "del": [],
    });
    let res = client
        .patch(format!("{backend}/users/{}", outsider.id))
        .headers(ga_headers.clone())
        .json(&patch)
        .send()
        .await?;
    expect_status!(res, 200);

    // now the outsider is managed: full details are visible, and it kept its other group
    let res = client
        .get(format!("{backend}/users/{}", outsider.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    let managed = expect_status_json!(res, 200, UserResponse);
    let groups = managed.groups.unwrap_or_default();
    assert!(groups.iter().any(|i| i == &setup.group));
    assert!(groups.iter().any(|i| i == &setup.other));

    // a groups-only PATCH that does NOT bring the user into scope is still rejected
    let outsider2 = create_user(
        &client,
        &backend,
        &admin,
        &setup.outsider2,
        vec!["user".to_string()],
        vec![setup.other.clone()],
    )
    .await?;
    let patch = serde_json::json!({
        "put": [{ "key": "given_name", "value": "Nope" }],
        "del": [],
    });
    let res = client
        .patch(format!("{backend}/users/{}", outsider2.id))
        .headers(ga_headers.clone())
        .json(&patch)
        .send()
        .await?;
    expect_status!(res, 403);

    // 12. self-management: the group admin may edit its OWN profile, even though it is
    // an admin (the account dashboard links here for self-service). PATCH is the real Admin UI
    // path; PUT is allowed too. Roles stay locked, so this cannot escalate.
    let patch = serde_json::json!({
        "put": [{ "key": "given_name", "value": "SelfEdit" }],
        "del": [],
    });
    let res = client
        .patch(format!("{backend}/users/{}", ga.id))
        .headers(ga_headers.clone())
        .json(&patch)
        .send()
        .await?;
    expect_status!(res, 200);

    // the same via PUT, keeping the (group-admin) role unchanged
    let res = client
        .get(format!("{backend}/users/{}", ga.id))
        .headers(ga_headers.clone())
        .send()
        .await?;
    let ga_now = expect_status_json!(res, 200, UserResponse);
    let mut self_upd = upd(&ga_now);
    self_upd.given_name = Some("SelfEditPut".to_string());
    let res = client
        .put(format!("{backend}/users/{}", ga.id))
        .headers(ga_headers.clone())
        .json(&self_upd)
        .send()
        .await?;
    expect_status!(res, 200);

    // 13. bulk email: a group admin may only send to a single group it manages, with an
    // `in_group` filter; everything else is rejected. Jobs are scheduled far in the future so
    // nothing is actually sent during the test.
    let future_ts = 4102444800i64; // 2100-01-01, safely in the future

    // a full admin can target any group, incl. one the group admin does not manage; this job
    // must stay invisible to the group admin
    let res = client
        .post(format!("{backend}/email"))
        .headers(admin.clone())
        .json(&serde_json::json!({
            "scheduled": future_ts,
            "filter_type": "in_group",
            "filter_value": &setup.other,
            "content_type": "text",
            "subject": "admin ga-other",
            "body": "x",
        }))
        .send()
        .await?;
    expect_status!(res, 200);

    // `EmailJob`s use the creation second as their primary key, so two jobs created within the
    // same second collide; wait one second before the next successful insert
    tokio::time::sleep(std::time::Duration::from_millis(1100)).await;

    // the group admin may send to a group it manages
    let res = client
        .post(format!("{backend}/email"))
        .headers(ga_headers.clone())
        .json(&serde_json::json!({
            "scheduled": future_ts,
            "filter_type": "in_group",
            "filter_value": &setup.group,
            "content_type": "text",
            "subject": "ga group",
            "body": "x",
        }))
        .send()
        .await?;
    expect_status!(res, 200);

    // ... but not to all users (no filter)
    let res = client
        .post(format!("{backend}/email"))
        .headers(ga_headers.clone())
        .json(&serde_json::json!({
            "scheduled": future_ts,
            "filter_type": "none",
            "content_type": "text",
            "subject": "ga none",
            "body": "x",
        }))
        .send()
        .await?;
    expect_status!(res, 403);

    // ... nor to a group it does not manage
    let res = client
        .post(format!("{backend}/email"))
        .headers(ga_headers.clone())
        .json(&serde_json::json!({
            "scheduled": future_ts,
            "filter_type": "in_group",
            "filter_value": &setup.other,
            "content_type": "text",
            "subject": "ga gaother",
            "body": "x",
        }))
        .send()
        .await?;
    expect_status!(res, 403);

    // ... nor with a role filter
    let res = client
        .post(format!("{backend}/email"))
        .headers(ga_headers.clone())
        .json(&serde_json::json!({
            "scheduled": future_ts,
            "filter_type": "has_role",
            "filter_value": "user",
            "content_type": "text",
            "subject": "ga role",
            "body": "x",
        }))
        .send()
        .await?;
    expect_status!(res, 403);

    // the group admin may list email jobs, but only sees jobs scoped to a group it manages
    let res = client
        .get(format!("{backend}/email"))
        .headers(ga_headers.clone())
        .send()
        .await?;
    let res = expect_status!(res, 200);
    let ga_jobs = res
        .json::<serde_json::Value>()
        .await?
        .as_array()
        .cloned()
        .unwrap_or_default();
    let mut ga_own_job_id = None;
    for job in &ga_jobs {
        assert_eq!(job["filter_type"], "in_group");
        assert_ne!(job["filter_value"], setup.other);
        if job["status"] == "Open" && ga_own_job_id.is_none() {
            ga_own_job_id = job["id"].as_i64();
        }
    }

    // a full admin sees everything, including the job of the other group; grab its id
    let res = client
        .get(format!("{backend}/email"))
        .headers(admin.clone())
        .send()
        .await?;
    let res = expect_status!(res, 200);
    let all_jobs = res
        .json::<serde_json::Value>()
        .await?
        .as_array()
        .cloned()
        .unwrap_or_default();
    let other_job_id = all_jobs
        .iter()
        .find(|j| j["filter_value"] == setup.other && j["status"] == "Open")
        .and_then(|j| j["id"].as_i64());

    // the group admin may cancel a job in its scope ...
    if let Some(id) = ga_own_job_id {
        let res = client
            .post(format!("{backend}/email/cancel/{id}"))
            .headers(ga_headers.clone())
            .send()
            .await?;
        expect_status!(res, 200);
    }
    // ... but not one outside it
    if let Some(id) = other_job_id {
        let res = client
            .post(format!("{backend}/email/cancel/{id}"))
            .headers(ga_headers.clone())
            .send()
            .await?;
        expect_status!(res, 403);
    }

    // --- cleanup: everything this test created is gone again
    for id in [&member.id, &outsider.id, &outsider2.id, &coadmin.id, &ga.id] {
        let res = client
            .delete(format!("{backend}/users/{id}"))
            .headers(admin.clone())
            .send()
            .await?;
        expect_status!(res, 204);
    }
    for group in [&setup.group, &setup.other] {
        let res = client
            .get(format!("{backend}/groups"))
            .headers(admin.clone())
            .send()
            .await?;
        let groups = expect_status_json!(res, 200, Vec<Group>);
        let id = groups
            .iter()
            .find(|g| g.name == *group)
            .unwrap_or_else(|| panic!("the group {group} exists"))
            .id
            .clone();
        let res = client
            .delete(format!("{backend}/groups/{id}"))
            .headers(admin.clone())
            .send()
            .await?;
        expect_status!(res, 200, "delete group {group}");
    }
    let res = client
        .get(format!("{backend}/roles"))
        .headers(admin.clone())
        .send()
        .await?;
    let roles = expect_status_json!(res, 200, Vec<Role>);
    let id = roles
        .iter()
        .find(|r| r.name == setup.role)
        .unwrap_or_else(|| panic!("the role {} exists", setup.role))
        .id
        .clone();
    let res = client
        .delete(format!("{backend}/roles/{id}"))
        .headers(admin)
        .send()
        .await?;
    expect_status!(res, 200, "delete the group-admin role");

    Ok(())
}
