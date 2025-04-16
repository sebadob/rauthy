use crate::database::DB;
use crate::entity::clients::Client;
use crate::entity::failed_scim_tasks;
use crate::entity::failed_scim_tasks::{FailedScimTask, ScimAction};
use crate::entity::groups::Group;
use crate::entity::scim_types::{
    ScimError, ScimGroup, ScimListResponse, ScimOp, ScimPatchOp, ScimPatchOperations, ScimResource,
    ScimUser,
};
use crate::entity::users::User;
use crate::entity::users_values::UserValues;
use cryptr::EncValue;
use hiqlite::{Param, params};
use rauthy_common::constants::APPLICATION_JSON_SCIM;
use rauthy_common::{HTTP_CLIENT, is_hiqlite};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};
use std::collections::HashMap;
use std::default::Default;
use std::fmt::Debug;
use tracing::{debug, error, warn};

#[derive(Clone, PartialEq)]
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
            "ClientScim {{ client_id: {}, bearer_token: <hidden>, base_endpoint: {}, \
            group_sync_prefix: {:?} }}",
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

    // fn url_group(&self, group: &Group) -> String {
    //     format!(
    //         "{}/Groups?filter=displayName%20eq%20%22{}%22",
    //         self.base_endpoint, group.name
    //     )
    // }

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
        match self.sync_groups_exec().await {
            Ok(_) => Ok(()),
            Err(err) => {
                let err = format!("Error during sync groups: {}", err);
                error!("{}", err);

                FailedScimTask::upsert(
                    &ScimAction::Sync,
                    &failed_scim_tasks::ScimResource::Groups,
                    &self.client_id,
                )
                .await?;

                Err(ErrorResponse::new(ErrorResponseType::Connection, err))
            }
        }
    }

    async fn sync_groups_exec(&self) -> Result<(), ErrorResponse> {
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
                return Err(ErrorResponse::new(
                    ErrorResponseType::Connection,
                    format!("SCIM GET /Groups errior: {:?}", err),
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

    /// Fetches a SCIM group from the Service Provider.
    /// Always tries via `externalId` first and will use a fallback to `displayName` if None
    /// has been returned for the id.
    async fn get_group(&self, group: &Group) -> Result<Option<ScimGroup>, ErrorResponse> {
        let url = format!(
            "{}/Groups?filter=externalId%20eq%20%22{}%22",
            self.base_endpoint, group.id
        );
        match self.get_group_with(group, url).await? {
            None => {
                let url = format!(
                    "{}/Groups?filter=displayName%20eq%20%22{}%22",
                    self.base_endpoint, group.name
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
                        ErrorResponseType::Connection,
                        format!(
                            "Received a User from SCIM client {} when expected a Group",
                            self.client_id
                        ),
                    )),
                    ScimResource::Group(remote) => {
                        if let Some(ext_id) = &remote.external_id {
                            if ext_id != &group.id {
                                return Err(ErrorResponse::new(
                                    ErrorResponseType::Connection,
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
                ErrorResponseType::Connection,
                format!(
                    "Error getting group from SCIM client {}: {:?}",
                    self.client_id, err
                ),
            ))
        }
    }

    /// Either creates or updates the group, depending on if it exists on remote already, or not.
    pub async fn create_update_group(&self, group: Group) -> Result<(), ErrorResponse> {
        if !self.should_sync_group(Some(&group)) {
            return Ok(());
        }

        let gid = group.id.clone();
        if let Some(remote) = self.get_group(&group).await? {
            if let Err(err) = self.update_group(group, remote).await {
                error!(
                    "Error during create group for SCIM client {}: {:?}",
                    self.client_id, err
                );
                FailedScimTask::upsert(
                    &ScimAction::Create,
                    &failed_scim_tasks::ScimResource::Group(gid),
                    &self.client_id,
                )
                .await?;
            }
        } else if let Err(err) = self.create_group(group).await {
            error!(
                "Error during update group for SCIM client {}: {:?}",
                self.client_id, err
            );
            FailedScimTask::upsert(
                &ScimAction::Update,
                &failed_scim_tasks::ScimResource::Group(gid),
                &self.client_id,
            )
            .await?;
        };

        Ok(())
    }

    async fn create_group(&self, group: Group) -> Result<Option<ScimGroup>, ErrorResponse> {
        if !self.should_sync_group(Some(&group)) {
            return Ok(None);
        }

        let local_id = group.id.clone();

        let payload = ScimGroup {
            // must be None on our side - managed by the client
            id: None,
            external_id: Some(group.id),
            display_name: group.name,
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
                Ok(Some(remote))
            } else {
                Err(ErrorResponse::new(
                    ErrorResponseType::Connection,
                    format!(
                        "SCIM client {} has not respected our group id",
                        self.client_id
                    ),
                ))
            }
        } else {
            let err = res.json::<ScimError>().await?;
            Err(ErrorResponse::new(
                ErrorResponseType::Connection,
                format!(
                    "SCIM client {} group creation failed: {:?}",
                    self.client_id, err
                ),
            ))
        }
    }

    async fn update_group(
        &self,
        group_local: Group,
        group_remote: ScimGroup,
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
        value.insert(
            "externalId".into(),
            serde_json::Value::String(group_local.id),
        );
        value.insert(
            "displayName".into(),
            serde_json::Value::String(group_local.name),
        );
        let payload = ScimPatchOp {
            operations: vec![ScimPatchOperations {
                op: ScimOp::Replace,
                path: None,
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
            Err(ErrorResponse::new(
                ErrorResponseType::Connection,
                format!(
                    "SCIM client {} group update failed: {:?}",
                    self.client_id, err
                ),
            ))
        }
    }

    pub async fn delete_group(&self, group: Group) -> Result<(), ErrorResponse> {
        let gid = group.id.clone();
        if let Err(err) = self.delete_group_exec(group).await {
            error!(
                "Error during delete group for SCIM client {}: {:?}",
                self.client_id, err
            );
            FailedScimTask::upsert(
                &ScimAction::Delete,
                &failed_scim_tasks::ScimResource::Group(gid),
                &self.client_id,
            )
            .await?;
        }

        Ok(())
    }

    async fn delete_group_exec(&self, group: Group) -> Result<(), ErrorResponse> {
        if !self.should_sync_group(Some(&group)) {
            return Ok(());
        }

        let remote_group = self.get_group(&group).await?;
        if remote_group.is_none() {
            debug!("Should delete remote group but does not exist - nothing to do");
            return Ok(());
        }
        let remote_id = if let Some(id) = remote_group.unwrap().id {
            id
        } else {
            return Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                "Remote SCIM group without an ID",
            ));
        };
        let url = format!("{}/Groups/{}", self.base_endpoint, remote_id);

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
                ErrorResponseType::Connection,
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
            self.base_endpoint, user_id
        );
        match self.get_user_with(user_id, user_email, url).await? {
            None => {
                let email = old_email.unwrap_or(user_email);
                let url = format!(
                    "{}/Users?filter=userName%20eq%20%22{}%22",
                    self.base_endpoint, email
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
                                return Err(ErrorResponse::new(ErrorResponseType::Connection, err));
                            }
                        }
                        Ok(Some(*remote))
                    }
                    ScimResource::Group(_) => Err(ErrorResponse::new(
                        ErrorResponseType::Connection,
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
                ErrorResponseType::Connection,
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

                FailedScimTask::upsert(
                    &ScimAction::Sync,
                    &failed_scim_tasks::ScimResource::Users(last_created_ts),
                    &self.client_id,
                )
                .await?;

                Err(ErrorResponse::new(ErrorResponseType::Connection, err))
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
        let mut users = User::find_for_scim_sync(*last_created_ts, 100).await?;

        while !users.is_empty() {
            for (user, values) in users {
                debug_assert_eq!(user.id, values.id);
                let created = user.created_at;

                match self
                    .create_update_user_exec(
                        user,
                        &client,
                        values,
                        None,
                        groups_local,
                        groups_remote,
                    )
                    .await
                {
                    Ok(_) => {
                        *last_created_ts = created;
                    }
                    Err(err) => {
                        error!(
                            "Error during sync users for SCIM client {}: {:?}",
                            self.client_id, err
                        );
                        FailedScimTask::upsert(
                            &ScimAction::Sync,
                            &failed_scim_tasks::ScimResource::Users(*last_created_ts),
                            &self.client_id,
                        )
                        .await?;
                    }
                }
            }

            users = User::find_for_scim_sync(*last_created_ts, 100).await?;
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
            let client = Client::find(client_scim.client_id.clone()).await?;
            client_scim
                .create_update_user_exec(
                    user.clone(),
                    &client,
                    uv.clone(),
                    None,
                    &groups_local,
                    &mut groups_remote,
                )
                .await?;
        }

        Ok(())
    }

    /// Either creates or updates the user, depending on if it exists on remote already, or not.
    pub async fn create_update_user_exec(
        &self,
        user: User,
        client: &Client,
        user_values: UserValues,
        old_email: Option<&str>,
        groups_local: &[Group],
        groups_remote: &mut HashMap<String, ScimGroup>,
    ) -> Result<(), ErrorResponse> {
        let scopes = format!("{},{}", client.scopes, client.default_scopes);
        let expected_groups = user.get_groups();

        let user_id = user.id.clone();
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
                if let Err(err) = self
                    .create_user(expected_groups, payload, groups_local, groups_remote)
                    .await
                {
                    error!(
                        "Error during create user {} for SCIM client {}: {:?}",
                        user_id, self.client_id, err
                    );
                    FailedScimTask::upsert(
                        &ScimAction::Create,
                        &failed_scim_tasks::ScimResource::User(user_id),
                        &self.client_id,
                    )
                    .await?;
                }
            }
            Some(user_remote) => {
                if let Err(err) = self
                    .update_user(
                        expected_groups,
                        payload,
                        user_remote,
                        groups_local,
                        groups_remote,
                    )
                    .await
                {
                    error!(
                        "Error during update user {} for SCIM client {}: {:?}",
                        user_id, self.client_id, err
                    );
                    FailedScimTask::upsert(
                        &ScimAction::Update,
                        &failed_scim_tasks::ScimResource::User(user_id),
                        &self.client_id,
                    )
                    .await?;
                }
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
            .post(format!("{}/Users", self.base_endpoint))
            .header(AUTHORIZATION, self.auth_header())
            .header(CONTENT_TYPE, APPLICATION_JSON_SCIM)
            .body(json)
            .send()
            .await?;

        if res.status().is_success() {
            let user_remote = res.json::<ScimUser>().await?;
            self.update_groups_assignment(
                groups_expected,
                update_payload.user_name,
                user_remote,
                groups_local,
                groups_remote,
            )
            .await?;
            Ok(())
        } else {
            let err = res.json::<ScimError>().await?;
            error!("{:?}", err);
            Err(ErrorResponse::new(
                ErrorResponseType::Connection,
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
        update_payload: ScimUser,
        user_remote: ScimUser,
        groups_local: &[Group],
        groups_remote: &mut HashMap<String, ScimGroup>,
    ) -> Result<(), ErrorResponse> {
        let json = serde_json::to_string(&update_payload)?;
        let res = HTTP_CLIENT
            .put(format!(
                "{}/Users/{}",
                self.base_endpoint,
                user_remote.id.unwrap_or_default()
            ))
            .header(AUTHORIZATION, self.auth_header())
            .header(CONTENT_TYPE, APPLICATION_JSON_SCIM)
            .body(json)
            .send()
            .await?;

        if res.status().is_success() {
            let user_remote = res.json::<ScimUser>().await?;
            self.update_groups_assignment(
                groups_expected,
                update_payload.user_name,
                user_remote,
                groups_local,
                groups_remote,
            )
            .await?;
            Ok(())
        } else {
            let err = res.json::<ScimError>().await?;
            Err(ErrorResponse::new(
                ErrorResponseType::Connection,
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
            FailedScimTask::upsert(
                &ScimAction::Delete,
                &failed_scim_tasks::ScimResource::User(user_id),
                &self.client_id,
            )
            .await?;
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
                ErrorResponseType::Internal,
                "Remote SCIM user without an ID",
            ));
        };
        let url = format!("{}/Users/{}", self.base_endpoint, remote_id);

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
                ErrorResponseType::Connection,
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
    ) -> Result<(), ErrorResponse> {
        if let Some(prefix) = &self.group_sync_prefix {
            user_groups_local.retain(|g| g.starts_with(prefix));
        }
        let user_id_remote = match user_remote.id.clone() {
            None => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Internal,
                    format!("empty remote user id for SCIM user {:?}", user_remote),
                ));
            }
            Some(id) => id,
        };
        let mut user_groups_remote = user_remote.groups.unwrap_or_default();

        while let Some(expected_name) = user_groups_local.pop() {
            let pos = user_groups_remote
                .iter()
                .position(|g| g.value == expected_name);
            if let Some(pos) = pos {
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
                    let mut remote = self.get_group(g).await?;
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
            let url = format!("{}/Groups/{}", self.base_endpoint, remote_group_id);

            // create the user - group mapping
            let mut value = HashMap::with_capacity(2);
            value.insert(
                "value".into(),
                serde_json::Value::String(user_id_remote.clone()),
            );
            value.insert(
                "display".into(),
                serde_json::Value::String(user_email.clone()),
            );
            let payload = ScimPatchOp {
                operations: vec![ScimPatchOperations {
                    op: ScimOp::Add,
                    path: Some("members".into()),
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
            if !res.status().is_success() {
                let err = res.json::<ScimError>().await?;
                error!(
                    "Error updating Groups assignment for SCIM client {}: {:?}",
                    self.client_id, err
                );
            }
        }

        // At this point, everything left in `user_groups_remote` is a mapping that must have been
        // done manually on the remote server, but does not match our records. We want to correct
        // it and remove the mappings.
        // TODO maybe add an auto-cleanup logic here
        if !user_groups_remote.is_empty() {
            warn!(
                "Found user - group mappings for SCIM Client {} and user {} that do not match our \
                records. Needs manual interaction: {:?}",
                self.client_id, user_email, user_groups_remote
            );
        }

        Ok(())
    }
}
