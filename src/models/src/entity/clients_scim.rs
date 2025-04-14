use crate::database::DB;
use crate::entity::groups::Group;
use crate::entity::scim_types::{
    ScimError, ScimGroup, ScimListResponse, ScimOp, ScimPatchOp, ScimPatchOperations, ScimResource,
};
use crate::entity::users::User;
use cryptr::EncValue;
use hiqlite::{Param, params};
use rauthy_common::constants::APPLICATION_JSON_SCIM;
use rauthy_common::{HTTP_CLIENT, is_hiqlite};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::collections::HashMap;
use std::fmt::Debug;
use tracing::{debug, error};

pub struct ClientScim {
    pub client_id: String,
    pub bearer_token: String,
    pub base_endpoint: String,
    pub sync_groups: bool,
    pub group_sync_prefix: Option<String>,
}

impl Debug for ClientScim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClientScim {{ client_id: {}, bearer_token: <hidden>, base_endpoint: {}, group_sync_prefix: {:?} }}",
            self.client_id, self.base_endpoint, self.group_sync_prefix
        )
    }
}

impl From<hiqlite::Row<'_>> for ClientScim {
    fn from(mut row: hiqlite::Row) -> ClientScim {
        let bearer_token = Self::decrypt_bearer_token(row.get("bearer_token"))
            .expect("Column clients_scim.bearer_token corrupted");
        Self {
            client_id: row.get("client_id"),
            bearer_token,
            base_endpoint: row.get("base_endpoint"),
            sync_groups: row.get("sync_groups"),
            group_sync_prefix: row.get("group_sync_prefix"),
        }
    }
}

impl From<tokio_postgres::Row> for ClientScim {
    fn from(row: tokio_postgres::Row) -> ClientScim {
        let bearer_token = Self::decrypt_bearer_token(row.get("bearer_token"))
            .expect("Column clients_scim.bearer_token corrupted");
        Self {
            client_id: row.get("client_id"),
            bearer_token,
            base_endpoint: row.get("base_endpoint"),
            sync_groups: row.get("sync_groups"),
            group_sync_prefix: row.get("group_sync_prefix"),
        }
    }
}

impl ClientScim {
    pub async fn upsert(
        client_id: String,
        bearer_token: &[u8],
        base_endpoint: String,
        sync_groups: bool,
        group_sync_prefix: Option<String>,
    ) -> Result<(), ErrorResponse> {
        let bearer_encrypted = EncValue::encrypt(bearer_token)?.into_bytes();
        let sql = r#"
INSERT INTO clients_scim (client_id, bearer_token, base_endpoint, sync_groups, group_sync_prefix)
VALUES ($1, $2, $3, $4, $5)
ON CONFLICT (client_id) DO UPDATE SET
    bearer_token = $2, base_endpoint = $3, sync_groups = $4, group_sync_prefix = $5"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        client_id,
                        bearer_encrypted.to_vec(),
                        base_endpoint,
                        sync_groups,
                        group_sync_prefix
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &client_id,
                    &bearer_encrypted.as_ref(),
                    &base_endpoint,
                    &sync_groups,
                    &group_sync_prefix,
                ],
            )
            .await?;
        }

        Ok(())
    }

    pub async fn find(client_id: String) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM clients_scim WHERE client_id = $1";
        let slf = if is_hiqlite() {
            DB::hql().query_map_one(sql, params!(client_id)).await?
        } else {
            DB::pg_query_one(sql, &[&client_id]).await?
        };
        Ok(slf)
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM clients_scim";
        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };
        Ok(res)
    }

    pub async fn delete(client_id: String) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM clients_scim WHERE client_id = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(client_id)).await?;
        } else {
            DB::pg_execute(sql, &[&client_id]).await?;
        }
        Ok(())
    }
}

impl ClientScim {
    fn decrypt_bearer_token(encrypted_token: Vec<u8>) -> Result<String, ErrorResponse> {
        let bytes = EncValue::try_from(encrypted_token)?.decrypt()?;
        let cleartext = String::from_utf8_lossy(bytes.as_ref()).to_string();
        Ok(cleartext)
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.bearer_token)
    }

    fn url_groups(&self, start_index: usize, count: usize) -> String {
        if let Some(prefix) = &self.group_sync_prefix {
            format!(
                "{}/Groups?filter=displayName%20sw%20%22{}%22&startIndex={}&count={}",
                self.base_endpoint, prefix, start_index, count
            )
        } else {
            format!(
                "{}/Groups?startIndex={}&count={}",
                self.base_endpoint, start_index, count
            )
        }
    }

    fn should_sync_group(&self, group: Option<&Group>) -> bool {
        if !self.sync_groups {
            debug!("Group syncing disabled for SCIM client {}", self.client_id);
            return false;
        }

        if let Some(g) = group {
            if let Some(prefix) = &self.group_sync_prefix {
                if !g.name.starts_with(prefix) {
                    debug!(
                        "Group name does not start with configured prefix for SCIM client {}",
                        self.client_id
                    );
                    return false;
                }
            }
        }

        true
    }

    /// Should be called if SCIM has been newly configured for a client or the config has been
    /// changed. This will fetch all existing groups from the client, compare them against local
    /// and update on remote if necessary.
    pub async fn sync_groups(&self) -> Result<(), ErrorResponse> {
        if !self.should_sync_group(None) {
            return Ok(());
        }

        let mut groups = Group::find_all().await?;
        if let Some(prefix) = &self.group_sync_prefix {
            groups.retain(|g| g.name.starts_with(prefix.as_str()));
        }

        // Querying each group on our side with a single `eq` filter could lead to huge amounts
        // of single queries. Instead, we want to potentially "waste" some resources and collect
        // all remote groups in memory before we start the comparison. This reduces the overall
        // number of HTTP requests, which will be the bottleneck for this operation.

        let mut start_index = 1;
        let count = 100;
        let mut url = self.url_groups(start_index, count);
        let mut group_pairs = Vec::with_capacity(groups.len());
        loop {
            let res = HTTP_CLIENT
                .get(url)
                .header(AUTHORIZATION, self.auth_header())
                .send()
                .await?;
            if !res.status().is_success() {
                let err = res.json::<ScimError>().await?;
                error!("Error retrieving SCIM groups: {:?}", err);
                return Err(ErrorResponse::new(
                    ErrorResponseType::Connection,
                    format!("SCIM /Groups errior: {:?}", err),
                ));
            }

            let lr = res.json::<ScimListResponse>().await?;
            let lr_len = lr.resources.len();
            if lr_len == 0 {
                debug!("SCIM ListResponse is empty - nothing to do");
                break;
            }

            for res in lr.resources {
                match res {
                    ScimResource::User(_) => {
                        return Err(ErrorResponse::new(
                            ErrorResponseType::Connection,
                            format!(
                                "SCIM Client {} returned a User when Group was requested",
                                self.client_id
                            ),
                        ));
                    }
                    ScimResource::Group(remote) => {
                        // we can drop the following groups here
                        // - has an external id that does not match any of ours
                        // - ha no external id and has a displayName that does not match any of ours
                        if let Some(ext_id) = &remote.external_id {
                            if let Some(pos) = groups.iter().position(|g| &g.id == ext_id) {
                                let local = groups.swap_remove(pos);
                                group_pairs.push((local, remote));
                            }
                        } else if let Some(pos) =
                            groups.iter().position(|g| g.name == remote.display_name)
                        {
                            let local = groups.swap_remove(pos);
                            group_pairs.push((local, remote));
                        }
                    }
                }
            }

            if lr_len < count {
                break;
            }
            start_index += count;
            url = self.url_groups(start_index, count);
        }

        // If we get here, we have 2 sets of data:
        // - all left-over local groups in `groups` do not exist on remote -> POST
        // - all group pairs in `group_pairs` do exist and need an update -> PUT
        debug!(
            "Groups to CREATE on SCIM client {}: {:?}",
            self.client_id, groups
        );
        debug!(
            "Groups to UPDATE on SCIM client {}: {:?}",
            self.client_id, group_pairs
        );

        for group in groups {
            self.create_group(group).await?;
        }

        for (local, remote) in group_pairs {
            self.update_group(local, remote).await?;
        }

        Ok(())
    }

    pub async fn create_group(&self, group: Group) -> Result<(), ErrorResponse> {
        if !self.should_sync_group(Some(&group)) {
            return Ok(());
        }

        let local_id = group.id.clone();

        let payload = ScimGroup {
            id: None,
            external_id: Some(group.id),
            display_name: group.name,
            members: None,
            ..Default::default()
        };
        let json = serde_json::to_string(&payload)?;
        let url = format!("{}/Groups", self.base_endpoint);
        let res = HTTP_CLIENT
            .post(url)
            .header(AUTHORIZATION, self.auth_header())
            .header(CONTENT_TYPE, APPLICATION_JSON_SCIM)
            .body(json)
            .send()
            .await?;

        if res.status().is_success() {
            let remote = res.json::<ScimGroup>().await?;
            if remote.external_id == Some(local_id) {
                Ok(())
            } else {
                let err = format!(
                    "SCIM client {} has not respected our group id",
                    self.client_id
                );
                error!("{}", err);
                Err(ErrorResponse::new(ErrorResponseType::Connection, err))
            }
        } else {
            let err = res.json::<ScimError>().await?;
            error!("{:?}", err);
            Err(ErrorResponse::new(
                ErrorResponseType::Connection,
                err.detail.unwrap_or_else(|| {
                    format!("SCIM client {} group creation failed", self.client_id)
                }),
            ))
        }
    }

    pub async fn update_group(
        &self,
        group_local: Group,
        group_remote: Box<ScimGroup>,
    ) -> Result<(), ErrorResponse> {
        if !self.should_sync_group(Some(&group_local)) {
            return Ok(());
        }
        let remote_id = if let Some(id) = group_remote.id {
            id
        } else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "Remote SCIM group without an ID",
            ));
        };
        let url = format!("{}/Groups/{}", self.base_endpoint, remote_id);

        let mut value = HashMap::with_capacity(2);
        value.insert("id".into(), serde_json::Value::String(remote_id));
        value.insert(
            "displayName".into(),
            serde_json::Value::String(group_local.name),
        );
        let payload = ScimPatchOp {
            operations: vec![ScimPatchOperations {
                op: ScimOp::Replace,
                value,
            }],
            ..Default::default()
        };
        let json = serde_json::to_string(&payload)?;
        let res = HTTP_CLIENT
            .patch(url)
            .header(AUTHORIZATION, self.auth_header())
            .header(CONTENT_TYPE, APPLICATION_JSON_SCIM)
            .body(json)
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            let err = res.json::<ScimError>().await?;
            error!("{:?}", err);
            Err(ErrorResponse::new(
                ErrorResponseType::Connection,
                err.detail.unwrap_or_else(|| {
                    format!("SCIM client {} group update failed", self.client_id)
                }),
            ))
        }
    }

    pub async fn delete_group(group: Group) -> Result<(), ErrorResponse> {
        // if !self.sync_groups {
        //     debug!("Group syncing disabled for SCIM client {}", self.client_id);
        //     return Ok(());
        // }

        todo!()
    }

    /// Fetches all users from all SCIM configured clients, compares the data on them vs local
    /// and sends update requests if necessary.
    pub async fn sync_users(&self, users: &[User]) -> Result<(), ErrorResponse> {
        todo!()
    }

    /// Directly send update requests to all SCIM configured clients. Makes sense after a local update.
    pub async fn update_user(groups: User) -> Result<(), ErrorResponse> {
        todo!()
    }

    pub async fn delete_user(user: User) -> Result<(), ErrorResponse> {
        todo!()
    }

    pub async fn update_groups_assignment(user: User) -> Result<(), ErrorResponse> {
        todo!()
    }
}
