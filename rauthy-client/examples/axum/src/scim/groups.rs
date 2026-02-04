use crate::scim::users::USERS;
use axum::extract::Path;
use rauthy_client::scim::types::{
    ScimError, ScimFilterBy, ScimGroup, ScimGroupMember, ScimGroupValue, ScimListQuery,
    ScimListResponse, ScimPatchOp, ScimResource, ScimToken,
};
use rauthy_client::secure_random;
use std::sync::LazyLock;
use tokio::sync::RwLock;
use tracing::info;

// Very simple group storage - DO NOT do it like that in production, we just want to keep it simple.
pub static GROUPS: LazyLock<RwLock<Vec<ScimGroup>>> = LazyLock::new(|| RwLock::new(Vec::new()));

// All SCIM type extractors already validate the proper Content-Type and the SCIM bearer token.
// This means if you end up in the request handler, the token has already been validated.

/// A request to get "all" users. The `ScimListQuery` basically defines pagination and the
/// expected results. Must always return a `ScimListResponse`.
///
/// By RFC, the `ScimListQuery` can include more fields, but Rauthy will only use `start_index` and
/// `count`.
pub async fn get_groups(query: ScimListQuery) -> Result<ScimListResponse, ScimError> {
    // The query index must be stable over time. For a real world implementation, a good idea would
    // be to use something like a creation timestamp, that never changes.
    let start_index = query.start_index.unwrap_or(0);

    let resources = find_groups(start_index, &query).await;
    let len = resources.len() as u32;

    Ok(ScimListResponse {
        items_per_page: query.count.unwrap_or(len),
        total_results: len,
        start_index,
        resources,
        ..Default::default()
    })
}

pub async fn post_group(group: ScimGroup) -> Result<ScimGroup, ScimError> {
    info!("Create group {:?}", group);
    save_group(group).await
}

// The `_: ScimToken` makes sure that the `Bearer` token will be checked against the configured
// SCIM token. This struct is empty and does nothing than adding the check inside the extractor.
// This is much more straight forward than manually passing the header value into a helper function.
// All other `Scim*` structs in extractors already do this token check.
pub async fn get_group(Path(id): Path<String>, _: ScimToken) -> Result<ScimGroup, ScimError> {
    match GROUPS
        .read()
        .await
        .iter()
        .find(|g| g.id.as_deref() == Some(&id))
    {
        Some(group) => Ok(group.clone()),
        None => Err(ScimError::new(404, Some("Group not found".into()))),
    }
}

pub async fn patch_group(Path(id): Path<String>, patch_op: ScimPatchOp) -> Result<(), ScimError> {
    for op in patch_op.operations {
        // PatchOps can become very complex and super annoying to work with, if you have a
        // stricly typed language. This example only shows the ones that Rauthy actually sends.
        // You get the least amount of boilerplate with this way of doing it.

        if let Ok(users) = op.try_as_add_member() {
            info!("Add users to group {}: {:?}", id, users);
            // Add users to a group
            let mut group_name = None;
            for group in GROUPS.write().await.iter_mut() {
                if group.id.as_deref() == Some(&id) {
                    if group.members.is_none() {
                        group.members = Some(Vec::with_capacity(users.len()));
                    }
                    group_name = Some(group.display_name.clone());
                    let members = group.members.as_mut().unwrap();

                    for user in &users {
                        members.push(ScimGroupMember {
                            value: user.user_id.to_string(),
                            display: Some(user.user_email.to_string()),
                        });
                    }

                    break;
                }
            }

            if let Some(group_name) = group_name {
                let user_ids = users.iter().map(|u| u.user_id).collect::<Vec<_>>();

                for user in USERS.write().await.iter_mut() {
                    if user_ids.contains(&user.id.as_deref().unwrap_or_default()) {
                        if user.groups.is_none() {
                            user.groups = Some(Vec::new());
                        }
                        user.groups.as_mut().unwrap().push(ScimGroupValue {
                            value: id.clone(),
                            display: group_name.clone(),
                        })
                    }
                }
            }
        } else if let Ok(replace) = op.try_as_replace_name() {
            info!(
                "Update group name from to {} for id: {}",
                replace.group_name, id
            );

            // Update / Replace the group name / externalId
            for group in GROUPS.write().await.iter_mut() {
                if group.id.as_deref() == Some(&id) {
                    group.external_id = Some(replace.external_id.to_string());
                    group.display_name = replace.group_name.to_string();
                    break;
                }
            }

            for user in USERS.write().await.iter_mut() {
                if user.groups.is_none() {
                    continue;
                }
                for group in user.groups.as_mut().unwrap().iter_mut() {
                    if group.value == id {
                        group.display = replace.group_name.to_string();
                        break;
                    }
                }
            }
        } else if let Ok(users) = op.try_as_remove_member() {
            info!("Remove users from group {}: {:?}", id, users);

            // Remove users from group
            for group in GROUPS.write().await.iter_mut() {
                if group.id.as_deref() == Some(&id) {
                    if group.members.is_none() {
                        return Ok(());
                    }

                    let users = users.iter().map(|u| u.user_id).collect::<Vec<_>>();
                    let members = group.members.as_mut().unwrap();
                    members.retain(|u| !users.contains(&u.value.as_str()));
                    break;
                }
            }

            let user_ids = users.iter().map(|u| u.user_id).collect::<Vec<_>>();
            for user in USERS.write().await.iter_mut() {
                if user_ids.contains(&user.id.as_deref().unwrap_or_default()) {
                    if user.groups.is_none() {
                        continue;
                    }
                    user.groups.as_mut().unwrap().retain(|g| g.value != id);
                }
            }
        } else {
            return Err(ScimError::new(400, Some("Invalid ScimPatchOp".into())));
        }
    }

    Ok(())
}

pub async fn delete_group(Path(id): Path<String>, _: ScimToken) -> Result<(), ScimError> {
    info!("Delete group {:?}", id);

    GROUPS
        .write()
        .await
        .retain(|g| g.id.as_deref() != Some(&id));

    for user in USERS.write().await.iter_mut() {
        if user.groups.is_none() {
            continue;
        }
        user.groups.as_mut().unwrap().retain(|g| g.value != id);
    }

    Ok(())
}

async fn find_groups(start_index: u32, query: &ScimListQuery) -> Vec<ScimResource> {
    // If any filter is given by Rauthy, usually a single result is expected
    match query.filter_by() {
        ScimFilterBy::ExternalId(id) => GROUPS
            .read()
            .await
            .iter()
            .filter_map(|u| {
                if u.external_id.as_deref() == Some(id) {
                    Some(ScimResource::Group(Box::new(u.clone())))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>(),
        ScimFilterBy::UserName(_) => {
            // Cannot happen for groups, only for user resources.
            // In production, better return an Error instead of panic.
            unreachable!()
        }
        ScimFilterBy::DisplayName(name) => GROUPS
            .read()
            .await
            .iter()
            .filter_map(|u| {
                if u.display_name == name {
                    Some(ScimResource::Group(Box::new(u.clone())))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>(),
        ScimFilterBy::None => GROUPS
            .read()
            .await
            .iter()
            .skip(start_index as usize)
            .take(query.count.unwrap_or(100) as usize)
            .map(|u| ScimResource::Group(Box::new(u.clone())))
            .collect::<Vec<_>>(),
    }
}

async fn save_group(mut group: ScimGroup) -> Result<ScimGroup, ScimError> {
    if let Some(id) = &group.id {
        let mut res = None;
        for g in GROUPS.write().await.iter_mut() {
            if g.id.as_deref() == Some(id) {
                g.external_id = group.external_id;
                g.display_name = group.display_name;
                // CAUTION: You MUST NOT update members here -> only doen via PatchOp!

                res = Some(g.clone());
                break;
            }
        }
        if let Some(group) = res {
            Ok(group)
        } else {
            Err(ScimError::new(404, Some("Group does not exist".into())))
        }
    } else {
        group.id = Some(secure_random(32));
        GROUPS.write().await.push(group.clone());
        Ok(group)
    }
}
