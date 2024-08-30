use crate::app_state::{AppState, DbTxn};
use crate::cache::{Cache, DB};
use crate::entity::clients::Client;
use crate::entity::user_attr::UserAttrConfigEntity;
use crate::entity::well_known::WellKnown;
use actix_web::web;
use rauthy_api_types::scopes::{ScopeRequest, ScopeResponse};
use rauthy_common::constants::{CACHE_TTL_APP, IDX_CLIENTS, IDX_SCOPES};
use rauthy_common::utils::new_store_id;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::collections::HashSet;
use tracing::debug;
use utoipa::ToSchema;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize, ToSchema)]
pub struct Scope {
    pub id: String,
    pub name: String,
    // Custom user attributes as CSV to include in the access token
    pub attr_include_access: Option<String>,
    // Custom user attributes as CSV to include in the id token
    pub attr_include_id: Option<String>,
}

// CRUD
impl Scope {
    // Inserts a new scope into the database
    pub async fn create(
        data: &web::Data<AppState>,
        scope_req: ScopeRequest,
    ) -> Result<Self, ErrorResponse> {
        // check for already existing scope
        let mut scopes = Scope::find_all(data).await?;
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
        let attrs = UserAttrConfigEntity::find_all_as_set(data).await?;
        let attr_include_access = Self::clean_up_attrs(scope_req.attr_include_access, &attrs);
        let attr_include_id = Self::clean_up_attrs(scope_req.attr_include_id, &attrs);

        let new_scope = Scope {
            id: new_store_id(),
            name: scope_req.scope,
            attr_include_access,
            attr_include_id,
        };
        sqlx::query!("insert into scopes (id, name, attr_include_access, attr_include_id) values ($1, $2, $3, $4)",
            new_scope.id,
            new_scope.name,
            new_scope.attr_include_access,
            new_scope.attr_include_id,
            ).execute(&data.db)
            .await?;

        scopes.push(new_scope.clone());
        DB::client()
            .put(Cache::App, IDX_SCOPES, &scopes, CACHE_TTL_APP)
            .await?;

        WellKnown::rebuild(data).await?;

        Ok(new_scope)
    }

    // Deletes a scope
    pub async fn delete(data: &web::Data<AppState>, id: &str) -> Result<(), ErrorResponse> {
        let scope = Scope::find(data, id).await?;
        if scope.name == "openid" {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "The 'openid' scope cannot be deleted",
            ));
        }

        // before deleting a scope, cleanup every client
        // get all clients with the to-be-deleted-scope assigned
        let mut clients = vec![];
        Client::find_all(data)
            .await?
            .into_iter()
            .filter(|c| c.scopes.contains(&scope.name) || c.default_scopes.contains(&scope.name))
            .for_each(|mut c| {
                c.delete_scope(&scope.name);
                clients.push(c);
            });

        // no need to evict the cache if no clients are updated
        let client = DB::client();
        if !clients.is_empty() {
            client.delete(Cache::App, IDX_CLIENTS).await?;
        }

        let mut txn = data.db.begin().await?;

        for client in clients {
            client.save(data, Some(&mut txn)).await?;
        }

        sqlx::query!("delete from scopes where id = $1", id)
            .execute(&mut *txn)
            .await?;

        txn.commit().await?;

        let scopes = Scope::find_all(data)
            .await?
            .into_iter()
            .filter(|s| s.id != scope.id)
            .collect::<Vec<Scope>>();
        client
            .put(Cache::App, IDX_SCOPES, &scopes, CACHE_TTL_APP)
            .await?;

        WellKnown::rebuild(data).await?;

        Ok(())
    }

    // Returns a single scope by id
    pub async fn find(data: &web::Data<AppState>, id: &str) -> Result<Self, ErrorResponse> {
        let res = sqlx::query_as!(Self, "SELECT * FROM scopes WHERE id = $1", id)
            .fetch_one(&data.db)
            .await?;
        Ok(res)
    }

    // Returns all existing scopes
    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let client = DB::client();
        if let Some(slf) = client.get(Cache::App, IDX_SCOPES).await? {
            return Ok(slf);
        }

        let res = sqlx::query_as!(Self, "SELECT * FROM scopes")
            .fetch_all(&data.db)
            .await?;

        client
            .put(Cache::App, IDX_SCOPES, &res, CACHE_TTL_APP)
            .await?;
        Ok(res)
    }

    // Updates a scope
    pub async fn update(
        data: &web::Data<AppState>,
        id: &str,
        scope_req: ScopeRequest,
    ) -> Result<Self, ErrorResponse> {
        let scope = Scope::find(data, id).await?;
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

        let mut txn = data.db.begin().await?;

        // if the name has changed, we need to update all connected clients
        let is_name_update = if scope.name != scope_req.scope {
            // find all clients with the old_name assigned
            let mut clients = vec![];
            Client::find_all(data)
                .await?
                .into_iter()
                .filter(|c| {
                    c.scopes.contains(&scope.name) || c.default_scopes.contains(&scope.name)
                })
                .for_each(|mut c| {
                    c.scopes = c.scopes.replace(&scope.name, &scope_req.scope);
                    c.default_scopes = c.default_scopes.replace(&scope.name, &scope_req.scope);
                    clients.push(c);
                });

            // no need to evict the cache if no clients are updated
            if !clients.is_empty() {
                DB::client().delete(Cache::App, IDX_CLIENTS).await?;
            }

            // Not awaiting all at once to prevent resource spikes
            for client in clients {
                client.save(data, Some(&mut txn)).await?;
            }

            true
        } else {
            false
        };

        debug!("scope_req: {:?}", scope_req);
        // check configured custom attributes and clean them up
        let attrs = UserAttrConfigEntity::find_all_as_set(data).await?;
        let attr_include_access = Self::clean_up_attrs(scope_req.attr_include_access, &attrs);
        let attr_include_id = Self::clean_up_attrs(scope_req.attr_include_id, &attrs);
        debug!("attr_include_access: {:?}", attr_include_access);
        debug!("attr_include_id: {:?}", attr_include_id);

        let new_scope = Scope {
            id: scope.id.clone(),
            name: scope_req.scope,
            attr_include_access,
            attr_include_id,
        };

        sqlx::query!(
            r#"UPDATE scopes
            SET name = $1, attr_include_access = $2, attr_include_id = $3
            WHERE id = $4"#,
            new_scope.name,
            new_scope.attr_include_access,
            new_scope.attr_include_id,
            new_scope.id,
        )
        .execute(&mut *txn)
        .await?;

        txn.commit().await?;

        let scopes = Scope::find_all(data)
            .await?
            .into_iter()
            .map(|mut s| {
                if s.id == new_scope.id {
                    s = new_scope.clone();
                }
                s
            })
            .collect::<Vec<Scope>>();

        DB::client()
            .put(Cache::App, IDX_SCOPES, &scopes, CACHE_TTL_APP)
            .await?;

        if is_name_update {
            WellKnown::rebuild(data).await?;
        }

        Ok(new_scope)
    }

    // Updates a scope
    pub async fn update_mapping_only(
        data: &web::Data<AppState>,
        id: &str,
        attr_include_access: Option<String>,
        attr_include_id: Option<String>,
        txn: &mut DbTxn<'_>,
    ) -> Result<(), ErrorResponse> {
        sqlx::query!(
            r#"UPDATE scopes
            SET attr_include_access = $1, attr_include_id = $2
            WHERE id = $3"#,
            attr_include_access,
            attr_include_id,
            id,
        )
        .execute(&mut **txn)
        .await?;

        let scopes = Scope::find_all(data)
            .await?
            .into_iter()
            .map(|mut s| {
                if s.id == id {
                    s.attr_include_access.clone_from(&attr_include_access);
                    s.attr_include_id.clone_from(&attr_include_id);
                }
                s
            })
            .collect::<Vec<Scope>>();

        DB::client()
            .put(Cache::App, IDX_SCOPES, &scopes, CACHE_TTL_APP)
            .await?;

        Ok(())
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
    pub fn is_custom(scope: &str) -> bool {
        scope != "openid" && scope != "profile" && scope != "email" && scope != "groups"
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
