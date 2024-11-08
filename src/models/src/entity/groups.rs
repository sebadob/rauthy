use crate::database::{Cache, DB};
use crate::entity::users::User;
use hiqlite::{params, Param, Params};
use rauthy_api_types::groups::NewGroupRequest;
use rauthy_common::constants::{CACHE_TTL_APP, IDX_GROUPS};
use rauthy_common::is_hiqlite;
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
    pub async fn create(group_req: NewGroupRequest) -> Result<Self, ErrorResponse> {
        let mut groups = Group::find_all().await?;
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

        if is_hiqlite() {
            DB::client()
                .execute(
                    "INSERT INTO groups (id, name) VALUES ($1, $2)",
                    params!(new_group.id.clone(), new_group.name.clone()),
                )
                .await?;
        } else {
            sqlx::query!(
                "INSERT INTO groups (id, name) VALUES ($1, $2)",
                new_group.id,
                new_group.name,
            )
            .execute(DB::conn())
            .await?;
        }

        groups.push(new_group.clone());
        DB::client()
            .put(Cache::App, IDX_GROUPS, &groups, CACHE_TTL_APP)
            .await?;

        Ok(new_group)
    }

    // Deletes a group
    pub async fn delete(id: String) -> Result<(), ErrorResponse> {
        let group = Group::find(id).await?;
        let users = User::find_with_group(&group.name).await?;

        if is_hiqlite() {
            let mut txn: Vec<(&str, Params)> = Vec::with_capacity(users.len() + 1);

            for mut user in users {
                user.delete_group(&group.name);
                user.save_txn_append(&mut txn);
            }

            txn.push((
                "DELETE FROM groups WHERE id = $1",
                params!(group.id.clone()),
            ));

            for res in DB::client().txn(txn).await? {
                let rows_affected = res?;
                debug_assert!(rows_affected == 1);
            }
        } else {
            let mut txn = DB::txn().await?;

            for mut user in users {
                user.delete_group(&group.name);
                user.save_txn(&mut txn).await?;
            }
            sqlx::query!("DELETE FROM groups WHERE id = $1", group.id)
                .execute(&mut *txn)
                .await?;

            txn.commit().await?;
        }

        let groups = Group::find_all()
            .await?
            .into_iter()
            .filter(|g| g.id != group.id)
            .collect::<Vec<Group>>();

        let client = DB::client();
        // clearing users cache is more safe and less resource intensive than trying to
        // update each single entry
        client.clear_cache(Cache::User).await?;
        client
            .put(Cache::App, IDX_GROUPS, &groups, CACHE_TTL_APP)
            .await?;

        Ok(())
    }

    // Returns a single group by id
    pub async fn find(id: String) -> Result<Self, ErrorResponse> {
        let res = if is_hiqlite() {
            DB::client()
                .query_as_one("SELECT * FROM groups WHERE id = $1", params!(id))
                .await?
        } else {
            sqlx::query_as!(Self, "SELECT * FROM groups WHERE id = $1", id,)
                .fetch_one(DB::conn())
                .await?
        };

        Ok(res)
    }

    // Returns all existing groups
    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let client = DB::client();
        if let Some(slf) = client.get(Cache::App, IDX_GROUPS).await? {
            return Ok(slf);
        }

        let res = if is_hiqlite() {
            client.query_as("SELECT * FROM groups", params!()).await?
        } else {
            sqlx::query_as!(Self, "SELECT * FROM groups")
                .fetch_all(DB::conn())
                .await?
        };

        client
            .put(Cache::App, IDX_GROUPS, &res, CACHE_TTL_APP)
            .await?;

        Ok(res)
    }

    // Updates a group
    pub async fn update(id: String, new_name: String) -> Result<Self, ErrorResponse> {
        let group = Group::find(id).await?;
        let users = User::find_with_group(&group.name).await?;

        let new_group = Self {
            id: group.id.clone(),
            name: new_name,
        };

        if is_hiqlite() {
            let mut txn: Vec<(&str, Params)> = Vec::with_capacity(users.len() + 1);

            for mut user in users {
                user.groups = Some(
                    user.groups
                        .as_ref()
                        .unwrap()
                        .replace(&group.name, &new_group.name),
                );
                user.save_txn_append(&mut txn);
            }

            txn.push((
                "UPDATE groups SET name = $1 WHERE id = $2",
                params!(new_group.name.clone(), new_group.id.clone()),
            ));

            for res in DB::client().txn(txn).await? {
                let rows_affected = res?;
                debug_assert!(rows_affected == 1);
            }
        } else {
            let mut txn = DB::txn().await?;

            for mut user in users {
                user.delete_group(&group.name);
                user.save_txn(&mut txn).await?;
            }
            sqlx::query!(
                "UPDATE groups SET name = $1 WHERE id = $2",
                new_group.name,
                new_group.id,
            )
            .execute(&mut *txn)
            .await?;

            txn.commit().await?;
        }

        let groups = Group::find_all()
            .await?
            .into_iter()
            .map(|mut g| {
                if g.id == group.id {
                    g.name.clone_from(&new_group.name);
                }
                g
            })
            .collect::<Vec<Group>>();

        let client = DB::client();
        client.clear_cache(Cache::User).await?;
        client
            .put(Cache::App, IDX_GROUPS, &groups, CACHE_TTL_APP)
            .await?;

        Ok(new_group)
    }
}

impl Group {
    // Sanitizes any bad data from an API request for adding / modifying groups and silently
    // dismissed all bad data.
    pub async fn sanitize(
        groups_opt: Option<Vec<String>>,
    ) -> Result<Option<String>, ErrorResponse> {
        if groups_opt.is_none() {
            return Ok(None);
        }

        let groups = groups_opt.unwrap();
        let mut res = String::with_capacity(groups.len());
        Group::find_all().await?.into_iter().for_each(|g| {
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
