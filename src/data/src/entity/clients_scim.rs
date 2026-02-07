use crate::database::DB;
use crate::entity::clients::Client;
use crate::entity::failed_scim_tasks::{FailedScimTask, ScimAction};
use crate::entity::groups::Group;
use crate::entity::scim_types::{ScimError, ScimGroup, ScimListResponse, ScimResource, ScimUser};
use crate::entity::scopes::Scope;
use crate::entity::users::User;
use crate::entity::users_values::UserValues;
use crate::rauthy_config::RauthyConfig;
use cryptr::EncValue;
use hiqlite_macros::params;
use rauthy_common::constants::APPLICATION_JSON_SCIM;
use rauthy_common::{http_client, is_hiqlite};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use std::collections::HashMap;
use std::default::Default;
use std::fmt::Debug;
use tokio::task;
use tracing::{debug, error, info};

#[derive(Clone, PartialEq)]
pub struct ClientScim {
    pub client_id: String,
    pub bearer_token: String,
    pub base_uri: String,
    pub sync_groups: bool,
    pub group_sync_prefix: Option<String>,
}

impl Debug for ClientScim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ClientScim {{ client_id: {}, bearer_token: <hidden>, base_uri: {}, \
            group_sync_prefix: {:?} }}",
            self.client_id, self.base_uri, self.group_sync_prefix
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
            base_uri: row.get("base_endpoint"),
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
            base_uri: row.get("base_endpoint"),
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
ON CONFLICT (client_id) DO UPDATE
SET bearer_token = $2, base_endpoint = $3, sync_groups = $4, group_sync_prefix = $5"#;

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

    pub async fn find_opt(client_id: String) -> Result<Option<Self>, ErrorResponse> {
        let sql = "SELECT * FROM clients_scim WHERE client_id = $1";
        let slf = if is_hiqlite() {
            DB::hql()
                .query_map_optional(sql, params!(client_id))
                .await?
        } else {
            DB::pg_query_opt(sql, &[&client_id]).await?
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

    pub async fn find_with_attr_mapping(attr_name: &str) -> Result<Vec<Self>, ErrorResponse> {
        let scopes = Scope::find_with_mapping(attr_name).await?;
        let clients = Client::find_all().await?;
        let mut res = Vec::with_capacity(clients.len());

        'outer: for client in clients {
            for scope in &scopes {
                if client.scopes.contains(&scope.name)
                    || client.default_scopes.contains(&scope.name)
                {
                    if let Some(scim) = Self::find_opt(client.id).await? {
                        res.push(scim);
                    }
                    continue 'outer;
                }
            }
        }
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
    pub fn decrypt_bearer_token(encrypted_token: Vec<u8>) -> Result<String, ErrorResponse> {
        let bytes = EncValue::try_from(encrypted_token)?.decrypt()?;
        let cleartext = String::from_utf8_lossy(bytes.as_ref()).to_string();
        Ok(cleartext)
    }

    fn auth_header(&self) -> String {
        format!("Bearer {}", self.bearer_token)
    }

    fn url_groups(&self, start_index: usize, count: usize) -> String {
        format!(
            "{}/Groups?startIndex={start_index}&count={count}",
            self.base_uri
        )
    }

    fn should_sync_groups(&self) -> bool {
        if !self.sync_groups {
            debug!("Group syncing disabled for SCIM client {}", self.client_id);
            return false;
        }

        true
    }

    fn is_group_sync_prefix_match(&self, group: &Group) -> bool {
        if let Some(prefix) = &self.group_sync_prefix
            && !group.name.starts_with(prefix)
        {
            debug!(
                "Group name does not start with configured prefix for SCIM client {}",
                self.client_id
            );
            return false;
        }
        true
    }

    /// Returns `true` if the `group_sync_prefix` matches any `user.group` or is `None`
    fn is_user_group_sync_prefix_match(&self, user: &User) -> bool {
        if let Some(prefix) = &self.group_sync_prefix {
            if user.groups.as_deref().unwrap_or_default().contains(prefix) {
                true
            } else {
                debug!(
                    user.email,
                    self.client_id,
                    "user has no matching group sync prefix '{prefix}' for SCIM client",
                );
                false
            }
        } else {
            true
        }
    }

    /// Triggers a complete SCIM sync for this client.
    /// Groups are synced in the foreground to catch possible config errors early. They should
    /// finish rather fast. User + group assignment syncs will be pushed into a background task,
    /// because this can take a very long time depending on the amount of existing users.
    pub async fn sync_full(self) -> Result<(), ErrorResponse> {
        // We want to sync the groups synchronous to catch possible config errors early.
        // The user sync however can take a very long time depending on the amount of users,
        // so it will be pushed into the background.
        let groups = if self.sync_groups {
            info!("Syncing SCIM groups for {}", self.client_id);
            self.sync_groups().await?;
            Group::find_all().await?
        } else {
            Vec::default()
        };

        task::spawn(async move {
            let mut groups_remote = HashMap::with_capacity(groups.len());
            info!("Syncing SCIM users for {}", self.client_id);
            if let Err(err) = self.sync_users(None, &groups, &mut groups_remote).await {
                error!(?err);
            }
        });

        Ok(())
    }

    /// Should be called if SCIM has been newly configured for a client or the config has been
    /// changed. This will fetch all existing groups from the client, compare them against local
    /// and update on remote if necessary.
    pub async fn sync_groups(&self) -> Result<(), ErrorResponse> {
        match self.sync_groups_exec().await {
            Ok(_) => Ok(()),
            Err(err) => {
                let err = format!("Error during sync groups: {err:?}");
                error!(err);

                FailedScimTask::upsert(&ScimAction::GroupsSync, &self.client_id).await?;

                Err(ErrorResponse::new(ErrorResponseType::Scim, err))
            }
        }
    }

    async fn sync_groups_exec(&self) -> Result<(), ErrorResponse> {
        info!("Syncing SCIM groups for {}", self.client_id);
        let mut groups = Group::find_all().await?;
        let mut start_index = 1;
        let count = 100;
        let mut url = self.url_groups(start_index, count);
        let mut group_pairs = Vec::with_capacity(groups.len());
        loop {
            let res = http_client()
                .get(url)
                .header(AUTHORIZATION, self.auth_header())
                .header(ACCEPT, APPLICATION_JSON_SCIM)
                .send()
                .await?;
            if !res.status().is_success() {
                let err = ScimError::extract_from_res(res).await;
                return Err(ErrorResponse::new(
                    ErrorResponseType::Scim,
                    format!("SCIM GET /Groups error: {err:?}"),
                ));
            }

            let lr = res.json::<ScimListResponse>().await?;
            let lr_len = lr.resources.len();
            if lr_len == 0 {
                debug!("SCIM ListResponse is empty - nothing left to do");
                break;
            }

            for res in lr.resources {
                match res {
                    ScimResource::User(_) => {
                        return Err(ErrorResponse::new(
                            ErrorResponseType::Scim,
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
                                group_pairs.push((local, *remote));
                            }
                        } else if let Some(pos) =
                            groups.iter().position(|g| g.name == remote.display_name)
                        {
                            let local = groups.swap_remove(pos);
                            group_pairs.push((local, *remote));
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

        // local groups that are expected on the server but do not exist yet
        if self.should_sync_groups() {
            for group in groups {
                if self.is_group_sync_prefix_match(&group) {
                    self.create_group(group).await?;
                }
            }

            // found group pairs - some may need to be deleted if we added a prefix for instance
            for (local, remote) in group_pairs {
                if self.is_group_sync_prefix_match(&local) {
                    self.update_group(local, remote, false).await?;
                } else if RauthyConfig::get().vars.scim.sync_delete_groups {
                    info!(
                        remote_group = ?remote,
                        scim_client = self.client_id,
                        "Deleting group on SCIM client",
                    );
                    self.delete_group(local, Some(remote)).await?;
                } else {
                    // In this case, we only want to un-set the `externalId` link to Rauthy and
                    // the deletion has to be done manually, which might be preferred in some
                    // situations.
                    self.update_group(local, remote, true).await?;
                }
            }
        } else if RauthyConfig::get().vars.scim.sync_delete_groups {
            // we need to clean up groups that might have been synced before,
            // in case sync was now turned off
            for (local, remote) in group_pairs {
                self.delete_group(local, Some(remote)).await?;
            }
        }

        Ok(())
    }

    /// Fetches a SCIM group from the Service Provider.
    /// Always tries via `externalId` first and will use a fallback to `displayName` if None
    /// has been returned for the id.
    async fn get_group(
        &self,
        group: &Group,
        ext_id_only: bool,
    ) -> Result<Option<ScimGroup>, ErrorResponse> {
        let url = format!(
            "{}/Groups?filter=externalId%20eq%20%22{}%22",
            self.base_uri, group.id
        );
        match self.get_group_with(group, url).await? {
            None => {
                if ext_id_only {
                    return Ok(None);
                }
                let url = format!(
                    "{}/Groups?filter=displayName%20eq%20%22{}%22",
                    self.base_uri, group.name
                );
                self.get_group_with(group, url).await
            }
            Some(g) => Ok(Some(g)),
        }
    }

    async fn get_group_with(
        &self,
        group: &Group,
        url: String,
    ) -> Result<Option<ScimGroup>, ErrorResponse> {
        let res = http_client()
            .get(url)
            .header(AUTHORIZATION, self.auth_header())
            .send()
            .await?;

        if res.status().is_success() {
            let mut lr = res.json::<ScimListResponse>().await?;
            if lr.resources.is_empty() {
                Ok(None)
            } else {
                let res = lr.resources.swap_remove(0);
                match res {
                    ScimResource::User(_) => Err(ErrorResponse::new(
                        ErrorResponseType::Scim,
                        format!(
                            "Received a User from SCIM client {} when expected a Group",
                            self.client_id
                        ),
                    )),
                    ScimResource::Group(remote) => {
                        if let Some(ext_id) = &remote.external_id
                            && ext_id != &group.id
                        {
                            return Err(ErrorResponse::new(
                                ErrorResponseType::Scim,
                                format!(
                                    "Error for SCIM client {} with group {}: Group has an external ID which does not match ours",
                                    self.client_id, group.name
                                ),
                            ));
                        }
                        Ok(Some(*remote))
                    }
                }
            }
        } else {
            let err = ScimError::extract_from_res(res).await;
            error!(?err);
            Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                format!(
                    "Error getting group from SCIM client {}: {err:?}",
                    self.client_id
                ),
            ))
        }
    }

    /// Either creates or updates the group, depending on if it exists on remote already, or not.
    /// Set `sync_maybe_delete` to `true` for syncs after config updates, if e.g. the sync prefix
    /// was changed. Never for create though!
    pub async fn create_update_group(&self, group: Group) -> Result<(), ErrorResponse> {
        if !self.should_sync_groups() || !self.is_group_sync_prefix_match(&group) {
            return Ok(());
        }

        let gid = group.id.clone();
        if let Some(remote) = self.get_group(&group, false).await? {
            let is_prefix_match = self.is_group_sync_prefix_match(&group);

            if is_prefix_match {
                if let Err(err) = self.update_group(group, remote, !is_prefix_match).await {
                    error!(
                        self.client_id,
                        ?err,
                        "Error during create group for SCIM client",
                    );
                    FailedScimTask::upsert(&ScimAction::GroupCreateUpdate(gid), &self.client_id)
                        .await?;
                }
            } else if RauthyConfig::get().vars.scim.sync_delete_groups {
                self.delete_group(group, Some(remote)).await?;
            }
        } else if let Err(err) = self.create_group(group).await {
            error!(
                self.client_id,
                ?err,
                "Error during update group for SCIM client",
            );
            FailedScimTask::upsert(&ScimAction::GroupCreateUpdate(gid), &self.client_id).await?;
        };

        Ok(())
    }

    /// Does no additional check if it should actually create the group - must be done beforehand.
    async fn create_group(&self, group: Group) -> Result<Option<ScimGroup>, ErrorResponse> {
        let local_id = group.id.clone();

        let payload = ScimGroup {
            // must be None on our side - managed by the client
            id: None,
            external_id: Some(group.id),
            display_name: group.name,
            ..Default::default()
        };
        debug!(?payload, "Creating SCIM group on remote");
        let json = serde_json::to_string(&payload)?;
        let url = format!("{}/Groups", self.base_uri);
        let res = http_client()
            .post(url)
            .header(AUTHORIZATION, self.auth_header())
            .header(ACCEPT, APPLICATION_JSON_SCIM)
            .header(CONTENT_TYPE, APPLICATION_JSON_SCIM)
            .body(json)
            .send()
            .await?;

        if res.status().is_success() {
            let remote = res.json::<ScimGroup>().await?;
            if remote.external_id == Some(local_id) {
                Ok(Some(remote))
            } else {
                Err(ErrorResponse::new(
                    ErrorResponseType::Scim,
                    format!(
                        "SCIM client {} has not respected our group id",
                        self.client_id
                    ),
                ))
            }
        } else {
            let err = ScimError::extract_from_res(res).await;
            Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                format!(
                    "SCIM client {} group creation failed: {err:?}",
                    self.client_id
                ),
            ))
        }
    }

    /// Does no additional check if it should actually update the group - must be done beforehand.
    ///
    /// Set `remove_ext_id_link` to update the group but remove the `externalId` to "unlink" the
    /// remote group with Rauthy.
    async fn update_group(
        &self,
        group_local: Group,
        group_remote: ScimGroup,
        remove_ext_id_link: bool,
    ) -> Result<(), ErrorResponse> {
        let remote_id = if let Some(id) = group_remote.id {
            id
        } else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                "Remote SCIM group without an ID",
            ));
        };
        let url = format!("{}/Groups/{remote_id}", self.base_uri);

        // let mut value_replace = HashMap::with_capacity(2);
        // if remove_ext_id_link {
        //     value_replace.insert("externalId".into(), serde_json::Value::Null);
        // } else {
        //     value_replace.insert(
        //         "externalId".into(),
        //         serde_json::Value::String(group_local.id),
        //     );
        // }
        // value_replace.insert(
        //     "displayName".into(),
        //     serde_json::Value::String(group_local.name),
        // );
        // let payload = ScimPatchOp {
        //     operations: vec![ScimPatchOperations {
        //         op: ScimOp::Replace,
        //         value: value_replace,
        //     }],
        //     ..Default::default()
        // };
        //
        // let json = serde_json::to_string(&payload)?;
        // debug!(
        //     "Serialized payload for ScimPatchOp with replace:\n{}\n",
        //     json
        // );

        // This manual json building is pretty ugly, but we can avoid quite a few unnecessary
        // memory allocation if we use the typed builder above.
        // We are always using the exact same json, so a static upfront string is much more efficient.
        let json_1 = "{\"schemas\":[\"urn:ietf:params:scim:api:messages:2.0:PatchOp\"],\"Operations\":[{\"op\":\"replace\",\"value\":{\"displayName\":\"";
        let json_2 = "\",\"externalId\":\"";
        let json_3 = "\"}}]}";
        let json_ext_id_null = "\",\"externalId\":null}}]}";

        let json = if remove_ext_id_link {
            format!("{json_1}{}{json_ext_id_null}", group_local.name)
        } else {
            format!(
                "{json_1}{}{json_2}{}{json_3}",
                group_local.name,
                group_local.id.as_str()
            )
        };

        let res = http_client()
            .patch(url)
            .header(AUTHORIZATION, self.auth_header())
            .header(ACCEPT, APPLICATION_JSON_SCIM)
            .header(CONTENT_TYPE, APPLICATION_JSON_SCIM)
            .body(json)
            .send()
            .await?;

        if res.status().is_success() {
            Ok(())
        } else {
            let err = ScimError::extract_from_res(res).await;
            Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                format!(
                    "SCIM client {} group update failed: {err:?}",
                    self.client_id
                ),
            ))
        }
    }

    pub async fn delete_group(
        &self,
        group: Group,
        remote_group: Option<ScimGroup>,
    ) -> Result<(), ErrorResponse> {
        let gid = group.id.clone();
        if let Err(err) = self.delete_group_exec(group, remote_group).await {
            error!(
                scim_client = self.client_id,
                ?err,
                "Error during delete group for SCIM client",
            );
            FailedScimTask::upsert(&ScimAction::GroupDelete(gid), &self.client_id).await?;
        }

        Ok(())
    }

    async fn delete_group_exec(
        &self,
        group: Group,
        remote_group: Option<ScimGroup>,
    ) -> Result<(), ErrorResponse> {
        if !self.should_sync_groups() && !RauthyConfig::get().vars.scim.sync_delete_groups {
            return Ok(());
        }

        let remote_group = match remote_group {
            None => self.get_group(&group, true).await?,
            Some(g) => Some(g),
        };
        if remote_group.is_none() {
            debug!("Should delete remote group but does not exist - nothing to do");
            return Ok(());
        }
        let remote_id = if let Some(id) = remote_group.unwrap().id {
            id
        } else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                "Remote SCIM group without an ID",
            ));
        };
        let url = format!("{}/Groups/{remote_id}", self.base_uri);

        let res = http_client()
            .delete(url)
            .header(AUTHORIZATION, self.auth_header())
            .send()
            .await?;
        if res.status().is_success() {
            Ok(())
        } else {
            let err = ScimError::extract_from_res(res).await;
            Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                format!(
                    "Error deleting group on SCIM client {}: {err:?}",
                    self.client_id
                ),
            ))
        }
    }

    /// Fetches a SCIM group from the Service Provider.
    /// Always tries via `externalId` first and will use a fallback to `userName` if None
    /// jas been returned for the id.
    async fn get_user(
        &self,
        user_id: &str,
        user_email: &str,
        old_email: Option<&str>,
    ) -> Result<Option<ScimUser>, ErrorResponse> {
        let url = format!(
            "{}/Users?filter=externalId%20eq%20%22{user_id}%22",
            self.base_uri
        );
        match self.get_user_with(user_id, user_email, url).await? {
            None => {
                let email = old_email.unwrap_or(user_email);
                let url = format!(
                    "{}/Users?filter=userName%20eq%20%22{email}%22",
                    self.base_uri
                );
                self.get_user_with(user_id, user_email, url).await
            }
            Some(user) => Ok(Some(user)),
        }
    }

    async fn get_user_with(
        &self,
        user_id: &str,
        user_email: &str,
        url: String,
    ) -> Result<Option<ScimUser>, ErrorResponse> {
        let res = http_client()
            .get(url)
            .header(AUTHORIZATION, self.auth_header())
            .send()
            .await?;

        if res.status().is_success() {
            let mut lr = res.json::<ScimListResponse>().await?;
            if lr.resources.is_empty() {
                Ok(None)
            } else {
                let res = lr.resources.swap_remove(0);
                match res {
                    ScimResource::User(remote) => {
                        if let Some(ext_id) = &remote.external_id
                            && ext_id != user_id
                        {
                            let err = format!(
                                "Error for SCIM client {} with user {user_email}: User has an \
                                    external ID which does not match ours",
                                self.client_id
                            );
                            error!("{}", err);
                            return Err(ErrorResponse::new(ErrorResponseType::Scim, err));
                        }
                        Ok(Some(*remote))
                    }
                    ScimResource::Group(_) => Err(ErrorResponse::new(
                        ErrorResponseType::Scim,
                        format!(
                            "Received a Group from SCIM client {} when expected a User",
                            self.client_id
                        ),
                    )),
                }
            }
        } else {
            let err = ScimError::extract_from_res(res).await;
            error!("{:?}", err);
            Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                format!(
                    "Error getting user from SCIM client {}: {err:?}",
                    self.client_id
                ),
            ))
        }
    }

    pub async fn sync_users(
        &self,
        // specify for retrying a `failed_scim_task` to resume work
        retry_failed_user_ts: Option<i64>,
        groups_local: &[Group],
        groups_remote: &mut HashMap<String, ScimGroup>,
    ) -> Result<(), ErrorResponse> {
        let mut last_created_ts = retry_failed_user_ts.unwrap_or(0);

        match self
            .sync_users_exec(&mut last_created_ts, groups_local, groups_remote)
            .await
        {
            Ok(_) => Ok(()),
            Err(err) => {
                let err = format!("Error during sync users: {err}");
                error!(err);

                FailedScimTask::upsert(&ScimAction::UsersSync(last_created_ts), &self.client_id)
                    .await?;

                Err(ErrorResponse::new(ErrorResponseType::Scim, err))
            }
        }
    }

    pub async fn sync_users_exec(
        &self,
        // specify for retrying a `failed_scim_task` to resume work
        last_created_ts: &mut i64,
        groups_local: &[Group],
        groups_remote: &mut HashMap<String, ScimGroup>,
    ) -> Result<(), ErrorResponse> {
        let client = Client::find(self.client_id.clone()).await?;

        let limit = 100;
        let mut users;
        loop {
            users = User::find_for_scim_sync(*last_created_ts, limit).await?;
            debug!(users_count = users.len(), "Users count for SCIM update");
            if users.is_empty() {
                break;
            }
            let len = users.len();

            for (user, values) in users {
                debug_assert_eq!(user.id, values.id);
                let created = user.created_at;

                let is_no_prefix_match = if let Some(prefix) = &self.group_sync_prefix {
                    !user.groups.as_deref().unwrap_or_default().contains(prefix)
                } else {
                    false
                };

                match if is_no_prefix_match && RauthyConfig::get().vars.scim.sync_delete_users {
                    self.delete_user(&user).await
                } else {
                    self.create_update_user_exec(
                        user,
                        &client,
                        values,
                        None,
                        groups_local,
                        groups_remote,
                        is_no_prefix_match,
                    )
                    .await
                } {
                    Ok(_) => {
                        *last_created_ts = created;
                    }
                    Err(err) => {
                        error!(
                            self.client_id,
                            ?err.message, "Error during sync users for SCIM client",
                        );
                        return Err(err);
                    }
                }
            }

            if len < limit as usize {
                break;
            }
        }

        Ok(())
    }

    /// Either creates or updates the user, depending on if it exists on remote already, or not.
    pub async fn create_update_user(user: User) -> Result<(), ErrorResponse> {
        let clients_scim = Self::find_all().await?;
        if clients_scim.is_empty() {
            return Ok(());
        }

        let groups_local = Group::find_all().await?;
        let mut groups_remote = HashMap::with_capacity(groups_local.len());

        let uv = UserValues {
            id: user.id.clone(),
            birthdate: None,
            phone: None,
            street: None,
            zip: None,
            city: None,
            country: None,
            preferred_username: None,
            tz: None,
        };

        for client_scim in clients_scim {
            let is_prefix_match = client_scim.is_user_group_sync_prefix_match(&user);

            if is_prefix_match {
                let client = Client::find(client_scim.client_id.clone()).await?;
                if let Err(err) = client_scim
                    .create_update_user_exec(
                        user.clone(),
                        &client,
                        uv.clone(),
                        None,
                        &groups_local,
                        &mut groups_remote,
                        !is_prefix_match,
                    )
                    .await
                {
                    error!(
                        user.id,
                        client_scim.client_id,
                        ?err,
                        "Error during update SCIM user",
                    );
                    FailedScimTask::upsert(
                        &ScimAction::UserCreateUpdate(user.id.clone()),
                        &client_scim.client_id,
                    )
                    .await?;
                }
            } else if RauthyConfig::get().vars.scim.sync_delete_users {
                client_scim.delete_user(&user).await?;
            }
        }

        Ok(())
    }

    /// Either creates or updates the user, depending on if it exists on remote already, or not.
    #[allow(clippy::too_many_arguments)]
    async fn create_update_user_exec(
        &self,
        user: User,
        client: &Client,
        user_values: UserValues,
        old_email: Option<&str>,
        groups_local: &[Group],
        groups_remote: &mut HashMap<String, ScimGroup>,
        unlink_ext_id: bool,
    ) -> Result<(), ErrorResponse> {
        let scopes = format!("{},{}", client.scopes, client.default_scopes);
        let expected_groups = user.get_groups();

        let payload = ScimUser::from_user_values(user, user_values, &scopes).await?;
        match self
            .get_user(
                payload.external_id.as_ref().unwrap(),
                &payload.user_name,
                old_email,
            )
            .await?
        {
            None => {
                if !unlink_ext_id {
                    self.create_user(expected_groups, payload, groups_local, groups_remote)
                    .await?;
                }
            }
            Some(user_remote) => {
                self.update_user(
                    expected_groups,
                    payload,
                    user_remote,
                    groups_local,
                    groups_remote,
                    unlink_ext_id,
                )
                .await?;
            }
        }

        Ok(())
    }

    /// Directly send update requests to all SCIM configured clients. Makes sense after a local update.
    async fn create_user(
        &self,
        groups_expected: Vec<String>,
        update_payload: ScimUser,
        groups_local: &[Group],
        groups_remote: &mut HashMap<String, ScimGroup>,
    ) -> Result<(), ErrorResponse> {
        let json = serde_json::to_string(&update_payload)?;
        let res = http_client()
            .post(format!("{}/Users", self.base_uri))
            .header(AUTHORIZATION, self.auth_header())
            .header(ACCEPT, APPLICATION_JSON_SCIM)
            .header(CONTENT_TYPE, APPLICATION_JSON_SCIM)
            .body(json)
            .send()
            .await?;

        if res.status().is_success() {
            let user_remote = res.json::<ScimUser>().await?;

            if self.should_sync_groups() {
                self.update_groups_assignment(
                    groups_expected,
                    update_payload.user_name,
                    user_remote,
                    groups_local,
                    groups_remote,
                    false,
                )
                .await?;
            }
            Ok(())
        } else {
            let err = ScimError::extract_from_res(res).await;
            error!("{:?}", err);
            Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                format!(
                    "Error creating user with SCIM client {}: {err:?}",
                    self.client_id
                ),
            ))
        }
    }

    /// Directly send update requests to all SCIM configured clients. Makes sense after a local update.
    async fn update_user(
        &self,
        groups_expected: Vec<String>,
        mut update_payload: ScimUser,
        user_remote: ScimUser,
        groups_local: &[Group],
        groups_remote: &mut HashMap<String, ScimGroup>,
        unlink_ext_id: bool,
    ) -> Result<(), ErrorResponse> {
        if unlink_ext_id {
            update_payload.external_id = None;
        }

        // add remote id, required i.e. by vCenter
        let remote_id = user_remote.id.clone().unwrap_or_default();
        update_payload.id = Some(remote_id.to_owned());

        let json = serde_json::to_string(&update_payload)?;
        let res = http_client()
            .put(format!(
                "{}/Users/{}",
                self.base_uri,
                user_remote.id.unwrap_or_default()
            ))
            .header(AUTHORIZATION, self.auth_header())
            .header(ACCEPT, APPLICATION_JSON_SCIM)
            .header(CONTENT_TYPE, APPLICATION_JSON_SCIM)
            .body(json)
            .send()
            .await?;

        if res.status().is_success() {
            let user_remote = res.json::<ScimUser>().await?;

            if self.should_sync_groups() {
                self.update_groups_assignment(
                    groups_expected,
                    update_payload.user_name,
                    user_remote,
                    groups_local,
                    groups_remote,
                    unlink_ext_id,
                )
                .await?;
            }

            Ok(())
        } else {
            let err = ScimError::extract_from_res(res).await;
            Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                format!(
                    "Error updating user with SCIM client {}: {err:?}",
                    self.client_id
                ),
            ))
        }
    }

    pub async fn delete_user(&self, user: &User) -> Result<(), ErrorResponse> {
        let user_id = user.id.clone();
        if let Err(err) = self.delete_user_exec(user).await {
            error!(
                user_id,
                scim_client = self.client_id,
                ?err,
                "Error during delete SCIM user",
            );
            FailedScimTask::upsert(&ScimAction::UserDelete(user_id), &self.client_id).await?;
        }

        Ok(())
    }

    pub async fn delete_user_exec(&self, user: &User) -> Result<(), ErrorResponse> {
        let remote_user = self.get_user(&user.id, &user.email, None).await?;
        if remote_user.is_none() {
            debug!("Should delete remote user but does not exist - nothing to do");
            return Ok(());
        }
        let remote_id = if let Some(id) = remote_user.unwrap().id {
            id
        } else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                "Remote SCIM user without an ID",
            ));
        };
        let url = format!("{}/Users/{remote_id}", self.base_uri);

        let res = http_client()
            .delete(url)
            .header(AUTHORIZATION, self.auth_header())
            .send()
            .await?;
        if res.status().is_success() {
            Ok(())
        } else {
            let err = ScimError::extract_from_res(res).await;
            Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                format!(
                    "Error deleting user on SCIM client {}: {err:?}",
                    self.client_id
                ),
            ))
        }
    }

    // The `groups_remote` functions as a cache here. The idea is to reduce the overall necessary
    // fetches if we are syncing all users for a client for instance.
    // If a group does not exist in the Vec, we need to look it up and / or create otherwise.
    async fn update_groups_assignment(
        &self,
        mut user_groups_local: Vec<String>,
        user_email: String,
        user_remote: ScimUser,
        groups_local: &[Group],
        groups_remote: &mut HashMap<String, ScimGroup>,
        unlink_all_groups: bool,
    ) -> Result<(), ErrorResponse> {
        if let Some(prefix) = &self.group_sync_prefix {
            user_groups_local.retain(|g| g.starts_with(prefix));
        }
        let user_id_remote = match user_remote.id.clone() {
            None => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Scim,
                    format!("empty remote user id for SCIM user {user_remote:?}"),
                ));
            }
            Some(id) => id,
        };
        let mut user_groups_remote = user_remote.groups.unwrap_or_default();
        debug!(?user_groups_local);
        debug!(?user_groups_remote);

        if !unlink_all_groups {
            while let Some(expected_name) = user_groups_local.pop() {
                debug!(
                    "Checking for local group '{expected_name}' in remote groups: {user_groups_remote:?}"
                );
                let pos = user_groups_remote
                    .iter()
                    .position(|g| g.display.as_deref() == Some(expected_name.as_str()));
                if let Some(pos) = pos {
                    debug!(
                        "Found matching local - remote group: {expected_name} - nothing left to do"
                    );
                    user_groups_remote.swap_remove(pos);
                    continue;
                }

                let group_remote = match groups_remote.get(&expected_name) {
                    None => {
                        debug!(
                            "Remote group {expected_name} does not exist yet or has no `externalId` \
                            mapping"
                        );
                        // Will happen, if the ScimGroup is needed for the first time, since the map
                        // will be empty at first. We need to fetch or create it and make sure it exists
                        // afterward.
                        let group_local = groups_local.iter().find(|g| g.name == expected_name);
                        if group_local.is_none() {
                            error!(
                                "Group {expected_name} does not exist in local database but is \
                                mapped to a user - this should never happen"
                            );
                            // do not return, try to execute as many mappings as possible
                            continue;
                        }
                        let g = group_local.unwrap();
                        let mut remote = self.get_group(g, false).await?;
                        if remote.is_none() {
                            remote = self.create_group(g.clone()).await?;
                        }
                        if remote.is_none() {
                            error!(
                                "SCIM group {expected_name} does not exist on Service Provider even \
                                 after creation - this should never happen"
                            );
                            // do not return, try to execute as many mappings as possible
                            continue;
                        }

                        let remote = remote.unwrap();
                        groups_remote.insert(expected_name.clone(), remote);
                        groups_remote.get(&expected_name).unwrap()
                    }
                    Some(g) => g,
                };

                let remote_group_id = group_remote.id.as_deref().unwrap_or_default();
                if remote_group_id.is_empty() {
                    error!(
                        "SCIM Client {} remote group id is empty - this should never happen",
                        self.client_id
                    );
                    continue;
                }
                let url = format!("{}/Groups/{remote_group_id}", self.base_uri);

                // Same situation as for group patching. This "ugly" way of json creation is a pretty
                // big efficiency gain, and we avoid many unnecessary allocations, since we have a
                // static json that is the same each time and we know in advance.
                let json_1 = "{\"schemas\":[\"urn:ietf:params:scim:api:messages:2.0:PatchOp\"],\"Operations\":[{\"op\":\"add\",\"path\":\"members\",\"value\":[{\"value\":\"";
                let json_2 = "\",\"display\":\"";
                let json_3 = "\"}]}]}";
                let json = format!("{json_1}{user_id_remote}{json_2}{user_email}{json_3}");
                debug!("Sending PatchOp for Group {url}:\n{json}");

                let res = http_client()
                    .patch(url)
                    .header(AUTHORIZATION, self.auth_header())
                    .header(ACCEPT, APPLICATION_JSON_SCIM)
                    .header(CONTENT_TYPE, APPLICATION_JSON_SCIM)
                    .body(json)
                    .send()
                    .await?;
                if !res.status().is_success() {
                    debug!(?res, "SCIM PatchOp Error");
                    let err = ScimError::extract_from_res(res).await;
                    error!(
                        self.client_id,
                        ?err,
                        "Error adding Group assignment for SCIM client",
                    );
                }
            }
        }

        // At this point, everything left in `user_groups_remote` is a mapping that must have been
        // done manually on the remote server, but does not match our records. We want to correct
        // it and remove the mappings.
        debug!("Left-Over user_groups_remote: {:?}", user_groups_remote);
        for group_remote in user_groups_remote {
            let json_1 = "{\"schemas\":[\"urn:ietf:params:scim:api:messages:2.0:PatchOp\"],\"Operations\":[{\"op\":\"remove\",\"path\":\"members\",\"value\":[{\"value\":\"";
            let json_2 = "\"}]}]}";
            let json = format!("{json_1}{user_id_remote}{json_2}");

            let url = format!("{}/Groups/{}", self.base_uri, group_remote.value);
            let res = http_client()
                .patch(url)
                .header(AUTHORIZATION, self.auth_header())
                .header(ACCEPT, APPLICATION_JSON_SCIM)
                .header(CONTENT_TYPE, APPLICATION_JSON_SCIM)
                .body(json)
                .send()
                .await?;
            if !res.status().is_success() {
                let err = ScimError::extract_from_res(res).await;
                error!(self.client_id, ?err, "Error removing SCIM Group assignment",);
            }
        }

        Ok(())
    }
}
