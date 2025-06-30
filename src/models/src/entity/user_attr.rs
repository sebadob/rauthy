use crate::database::{Cache, DB};
use crate::entity::scopes::Scope;
use crate::entity::users::User;
use deadpool_postgres::GenericClient;
use hiqlite::Params;
use hiqlite_macros::params;
use rauthy_api_types::users::{
    UserAttrConfigRequest, UserAttrConfigTyp, UserAttrConfigValueResponse, UserAttrValueResponse,
    UserAttrValuesUpdateRequest,
};
use rauthy_common::constants::{CACHE_TTL_APP, CACHE_TTL_USER, IDX_USER_ATTR_CONFIG};
use rauthy_common::is_hiqlite;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{HashMap, HashSet};

// Additional custom attributes for users. These can be set for every user and then mapped to a
// scope, to include them in JWT tokens.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserAttrConfigEntity {
    pub name: String,
    pub desc: Option<String>,
    pub default_value: Option<Vec<u8>>,
    pub typ: Option<UserAttrConfigTyp>,
    pub user_editable: bool,
}

impl From<hiqlite::Row<'_>> for UserAttrConfigEntity {
    fn from(mut row: hiqlite::Row<'_>) -> Self {
        let typ = if let Ok(s) = row.try_get::<String>("typ") {
            UserAttrConfigTyp::try_from(s.as_str()).ok()
        } else {
            None
        };

        Self {
            name: row.get("name"),
            desc: row.get("desc"),
            default_value: row.get("default_value"),
            typ,
            user_editable: row.get("user_editable"),
        }
    }
}

impl From<tokio_postgres::Row> for UserAttrConfigEntity {
    fn from(row: tokio_postgres::Row) -> Self {
        let typ = if let Ok(s) = row.try_get::<_, String>("typ") {
            UserAttrConfigTyp::try_from(s.as_str()).ok()
        } else {
            None
        };

        Self {
            name: row.get("name"),
            desc: row.get("desc"),
            default_value: row.get("default_value"),
            typ,
            user_editable: row.get("user_editable"),
        }
    }
}

// CRUD
impl UserAttrConfigEntity {
    #[inline(always)]
    fn cache_idx(name: &str) -> String {
        format!("{IDX_USER_ATTR_CONFIG}_{name}")
    }

    pub async fn clear_cache_all() -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::App, IDX_USER_ATTR_CONFIG).await?;
        Ok(())
    }

    pub async fn create(new_attr: UserAttrConfigRequest) -> Result<Self, ErrorResponse> {
        if Self::find(new_attr.name.clone()).await.is_ok() {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "User attribute config does already exist",
            ));
        }

        let typ = new_attr.typ.as_ref().map(|t| t.as_str());
        let user_editable = new_attr.user_editable.unwrap_or(false);
        let default_value = if let Some(bytes) = &new_attr.default_value {
            Some(serde_json::to_vec(bytes)?)
        } else {
            None
        };

        if is_hiqlite() {
            DB::hql()
                .execute(
                    r#"
INSERT INTO user_attr_config (name, desc, default_value, typ, user_editable)
VALUES ($1, $2, $3, $4, $5)"#,
                    params!(
                        &new_attr.name,
                        &new_attr.desc,
                        &default_value,
                        typ,
                        user_editable
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                r#"
INSERT INTO user_attr_config (name, "desc", default_value, typ, user_editable)
VALUES ($1, $2, $3, $4, $5)"#,
                &[
                    &new_attr.name,
                    &new_attr.desc,
                    &default_value,
                    &typ,
                    &user_editable,
                ],
            )
            .await?;
        };

        let mut attrs = UserAttrConfigEntity::find_all().await?;

        let slf = Self {
            name: new_attr.name.clone(),
            desc: new_attr.desc.clone(),
            default_value: default_value.clone(),
            typ: new_attr.typ.clone(),
            user_editable: new_attr.user_editable.unwrap_or(false),
        };
        attrs.push(slf.clone());
        DB::hql()
            .put(Cache::App, IDX_USER_ATTR_CONFIG, &attrs, CACHE_TTL_APP)
            .await?;

        Ok(slf)
    }

    pub async fn delete(name: String) -> Result<(), ErrorResponse> {
        // we do this empty check beforehand to avoid much more unnecessary work if it does not exist anyway
        let slf = Self::find(name.clone()).await?;

        // delete all possible scope mappings
        let scopes = Scope::find_all().await?;
        let mut scope_updates = Vec::new();
        for s in scopes {
            let mut needs_update = false;

            let attr_include_access = if let Some(access) = s.attr_include_access {
                if access.contains(&name) {
                    needs_update = true;
                    let a = access.replace(&format!("{name},"), "").replace(&name, "");
                    if a.is_empty() { None } else { Some(a) }
                } else {
                    Some(access)
                }
            } else {
                None
            };

            let attr_include_id = if let Some(id) = s.attr_include_id {
                if id.contains(&name) {
                    needs_update = true;
                    let i = id.replace(&format!("{name},"), "").replace(&name, "");
                    if i.is_empty() { None } else { Some(i) }
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

        let client = DB::hql();
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
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            for (id, attr_include_access, attr_include_id) in scope_updates {
                Scope::update_mapping_only(&id, attr_include_access, attr_include_id, &txn).await?;
            }

            user_attr_cache_cleanup_keys =
                UserAttrValueEntity::delete_all_by_key(&name, &txn).await?;

            DB::pg_txn_append(
                &txn,
                "DELETE FROM user_attr_config WHERE name  = $1",
                &[&name],
            )
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

    pub async fn find(name: String) -> Result<Self, ErrorResponse> {
        let client = DB::hql();
        if let Some(slf) = client.get(Cache::App, Self::cache_idx(&name)).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM user_attr_config WHERE name = $1";
        let slf: Self = if is_hiqlite() {
            client.query_as_one(sql, params!(name)).await?
        } else {
            DB::pg_query_one(sql, &[&name]).await?
        };

        client
            .put(Cache::App, Self::cache_idx(&slf.name), &slf, CACHE_TTL_APP)
            .await?;

        Ok(slf)
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let client = DB::hql();
        if let Some(slf) = client.get(Cache::App, IDX_USER_ATTR_CONFIG).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM user_attr_config";
        let res = if is_hiqlite() {
            client.query_as(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };

        client
            .put(Cache::App, IDX_USER_ATTR_CONFIG, &res, CACHE_TTL_APP)
            .await?;

        Ok(res)
    }

    #[inline]
    pub async fn find_all_user_editable() -> Result<Vec<Self>, ErrorResponse> {
        // Self::find_all() is always cached. Therefore,
        // it is not that huge of a waste, if we filter later.
        Ok(Self::find_all()
            .await?
            .into_iter()
            .filter(|c| c.user_editable)
            .collect::<Vec<_>>())
    }

    #[inline]
    pub async fn find_with_default_value() -> Result<Vec<Self>, ErrorResponse> {
        // Even though with naively fetch all of them first and filter later, the `Self::find_all()`
        // is cached locally and therefore very fast. The total amount of custom attributes will
        // be small anyway.
        Ok(Self::find_all()
            .await?
            .into_iter()
            .filter(|v| v.default_value.is_some())
            .collect())
    }

    pub async fn update(
        name: String,
        req_data: UserAttrConfigRequest,
    ) -> Result<Self, ErrorResponse> {
        let mut slf = Self::find(name.clone()).await?;

        slf.name.clone_from(&req_data.name);
        slf.desc.clone_from(&req_data.desc);
        slf.user_editable = req_data.user_editable.unwrap_or(false);
        slf.default_value = if let Some(v) = &req_data.default_value {
            Some(serde_json::to_vec(v)?)
        } else {
            None
        };

        let client = DB::hql();
        let mut scope_updates = Vec::new();

        // we only need to update pre-computed data in other places if the name changes
        let user_attr_ids_cleanup = if name != req_data.name {
            let sql = "SELECT user_id FROM user_attr_values WHERE key = $1";

            let user_attr_cache_clear_idxs = if is_hiqlite() {
                client
                    .query_raw(sql, params!(&name))
                    .await?
                    .into_iter()
                    .map(|mut row| row.get::<String>("user_id"))
                    .collect::<Vec<_>>()
            } else {
                DB::pg_query_rows(sql, &[&name], 0)
                    .await?
                    .into_iter()
                    .map(|row| row.get::<_, String>("user_id"))
                    .collect::<Vec<_>>()
            };

            // update all possible scope mappings
            let scopes = Scope::find_all().await?;
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

        let typ = &slf.typ.as_ref().map(|t| t.as_str());

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
                r#"
UPDATE user_attr_config
SET name  = $1, desc = $2, default_value = $3, typ = $4, user_editable = $5
WHERE name = $6"#,
                params!(
                    &slf.name,
                    &slf.desc,
                    &slf.default_value,
                    typ,
                    slf.user_editable,
                    name
                ),
            ));

            client.txn(txn).await?;
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            for (id, attr_include_access, attr_include_id) in scope_updates {
                Scope::update_mapping_only(&id, attr_include_access, attr_include_id, &txn).await?;
            }

            // TODO double check if we really don't need another user_attr_values update here

            DB::pg_txn_append(
                &txn,
                r#"
UPDATE user_attr_config
SET name  = $1, "desc" = $2, default_value = $3, typ = $4, user_editable = $5
WHERE name = $6"#,
                &[
                    &slf.name,
                    &slf.desc,
                    &slf.default_value,
                    typ,
                    &slf.user_editable,
                    &name,
                ],
            )
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
    pub async fn find_all_as_set() -> Result<HashSet<String>, ErrorResponse> {
        let attrs = Self::find_all().await?;

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
            default_value: value
                .default_value
                .map(|v| serde_json::from_slice(&v).unwrap_or_default()),
            typ: value.typ,
            user_editable: value.user_editable,
        }
    }
}

/// The value for a pre-defined UserAttrConfig with all `serde_json::Value` being valid values.
/// Important: There is no further input validation / restriction
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserAttrValueEntity {
    pub user_id: String,
    pub key: String,
    pub value: Vec<u8>,
}

impl From<tokio_postgres::Row> for UserAttrValueEntity {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            user_id: row.get("user_id"),
            key: row.get("key"),
            value: row.get("value"),
        }
    }
}

impl UserAttrValueEntity {
    pub async fn clear_cache(cache_idx: String) -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::User, cache_idx).await?;
        Ok(())
    }

    /// You MUST `UserAttrValueEntity::clear_cache()` for returned UserCacheKeys
    /// after successful txn commit!
    pub async fn delete_all_by_key(
        key: &str,
        txn: &deadpool_postgres::Transaction<'_>,
    ) -> Result<Vec<String>, ErrorResponse> {
        let cache_idxs = DB::pg_query_rows(
            "SELECT user_id FROM user_attr_values WHERE key = $1",
            &[&key],
            0,
        )
        .await?
        .into_iter()
        .map(|r| {
            let user_id: String = r.get("user_id");
            Self::cache_idx(&user_id)
        })
        .collect::<Vec<_>>();

        DB::pg_txn_append(txn, "DELETE FROM user_attr_values WHERE key = $1", &[&key]).await?;

        Ok(cache_idxs)
    }

    /// You MUST `UserAttrValueEntity::clear_cache()` for returned UserCacheKeys
    /// after successful txn commit!
    pub async fn delete_all_by_key_append(
        key: &str,
        txn: &mut Vec<(&str, Params)>,
    ) -> Result<Vec<String>, ErrorResponse> {
        let cache_idxs = DB::hql()
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

    pub async fn find_for_user(user_id: &str) -> Result<Vec<Self>, ErrorResponse> {
        let idx = Self::cache_idx(user_id);
        let client = DB::hql();

        if let Some(slf) = client.get(Cache::User, &idx).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM user_attr_values WHERE user_id = $1";
        let res = if is_hiqlite() {
            client.query_as(sql, params!(user_id)).await?
        } else {
            DB::pg_query(sql, &[&user_id], 0).await?
        };

        client.put(Cache::User, idx, &res, CACHE_TTL_USER).await?;

        Ok(res)
    }

    /// Finds all existing values for the user and expands the result with default values from
    /// config entities, if any exist.
    #[inline]
    pub async fn find_for_user_with_defaults(user_id: &str) -> Result<Vec<Self>, ErrorResponse> {
        let mut res = Self::find_for_user(user_id).await?;
        let defaults = UserAttrConfigEntity::find_with_default_value().await?;

        // The total amount of user attrs with default values is probably very low, which makes
        // a `Vec<_>` and iterating over all of them the better option compared to a HashMap.
        for default in defaults {
            if !res.iter().any(|v| v.key == default.name) {
                debug_assert!(default.default_value.is_some());
                if let Some(value) = default.default_value {
                    res.push(Self {
                        user_id: user_id.to_string(),
                        key: default.name,
                        value,
                    });
                }
            }
        }

        Ok(res)
    }

    #[inline]
    pub async fn find_key_value_by_scope(
        scopes: &str,
        user_id: String,
    ) -> Result<Option<HashMap<String, serde_json::Value>>, ErrorResponse> {
        let values = Self::find_for_user(&user_id).await?;
        if values.is_empty() {
            return Ok(None);
        }

        let scopes = Scope::find_all()
            .await?
            .into_iter()
            .filter_map(|s| {
                if Scope::is_custom(&s.name)
                    && scopes.contains(&s.name)
                    && s.attr_include_access.is_some()
                    || s.attr_include_id.is_some()
                {
                    let mut incl = s.attr_include_access.unwrap_or_default();
                    incl.push_str(&s.attr_include_id.unwrap_or_default());
                    Some(incl)
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let mut res = HashMap::with_capacity(values.len());
        for value in values {
            for scope in &scopes {
                if scope.contains(&value.key) {
                    if let Ok(json) = serde_json::from_slice(value.value.as_slice()) {
                        res.insert(value.key, json);
                    }
                    break;
                }
            }
        }

        if res.is_empty() {
            Ok(None)
        } else {
            Ok(Some(res))
        }
    }

    pub async fn update_for_user(
        user_id: &str,
        req_data: UserAttrValuesUpdateRequest,
    ) -> Result<Vec<Self>, ErrorResponse> {
        // Not necessary for the operation and correctness, but look up the user first and return
        // an error, if it does not exist at all, for a better user experience.
        User::exists(user_id.to_string()).await?;

        let delete_value = |value: &Value| {
            if let Some(s) = value.as_str() {
                s.is_empty() || value == &Value::Null
            } else {
                value == &Value::Null
            }
        };

        let client = DB::hql();

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
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            for value in req_data.values {
                if delete_value(&value.value) {
                    DB::pg_txn_append(
                        &txn,
                        "DELETE FROM user_attr_values WHERE user_id = $1 AND key = $2",
                        &[&user_id, &value.key],
                    )
                    .await?;
                } else {
                    let v = serde_json::to_vec(&value.value)?;
                    DB::pg_txn_append(
                        &txn,
                        r#"
    INSERT INTO user_attr_values (user_id, key, value)
    VALUES ($1, $2, $3)
    ON CONFLICT(user_id, key) DO UPDATE SET value = $3"#,
                        &[&user_id, &value.key, &v],
                    )
                    .await?;
                }
            }

            txn.commit().await?;

            DB::pg_query(
                "SELECT * FROM user_attr_values WHERE user_id = $1",
                &[&user_id],
                0,
            )
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
        format!("{IDX_USER_ATTR_CONFIG}{user_id}")
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
