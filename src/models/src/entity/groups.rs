use crate::app_state::AppState;
use crate::cache::{Cache, DB};
use crate::entity::users::User;
use actix_web::web;
use rauthy_api_types::groups::NewGroupRequest;
use rauthy_common::constants::{CACHE_TTL_APP, IDX_GROUPS};
use rauthy_common::utils::new_store_id;
use rauthy_error::{ErrorResponse, ErrorResponseType};
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
            "INSERT INTO groups (id, name) VALUES ($1, $2)",
            new_group.id,
            new_group.name,
        )
        .execute(&data.db)
        .await?;

        groups.push(new_group.clone());
        DB::client()
            .put(Cache::App, IDX_GROUPS, &groups, CACHE_TTL_APP)
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

        let mut txn = data.db.begin().await?;

        // TODO better smt like 'await_all' or less resource usage?
        // TODO -> wrap in big single txn once migration to hiqlite is done
        for user in users {
            user.save(data, None, Some(&mut txn)).await?;
        }

        sqlx::query!("DELETE FROM groups WHERE id = $1", group.id)
            .execute(&mut *txn)
            .await?;
        txn.commit().await?;

        let groups = Group::find_all(data)
            .await?
            .into_iter()
            .filter(|g| g.id != group.id)
            .collect::<Vec<Group>>();
        DB::client()
            .put(Cache::App, IDX_GROUPS, &groups, CACHE_TTL_APP)
            .await?;

        Ok(())
    }

    // Returns a single group by id
    pub async fn find(data: &web::Data<AppState>, id: String) -> Result<Self, ErrorResponse> {
        let res = sqlx::query_as!(Self, "SELECT * FROM groups WHERE id = $1", id,)
            .fetch_one(&data.db)
            .await?;
        Ok(res)
    }

    // Returns all existing groups
    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let client = DB::client();
        if let Some(slf) = client.get(Cache::App, IDX_GROUPS).await? {
            return Ok(slf);
        }

        let res = sqlx::query_as!(Self, "SELECT * FROM groups")
            .fetch_all(&data.db)
            .await?;

        client
            .put(Cache::App, IDX_GROUPS, &res, CACHE_TTL_APP)
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

        let mut txn = data.db.begin().await?;

        for user in users {
            // TODO better smt like 'await_all' or less resource usage?
            // TODO -> wrap in big single txn once migration to hiqlite is done
            user.save(data, None, Some(&mut txn)).await?;
        }

        let new_group = Group {
            id: group.id.clone(),
            name: new_name,
        };

        sqlx::query!(
            "UPDATE groups SET name = $1 WHERE id = $2",
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
        DB::client()
            .put(Cache::App, IDX_GROUPS, &groups, CACHE_TTL_APP)
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
