use axum::extract::Path;
use rauthy_client::scim::types::{
    ScimError, ScimFilterBy, ScimListQuery, ScimListResponse, ScimResource, ScimToken, ScimUser,
};
use rauthy_client::secure_random;
use std::sync::LazyLock;
use tokio::sync::RwLock;
use tracing::info;

// Very simple user storage - DO NOT do it like that in production, we just want to keep it simple.
pub static USERS: LazyLock<RwLock<Vec<ScimUser>>> = LazyLock::new(|| RwLock::new(Vec::new()));

// All SCIM type extractors already validate the proper Content-Type and the SCIM bearer token.
// This means if you end up in the request handler, the token has already been validated.

/// A request to get "all" users. The `ScimListQuery` basically deinfes pagination and the
/// expected results. Must always return a `ScimListResponse`.
///
/// By RFC, the `ScimListQuery` can include more fields, but Rauthy will only use `start_index` and
/// `count`.
pub async fn get_users(query: ScimListQuery) -> Result<ScimListResponse, ScimError> {
    // The query index must be stable over time. For a real world implementation, a good idea would
    // be to use something like a creation timestamp, that never changes.
    let start_index = query.start_index.unwrap_or(0);

    let resources = find_users(start_index, &query).await;
    let len = resources.len() as u32;

    Ok(ScimListResponse {
        items_per_page: query.count.unwrap_or(len),
        total_results: len,
        start_index,
        resources,
        ..Default::default()
    })
}

pub async fn post_user(user: ScimUser) -> Result<ScimUser, ScimError> {
    info!("Create user {:?}", user);
    save_user(user).await
}

// The `_: ScimToken` makes sure that the `Bearer` token will be checked against the configured
// SCIM token. This struct is empty and does nothing than adding the check inside the extractor.
// This is much more straight forward than manually passing the header value into a helper function.
// All other `Scim*` structs in extractors already do this token check.
pub async fn get_user(Path(id): Path<String>, _: ScimToken) -> Result<ScimUser, ScimError> {
    match USERS
        .read()
        .await
        .iter()
        .find(|u| u.id.as_deref() == Some(&id))
    {
        Some(user) => Ok(user.clone()),
        None => Err(ScimError::new(404, Some("User not found".into()))),
    }
}

pub async fn put_user(Path(id): Path<String>, mut user: ScimUser) -> Result<ScimUser, ScimError> {
    info!("Update user {}: {:?}", id, user);

    // just overwrite the users id so we can re-use our simple logic
    user.id = Some(id);
    save_user(user).await
}

pub async fn delete_user(Path(id): Path<String>, _: ScimToken) -> Result<(), ScimError> {
    USERS.write().await.retain(|u| u.id.as_deref() != Some(&id));
    Ok(())
}

async fn find_users(start_index: u32, query: &ScimListQuery) -> Vec<ScimResource> {
    // If any filter is given by Rauthy, usually a single result is expected
    match query.filter_by() {
        ScimFilterBy::ExternalId(id) => USERS
            .read()
            .await
            .iter()
            .filter_map(|u| {
                if u.external_id.as_deref() == Some(id) {
                    Some(ScimResource::User(Box::new(u.clone())))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>(),
        ScimFilterBy::UserName(name) => USERS
            .read()
            .await
            .iter()
            .filter_map(|u| {
                if u.user_name == name {
                    Some(ScimResource::User(Box::new(u.clone())))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>(),
        ScimFilterBy::DisplayName(_) => {
            // Cannot happen for users, only for group resources.
            // In production, better return an Error instead of panic.
            unreachable!()
        }
        ScimFilterBy::None => USERS
            .read()
            .await
            .iter()
            .skip(start_index as usize)
            .take(query.count.unwrap_or(100) as usize)
            .map(|u| ScimResource::User(Box::new(u.clone())))
            .collect::<Vec<_>>(),
    }
}

async fn save_user(mut user: ScimUser) -> Result<ScimUser, ScimError> {
    if let Some(id) = &user.id {
        let mut res = None;
        for u in USERS.write().await.iter_mut() {
            if u.id.as_deref() == Some(id) {
                u.external_id = user.external_id;
                u.user_name = user.user_name;
                u.name = user.name;
                u.display_name = user.display_name;
                u.preferred_language = user.preferred_language;
                u.locale = user.locale;
                u.active = user.active;
                u.emails = user.emails;
                u.phone_numbers = user.phone_numbers;
                u.photos = user.photos;
                u.addresses = user.addresses;
                // CAUTION: You MUST NOT upgrade any groups assignments here!
                u.roles = user.roles;
                u.custom = user.custom;

                res = Some(u.clone());
                break;
            }
        }
        if let Some(user) = res {
            Ok(user)
        } else {
            Err(ScimError::new(404, Some("User does not exist".into())))
        }
    } else {
        user.id = Some(secure_random(32));
        USERS.write().await.push(user.clone());
        Ok(user)
    }
}
