use crate::app_state::AppState;
use crate::database::{Cache, DB};
use crate::entity::users::User;
use actix_web::web;
use hiqlite::{params, Param, Params};
use rauthy_api_types::roles::NewRoleRequest;
use rauthy_common::constants::{CACHE_TTL_APP, IDX_ROLES};
use rauthy_common::is_hiqlite;
use rauthy_common::utils::new_store_id;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Role {
    pub id: String,
    pub name: String,
}

// CRUD
impl Role {
    // Inserts a new role into the database
    pub async fn create(role_req: NewRoleRequest) -> Result<Self, ErrorResponse> {
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

        if is_hiqlite() {
            DB::client()
                .execute(
                    "INSERT INTO roles (id, name) VALUES ($1, $2)",
                    params!(new_role.id.clone(), new_role.name.clone()),
                )
                .await?;
        } else {
            sqlx::query!(
                "INSERT INTO roles (id, name) VALUES ($1, $2)",
                new_role.id,
                new_role.name,
            )
            .execute(DB::conn())
            .await?;
        }

        roles.push(new_role.clone());
        DB::client()
            .put(Cache::App, IDX_ROLES, &roles, CACHE_TTL_APP)
            .await?;

        Ok(new_role)
    }

    // Deletes a role
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

        if is_hiqlite() {
            let mut txn: Vec<(&str, Params)> = Vec::with_capacity(users.len() + 1);

            for mut user in users {
                user.delete_role(&role.name);
                user.save_txn_append(&mut txn);
            }

            txn.push(("DELETE FROM roles WHERE id = $1", params!(role.id.clone())));

            for res in DB::client().txn(txn).await? {
                let rows_affected = res?;
                debug_assert!(rows_affected == 1);
            }
        } else {
            let mut txn = DB::txn().await?;

            for mut user in users {
                user.delete_role(&role.name);
                user.save_txn(&mut txn).await?;
            }
            sqlx::query!("DELETE FROM roles WHERE id = $1", id)
                .execute(&mut *txn)
                .await?;

            txn.commit().await?;
        }

        let roles = Role::find_all()
            .await?
            .into_iter()
            .filter(|r| r.id != role.id)
            .collect::<Vec<Role>>();

        let client = DB::client();
        // clearing users cache is more safe and less resource intensive than trying to
        // update each single entry
        client.clear_cache(Cache::User).await?;
        client
            .put(Cache::App, IDX_ROLES, &roles, CACHE_TTL_APP)
            .await?;

        Ok(())
    }

    // Returns a single role by id
    pub async fn find(id: &str) -> Result<Self, ErrorResponse> {
        let res = if is_hiqlite() {
            DB::client()
                .query_as_one("SELECT * FROM roles WHERE id = $1", params!(id))
                .await?
        } else {
            sqlx::query_as!(Self, "SELECT * FROM roles WHERE id = $1", id)
                .fetch_one(DB::conn())
                .await?
        };

        Ok(res)
    }

    // Returns all existing roles
    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let client = DB::client();
        if let Some(slf) = client.get(Cache::App, IDX_ROLES).await? {
            return Ok(slf);
        }

        let res = if is_hiqlite() {
            DB::client()
                .query_as("SELECT * FROM roles", params!())
                .await?
        } else {
            sqlx::query_as!(Self, "SELECT * FROM roles")
                .fetch_all(DB::conn())
                .await?
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

        if is_hiqlite() {
            let mut txn: Vec<(&str, Params)> = Vec::with_capacity(users.len() + 1);

            for mut user in users {
                user.roles = user.roles.replace(&role.name, &new_role.name);
                user.save_txn_append(&mut txn);
            }

            txn.push((
                "UPDATE roles SET name = $1 WHERE id = $2",
                params!(new_role.name.clone(), new_role.id.clone()),
            ));

            for res in DB::client().txn(txn).await? {
                let rows_affected = res?;
                debug_assert!(rows_affected == 1);
            }
        } else {
            let mut txn = DB::txn().await?;

            for mut user in users {
                user.delete_role(&role.name);
                user.save_txn(&mut txn).await?;
            }
            sqlx::query!(
                "UPDATE roles SET name = $1 WHERE id = $2",
                new_role.name,
                new_role.id,
            )
            .execute(&mut *txn)
            .await?;

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

        let client = DB::client();
        client.clear_cache(Cache::User).await?;
        DB::client()
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
