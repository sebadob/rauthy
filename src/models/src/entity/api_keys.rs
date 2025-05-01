use crate::database::{Cache, DB};
use chrono::Utc;
use cryptr::{EncKeys, EncValue};
use hiqlite_macros::params;
use rauthy_api_types::api_keys::ApiKeyResponse;
use rauthy_common::constants::{API_KEY_LENGTH, CACHE_TTL_APP};
use rauthy_common::is_hiqlite;
use rauthy_common::utils::{deserialize, get_rand, serialize};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use ring::digest;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};

#[derive(Clone, Serialize, Deserialize)]
pub struct ApiKeyEntity {
    pub name: String,
    pub secret: Vec<u8>,
    pub created: i64,
    pub expires: Option<i64>,
    pub enc_key_id: String,
    pub access: Vec<u8>,
}

impl From<tokio_postgres::Row> for ApiKeyEntity {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            name: row.get("name"),
            secret: row.get("secret"),
            created: row.get("created"),
            expires: row.get("expires"),
            enc_key_id: row.get("enc_key_id"),
            access: row.get("access"),
        }
    }
}

impl Debug for ApiKeyEntity {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, secret: <hidden>, created: {}, expires: {:?}, enc_key_id: {}, access: {:?}",
            self.name, self.created, self.expires, self.enc_key_id, self.access
        )
    }
}

impl ApiKeyEntity {
    pub async fn create(
        name: String,
        expires: Option<i64>,
        access: Vec<ApiKeyAccess>,
    ) -> Result<String, ErrorResponse> {
        let created = Utc::now().timestamp();
        let secret_plain = get_rand(API_KEY_LENGTH);
        let secret_hash = digest::digest(&digest::SHA256, secret_plain.as_bytes());
        let secret_enc = EncValue::encrypt(secret_hash.as_ref())?
            .into_bytes()
            .to_vec();

        let access_bytes = serialize(&access)?;
        let access_enc = EncValue::encrypt(&access_bytes)?.into_bytes().to_vec();

        let enc_key_active = &EncKeys::get_static().enc_key_active;
        let secret_fmt = format!("{}${}", name, secret_plain);

        let sql = r#"
INSERT INTO
api_keys (name, secret, created, expires, enc_key_id, access)
VALUES ($1, $2, $3, $4, $5, $6)"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        name,
                        secret_enc,
                        created,
                        expires,
                        enc_key_active.clone(),
                        access_enc
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &name,
                    &secret_enc,
                    &created,
                    &expires,
                    enc_key_active,
                    &access_enc,
                ],
            )
            .await?;
        }

        Ok(secret_fmt)
    }

    pub async fn delete(name: &str) -> Result<(), ErrorResponse> {
        let sql = "DELETE FROM api_keys WHERE name = $1";
        if is_hiqlite() {
            DB::hql().execute(sql, params!(name)).await?;
        } else {
            DB::pg_execute(sql, &[&name]).await?;
        }

        Self::cache_invalidate(name).await?;
        Ok(())
    }

    pub async fn find(name: &str) -> Result<Self, ErrorResponse> {
        let sql = "SELECT * FROM api_keys WHERE name = $1";
        let res = if is_hiqlite() {
            DB::hql().query_as_one(sql, params!(name)).await?
        } else {
            DB::pg_query_one(sql, &[&name]).await?
        };

        Ok(res)
    }

    pub async fn find_all() -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM api_keys";
        let res = if is_hiqlite() {
            DB::hql().query_as(sql, params!()).await?
        } else {
            DB::pg_query(sql, &[], 0).await?
        };

        Ok(res)
    }

    pub async fn generate_secret(name: &str) -> Result<String, ErrorResponse> {
        let entity = ApiKeyEntity::find(name).await?;
        let api_key = entity.into_api_key()?;

        // generate a new secret
        let secret_plain = get_rand(API_KEY_LENGTH);
        let hash = digest::digest(&digest::SHA256, secret_plain.as_bytes());
        let secret_enc = EncValue::encrypt(hash.as_ref())?.into_bytes().to_vec();

        // re-encrypt access rights with possibly new active key as well
        let access_bytes = serialize(&api_key.access)?;
        let access_enc = EncValue::encrypt(&access_bytes)?.into_bytes().to_vec();

        let enc_key_active = &EncKeys::get_static().enc_key_active;

        let sql = "UPDATE api_keys SET secret = $1, enc_key_id = $2, access = $3 WHERE name = $4";

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        secret_enc,
                        enc_key_active.clone(),
                        access_enc,
                        name.to_string()
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(sql, &[&secret_enc, enc_key_active, &access_enc, &name]).await?;
        }

        Self::cache_invalidate(name).await?;

        let secret_fmt = format!("{}${}", name, secret_plain);
        Ok(secret_fmt)
    }

    /// Updates the API Key. Does NOT update the secret in any way!
    pub async fn update(
        name: &str,
        expires: Option<i64>,
        access: Vec<ApiKeyAccess>,
    ) -> Result<(), ErrorResponse> {
        let entity = ApiKeyEntity::find(name).await?;
        let api_key = entity.into_api_key()?;

        let secret_enc = EncValue::encrypt(&api_key.secret)?.into_bytes().to_vec();

        let access_bytes = serialize(&access)?;
        let access_enc = EncValue::encrypt(&access_bytes)?.into_bytes().to_vec();

        let enc_key_active = &EncKeys::get_static().enc_key_active;

        let sql = r#"
UPDATE api_keys
SET secret = $1, expires = $2, enc_key_id = $3, access = $4
WHERE name = $5"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        secret_enc,
                        expires,
                        enc_key_active.clone(),
                        access_enc,
                        name.to_string()
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[&secret_enc, &expires, enc_key_active, &access_enc, &name],
            )
            .await?;
        }

        Self::cache_invalidate(name).await?;

        Ok(())
    }

    pub async fn save(self) -> Result<(), ErrorResponse> {
        let name = self.name.clone();

        let sql = r#"
UPDATE api_keys
SET secret = $1, expires = $2, enc_key_id = $3, access = $4
WHERE name = $5"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        self.secret,
                        self.expires,
                        self.enc_key_id,
                        self.access,
                        self.name
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &self.secret,
                    &self.expires,
                    &self.enc_key_id,
                    &self.access,
                    &self.name,
                ],
            )
            .await?;
        }

        Self::cache_invalidate(&name).await?;

        Ok(())
    }
}

impl ApiKeyEntity {
    #[inline]
    fn cache_idx(name: &str) -> String {
        format!("api_key_{}", name)
    }

    #[inline]
    async fn cache_invalidate(name: &str) -> Result<(), ErrorResponse> {
        let idx = Self::cache_idx(name);
        DB::hql().delete(Cache::App, idx).await?;
        Ok(())
    }

    #[inline(always)]
    pub async fn api_key_from_token_validated(token: &str) -> Result<ApiKey, ErrorResponse> {
        let (name, secret) = token.split_once('$').ok_or_else(|| {
            ErrorResponse::new(ErrorResponseType::BadRequest, "Malformed API-Key")
        })?;

        let client = DB::hql();
        let idx = Self::cache_idx(name);
        let api_key = if let Some(key) = client.get(Cache::App, &idx).await? {
            key
        } else {
            let key = Self::find(name).await?.into_api_key()?;
            client.put(Cache::App, idx, &key, CACHE_TTL_APP).await?;
            key
        };

        api_key.validate_secret(secret)?;

        Ok(api_key)
    }

    pub fn into_api_key(self) -> Result<ApiKey, ErrorResponse> {
        let secret = EncValue::try_from(self.secret)?.decrypt()?.to_vec();
        let access_dec = EncValue::try_from(self.access)?.decrypt()?.to_vec();
        let access = deserialize::<Vec<ApiKeyAccess>>(&access_dec)?;

        Ok(ApiKey {
            name: self.name,
            secret,
            created: self.created,
            expires: self.expires,
            access,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AccessGroup {
    Blacklist,
    Clients,
    Events,
    Generic,
    Groups,
    Roles,
    Secrets,
    Sessions,
    Scopes,
    UserAttributes,
    Users,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum AccessRights {
    Read,
    Create,
    Update,
    Delete,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ApiKeyAccess {
    pub group: AccessGroup,
    pub access_rights: Vec<AccessRights>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub name: String,
    /// SHA256 hashed secret key
    pub secret: Vec<u8>,
    pub created: i64,
    pub expires: Option<i64>,
    pub access: Vec<ApiKeyAccess>,
}

impl Debug for ApiKey {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "name: {}, secret: <hidden>, created: {}, expires: {:?}, access: {:?}",
            self.name, self.created, self.expires, self.access
        )
    }
}

impl ApiKey {
    #[inline(always)]
    pub fn validate_access(
        &self,
        group: &AccessGroup,
        access_rights: &AccessRights,
    ) -> Result<(), ErrorResponse> {
        for a in &self.access {
            if &a.group == group {
                return if a.access_rights.contains(access_rights) {
                    Ok(())
                } else {
                    Err(ErrorResponse::new(
                        ErrorResponseType::Forbidden,
                        "Access denied",
                    ))
                };
            }
        }
        Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "Access denied",
        ))
    }

    #[inline(always)]
    pub fn validate_secret(&self, secret: &str) -> Result<(), ErrorResponse> {
        if let Some(exp) = self.expires {
            if Utc::now().timestamp() > exp {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Unauthorized,
                    "API Key has expired",
                ));
            }
        }

        let hash = digest::digest(&digest::SHA256, secret.as_bytes());
        if hash.as_ref() == self.secret.as_slice() {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid API-Key",
            ))
        }
    }
}

impl From<ApiKey> for ApiKeyResponse {
    fn from(value: ApiKey) -> Self {
        Self {
            name: value.name,
            created: value.created,
            expires: value.expires,
            access: value
                .access
                .into_iter()
                .map(rauthy_api_types::api_keys::ApiKeyAccess::from)
                .collect(),
        }
    }
}

impl From<AccessGroup> for rauthy_api_types::api_keys::AccessGroup {
    fn from(value: AccessGroup) -> Self {
        match value {
            AccessGroup::Blacklist => Self::Blacklist,
            AccessGroup::Clients => Self::Clients,
            AccessGroup::Events => Self::Events,
            AccessGroup::Generic => Self::Generic,
            AccessGroup::Groups => Self::Groups,
            AccessGroup::Roles => Self::Roles,
            AccessGroup::Secrets => Self::Secrets,
            AccessGroup::Sessions => Self::Sessions,
            AccessGroup::Scopes => Self::Scopes,
            AccessGroup::UserAttributes => Self::UserAttributes,
            AccessGroup::Users => Self::Users,
        }
    }
}

impl From<AccessRights> for rauthy_api_types::api_keys::AccessRights {
    fn from(value: AccessRights) -> Self {
        match value {
            AccessRights::Read => Self::Read,
            AccessRights::Create => Self::Create,
            AccessRights::Update => Self::Update,
            AccessRights::Delete => Self::Delete,
        }
    }
}

impl From<ApiKeyAccess> for rauthy_api_types::api_keys::ApiKeyAccess {
    fn from(value: ApiKeyAccess) -> Self {
        Self {
            group: value.group.into(),
            access_rights: value
                .access_rights
                .into_iter()
                .map(|ar| ar.into())
                .collect(),
        }
    }
}

impl From<rauthy_api_types::api_keys::AccessGroup> for AccessGroup {
    fn from(value: rauthy_api_types::api_keys::AccessGroup) -> Self {
        match value {
            rauthy_api_types::api_keys::AccessGroup::Blacklist => Self::Blacklist,
            rauthy_api_types::api_keys::AccessGroup::Clients => Self::Clients,
            rauthy_api_types::api_keys::AccessGroup::Events => Self::Events,
            rauthy_api_types::api_keys::AccessGroup::Generic => Self::Generic,
            rauthy_api_types::api_keys::AccessGroup::Groups => Self::Groups,
            rauthy_api_types::api_keys::AccessGroup::Roles => Self::Roles,
            rauthy_api_types::api_keys::AccessGroup::Secrets => Self::Secrets,
            rauthy_api_types::api_keys::AccessGroup::Sessions => Self::Sessions,
            rauthy_api_types::api_keys::AccessGroup::Scopes => Self::Scopes,
            rauthy_api_types::api_keys::AccessGroup::UserAttributes => Self::UserAttributes,
            rauthy_api_types::api_keys::AccessGroup::Users => Self::Users,
        }
    }
}

impl From<rauthy_api_types::api_keys::AccessRights> for AccessRights {
    fn from(value: rauthy_api_types::api_keys::AccessRights) -> Self {
        match value {
            rauthy_api_types::api_keys::AccessRights::Read => Self::Read,
            rauthy_api_types::api_keys::AccessRights::Create => Self::Create,
            rauthy_api_types::api_keys::AccessRights::Update => Self::Update,
            rauthy_api_types::api_keys::AccessRights::Delete => Self::Delete,
        }
    }
}

impl From<rauthy_api_types::api_keys::ApiKeyAccess> for ApiKeyAccess {
    fn from(value: rauthy_api_types::api_keys::ApiKeyAccess) -> Self {
        Self {
            group: value.group.into(),
            access_rights: value
                .access_rights
                .into_iter()
                .map(|ar| ar.into())
                .collect(),
        }
    }
}
