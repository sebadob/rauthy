use crate::database::{Cache, DB};
use crate::entity::password::PasswordPolicy;
use crate::entity::users::User;
use crate::rauthy_config::RauthyConfig;
use chrono::Utc;
use hiqlite::{Params, params};
use rauthy_api_types::users::PasskeyResponse;
use rauthy_common::constants::IDX_WEBAUTHN;
use rauthy_common::is_hiqlite;
use rauthy_derive::FromPgRow;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use webauthn_rs::prelude::{CredentialID, Passkey, Uuid};

#[derive(Clone, Deserialize, Serialize, FromPgRow)]
pub struct PasskeyEntity {
    pub user_id: String,
    pub name: String,
    pub passkey_user_id: String,
    pub passkey: String,
    pub credential_id: Vec<u8>,
    pub registered: i64,
    pub last_used: i64,
    pub user_verified: Option<bool>,
    pub resident_key: Option<bool>,
}

impl Debug for PasskeyEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "PasskeyEntity {{ user_id: {}, name: {}, passkey_user_id: {}, passkey: <hidden>, \
        credential_id: <hidden>, registered: {}, last_used: {}, user_verified: {:?}, \
        resident_key: {:?} }}",
            self.user_id,
            self.name,
            self.passkey_user_id,
            self.registered,
            self.last_used,
            self.user_verified,
            self.resident_key,
        )
    }
}

// CRUD
impl PasskeyEntity {
    /// If the `User` is `Some(_)`, a `User::save()` will be included in the `txn`
    pub async fn create(
        user_id: String,
        user: Option<User>,
        passkey_user_id: Uuid,
        name: String,
        pk: Passkey,
        user_verified: bool,
        resident_key: Option<bool>,
    ) -> Result<(), ErrorResponse> {
        // json, because bincode does not support deserialize from any, which would be the case here
        let passkey = serde_json::to_string(&pk)?;
        let now = Utc::now().timestamp();

        let entity = Self {
            user_id,
            name,
            passkey_user_id: passkey_user_id.to_string(),
            passkey,
            credential_id: pk.cred_id().to_vec(),
            registered: now,
            last_used: now,
            user_verified: Some(user_verified),
            resident_key,
        };

        let user_email = user.as_ref().map(|u| u.email.clone());
        let client = DB::hql();

        let sql = r#"
INSERT INTO passkeys (
    user_id, name, passkey_user_id, passkey, credential_id, registered, last_used, user_verified,
    resident_key
)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#;

        if is_hiqlite() {
            let mut txn = Vec::with_capacity(2);

            if let Some(user) = user {
                debug_assert!(user.webauthn_user_id.is_some());
                user.save_txn_append(&mut txn);
            }

            txn.push((
                sql,
                params!(
                    &entity.user_id,
                    entity.name,
                    entity.passkey_user_id,
                    entity.passkey,
                    entity.credential_id,
                    now,
                    now,
                    entity.user_verified,
                    entity.resident_key
                ),
            ));

            client.txn(txn).await?;
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            if let Some(user) = user {
                debug_assert!(user.webauthn_user_id.is_some());
                user.save_txn(&txn).await?;
            }
            DB::pg_txn_append(
                &txn,
                sql,
                &[
                    &entity.user_id,
                    &entity.name,
                    &entity.passkey_user_id,
                    &entity.passkey,
                    &entity.credential_id,
                    &now,
                    &now,
                    &entity.user_verified,
                    &entity.resident_key,
                ],
            )
            .await?;

            txn.commit().await?;
        }

        if let Some(email) = user_email {
            User::invalidate_cache(&entity.user_id, &email).await?;
        }
        client
            .delete(Cache::Webauthn, Self::cache_idx_user(&entity.user_id))
            .await?;
        client
            .delete(
                Cache::Webauthn,
                Self::cache_idx_user_with_uv(&entity.user_id),
            )
            .await?;
        client
            .delete(Cache::Webauthn, Self::cache_idx_creds(&entity.user_id))
            .await?;

        Ok(())
    }

    pub async fn count_for_user(user_id: String) -> Result<i64, ErrorResponse> {
        let sql = "SELECT COUNT (*) AS count FROM passkeys WHERE user_id = $1";
        let count: i64 = if is_hiqlite() {
            DB::hql()
                .query_raw_one(sql, params!(user_id))
                .await?
                .get("count")
        } else {
            DB::pg_query_one_row(sql, &[&user_id]).await?.get("count")
        };

        Ok(count)
    }

    pub async fn delete(user_id: String, name: String) -> Result<(), ErrorResponse> {
        // if we delete a passkey, we must check if this is the last existing one for the user
        let pk_count = Self::count_for_user(user_id.clone()).await?;

        let mut user_to_save: Option<User> = None;
        let mut user_email: Option<String> = None;

        if pk_count < 2 {
            let mut user = User::find(user_id.clone()).await?;
            user.webauthn_user_id = None;

            // We need to check against the current password policy
            // if the password should expire again.
            let policy = PasswordPolicy::find().await?;
            if let Some(valid_days) = policy.valid_days {
                if user.password.is_some() {
                    user.password_expires = Some(
                        Utc::now()
                            .add(chrono::Duration::days(valid_days as i64))
                            .timestamp(),
                    );
                } else {
                    user.password_expires = None;
                }
            }

            user_email = Some(user.email.clone());
            user_to_save = Some(user);
        }

        if is_hiqlite() {
            let mut txn = Vec::with_capacity(2);

            Self::delete_by_id_name_append(user_id.clone(), name.clone(), &mut txn);
            if let Some(user) = user_to_save {
                user.save_txn_append(&mut txn);
            }

            DB::hql().txn(txn).await?;
        } else {
            let mut cl = DB::pg().await?;
            let txn = cl.transaction().await?;

            Self::delete_by_id_name(&user_id, &name, &txn).await?;
            if let Some(user) = user_to_save {
                user.save_txn(&txn).await?;
            }

            txn.commit().await?;
        }

        Self::clear_caches_by_id_name(&user_id, user_email, &name).await?;

        Ok(())
    }

    /// MUST call `PasskeyEntity::clear_caches_by_id_name()` after txn commit!
    async fn delete_by_id_name(
        user_id: &str,
        name: &str,
        txn: &deadpool_postgres::Transaction<'_>,
    ) -> Result<(), ErrorResponse> {
        DB::pg_txn_append(
            txn,
            "DELETE FROM passkeys WHERE user_id = $1 AND name = $2",
            &[&user_id, &name],
        )
        .await?;
        Ok(())
    }

    /// MUST call `PasskeyEntity::clear_caches_by_id_name()` after txn commit!
    fn delete_by_id_name_append(user_id: String, name: String, txn: &mut Vec<(&str, Params)>) {
        txn.push((
            "DELETE FROM passkeys WHERE user_id = $1 AND name = $2",
            params!(user_id, name),
        ));
    }

    async fn clear_caches_by_id_name(
        user_id: &str,
        user_email: Option<String>,
        name: &str,
    ) -> Result<(), ErrorResponse> {
        let client = DB::hql();

        if let Some(email) = user_email {
            User::invalidate_cache(user_id, &email).await?;
        }

        client
            .delete(Cache::Webauthn, Self::cache_idx_single(user_id, name))
            .await?;
        client
            .delete(Cache::Webauthn, Self::cache_idx_user(user_id))
            .await?;
        client
            .delete(Cache::Webauthn, Self::cache_idx_user_with_uv(user_id))
            .await?;
        client
            .delete(Cache::Webauthn, Self::cache_idx_creds(user_id))
            .await?;

        Ok(())
    }

    pub async fn find(user_id: &str, name: &str) -> Result<Self, ErrorResponse> {
        let idx = Self::cache_idx_single(user_id, name);
        let client = DB::hql();

        if let Some(slf) = client.get(Cache::Webauthn, &idx).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM passkeys WHERE user_id = $1 AND name = $2";
        let slf = if is_hiqlite() {
            client.query_as_one(sql, params!(user_id, name)).await?
        } else {
            DB::pg_query_one(sql, &[&user_id, &name]).await?
        };

        let ttl = Some(RauthyConfig::get().vars.webauthn.req_exp as i64);
        client.put(Cache::Webauthn, idx, &slf, ttl).await?;

        Ok(slf)
    }

    pub async fn find_rk_by_cred_id(cred_id: &[u8]) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM passkeys WHERE credential_id = $1 AND resident_key = true";

        let slf = if is_hiqlite() {
            DB::hql()
                .query_as_one(sql, params!(cred_id))
                .await
                .map_err(|_| {
                    ErrorResponse::new(ErrorResponseType::NotFound, "Unknown Resident Key")
                })?
        } else {
            DB::pg_query_one(sql, &[&cred_id]).await.map_err(|_| {
                ErrorResponse::new(ErrorResponseType::NotFound, "Unknown Resident Key")
            })?
        };

        Ok(slf)
    }

    pub async fn find_cred_ids_for_user(user_id: &str) -> Result<Vec<CredentialID>, ErrorResponse> {
        let idx = Self::cache_idx_creds(user_id);
        let client = DB::hql();

        let opt: Option<Vec<Vec<u8>>> = client.get(Cache::Webauthn, &idx).await?;
        if let Some(bytes) = opt {
            return Ok(bytes.into_iter().map(CredentialID::from).collect());
        }

        let sql = "SELECT credential_id FROM passkeys WHERE user_id = $1";
        let creds = if is_hiqlite() {
            client
                .query_raw(sql, params!(user_id))
                .await?
                .into_iter()
                .map(|mut r| r.get::<Vec<u8>>("credential_id"))
                .collect::<Vec<_>>()
        } else {
            DB::pg_query_rows(sql, &[&user_id], 2)
                .await?
                .into_iter()
                .map(|r| r.get::<_, Vec<u8>>("credential_id"))
                .collect::<Vec<_>>()
        };

        let ttl = Some(RauthyConfig::get().vars.webauthn.req_exp as i64);
        client.put(Cache::Webauthn, idx, &creds, ttl).await?;

        Ok(creds.into_iter().map(CredentialID::from).collect())
    }

    pub async fn find_for_user(user_id: &str) -> Result<Vec<Self>, ErrorResponse> {
        let idx = Self::cache_idx_user(user_id);
        let client = DB::hql();

        if let Some(slf) = client.get(Cache::Webauthn, &idx).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM passkeys WHERE user_id = $1";
        let pks = if is_hiqlite() {
            client.query_as(sql, params!(user_id)).await?
        } else {
            DB::pg_query(sql, &[&user_id], 2).await?
        };

        let ttl = Some(RauthyConfig::get().vars.webauthn.req_exp as i64);
        client.put(Cache::Webauthn, idx, &pks, ttl).await?;

        Ok(pks)
    }

    pub async fn find_for_user_with_uv(user_id: &str) -> Result<Vec<Self>, ErrorResponse> {
        let idx = Self::cache_idx_user_with_uv(user_id);
        let client = DB::hql();

        if let Some(slf) = client.get(Cache::Webauthn, &idx).await? {
            return Ok(slf);
        }

        let sql = "SELECT * FROM passkeys WHERE user_id = $1 AND user_verified = true";
        let pks = if is_hiqlite() {
            client.query_as(sql, params!(user_id)).await?
        } else {
            DB::pg_query(sql, &[&user_id], 2).await?
        };

        let ttl = Some(RauthyConfig::get().vars.webauthn.req_exp as i64);
        client.put(Cache::Webauthn, idx, &pks, ttl).await?;

        Ok(pks)
    }

    pub async fn update_passkey(&self) -> Result<(), ErrorResponse> {
        let client = DB::hql();

        let sql = r#"
UPDATE passkeys
SET passkey = $1, last_used = $2
WHERE user_id = $3 AND name = $4"#;

        if is_hiqlite() {
            client
                .execute(
                    sql,
                    params!(&self.passkey, self.last_used, &self.user_id, &self.name),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[&self.passkey, &self.last_used, &self.user_id, &self.name],
            )
            .await?;
        }

        let ttl = Some(RauthyConfig::get().vars.webauthn.req_exp as i64);
        client
            .put(
                Cache::Webauthn,
                Self::cache_idx_single(&self.user_id, &self.name),
                self,
                ttl,
            )
            .await?;

        client
            .delete(Cache::Webauthn, Self::cache_idx_user(&self.user_id))
            .await?;
        client
            .delete(Cache::Webauthn, Self::cache_idx_user_with_uv(&self.user_id))
            .await?;

        Ok(())
    }
}

impl PasskeyEntity {
    #[inline]
    pub fn get_pk(&self) -> Passkey {
        // Passkeys cannot be serialized with bincode -> no support for deserialize from any
        serde_json::from_str(&self.passkey).unwrap()
    }

    /// Index for a single passkey for a user
    #[inline]
    fn cache_idx_single(user_id: &str, name: &str) -> String {
        format!("{IDX_WEBAUTHN}{user_id}{name}")
    }

    /// Index for all passkeys a user has
    #[inline]
    fn cache_idx_user(user_id: &str) -> String {
        format!("{IDX_WEBAUTHN}{user_id}")
    }

    /// Index for all passkeys a user has
    #[inline]
    fn cache_idx_user_with_uv(user_id: &str) -> String {
        format!("{IDX_WEBAUTHN}_UV_{user_id}",)
    }

    /// Index for credentials for a user
    #[inline]
    fn cache_idx_creds(user_id: &str) -> String {
        format!("{IDX_WEBAUTHN}{user_id}_creds")
    }
}

impl From<PasskeyEntity> for PasskeyResponse {
    fn from(value: PasskeyEntity) -> Self {
        Self {
            name: value.name,
            registered: value.registered,
            last_used: value.last_used,
            user_verified: value.user_verified,
            resident_key: value.resident_key,
        }
    }
}
