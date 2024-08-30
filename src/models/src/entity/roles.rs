use crate::app_state::AppState;
use crate::cache::{Cache, DB};
use crate::entity::users::User;
use actix_web::web;
use rauthy_api_types::roles::NewRoleRequest;
use rauthy_common::constants::{CACHE_TTL_APP, IDX_ROLES};
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
    pub async fn create(
        data: &web::Data<AppState>,
        role_req: NewRoleRequest,
    ) -> Result<Self, ErrorResponse> {
        let mut roles = Role::find_all(data).await?;
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
        sqlx::query!(
            "INSERT INTO roles (id, name) VALUES ($1, $2)",
            new_role.id,
            new_role.name,
        )
        .execute(&data.db)
        .await?;

        roles.push(new_role.clone());
        DB::client()
            .put(Cache::App, IDX_ROLES, &roles, CACHE_TTL_APP)
            .await?;

        Ok(new_role)
    }

    // Deletes a role
    pub async fn delete(data: &web::Data<AppState>, id: &str) -> Result<(), ErrorResponse> {
        let role = Role::find(data, id).await?;

        // prevent deletion of 'rauthy_admin'
        if role.name == "rauthy_admin" {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Anti-Lockout Rule: The 'rauthy_admin' role cannot be deleted",
            ));
        }

        // before deleting a role, cleanup every user
        // get all users with the to-be-deleted-role assigned
        let mut users = vec![];
        // TODO wrap in transaction after migration
        User::find_all(data)
            .await?
            .into_iter()
            .filter(|u| u.roles.contains(&role.name))
            .for_each(|mut u| {
                u.delete_role(&role.name);
                users.push(u);
            });

        let mut txn = data.db.begin().await?;

        for user in users {
            // TODO wrap inside single txn after hiqlite migrations
            user.save(data, None, Some(&mut txn)).await?;
        }

        sqlx::query!("DELETE FROM roles WHERE id = $1", id)
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;
        // DATA_STORE.del(Cf::Roles, role.id.clone()).await?;

        let roles = Role::find_all(data)
            .await?
            .into_iter()
            .filter(|r| r.id != role.id)
            .collect::<Vec<Role>>();
        DB::client()
            .put(Cache::App, IDX_ROLES, &roles, CACHE_TTL_APP)
            .await?;

        Ok(())
    }

    // Returns a single role by id
    pub async fn find(data: &web::Data<AppState>, id: &str) -> Result<Self, ErrorResponse> {
        let res = sqlx::query_as!(Self, "SELECT * FROM roles WHERE id = $1", id)
            .fetch_one(&data.db)
            .await?;

        Ok(res)
    }

    // Returns all existing roles
    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let client = DB::client();
        if let Some(slf) = client.get(Cache::App, IDX_ROLES).await? {
            return Ok(slf);
        }

        let res = sqlx::query_as!(Self, "SELECT * FROM roles")
            .fetch_all(&data.db)
            .await?;

        client
            .put(Cache::App, IDX_ROLES, &res, CACHE_TTL_APP)
            .await?;
        Ok(res)
    }

    // Updates a role
    pub async fn update(
        data: &web::Data<AppState>,
        id: String,
        new_name: String,
    ) -> Result<Self, ErrorResponse> {
        let role = Role::find(data, &id).await?;

        // find all users with the old_name assigned
        let mut users = vec![];
        User::find_all(data)
            .await?
            .into_iter()
            .filter(|u| u.roles.contains(&role.name))
            .for_each(|mut u| {
                u.roles = u.roles.replace(&role.name, &new_name);
                users.push(u);
            });

        let mut txn = data.db.begin().await?;

        for user in users {
            user.save(data, None, Some(&mut txn)).await?;
        }

        let new_role = Role { id, name: new_name };
        sqlx::query!(
            "UPDATE roles SET name = $1 WHERE id = $2",
            new_role.name,
            new_role.id,
        )
        .execute(&mut *txn)
        .await?;

        txn.commit().await?;

        let roles = Role::find_all(data)
            .await?
            .into_iter()
            .map(|mut r| {
                if r.id == role.id {
                    r.name.clone_from(&new_role.name);
                }
                r
            })
            .collect::<Vec<Role>>();
        DB::client()
            .put(Cache::App, IDX_ROLES, &roles, CACHE_TTL_APP)
            .await?;

        Ok(new_role)
    }
}

impl Role {
    pub async fn sanitize(
        data: &web::Data<AppState>,
        rls: Vec<String>,
    ) -> Result<String, ErrorResponse> {
        let mut res = String::with_capacity(rls.len());
        Role::find_all(data).await?.into_iter().for_each(|r| {
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
