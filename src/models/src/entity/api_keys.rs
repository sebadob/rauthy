use crate::app_state::{AppState, DbPool};
use crate::cache::{Cache, DB};
use actix_web::web;
use chrono::Utc;
use cryptr::{EncKeys, EncValue};
use rauthy_api_types::api_keys::ApiKeyResponse;
use rauthy_common::constants::{API_KEY_LENGTH, CACHE_TTL_APP};
use rauthy_common::utils::get_rand;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use ring::digest;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct ApiKeyEntity {
    pub name: String,
    pub secret: Vec<u8>,
    pub created: i64,
    pub expires: Option<i64>,
    pub enc_key_id: String,
    pub access: Vec<u8>,
}

impl ApiKeyEntity {
    pub async fn create(
        db: &DbPool,
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

        let access_bytes = bincode::serialize(&access)?;
        let access_enc = EncValue::encrypt(&access_bytes)?.into_bytes().to_vec();

        let enc_key_active = &EncKeys::get_static().enc_key_active;

        query!(
            r#"INSERT INTO
            api_keys (name, secret, created, expires, enc_key_id, access)
            VALUES ($1, $2, $3, $4, $5, $6)"#,
            name,
            secret_enc,
            created,
            expires,
            enc_key_active,
            access_enc,
        )
        .execute(db)
        .await?;

        let secret_fmt = format!("{}${}", name, secret_plain);
        Ok(secret_fmt)
    }

    pub async fn delete(data: &web::Data<AppState>, name: &str) -> Result<(), ErrorResponse> {
        query!("DELETE FROM api_keys WHERE name = $1", name)
            .execute(&data.db)
            .await?;

        Self::cache_invalidate(name).await?;
        Ok(())
    }

    pub async fn find(data: &web::Data<AppState>, name: &str) -> Result<Self, ErrorResponse> {
        let res = query_as!(Self, "SELECT * FROM api_keys WHERE name = $1", name)
            .fetch_one(&data.db)
            .await?;
        Ok(res)
    }

    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let res = query_as!(Self, "SELECT * FROM api_keys")
            .fetch_all(&data.db)
            .await?;
        Ok(res)
    }

    pub async fn generate_secret(
        data: &web::Data<AppState>,
        name: &str,
    ) -> Result<String, ErrorResponse> {
        let entity = ApiKeyEntity::find(data, name).await?;
        let api_key = entity.into_api_key()?;

        // generate a new secret
        let secret_plain = get_rand(API_KEY_LENGTH);
        let hash = digest::digest(&digest::SHA256, secret_plain.as_bytes());
        let secret_enc = EncValue::encrypt(hash.as_ref())?.into_bytes().to_vec();

        // re-encrypt access rights with possibly new active key as well
        let access_bytes = bincode::serialize(&api_key.access)?;
        let access_enc = EncValue::encrypt(&access_bytes)?.into_bytes().to_vec();

        let enc_key_active = &EncKeys::get_static().enc_key_active;

        query!(
            "UPDATE api_keys SET secret = $1, enc_key_id = $2, access = $3 WHERE name = $4",
            secret_enc,
            enc_key_active,
            access_enc,
            name,
        )
        .execute(&data.db)
        .await?;

        Self::cache_invalidate(name).await?;

        let secret_fmt = format!("{}${}", name, secret_plain);
        Ok(secret_fmt)
    }

    /// Updates the API Key. Does NOT update the secret in any way!
    pub async fn update(
        data: &web::Data<AppState>,
        name: &str,
        expires: Option<i64>,
        access: Vec<ApiKeyAccess>,
    ) -> Result<(), ErrorResponse> {
        let entity = ApiKeyEntity::find(data, name).await?;
        let api_key = entity.into_api_key()?;

        let secret_enc = EncValue::encrypt(&api_key.secret)?.into_bytes().to_vec();

        let access_bytes = bincode::serialize(&access)?;
        let access_enc = EncValue::encrypt(&access_bytes)?.into_bytes().to_vec();

        let enc_key_active = &EncKeys::get_static().enc_key_active;

        query!(
            r#"UPDATE
            api_keys set secret = $1, expires = $2, enc_key_id = $3, access = $4
            WHERE name = $5"#,
            secret_enc,
            expires,
            enc_key_active,
            access_enc,
            name,
        )
        .execute(&data.db)
        .await?;

        Self::cache_invalidate(name).await?;

        Ok(())
    }

    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        query!(
            r#"UPDATE api_keys
            SET secret = $1, expires = $2, enc_key_id = $3, access = $4
            WHERE name = $5"#,
            self.secret,
            self.expires,
            self.enc_key_id,
            self.access,
            self.name,
        )
        .execute(&data.db)
        .await?;

        Self::cache_invalidate(&self.name).await?;

        Ok(())
    }
}

impl ApiKeyEntity {
    fn cache_idx(name: &str) -> String {
        format!("api_key_{}", name)
    }

    #[inline]
    async fn cache_invalidate(name: &str) -> Result<(), ErrorResponse> {
        let idx = Self::cache_idx(name);
        DB::client().delete(Cache::App, idx).await?;
        Ok(())
    }

    #[inline(always)]
    pub async fn api_key_from_token_validated(
        data: &web::Data<AppState>,
        token: &str,
    ) -> Result<ApiKey, ErrorResponse> {
        let (name, secret) = token.split_once('$').ok_or_else(|| {
            ErrorResponse::new(ErrorResponseType::BadRequest, "Malformed API-Key")
        })?;

        let client = DB::client();
        let idx = Self::cache_idx(name);
        let api_key = if let Some(key) = client.get(Cache::App, &idx).await? {
            key
        } else {
            let key = Self::find(data, name).await?.into_api_key()?;
            client.put(Cache::App, idx, &key, CACHE_TTL_APP).await?;
            key
        };

        api_key.validate_secret(secret)?;

        Ok(api_key)
    }

    pub fn into_api_key(self) -> Result<ApiKey, ErrorResponse> {
        let secret = EncValue::try_from(self.secret)?.decrypt()?.to_vec();
        let access_dec = EncValue::try_from(self.access)?.decrypt()?.to_vec();
        let access = bincode::deserialize::<Vec<ApiKeyAccess>>(&access_dec)?;

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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub name: String,
    /// SHA256 hashed secret key
    pub secret: Vec<u8>,
    pub created: i64,
    pub expires: Option<i64>,
    pub access: Vec<ApiKeyAccess>,
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
