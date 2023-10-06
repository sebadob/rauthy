use crate::app_state::AppState;
use crate::entity::users::User;
use crate::request::NewRoleRequest;
use actix_web::web;
use rauthy_common::constants::{CACHE_NAME_12HR, IDX_ROLES, IDX_USERS};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::new_store_id;
use redhac::{cache_get, cache_get_from, cache_get_value, cache_insert, cache_remove, AckLevel};
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
                    "Role already exists".to_string(),
                ));
            }
        }

        let new_role = Role {
            id: new_store_id(),
            name: role_req.role,
        };
        sqlx::query!(
            "insert into roles (id, name) values ($1, $2)",
            new_role.id,
            new_role.name,
        )
        .execute(&data.db)
        .await?;

        roles.push(new_role.clone());
        cache_insert(
            CACHE_NAME_12HR.to_string(),
            IDX_ROLES.to_string(),
            &data.caches.ha_cache_config,
            &roles,
            AckLevel::Quorum,
        )
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
                "Anti-Lockout Rule: The 'rauthy_admin' role cannot be deleted".to_string(),
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

        for user in users {
            user.save(data, None, Some(&mut txn)).await?;
        }

        sqlx::query!("delete from roles where id = $1", id)
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;
        // DATA_STORE.del(Cf::Roles, role.id.clone()).await?;

        let roles = Role::find_all(data)
            .await?
            .into_iter()
            .filter(|r| r.id != role.id)
            .collect::<Vec<Role>>();
        cache_insert(
            CACHE_NAME_12HR.to_string(),
            IDX_ROLES.to_string(),
            &data.caches.ha_cache_config,
            &roles,
            AckLevel::Quorum,
        )
        .await?;

        Ok(())
    }

    // Returns a single role by id
    pub async fn find(data: &web::Data<AppState>, id: &str) -> Result<Self, ErrorResponse> {
        let res = sqlx::query_as!(Self, "select * from roles where id = $1", id)
            .fetch_one(&data.db)
            .await?;

        Ok(res)
    }

    // Returns all existing roles
    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let roles = cache_get!(
            Vec<Role>,
            CACHE_NAME_12HR.to_string(),
            IDX_ROLES.to_string(),
            &data.caches.ha_cache_config,
            false
        )
        .await?;
        if let Some(roles) = roles {
            return Ok(roles);
        }

        let res = sqlx::query_as!(Self, "select * from roles")
            .fetch_all(&data.db)
            .await?;

        cache_insert(
            CACHE_NAME_12HR.to_string(),
            IDX_ROLES.to_string(),
            &data.caches.ha_cache_config,
            &res,
            AckLevel::Leader,
        )
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

        // no need to evict the cache if no users are updated
        if !users.is_empty() {
            cache_remove(
                CACHE_NAME_12HR.to_string(),
                IDX_USERS.to_string(),
                &data.caches.ha_cache_config,
                AckLevel::Leader,
            )
            .await?;
        }

        let mut txn = data.db.begin().await?;

        for user in users {
            user.save(data, None, Some(&mut txn)).await?;
        }

        let new_role = Role { id, name: new_name };
        sqlx::query!(
            "update roles set name = $1 where id = $2",
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
                    r.name = new_role.name.clone();
                }
                r
            })
            .collect::<Vec<Role>>();
        cache_insert(
            CACHE_NAME_12HR.to_string(),
            IDX_ROLES.to_string(),
            &data.caches.ha_cache_config,
            &roles,
            AckLevel::Quorum,
        )
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
