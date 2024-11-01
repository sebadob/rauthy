use crate::app_state::{AppState, DbTxn};
use crate::entity::scopes::Scope;
use crate::entity::users::User;
use crate::hiqlite::{Cache, DB};
use actix_web::web;
use hiqlite::{params, Param, Params};
use rauthy_api_types::users::{
    UserAttrConfigRequest, UserAttrConfigValueResponse, UserAttrValueResponse,
    UserAttrValuesUpdateRequest,
};
use rauthy_common::constants::{CACHE_TTL_APP, CACHE_TTL_USER, IDX_USER_ATTR_CONFIG};
use rauthy_common::is_hiqlite;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{FromRow, Row};
use std::collections::HashSet;

// Additional custom attributes for users. These can be set for every user and then mapped to a
// scope, to include them in JWT tokens.
#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct UserAttrConfigEntity {
    pub name: String,
    pub desc: Option<String>,
}

// CRUD
impl UserAttrConfigEntity {
    #[inline(always)]
    fn cache_idx(name: &str) -> String {
        format!("{}_{}", IDX_USER_ATTR_CONFIG, name)
    }

    pub async fn clear_cache_all() -> Result<(), ErrorResponse> {
        DB::client()
            .delete(Cache::App, IDX_USER_ATTR_CONFIG)
            .await?;
        Ok(())
    }

    pub async fn create(
        data: &web::Data<AppState>,
        new_attr: UserAttrConfigRequest,
    ) -> Result<Self, ErrorResponse> {
        if Self::find(data, new_attr.name.clone()).await.is_ok() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "User attribute config does already exist",
            ));
        }

        if is_hiqlite() {
            DB::client()
                .execute(
                    "INSERT INTO user_attr_config (name, \"desc\") VALUES ($1, $2)",
                    params!(&new_attr.name, &new_attr.desc),
                )
                .await?;
        } else {
            sqlx::query!(
                "INSERT INTO user_attr_config (name, \"desc\") VALUES ($1, $2)",
                new_attr.name,
                new_attr.desc,
            )
            .execute(&data.db)
            .await?;
        };

        let mut attrs = UserAttrConfigEntity::find_all(data).await?;
        let slf = Self {
            name: new_attr.name.clone(),
            desc: new_attr.desc.clone(),
        };
        attrs.push(slf.clone());
        DB::client()
            .put(Cache::App, IDX_USER_ATTR_CONFIG, &attrs, CACHE_TTL_APP)
            .await?;

        Ok(slf)
    }

    pub async fn delete(data: &web::Data<AppState>, name: String) -> Result<(), ErrorResponse> {
        // we do this empty check beforehand to avoid much more unnecessary work if it does not exist anyway
        let slf = Self::find(data, name.clone()).await?;

        // delete all possible scope mappings
        let scopes = Scope::find_all(data).await?;
        let mut scope_updates = Vec::new();
        for s in scopes {
            let mut needs_update = false;

            let attr_include_access = if let Some(access) = s.attr_include_access {
                if access.contains(&name) {
                    needs_update = true;
                    let a = access.replace(&format!("{},", name), "").replace(&name, "");
                    if a.is_empty() {
                        None
                    } else {
                        Some(a)
                    }
                } else {
                    Some(access)
                }
            } else {
                None
            };

            let attr_include_id = if let Some(id) = s.attr_include_id {
                if id.contains(&name) {
                    needs_update = true;
                    let i = id.replace(&format!("{},", name), "").replace(&name, "");
                    if i.is_empty() {
                        None
                    } else {
                        Some(i)
                    }
                } else {
                    Some(id)
                }
            } else {
                None
            };

            if needs_update {
                scope_updates.push((s.id, attr_include_access, attr_include_id));
            }
        }

        let client = DB::client();
        let user_attr_cache_cleanup_keys;

        if is_hiqlite() {
            // we can't know the exact capacity upfront, the +8 just reserves some additional space
            let mut txn = Vec::with_capacity(scope_updates.len() + 8);

            for (id, attr_include_access, attr_include_id) in scope_updates {
                Scope::update_mapping_only_append(
                    &id,
                    attr_include_access,
                    attr_include_id,
                    &mut txn,
                );
            }

            user_attr_cache_cleanup_keys =
                UserAttrValueEntity::delete_all_by_key_append(&name, &mut txn).await?;

            txn.push((
                "DELETE FROM user_attr_config WHERE name  = $1",
                params!(name),
            ));

            client.txn(txn).await?;
        } else {
            let mut txn = data.db.begin().await?;

            for (id, attr_include_access, attr_include_id) in scope_updates {
                Scope::update_mapping_only(&id, attr_include_access, attr_include_id, &mut txn)
                    .await?;
            }

            user_attr_cache_cleanup_keys =
                UserAttrValueEntity::delete_all_by_key(data, &name, &mut txn).await?;

            sqlx::query!("DELETE FROM user_attr_config WHERE name  = $1", name)
                .execute(&mut *txn)
                .await?;

            txn.commit().await?;
        }

        for key in user_attr_cache_cleanup_keys {
            UserAttrValueEntity::clear_cache(key).await?;
        }

        client
            .delete(Cache::App, Self::cache_idx(&slf.name))
            .await?;
        Self::clear_cache_all().await?;
        Scope::clear_cache().await?;

        Ok(())
    }

    pub async fn find(data: &web::Data<AppState>, name: String) -> Result<Self, ErrorResponse> {
        let client = DB::client();
        if let Some(slf) = client.get(Cache::App, Self::cache_idx(&name)).await? {
            return Ok(slf);
        }

        let slf = if is_hiqlite() {
            client
                .query_as_one(
                    "SELECT * FROM user_attr_config WHERE name = $1",
                    params!(name),
                )
                .await?
        } else {
            sqlx::query_as!(Self, "SELECT * FROM user_attr_config WHERE name = $1", name)
                .fetch_one(&data.db)
                .await?
        };

        client
            .put(Cache::App, Self::cache_idx(&slf.name), &slf, CACHE_TTL_APP)
            .await?;

        Ok(slf)
    }

    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let client = DB::client();
        if let Some(slf) = client.get(Cache::App, IDX_USER_ATTR_CONFIG).await? {
            return Ok(slf);
        }

        let res = if is_hiqlite() {
            client
                .query_as("SELECT * FROM user_attr_config", params!())
                .await?
        } else {
            sqlx::query_as!(Self, "SELECT * FROM user_attr_config")
                .fetch_all(&data.db)
                .await?
        };

        client
            .put(Cache::App, IDX_USER_ATTR_CONFIG, &res, CACHE_TTL_APP)
            .await?;

        Ok(res)
    }

    pub async fn update(
        data: &web::Data<AppState>,
        name: String,
        req_data: UserAttrConfigRequest,
    ) -> Result<Self, ErrorResponse> {
        let mut slf = Self::find(data, name.clone()).await?;

        slf.name.clone_from(&req_data.name);
        slf.desc.clone_from(&req_data.desc);

        let client = DB::client();
        let mut scope_updates = Vec::new();

        // we only need to update pre-computed data in other places if the name changes
        let user_attr_ids_cleanup = if name != req_data.name {
            let user_attr_cache_clear_idxs = if is_hiqlite() {
                client
                    .query_raw(
                        "SELECT user_id FROM user_attr_values WHERE key = $1",
                        params!(&name),
                    )
                    .await?
                    .into_iter()
                    .map(|mut row| row.get::<String>("user_id"))
                    .collect::<Vec<_>>()
            } else {
                sqlx::query("SELECT user_id FROM user_attr_values WHERE key = $1")
                    .bind(&name)
                    .fetch_all(&data.db)
                    .await?
                    .into_iter()
                    .map(|row| row.get("user_id"))
                    .collect::<Vec<_>>()
            };

            // update all possible scope mappings
            let scopes = Scope::find_all(data).await?;
            for scope in scopes {
                let mut needs_update = false;

                let attr_include_access = if let Some(access) = scope.attr_include_access {
                    if access.contains(&name) {
                        needs_update = true;
                        let a = access.replace(&name, &req_data.name);
                        Some(a)
                    } else {
                        Some(access)
                    }
                } else {
                    None
                };

                let attr_include_id = if let Some(id) = scope.attr_include_id {
                    if id.contains(&name) {
                        needs_update = true;
                        let i = id.replace(&name, &req_data.name);
                        Some(i)
                    } else {
                        Some(id)
                    }
                } else {
                    None
                };

                if needs_update {
                    scope_updates.push((scope.id, attr_include_access, attr_include_id));
                }
            }

            user_attr_cache_clear_idxs
        } else {
            Vec::default()
        };

        if is_hiqlite() {
            let mut txn = Vec::with_capacity(scope_updates.len() + 1);

            for (id, attr_include_access, attr_include_id) in scope_updates {
                Scope::update_mapping_only_append(
                    &id,
                    attr_include_access,
                    attr_include_id,
                    &mut txn,
                );
            }

            // TODO double check after hiqlite migration if we really don't
            // need another user_attr_values update here

            txn.push((
                "UPDATE user_attr_config SET name  = $1, \"desc\" = $2 WHERE name = $3",
                params!(&slf.name, &slf.desc, name),
            ));

            client.txn(txn).await?;
        } else {
            let mut txn = data.db.begin().await?;

            for (id, attr_include_access, attr_include_id) in scope_updates {
                Scope::update_mapping_only(&id, attr_include_access, attr_include_id, &mut txn)
                    .await?;
            }

            // TODO double check after hiqlite migration if we really don't
            // need another user_attr_values update here

            sqlx::query!(
                "UPDATE user_attr_config SET name  = $1, \"desc\" = $2 WHERE name = $3",
                slf.name,
                slf.desc,
                name,
            )
            .execute(&mut *txn)
            .await?;

            txn.commit().await?;
        }

        for user_id in user_attr_ids_cleanup {
            let key = UserAttrValueEntity::cache_idx(&user_id);
            UserAttrValueEntity::clear_cache(key).await?;
        }
        Self::clear_cache_all().await?;
        Scope::clear_cache().await?;

        Ok(slf)
    }
}

impl UserAttrConfigEntity {
    pub async fn find_all_as_set(
        data: &web::Data<AppState>,
    ) -> Result<HashSet<String>, ErrorResponse> {
        let attrs = Self::find_all(data).await?;

        let mut set = HashSet::with_capacity(attrs.len());
        for a in attrs {
            set.insert(a.name);
        }

        Ok(set)
    }

    pub fn names_hash_set(mut slf: Vec<Self>) -> HashSet<String> {
        let mut res = HashSet::with_capacity(slf.len());
        slf.drain(..).for_each(|s| {
            res.insert(s.name);
        });
        res
    }
}

impl From<UserAttrConfigEntity> for UserAttrConfigValueResponse {
    fn from(value: UserAttrConfigEntity) -> Self {
        Self {
            name: value.name,
            desc: value.desc,
        }
    }
}

/// The value for a pre-defined UserAttrConfig with all `serde_json::Value` being valid values.
/// Important: There is no further input validation / restriction
#[derive(Clone, Debug, FromRow, Serialize, Deserialize)]
pub struct UserAttrValueEntity {
    pub user_id: String,
    pub key: String,
    pub value: Vec<u8>,
}

impl UserAttrValueEntity {
    pub async fn clear_cache(cache_idx: String) -> Result<(), ErrorResponse> {
        DB::client().delete(Cache::User, cache_idx).await?;
        Ok(())
    }

    /// You MUST `UserAttrValueEntity::clear_cache()` for returned UserCacheKeys
    /// after successful txn commit!
    pub async fn delete_all_by_key(
        data: &web::Data<AppState>,
        key: &str,
        txn: &mut DbTxn<'_>,
    ) -> Result<Vec<String>, ErrorResponse> {
        let cache_idxs =
            sqlx::query_as!(Self, "SELECT * FROM user_attr_values WHERE key = $1", key)
                .fetch_all(&data.db)
                .await?
                .into_iter()
                .map(|a| Self::cache_idx(&a.user_id))
                .collect::<Vec<_>>();

        sqlx::query!("DELETE FROM user_attr_values WHERE key = $1", key)
            .execute(&mut **txn)
            .await?;

        Ok(cache_idxs)
    }

    /// You MUST `UserAttrValueEntity::clear_cache()` for returned UserCacheKeys
    /// after successful txn commit!
    pub async fn delete_all_by_key_append(
        key: &str,
        txn: &mut Vec<(&str, Params)>,
    ) -> Result<Vec<String>, ErrorResponse> {
        let cache_idxs = DB::client()
            .query_raw(
                "SELECT user_id FROM user_attr_values WHERE key = $1",
                params!(key),
            )
            .await?
            .into_iter()
            .map(|mut row| Self::cache_idx(&row.get::<String>("user_id")))
            .collect::<Vec<_>>();

        txn.push(("DELETE FROM user_attr_values WHERE key = $1", params!(key)));

        Ok(cache_idxs)
    }

    pub async fn find_for_user(
        data: &web::Data<AppState>,
        user_id: &str,
    ) -> Result<Vec<Self>, ErrorResponse> {
        let idx = Self::cache_idx(user_id);
        let client = DB::client();

        if let Some(slf) = client.get(Cache::User, &idx).await? {
            return Ok(slf);
        }

        let res = if is_hiqlite() {
            client
                .query_as(
                    "SELECT * FROM user_attr_values WHERE user_id = $1",
                    params!(user_id),
                )
                .await?
        } else {
            sqlx::query_as!(
                Self,
                "SELECT * FROM user_attr_values WHERE user_id = $1",
                user_id
            )
            .fetch_all(&data.db)
            .await?
        };

        client.put(Cache::User, idx, &res, CACHE_TTL_USER).await?;

        Ok(res)
    }

    pub async fn update_for_user(
        data: &web::Data<AppState>,
        user_id: &str,
        req_data: UserAttrValuesUpdateRequest,
    ) -> Result<Vec<Self>, ErrorResponse> {
        // Not necessary for the operation and correctness, but look up the user first and return
        // an error, if it does not exist at all, for a better user experience.
        User::exists(data, user_id.to_string()).await?;

        let delete_value = |value: &Value| {
            if let Some(s) = value.as_str() {
                s.is_empty() || value == &Value::Null
            } else {
                value == &Value::Null
            }
        };

        let client = DB::client();

        let res = if is_hiqlite() {
            let mut txn = Vec::with_capacity(req_data.values.len());

            for value in req_data.values {
                if delete_value(&value.value) {
                    txn.push((
                        "DELETE FROM user_attr_values WHERE user_id = $1 AND key = $2",
                        params!(user_id, value.key),
                    ));
                } else {
                    let v = serde_json::to_vec(&value.value)?;
                    txn.push((
                        r#"
INSERT INTO user_attr_values (user_id, key, value)
VALUES ($1, $2, $3)
ON CONFLICT(user_id, key) DO UPDATE SET value = $3"#,
                        params!(user_id, value.key, v),
                    ));
                }
            }

            client.txn(txn).await?;
            client
                .query_as(
                    "SELECT * FROM user_attr_values WHERE user_id = $1",
                    params!(user_id),
                )
                .await?
        } else {
            let mut txn = data.db.begin().await?;

            for value in req_data.values {
                if delete_value(&value.value) {
                    sqlx::query!(
                        "DELETE FROM user_attr_values WHERE user_id = $1 AND key = $2",
                        user_id,
                        value.key,
                    )
                    .execute(&mut *txn)
                    .await?;
                } else {
                    let v = serde_json::to_vec(&value.value)?;
                    sqlx::query!(
                        r#"
    INSERT INTO user_attr_values (user_id, key, value)
    VALUES ($1, $2, $3)
    ON CONFLICT(user_id, key) DO UPDATE SET value = $3"#,
                        user_id,
                        value.key,
                        v,
                    )
                    .execute(&mut *txn)
                    .await?;
                }
            }

            txn.commit().await?;

            sqlx::query_as!(
                Self,
                "SELECT * FROM user_attr_values WHERE user_id = $1",
                user_id
            )
            .fetch_all(&data.db)
            .await?
        };

        let idx = Self::cache_idx(user_id);
        client.put(Cache::User, idx, &res, CACHE_TTL_USER).await?;

        Ok(res)
    }
}

impl UserAttrValueEntity {
    #[inline]
    fn cache_idx(user_id: &str) -> String {
        format!("{}{}", IDX_USER_ATTR_CONFIG, user_id)
    }
}

impl From<UserAttrValueEntity> for UserAttrValueResponse {
    fn from(value: UserAttrValueEntity) -> Self {
        let val = serde_json::from_slice(&value.value).unwrap();
        Self {
            key: value.key,
            value: val,
        }
    }
}
