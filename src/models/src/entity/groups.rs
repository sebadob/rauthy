use crate::app_state::AppState;
use crate::entity::users::User;
use actix_web::web;
use rauthy_api_types::groups::NewGroupRequest;
use rauthy_common::constants::{CACHE_NAME_12HR, IDX_GROUPS, IDX_USERS};
use rauthy_common::utils::new_store_id;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use redhac::{cache_get, cache_get_from, cache_get_value, cache_insert, cache_remove, AckLevel};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Group {
    pub id: String,
    pub name: String,
}

// CRUD
impl Group {
    // Inserts a new group into the database
    pub async fn create(
        data: &web::Data<AppState>,
        group_req: NewGroupRequest,
    ) -> Result<Self, ErrorResponse> {
        let mut groups = Group::find_all(data).await?;
        for g in &groups {
            if g.name == group_req.group {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Group already exists",
                ));
            }
        }

        let new_group = Group {
            id: new_store_id(),
            name: group_req.group,
        };

        sqlx::query!(
            "insert into groups (id, name) values ($1, $2)",
            new_group.id,
            new_group.name,
        )
        .execute(&data.db)
        .await?;

        groups.push(new_group.clone());
        cache_insert(
            CACHE_NAME_12HR.to_string(),
            IDX_GROUPS.to_string(),
            &data.caches.ha_cache_config,
            &groups,
            AckLevel::Quorum,
        )
        .await?;

        Ok(new_group)
    }

    // Deletes a group
    pub async fn delete(data: &web::Data<AppState>, id: String) -> Result<(), ErrorResponse> {
        let group = Group::find(data, id).await?;

        // before deleting a group, cleanup every user
        // get all users with the to-be-deleted-group assigned
        let mut users = vec![];
        User::find_all(data)
            .await?
            .into_iter()
            .filter(|u| u.groups.is_some() && u.groups.as_ref().unwrap().contains(&group.name))
            .for_each(|mut u| {
                u.delete_group(&group.name);
                users.push(u);
            });

        // no need to evict the cache if no users are updated
        if !users.is_empty() {
            cache_remove(
                CACHE_NAME_12HR.to_string(),
                IDX_USERS.to_string(),
                &data.caches.ha_cache_config,
                AckLevel::Quorum,
            )
            .await?;
        }

        let mut txn = data.db.begin().await?;

        // TODO better smt like 'await_all' or less resource usage?
        for user in users {
            user.save(data, None, Some(&mut txn)).await?;
        }

        sqlx::query!("delete from groups where id = $1", group.id)
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;

        let groups = Group::find_all(data)
            .await?
            .into_iter()
            .filter(|g| g.id != group.id)
            .collect::<Vec<Group>>();
        cache_insert(
            CACHE_NAME_12HR.to_string(),
            IDX_GROUPS.to_string(),
            &data.caches.ha_cache_config,
            &groups,
            AckLevel::Quorum,
        )
        .await?;

        Ok(())
    }

    // Returns a single group by id
    pub async fn find(data: &web::Data<AppState>, id: String) -> Result<Self, ErrorResponse> {
        let res = sqlx::query_as!(Self, "select * from groups where id = $1", id,)
            .fetch_one(&data.db)
            .await?;

        Ok(res)
    }

    // Returns all existing groups
    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let groups = cache_get!(
            Vec<Group>,
            CACHE_NAME_12HR.to_string(),
            IDX_GROUPS.to_string(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if let Some(groups) = groups {
            return Ok(groups);
        }

        let res = sqlx::query_as!(Self, "select * from groups")
            .fetch_all(&data.db)
            .await?;

        cache_insert(
            CACHE_NAME_12HR.to_string(),
            IDX_GROUPS.to_string(),
            &data.caches.ha_cache_config,
            &res,
            AckLevel::Leader,
        )
        .await?;
        Ok(res)
    }

    // Updates a group
    pub async fn update(
        data: &web::Data<AppState>,
        id: String,
        new_name: String,
    ) -> Result<Self, ErrorResponse> {
        let group = Group::find(data, id).await?;

        // find all users with the old_name assigned
        let mut users = vec![];
        User::find_all(data)
            .await?
            .into_iter()
            .filter(|u| u.groups.is_some() && u.groups.as_ref().unwrap().contains(&group.name))
            .for_each(|mut u| {
                u.groups = Some(u.groups.as_ref().unwrap().replace(&group.name, &new_name));
                users.push(u);
            });

        // no need to evict the cache if no users are updated
        if !users.is_empty() {
            cache_remove(
                CACHE_NAME_12HR.to_string(),
                IDX_USERS.to_string(),
                &data.caches.ha_cache_config,
                AckLevel::Quorum,
            )
            .await?;
        }

        // TODO better smt like 'await_all' or less resource usage?
        let mut txn = data.db.begin().await?;

        for user in users {
            user.save(data, None, Some(&mut txn)).await?;
        }

        let new_group = Group {
            id: group.id.clone(),
            name: new_name,
        };

        sqlx::query!(
            "update groups set name = $1 where id = $2",
            new_group.name,
            new_group.id,
        )
        .execute(&mut *txn)
        .await?;

        txn.commit().await?;

        let groups = Group::find_all(data)
            .await?
            .into_iter()
            .map(|mut g| {
                if g.id == group.id {
                    g.name.clone_from(&new_group.name);
                }
                g
            })
            .collect::<Vec<Group>>();
        cache_insert(
            CACHE_NAME_12HR.to_string(),
            IDX_GROUPS.to_string(),
            &data.caches.ha_cache_config,
            &groups,
            AckLevel::Quorum,
        )
        .await?;

        Ok(new_group)
    }
}

impl Group {
    // Sanitizes any bad data from an API request for adding / modifying groups and silently
    // dismissed all bad data.
    pub async fn sanitize(
        data: &web::Data<AppState>,
        groups_opt: Option<Vec<String>>,
    ) -> Result<Option<String>, ErrorResponse> {
        if groups_opt.is_none() {
            return Ok(None);
        }

        let groups = groups_opt.unwrap();
        let mut res = String::with_capacity(groups.len());
        Group::find_all(data).await?.into_iter().for_each(|g| {
            if groups.contains(&g.name) {
                res.push_str(g.name.as_str());
                res.push(',');
            }
        });

        if res.is_empty() {
            Ok(None)
        } else {
            res.remove(res.len() - 1);
            Ok(Some(res))
        }
    }
}
