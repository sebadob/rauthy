use crate::database::{Cache, DB};
use crate::entity::users::User;
use deadpool_postgres::GenericClient;
use hiqlite::Params;
use hiqlite_macros::params;
use rauthy_api_types::roles::RoleRequest;
use rauthy_common::constants::{CACHE_TTL_APP, IDX_ROLES};
use rauthy_common::is_hiqlite;
use rauthy_common::utils::new_store_id;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Role {
    pub id: String,
    pub name: String,
}

impl From<tokio_postgres::Row> for Role {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
        }
    }
}

// CRUD
impl Role {
    pub async fn create(role_req: RoleRequest) -> Result<Self, ErrorResponse> {
        let mut roles = Role::find_all().await?;
        for s in &roles {
            if s.name == role_req.role {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Role already exists",
                ));
            }
        }

        let new_role = Role {
            id: new_store_id(),
            name: role_req.role,
        };

        let sql = "INSERT INTO roles (id, name) VALUES ($1, $2)";
        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(new_role.id.clone(), new_role.name.clone()))
                .await?;
        } else {
            DB::pg_execute(sql, &[&new_role.id, &new_role.name]).await?;
        }

        roles.push(new_role.clone());
        DB::hql()
            .put(Cache::App, IDX_ROLES, &roles, CACHE_TTL_APP)
            .await?;

        Ok(new_role)
    }

    pub async fn delete(id: &str) -> Result<(), ErrorResponse> {
        let role = Role::find(id).await?;

        // prevent deletion of 'rauthy_admin'
        if role.name == "rauthy_admin" {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Anti-Lockout Rule: The 'rauthy_admin' role cannot be deleted",
            ));
        }

        let users = User::find_with_role(&role.name).await?;

        let sql = "DELETE FROM roles WHERE id = $1";
        if is_hiqlite() {
            let mut txn: Vec<(&str, Params)> = Vec::with_capacity(users.len() + 1);

            for mut user in users {
                user.delete_role(&role.name);
                user.save_txn_append(&mut txn);
            }

            txn.push((sql, params!(role.id.clone())));

            for res in DB::hql().txn(txn).await? {
                let rows_affected = res?;
                debug_assert!(rows_affected == 1);
            }
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            for mut user in users {
                user.delete_role(&role.name);
                user.save_txn(&txn).await?;
            }
            DB::pg_txn_append(&txn, sql, &[&id]).await?;

            txn.commit().await?;
        }

        let roles = Role::find_all()
            .await?
            .into_iter()
            .filter(|r| r.id != role.id)
            .collect::<Vec<Role>>();

        let client = DB::hql();
        // clearing users cache is more safe and less resource intensive than trying to
        // update each single entry
        client.clear_cache(Cache::User).await?;
        client
            .put(Cache::App, IDX_ROLES, &roles, CACHE_TTL_APP)
            .await?;

        Ok(())
    }

    pub async fn find(id: &str) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM roles WHERE id = $1";
        let res = if is_hiqlite() {
            DB::hql().query_as_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        Ok(res)
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let client = DB::hql();
        if let Some(slf) = client.get(Cache::App, IDX_ROLES).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM roles";
        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 3).await?
        };

        client
            .put(Cache::App, IDX_ROLES, &res, CACHE_TTL_APP)
            .await?;
        Ok(res)
    }

    // Updates a role
    pub async fn update(id: String, new_name: String) -> Result<Self, ErrorResponse> {
        let role = Role::find(&id).await?;
        let users = User::find_with_role(&role.name).await?;

        let new_role = Self {
            id: role.id.clone(),
            name: new_name,
        };

        let sql = "UPDATE roles SET name = $1 WHERE id = $2";
        if is_hiqlite() {
            let mut txn: Vec<(&str, Params)> = Vec::with_capacity(users.len() + 1);

            for mut user in users {
                user.roles = user.roles.replace(&role.name, &new_role.name);
                user.save_txn_append(&mut txn);
            }

            txn.push((sql, params!(new_role.name.clone(), new_role.id.clone())));

            for res in DB::hql().txn(txn).await? {
                let rows_affected = res?;
                debug_assert!(rows_affected == 1);
            }
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            for mut user in users {
                user.delete_role(&role.name);
                user.save_txn(&txn).await?;
            }
            DB::pg_txn_append(&txn, sql, &[&new_role.name, &new_role.id]).await?;

            txn.commit().await?;
        }

        let roles = Role::find_all()
            .await?
            .into_iter()
            .map(|mut r| {
                if r.id == role.id {
                    r.name.clone_from(&new_role.name);
                }
                r
            })
            .collect::<Vec<Role>>();

        let client = DB::hql();
        client.clear_cache(Cache::User).await?;
        DB::hql()
            .put(Cache::App, IDX_ROLES, &roles, CACHE_TTL_APP)
            .await?;

        Ok(new_role)
    }
}

impl Role {
    pub async fn sanitize(rls: Vec<String>) -> Result<String, ErrorResponse> {
        let mut res = String::with_capacity(rls.len());
        Role::find_all().await?.into_iter().for_each(|r| {
            if rls.contains(&r.name) {
                res.push_str(r.name.as_str());
                res.push(',');
            }
        });
        if !res.is_empty() {
            res.remove(res.len() - 1);
        }
        Ok(res)
    }
}
