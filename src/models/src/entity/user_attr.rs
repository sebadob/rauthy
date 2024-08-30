use crate::app_state::{AppState, DbTxn};
use crate::cache::{Cache, DB};
use crate::entity::scopes::Scope;
use crate::entity::users::User;
use actix_web::web;
use rauthy_api_types::users::{
    UserAttrConfigRequest, UserAttrConfigValueResponse, UserAttrValueResponse,
    UserAttrValuesUpdateRequest,
};
use rauthy_common::constants::{CACHE_TTL_APP, CACHE_TTL_USER, IDX_USER_ATTR_CONFIG};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;
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

        #[cfg(not(feature = "postgres"))]
        let q = sqlx::query!(
            "INSERT INTO user_attr_config (name, desc) VALUES ($1, $2)",
            new_attr.name,
            new_attr.desc,
        );

        #[cfg(feature = "postgres")]
        let q = sqlx::query!(
            "INSERT INTO user_attr_config (name, \"desc\") VALUES ($1, $2)",
            new_attr.name,
            new_attr.desc,
        );

        q.execute(&data.db).await?;

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

        let mut txn = data.db.begin().await?;

        // delete all possible scope mappings
        let scopes = Scope::find_all(data).await?;
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
                Scope::update_mapping_only(
                    data,
                    &s.id,
                    attr_include_access,
                    attr_include_id,
                    &mut txn,
                )
                .await?;
            }
        }

        let client = DB::client();
        client
            .delete(Cache::App, Self::cache_idx(&slf.name))
            .await?;

        UserAttrValueEntity::delete_all_by_key(data, &name, &mut txn).await?;

        sqlx::query!("DELETE FROM user_attr_config WHERE name  = $1", name)
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;

        let attrs = Self::find_all(data)
            .await?
            .into_iter()
            .filter(|a| a.name != name)
            .collect::<Vec<Self>>();

        client
            .put(Cache::App, IDX_USER_ATTR_CONFIG, &attrs, CACHE_TTL_APP)
            .await?;

        Ok(())
    }

    pub async fn find(data: &web::Data<AppState>, name: String) -> Result<Self, ErrorResponse> {
        let client = DB::client();
        if let Some(slf) = client.get(Cache::App, Self::cache_idx(&name)).await? {
            return Ok(slf);
        }

        let attr = sqlx::query_as!(Self, "SELECT * FROM user_attr_config WHERE name = $1", name)
            .fetch_one(&data.db)
            .await?;

        client
            .put(
                Cache::App,
                Self::cache_idx(&attr.name),
                &attr,
                CACHE_TTL_APP,
            )
            .await?;
        Ok(attr)
    }

    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let client = DB::client();
        if let Some(slf) = client.get(Cache::App, IDX_USER_ATTR_CONFIG).await? {
            return Ok(slf);
        }

        let res = sqlx::query_as!(Self, "SELECT * FROM user_attr_config")
            .fetch_all(&data.db)
            .await?;

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

        let is_name_update = name != req_data.name;

        // collect all current value IDs with the setting for cache clear on success
        let cache_idxs = if is_name_update {
            let idx = sqlx::query_as::<_, UserAttrValueEntity>(
                "SELECT * FROM user_attr_values WHERE key = $1",
            )
            .bind(&name)
            .fetch_all(&data.db)
            .await?
            .into_iter()
            .map(|a| UserAttrValueEntity::cache_idx(&a.user_id))
            .collect::<Vec<String>>();
            Some(idx)
        } else {
            None
        };

        let mut txn = data.db.begin().await?;

        #[cfg(not(feature = "postgres"))]
        let q = sqlx::query!(
            "UPDATE user_attr_config SET name  = $1, desc = $2 WHERE name = $3",
            slf.name,
            slf.desc,
            name,
        );

        #[cfg(feature = "postgres")]
        let q = sqlx::query!(
            "UPDATE user_attr_config SET name  = $1, \"desc\" = $2 WHERE name = $3",
            slf.name,
            slf.desc,
            name,
        );

        q.execute(&mut *txn).await?;

        if is_name_update {
            // update all possible scope mappings
            let scopes = Scope::find_all(data).await?;
            for s in scopes {
                let mut needs_update = false;

                let attr_include_access = if let Some(access) = s.attr_include_access {
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

                let attr_include_id = if let Some(id) = s.attr_include_id {
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
                    Scope::update_mapping_only(
                        data,
                        &s.id,
                        attr_include_access,
                        attr_include_id,
                        &mut txn,
                    )
                    .await?;
                }
            }
        }

        txn.commit().await?;

        let attrs = Self::find_all(data)
            .await?
            .into_iter()
            .map(|mut attr| {
                if attr.name == name {
                    attr.name.clone_from(&slf.name);
                    attr.desc.clone_from(&slf.desc);
                }
                attr
            })
            .collect::<Vec<Self>>();

        let client = DB::client();
        client
            .put(Cache::App, IDX_USER_ATTR_CONFIG, &attrs, CACHE_TTL_APP)
            .await?;

        if let Some(idxs) = cache_idxs {
            for idx in idxs {
                client.delete(Cache::User, idx).await?;
            }
        }

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
    pub async fn delete_all_by_key(
        data: &web::Data<AppState>,
        key: &str,
        txn: &mut DbTxn<'_>,
    ) -> Result<(), ErrorResponse> {
        let cache_idxs =
            sqlx::query_as!(Self, "SELECT * FROM user_attr_values WHERE key = $1", key)
                .fetch_all(&data.db)
                .await?
                .into_iter()
                .map(|a| Self::cache_idx(&a.user_id))
                .collect::<Vec<String>>();

        let client = DB::client();
        for idx in cache_idxs {
            client.delete(Cache::User, idx).await?;
        }

        sqlx::query!("DELETE FROM user_attr_values WHERE key = $1", key)
            .execute(&mut **txn)
            .await?;

        Ok(())
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

        let res = sqlx::query_as!(
            Self,
            "SELECT * FROM user_attr_values WHERE user_id = $1",
            user_id
        )
        .fetch_all(&data.db)
        .await?;

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

        for value in req_data.values {
            let del = if let Some(s) = value.value.as_str() {
                s.is_empty()
            } else {
                false
            };

            if del || value.value == Value::Null {
                sqlx::query!(
                    "DELETE FROM user_attr_values WHERE user_id = $1 AND key = $2",
                    user_id,
                    value.key,
                )
                .execute(&data.db)
                .await?;
            } else {
                let v = serde_json::to_vec(&value.value).unwrap();

                #[cfg(not(feature = "postgres"))]
                let q = sqlx::query!(
                    r#"INSERT OR REPLACE INTO
                    user_attr_values (user_id, key, value)
                    VALUES ($1, $2, $3)"#,
                    user_id,
                    value.key,
                    v,
                );

                #[cfg(feature = "postgres")]
                let q = sqlx::query!(
                    r#"INSERT INTO user_attr_values (user_id, key, value)
                    VALUES ($1, $2, $3)
                    ON CONFLICT(user_id, key) DO UPDATE SET value = $3"#,
                    user_id,
                    value.key,
                    v,
                );

                q.execute(&data.db).await?;
            }
        }

        // 2nd query again to have more compatibility
        let res = sqlx::query_as!(
            Self,
            "SELECT * FROM user_attr_values WHERE user_id = $1",
            user_id
        )
        .fetch_all(&data.db)
        .await?;

        let idx = Self::cache_idx(user_id);
        DB::client()
            .put(Cache::User, idx, &res, CACHE_TTL_USER)
            .await?;

        Ok(res)
    }
}

impl UserAttrValueEntity {
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
