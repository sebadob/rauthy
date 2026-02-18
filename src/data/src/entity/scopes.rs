use crate::database::{Cache, DB};
use crate::entity::clients::Client;
use crate::entity::user_attr::UserAttrConfigEntity;
use crate::entity::well_known::WellKnown;
use deadpool_postgres::GenericClient;
use hiqlite::Params;
use hiqlite_macros::params;
use rauthy_api_types::scopes::{ScopeRequest, ScopeResponse};
use rauthy_common::constants::{CACHE_TTL_APP, IDX_CLIENTS, IDX_SCOPES};
use rauthy_common::is_hiqlite;
use rauthy_common::utils::new_store_id;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use tracing::debug;
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Scope {
    pub id: String,
    pub name: String,
    // Custom user attributes as CSV to include in the access token
    pub attr_include_access: Option<String>,
    // Custom user attributes as CSV to include in the id token
    pub attr_include_id: Option<String>,
}

impl From<tokio_postgres::Row> for Scope {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            name: row.get("name"),
            attr_include_access: row.get("attr_include_access"),
            attr_include_id: row.get("attr_include_id"),
        }
    }
}

// CRUD
impl Scope {
    pub async fn clear_cache() -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::App, IDX_SCOPES).await?;
        Ok(())
    }

    pub async fn create(scope_req: ScopeRequest) -> Result<Self, ErrorResponse> {
        // check for already existing scope
        let mut scopes = Scope::find_all().await?;
        for s in &scopes {
            if s.name == scope_req.scope {
                return Err(ErrorResponse::new(
                    ErrorResponseType::BadRequest,
                    "Scope already exists",
                ));
            }
        }

        if !Scope::is_custom(&scope_req.scope)
            && (scope_req.attr_include_access.is_some() || scope_req.attr_include_id.is_some())
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "OpenID default scopes cannot have custom mappings",
            ));
        }

        // check configured custom attributes and clean them up
        let attrs = UserAttrConfigEntity::find_all_as_set().await?;
        let attr_include_access = Self::clean_up_attrs(scope_req.attr_include_access, &attrs);
        let attr_include_id = Self::clean_up_attrs(scope_req.attr_include_id, &attrs);

        let new_scope = Scope {
            id: new_store_id(),
            name: scope_req.scope,
            attr_include_access,
            attr_include_id,
        };

        if is_hiqlite() {
            DB::hql()
                .execute(
                    r#"
INSERT INTO scopes (id, name, attr_include_access, attr_include_id)
VALUES ($1, $2, $3, $4)"#,
                    params!(
                        &new_scope.id,
                        &new_scope.name,
                        &new_scope.attr_include_access,
                        &new_scope.attr_include_id
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                r#"
    INSERT INTO scopes (id, name, attr_include_access, attr_include_id)
    VALUES ($1, $2, $3, $4)"#,
                &[
                    &new_scope.id,
                    &new_scope.name,
                    &new_scope.attr_include_access,
                    &new_scope.attr_include_id,
                ],
            )
            .await?;
        }

        scopes.push(new_scope.clone());
        DB::hql()
            .put(Cache::App, IDX_SCOPES, &scopes, CACHE_TTL_APP)
            .await?;

        WellKnown::rebuild().await?;

        Ok(new_scope)
    }

    pub async fn delete(id: &str) -> Result<(), ErrorResponse> {
        let scope = Scope::find(id).await?;
        if scope.name == "openid" {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "The 'openid' scope cannot be deleted",
            ));
        }

        let mut clients = Client::find_with_scope(&scope.name).await?;

        if is_hiqlite() {
            let mut txn: Vec<(&str, Params)> = Vec::with_capacity(clients.len() + 1);

            for client in &mut clients {
                client.delete_scope(&scope.name);
                client.save_txn_append(&mut txn);
            }
            txn.push(("DELETE FROM scopes WHERE id = $1", params!(&scope.id)));

            for res in DB::hql().txn(txn).await? {
                let rows_affected = res?;
                debug_assert!(rows_affected == 1);
            }
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            for client in &mut clients {
                client.delete_scope(&scope.name);
                client.save_txn(&txn).await?;
            }
            DB::pg_txn_append(&txn, "DELETE FROM scopes WHERE id = $1", &[&id]).await?;

            txn.commit().await?;
        }

        let scopes = Scope::find_all()
            .await?
            .into_iter()
            .filter(|s| s.id != scope.id)
            .collect::<Vec<Scope>>();

        let client = DB::hql();
        // no need to evict the cache if no clients are updated
        if !clients.is_empty() {
            client.delete(Cache::App, IDX_CLIENTS).await?;
        }

        for client in clients {
            client.save_cache().await?;
        }

        client
            .put(Cache::App, IDX_SCOPES, &scopes, CACHE_TTL_APP)
            .await?;

        WellKnown::rebuild().await?;

        Ok(())
    }

    pub async fn find(id: &str) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM scopes WHERE id = $1";
        let res = if is_hiqlite() {
            DB::hql().query_as_one(sql, params!(id)).await?
        } else {
            DB::pg_query_one(sql, &[&id]).await?
        };

        Ok(res)
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let client = DB::hql();
        if let Some(slf) = client.get(Cache::App, IDX_SCOPES).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM scopes";
        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 6).await?
        };

        client
            .put(Cache::App, IDX_SCOPES, &res, CACHE_TTL_APP)
            .await?;

        Ok(res)
    }

    pub async fn find_with_mapping(attr_name: &str) -> Result<Vec<Self>, ErrorResponse> {
        Ok(Scope::find_all()
            .await?
            .into_iter()
            .filter(|s| {
                if !Scope::is_custom(&s.name) {
                    false
                } else {
                    let access = s.attr_include_access.as_deref().unwrap_or_default();
                    let id = s.attr_include_id.as_deref().unwrap_or_default();
                    access.contains(attr_name) || id.contains(attr_name)
                }
            })
            .collect::<Vec<_>>())
    }

    pub async fn update(id: &str, scope_req: ScopeRequest) -> Result<Self, ErrorResponse> {
        let scope = Scope::find(id).await?;
        if scope.name == "openid" {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "The 'openid' scope cannot be changed",
            ));
        }

        if !Scope::is_custom(&scope_req.scope)
            && (scope_req.attr_include_access.is_some() || scope_req.attr_include_id.is_some())
        {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "OpenID default scopes cannot have custom mappings",
            ));
        }

        // we only need to update clients with pre-computed values if the name
        // has been changed, but can skip them if it's about the attribute mapping
        let clients = if scope.name != scope_req.scope {
            let clients = Client::find_with_scope(&scope.name)
                .await?
                .into_iter()
                .map(|mut c| {
                    c.scopes = c.scopes.replace(&scope.name, &scope_req.scope);
                    c.default_scopes = c.default_scopes.replace(&scope.name, &scope_req.scope);
                    c
                })
                .collect::<Vec<_>>();
            Some(clients)
        } else {
            None
        };
        let is_name_update = clients.is_some();

        // check configured custom attributes and clean them up
        let attrs = UserAttrConfigEntity::find_all_as_set().await?;
        let attr_include_access = Self::clean_up_attrs(scope_req.attr_include_access, &attrs);
        let attr_include_id = Self::clean_up_attrs(scope_req.attr_include_id, &attrs);
        debug!(?attr_include_access);
        debug!(?attr_include_id);

        let new_scope = Scope {
            id: scope.id.clone(),
            name: scope_req.scope,
            attr_include_access,
            attr_include_id,
        };

        if is_hiqlite() {
            let len = clients.as_ref().map(|c| c.len()).unwrap_or_default() + 1;
            let mut txn: Vec<(&str, Params)> = Vec::with_capacity(len);

            if let Some(clients) = &clients {
                for client in clients {
                    client.save_txn_append(&mut txn);
                }
            }
            txn.push((
                r#"
UPDATE scopes
SET name = $1, attr_include_access = $2, attr_include_id = $3
WHERE id = $4"#,
                params!(
                    &new_scope.name,
                    &new_scope.attr_include_access,
                    &new_scope.attr_include_id,
                    &new_scope.id
                ),
            ));

            for res in DB::hql().txn(txn).await? {
                let rows_affected = res?;
                debug_assert!(rows_affected == 1);
            }
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            if let Some(clients) = &clients {
                for client in clients {
                    client.save_txn(&txn).await?;
                }
            }
            DB::pg_txn_append(
                &txn,
                r#"
UPDATE scopes
SET name = $1, attr_include_access = $2, attr_include_id = $3
WHERE id = $4"#,
                &[
                    &new_scope.name,
                    &new_scope.attr_include_access,
                    &new_scope.attr_include_id,
                    &new_scope.id,
                ],
            )
            .await?;

            txn.commit().await?;
        }

        if let Some(clients) = clients {
            for client in clients {
                client.save_cache().await?;
            }
        }

        let scopes = Scope::find_all()
            .await?
            .into_iter()
            .map(|mut s| {
                if s.id == new_scope.id {
                    s = new_scope.clone();
                }
                s
            })
            .collect::<Vec<Scope>>();

        let client = DB::hql();
        DB::hql()
            .put(Cache::App, IDX_SCOPES, &scopes, CACHE_TTL_APP)
            .await?;

        if is_name_update {
            client.delete(Cache::App, IDX_CLIENTS).await?;
            WellKnown::rebuild().await?;
        }

        Ok(new_scope)
    }

    /// If you use this in a transactions, you MUST `Scope::clear_cache()` after successful commit!
    pub async fn update_mapping_only(
        id: &str,
        attr_include_access: Option<String>,
        attr_include_id: Option<String>,
        txn: &deadpool_postgres::Transaction<'_>,
    ) -> Result<(), ErrorResponse> {
        DB::pg_txn_append(
            txn,
            r#"
    UPDATE scopes
    SET attr_include_access = $1, attr_include_id = $2
    WHERE id = $3"#,
            &[&attr_include_access, &attr_include_id, &id],
        )
        .await?;
        Ok(())
    }

    /// If you use this in a transactions, you MUST `Scope::clear_cache()` after successful commit!
    pub fn update_mapping_only_append(
        id: &str,
        attr_include_access: Option<String>,
        attr_include_id: Option<String>,
        txn: &mut Vec<(&str, Params)>,
    ) {
        txn.push((
            r#"
    UPDATE scopes
    SET attr_include_access = $1, attr_include_id = $2
    WHERE id = $3"#,
            params!(attr_include_access, attr_include_id, id),
        ));
    }
}

impl Scope {
    pub fn clean_up_attrs(
        req_attrs: Option<Vec<String>>,
        existing_attrs: &HashSet<String>,
    ) -> Option<String> {
        req_attrs.as_ref()?;
        let res = req_attrs?
            .drain(..)
            .filter(|a| !a.is_empty() && existing_attrs.contains(a))
            .collect::<Vec<String>>()
            .join(",");

        Some(res)
    }

    // Accepts a string of scopes seperated by \s and returns a `Vec<&str>` containing all
    // non-custom scopes.
    /// Note: `groups` is not a default scope, but it will be handled like one for performance
    /// and efficiency reasons.
    #[inline]
    pub fn extract_custom(scopes: &str) -> HashSet<&str> {
        let mut res = HashSet::new();
        for s in scopes.split(' ') {
            if Self::is_custom(s) {
                res.insert(s);
            }
        }
        res
    }

    /// Returns `true` if the given scope is not a default OIDC scope.
    /// Note: `groups` is not a default scope, but it will be handled like one for performance
    /// and efficiency reasons.
    #[inline]
    pub fn is_custom(scope: &str) -> bool {
        scope != "address"
            && scope != "email"
            && scope != "groups"
            && scope != "openid"
            && scope != "phone"
            && scope != "profile"
    }

    /// Returns `true` if the `requested` scope matches the `allowed` scope.
    /// Supports hierarchical matching for URNs if Matrix support is enabled
    /// (e.g. `...:device` matches `...:device:ID`), as required by Matrix (MSC3861).
    /// Reference: https://github.com/matrix-org/matrix-spec-proposals/pull/3861
    #[inline]
    pub fn matches(allowed: &str, requested: &str, matrix_enabled: bool) -> bool {
        if allowed == requested {
            return true;
        }

        if matrix_enabled && let Some(suffix) = requested.strip_prefix(allowed) {
            return suffix.starts_with(':');
        }

        false
    }
}

impl From<Scope> for ScopeResponse {
    fn from(value: Scope) -> Self {
        let attr_include_access = value
            .attr_include_access
            .map(|attr| attr.split(',').map(String::from).collect());
        let attr_include_id = value
            .attr_include_id
            .map(|attr| attr.split(',').map(String::from).collect());

        Self {
            id: value.id,
            name: value.name,
            attr_include_access,
            attr_include_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use pretty_assertions::assert_eq;
    use serde_json::Value;

    #[tokio::test]
    async fn test_generic_json() {
        let obj =
            "{\"name\":\"Max\",\"lastName\":\"Power\",\"age\":1337,\"data\":[1,13,\"44String\"]}";

        let json = serde_json::from_str::<HashMap<String, Value>>(obj).unwrap();

        let name = json.get("name").unwrap();
        assert_eq!(name.as_str().unwrap(), "Max");

        let last_name = json.get("lastName").unwrap();
        assert_eq!(last_name.as_str().unwrap(), "Power");

        let age = json.get("age").unwrap();
        assert_eq!(age.as_u64().unwrap(), 1337);

        let data = json.get("data").unwrap();
        let arr = data.as_array().unwrap();
        assert_eq!(arr.len(), 3);

        // try to wrap the "any data" in the bigger json
        let wrapper = format!("{{\"custScope\":\"{}\"}}", "CustomValue1337");
        let json = serde_json::from_str::<HashMap<String, Value>>(&wrapper).unwrap();
        let data = json.get("custScope").unwrap().as_str().unwrap();
        assert_eq!(data, "CustomValue1337");

        let wrapper = format!("{{\"custScope\":{}}}", obj);
        let json = serde_json::from_str::<HashMap<String, Value>>(&wrapper).unwrap();
        let data = json.get("custScope").unwrap().as_object().unwrap();
        let name = data.get("name").unwrap().as_str().unwrap();
        assert_eq!(name, "Max");
    }
}
