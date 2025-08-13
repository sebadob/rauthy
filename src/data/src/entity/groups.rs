use crate::database::{Cache, DB};
use crate::entity::users::User;
use deadpool_postgres::GenericClient;
use hiqlite::Params;
use hiqlite_macros::params;
use rauthy_api_types::groups::GroupRequest;
use rauthy_common::constants::{CACHE_TTL_APP, IDX_GROUPS};
use rauthy_common::is_hiqlite;
use rauthy_common::utils::new_store_id;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Group {
    pub id: String,
    pub name: String,
}

impl From<tokio_postgres::Row> for Group {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
        }
    }
}

// CRUD
impl Group {
    // Inserts a new group into the database
    pub async fn create(group_req: GroupRequest) -> Result<Self, ErrorResponse> {
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

        let sql = "INSERT INTO groups (id, name) VALUES ($1, $2)";
        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(new_group.id.clone(), new_group.name.clone()))
                .await?;
        } else {
            DB::pg_execute(sql, &[&new_group.id, &new_group.name]).await?;
        }

        groups.push(new_group.clone());
        DB::hql()
            .put(Cache::App, IDX_GROUPS, &groups, CACHE_TTL_APP)
            .await?;

        Ok(new_group)
    }

    // Deletes a group
    pub async fn delete(id: String) -> Result<(), ErrorResponse> {
        let group = Group::find(id).await?;
        let users = User::find_with_group(&group.name).await?;

        let sql = "DELETE FROM groups WHERE id = $1";

        if is_hiqlite() {
            let mut txn: Vec<(&str, Params)> = Vec::with_capacity(users.len() + 1);

            for mut user in users {
                user.delete_group(&group.name);
                user.save_txn_append(&mut txn);
            }

            txn.push((sql, params!(group.id.clone())));

            for res in DB::hql().txn(txn).await? {
                let rows_affected = res?;
                debug_assert!(rows_affected == 1);
            }
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            for mut user in users {
                user.delete_group(&group.name);
                user.save_txn(&txn).await?;
            }
            DB::pg_txn_append(&txn, sql, &[&group.id]).await?;

            txn.commit().await?;
        }

        let groups = Group::find_all()
            .await?
            .into_iter()
            .filter(|g| g.id != group.id)
            .collect::<Vec<Group>>();

        let client = DB::hql();
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
        let sql = "SELECT * FROM groups WHERE id = $1";
        let res = if is_hiqlite() {
            DB::hql().query_as_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        Ok(res)
    }

    // Returns all existing groups
    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let client = DB::hql();
        if let Some(slf) = client.get(Cache::App, IDX_GROUPS).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM groups";
        let res = if is_hiqlite() {
            client.query_as(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 2).await?
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

        let sql = "UPDATE groups SET name = $1 WHERE id = $2";
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

            txn.push((sql, params!(new_group.name.clone(), new_group.id.clone())));

            for res in DB::hql().txn(txn).await? {
                let rows_affected = res?;
                debug_assert!(rows_affected == 1);
            }
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            for mut user in users {
                user.delete_group(&group.name);
                user.save_txn(&txn).await?;
            }
            DB::pg_txn_append(&txn, sql, &[&new_group.name, &new_group.id]).await?;

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

        let client = DB::hql();
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
        let mut res = String::with_capacity(groups.len() * 8);
        Group::find_all().await?.into_iter().for_each(|g| {
            if groups.contains(&g.name) {
                res.push_str(g.name.as_str());
                res.push(',');
            }
        });

        if res.is_empty() {
            Ok(None)
        } else {
            res.pop();
            Ok(Some(res))
        }
    }
}
