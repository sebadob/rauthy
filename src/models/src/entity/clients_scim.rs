use crate::database::DB;
use crate::entity::clients::Client;
use crate::entity::failed_scim_tasks::{FailedScimTask, ScimAction};
use crate::entity::groups::Group;
use crate::entity::scim_types::{ScimError, ScimGroup, ScimListResponse, ScimResource, ScimUser};
use crate::entity::users::User;
use crate::entity::users_values::UserValues;
use cryptr::EncValue;
use hiqlite::{Param, params};
use rauthy_common::constants::APPLICATION_JSON_SCIM;
use rauthy_common::{HTTP_CLIENT, is_hiqlite};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use std::collections::HashMap;
use std::default::Default;
use std::env;
use std::fmt::Debug;
use std::sync::LazyLock;
use tracing::{debug, error, info};

static SCIM_SYNC_DELETE_GROUPS: LazyLock<bool> = LazyLock::new(|| {
    env::var("SCIM_SYNC_DELETE_GROUPS")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("Cannot parse SCIM_SYNC_DELETE_GROUPS as bool")
});
static SCIM_SYNC_DELETE_USERS: LazyLock<bool> = LazyLock::new(|| {
    env::var("SCIM_SYNC_DELETE_USERS")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("Cannot parse SCIM_SYNC_DELETE_USERS as bool")
});

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
        format!(
            "{}/Groups?startIndex={}&count={}",
            self.base_uri, start_index, count
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
        if let Some(prefix) = &self.group_sync_prefix {
            if !group.name.starts_with(prefix) {
                debug!(
                    "Group name does not start with configured prefix for SCIM client {}",
                    self.client_id
                );
                return false;
            }
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
                    "user '{}' has no matching group sync prefix '{}' for SCIM client '{}'",
                    user.email, prefix, self.client_id
                );
                false
            }
        } else {
            true
        }
    }

    /// Should be called if SCIM has been newly configured for a client or the config has been
    /// changed. This will fetch all existing groups from the client, compare them against local
    /// and update on remote if necessary.
    pub async fn sync_groups(&self) -> Result<(), ErrorResponse> {
        match self.sync_groups_exec().await {
            Ok(_) => Ok(()),
            Err(err) => {
                let err = format!("Error during sync groups: {}", err);
                error!("{}", err);

                FailedScimTask::upsert(&ScimAction::GroupsSync, &self.client_id).await?;

                Err(ErrorResponse::new(ErrorResponseType::Scim, err))
            }
        }
    }

    async fn sync_groups_exec(&self) -> Result<(), ErrorResponse> {
        let mut groups = Group::find_all().await?;
        let mut start_index = 1;
        let count = 100;
        let mut url = self.url_groups(start_index, count);
        let mut group_pairs = Vec::with_capacity(groups.len());
        loop {
            let res = HTTP_CLIENT
                .get(url)
                .header(AUTHORIZATION, self.auth_header())
                .header(ACCEPT, APPLICATION_JSON_SCIM)
                .send()
                .await?;
            if !res.status().is_success() {
                let err = res.json::<ScimError>().await?;
                return Err(ErrorResponse::new(
                    ErrorResponseType::Scim,
                    format!("SCIM GET /Groups errior: {:?}", err),
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
                } else if *SCIM_SYNC_DELETE_GROUPS {
                    info!(
                        "Deleting group {:?} on SCIM client {}",
                        remote, self.client_id
                    );
                    self.delete_group(local, Some(remote)).await?;
                } else {
                    // In this case, we only want to un-set the `externalId` link to Rauthy and
                    // the deletion has to be done manually, which might be preferred in some
                    // situations.
                    self.update_group(local, remote, true).await?;
                }
            }
        } else if *SCIM_SYNC_DELETE_GROUPS {
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
        let res = HTTP_CLIENT
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
                        if let Some(ext_id) = &remote.external_id {
                            if ext_id != &group.id {
                                return Err(ErrorResponse::new(
                                    ErrorResponseType::Scim,
                                    format!(
                                        "Error for SCIM client {} with group {}: Group has an external ID which does not match ours",
                                        self.client_id, group.name
                                    ),
                                ));
                            }
                        }
                        Ok(Some(*remote))
                    }
                }
            }
        } else {
            let err = res.json::<ScimError>().await?;
            error!("{:?}", err);
            Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                format!(
                    "Error getting group from SCIM client {}: {:?}",
                    self.client_id, err
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
                        "Error during create group for SCIM client {}: {:?}",
                        self.client_id, err
                    );
                    FailedScimTask::upsert(&ScimAction::GroupCreateUpdate(gid), &self.client_id)
                        .await?;
                }
            } else if *SCIM_SYNC_DELETE_GROUPS {
                self.delete_group(group, Some(remote)).await?;
            }
        } else if let Err(err) = self.create_group(group).await {
            error!(
                "Error during update group for SCIM client {}: {:?}",
                self.client_id, err
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
        let json = serde_json::to_string(&payload)?;
        let url = format!("{}/Groups", self.base_uri);
        let res = HTTP_CLIENT
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
            let err = res.json::<ScimError>().await?;
            Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                format!(
                    "SCIM client {} group creation failed: {:?}",
                    self.client_id, err
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
        let url = format!("{}/Groups/{}", self.base_uri, remote_id);

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

        let res = HTTP_CLIENT
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
            let err = res.json::<ScimError>().await?;
            Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                format!(
                    "SCIM client {} group update failed: {:?}",
                    self.client_id, err
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
                "Error during delete group for SCIM client {}: {:?}",
                self.client_id, err
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
        if !self.should_sync_groups() && !*SCIM_SYNC_DELETE_GROUPS {
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
        let url = format!("{}/Groups/{}", self.base_uri, remote_id);

        let res = HTTP_CLIENT
            .delete(url)
            .header(AUTHORIZATION, self.auth_header())
            .send()
            .await?;
        if res.status().is_success() {
            Ok(())
        } else {
            let err = res.json::<ScimError>().await?;
            Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                format!(
                    "Error deleting group on SCIM client {}: {:?}",
                    self.client_id, err
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
            "{}/Users?filter=externalId%20eq%20%22{}%22",
            self.base_uri, user_id
        );
        match self.get_user_with(user_id, user_email, url).await? {
            None => {
                let email = old_email.unwrap_or(user_email);
                let url = format!(
                    "{}/Users?filter=userName%20eq%20%22{}%22",
                    self.base_uri, email
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
        let res = HTTP_CLIENT
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
                        if let Some(ext_id) = &remote.external_id {
                            if ext_id != user_id {
                                let err = format!(
                                    "Error for SCIM client {} with user {}: User has an external ID which does not match ours",
                                    self.client_id, user_email
                                );
                                error!("{}", err);
                                return Err(ErrorResponse::new(ErrorResponseType::Scim, err));
                            }
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
            let err = res.json::<ScimError>().await?;
            error!("{:?}", err);
            Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                format!(
                    "Error getting user from SCIM client {}: {:?}",
                    self.client_id, err
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
                let err = format!("Error during sync users: {}", err);
                error!("{}", err);

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
            debug!("Users count for SCIM update: {:?}", users.len());
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

                match if is_no_prefix_match && *SCIM_SYNC_DELETE_USERS {
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
                            "Error during sync users for SCIM client {}: {:?}",
                            self.client_id, err.message
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
                        "Error during update user {} for SCIM client {}: {:?}",
                        user.id, client_scim.client_id, err
                    );
                    FailedScimTask::upsert(
                        &ScimAction::UserCreateUpdate(user.id.clone()),
                        &client_scim.client_id,
                    )
                    .await?;
                }
            } else if *SCIM_SYNC_DELETE_USERS {
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

        let payload = ScimUser::from_user_values(user, user_values, &scopes);
        match self
            .get_user(
                payload.external_id.as_ref().unwrap(),
                &payload.user_name,
                old_email,
            )
            .await?
        {
            None => {
                self.create_user(expected_groups, payload, groups_local, groups_remote)
                    .await?;
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
        let res = HTTP_CLIENT
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
            let err = res.json::<ScimError>().await?;
            error!("{:?}", err);
            Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                format!(
                    "Error creating user with SCIM client {}: {:?}",
                    self.client_id, err
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

        let json = serde_json::to_string(&update_payload)?;
        let res = HTTP_CLIENT
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
            let err = res.json::<ScimError>().await?;
            Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                format!(
                    "Error updating user with SCIM client {}: {:?}",
                    self.client_id, err
                ),
            ))
        }
    }

    pub async fn delete_user(&self, user: &User) -> Result<(), ErrorResponse> {
        let user_id = user.id.clone();
        if let Err(err) = self.delete_user_exec(user).await {
            error!(
                "Error during delete user {} for SCIM client {}: {:?}",
                user_id, self.client_id, err
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
        let url = format!("{}/Users/{}", self.base_uri, remote_id);

        let res = HTTP_CLIENT
            .delete(url)
            .header(AUTHORIZATION, self.auth_header())
            .send()
            .await?;
        if res.status().is_success() {
            Ok(())
        } else {
            let err = res.json::<ScimError>().await?;
            Err(ErrorResponse::new(
                ErrorResponseType::Scim,
                format!(
                    "Error deleting user on SCIM client {}: {:?}",
                    self.client_id, err
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
                    format!("empty remote user id for SCIM user {:?}", user_remote),
                ));
            }
            Some(id) => id,
        };
        let mut user_groups_remote = user_remote.groups.unwrap_or_default();
        debug!("user_groups_local: {:?}", user_groups_local);
        debug!("user_groups_remote: {:?}", user_groups_remote);

        if !unlink_all_groups {
            while let Some(expected_name) = user_groups_local.pop() {
                debug!(
                    "Checking for local group '{}' in remote groups: {:?}",
                    expected_name, user_groups_local
                );
                let pos = user_groups_remote
                    .iter()
                    .position(|g| g.display.as_deref() == Some(expected_name.as_str()));
                if let Some(pos) = pos {
                    debug!("Found matching local - remote group: {}", expected_name);
                    user_groups_remote.swap_remove(pos);
                    continue;
                }

                let group_remote = match groups_remote.get(&expected_name) {
                    None => {
                        // Will happen, if the ScimGroup is needed for the first time, since the map
                        // will be empty at first. We need to fetch or create it and make sure it exists
                        // afterward.
                        let group_local = groups_local.iter().find(|g| g.name == expected_name);
                        if group_local.is_none() {
                            let err = format!(
                                "Group {} does not exist in local database but is mapped to a user - this should never happen",
                                expected_name
                            );
                            error!("{}", err);
                            // do not return, try to execute as many mappings as possible
                            continue;
                        }
                        let g = group_local.unwrap();
                        let mut remote = self.get_group(g, false).await?;
                        if remote.is_none() {
                            remote = self.create_group(g.clone()).await?;
                        }
                        if remote.is_none() {
                            let err = format!(
                                "SCIM group {} does not exist on Service Provider even after creation - this should never happen",
                                expected_name
                            );
                            error!("{}", err);
                            // do not return, try to execute as many mappings as possible
                            continue;
                        }

                        let remote = remote.unwrap();
                        groups_remote.insert(expected_name.clone(), remote);
                        groups_remote
                            .get(&expected_name)
                            .expect("The just inserted group to exist")
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
                let url = format!("{}/Groups/{}", self.base_uri, remote_group_id);

                // create the user - group mapping
                // let mut value = HashMap::with_capacity(2);
                // value.insert(
                //     "value".into(),
                //     serde_json::Value::String(user_id_remote.clone()),
                // );
                // value.insert(
                //     "display".into(),
                //     serde_json::Value::String(user_email.clone()),
                // );
                // let payload = ScimPatchOpWithPath {
                //     operations: vec![ScimPatchOperationsWithPath {
                //         op: ScimOp::Add,
                //         path: "members".into(),
                //         value: vec![value],
                //     }],
                //     ..Default::default()
                // };
                // let json = serde_json::to_string(&payload)?;
                // debug!(
                //     "Serialized payload for ScimPatchOpWithPath with add:\n{}\n",
                //     json
                // );

                //                 let pre = r#"{"schemas":["urn:ietf:params:scim:api:messages:2.0:PatchOp"],
                // "Operations":[{"op":"add","path":"members","value":[{"value":"9eb2bd20-c70c-48fb-a0fb-ff1daccfa23f",
                // "display":"test_admin@localhost.de"}]}]}"#;

                // Same situation as for group patching. This "ugly" way of json creation is a pretty
                // big efficiency gain, and we avoid many unnecessary allocations, since we have a
                // static json that is the same each time and we know in advance.
                let json_1 = "{\"schemas\":[\"urn:ietf:params:scim:api:messages:2.0:PatchOp\"],\"Operations\":[{\"op\":\"add\",\"path\":\"members\",\"value\":[{\"value\":\"";
                let json_2 = "\",\"display\":\"";
                let json_3 = "\"}]}]}";
                let json = format!("{json_1}{}{json_2}{}{json_3}", user_id_remote, user_email);

                let res = HTTP_CLIENT
                    .patch(url)
                    .header(AUTHORIZATION, self.auth_header())
                    .header(ACCEPT, APPLICATION_JSON_SCIM)
                    .header(CONTENT_TYPE, APPLICATION_JSON_SCIM)
                    .body(json)
                    .send()
                    .await?;
                if !res.status().is_success() {
                    let err = res.json::<ScimError>().await?;
                    error!(
                        "Error adding Group assignment for SCIM client {}: {:?}",
                        self.client_id, err
                    );
                }
            }
        }

        // At this point, everything left in `user_groups_remote` is a mapping that must have been
        // done manually on the remote server, but does not match our records. We want to correct
        // it and remove the mappings.
        debug!("Left-Over user_groups_remote: {:?}", user_groups_remote);
        for group_remote in user_groups_remote {
            // let mut value = HashMap::with_capacity(1);
            // value.insert(
            //     "value".into(),
            //     serde_json::Value::String(user_id_remote.clone()),
            // );
            // let payload = ScimPatchOpWithPath {
            //     operations: vec![ScimPatchOperationsWithPath {
            //         op: ScimOp::Remove,
            //         path: "members".into(),
            //         value: vec![value],
            //     }],
            //     ..Default::default()
            // };
            // let json = serde_json::to_string(&payload)?;
            // debug!(
            //     "Serialized payload for ScimPatchOpWithPath with remove:\n{}\n",
            //     json
            // );

            //             let pre = r#"{"schemas":["urn:ietf:params:scim:api:messages:2.0:PatchOp"],
            // "Operations":[{"op":"remove","path":"members","value":[{"value":"9eb2bd20-c70c-48fb-a0fb-ff1daccfa23f"}]}]}"#;

            let json_1 = "{\"schemas\":[\"urn:ietf:params:scim:api:messages:2.0:PatchOp\"],\"Operations\":[{\"op\":\"remove\",\"path\":\"members\",\"value\":[{\"value\":\"";
            let json_2 = "\"}]}]}";
            let json = format!("{json_1}{}{json_2}", user_id_remote);

            let url = format!("{}/Groups/{}", self.base_uri, group_remote.value);
            let res = HTTP_CLIENT
                .patch(url)
                .header(AUTHORIZATION, self.auth_header())
                .header(ACCEPT, APPLICATION_JSON_SCIM)
                .header(CONTENT_TYPE, APPLICATION_JSON_SCIM)
                .body(json)
                .send()
                .await?;
            if !res.status().is_success() {
                let err = res.json::<ScimError>().await?;
                error!(
                    "Error removing Group assignment for SCIM client {}: {:?}",
                    self.client_id, err
                );
            }
        }

        Ok(())
    }
}
