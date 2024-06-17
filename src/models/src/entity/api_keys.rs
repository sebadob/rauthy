use crate::app_state::{AppState, DbPool};
use actix_web::web;
use chrono::Utc;
use cryptr::{EncKeys, EncValue};
use rauthy_common::constants::{API_KEY_LENGTH, CACHE_NAME_12HR};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::get_rand;
use redhac::{cache_del, cache_get, cache_get_from, cache_get_value, cache_put};
use ring::digest;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow};
use utoipa::ToSchema;

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

        Self::cache_invalidate(data, name).await?;
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

        Self::cache_invalidate(data, name).await?;

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

        Self::cache_invalidate(data, name).await?;

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

        Self::cache_invalidate(data, &self.name).await?;

        Ok(())
    }
}

impl ApiKeyEntity {
    fn cache_idx(name: &str) -> String {
        format!("api_key_{}", name)
    }

    async fn cache_invalidate(data: &web::Data<AppState>, name: &str) -> Result<(), ErrorResponse> {
        let idx = Self::cache_idx(name);
        cache_del(
            CACHE_NAME_12HR.to_string(),
            idx,
            &data.caches.ha_cache_config,
        )
        .await?;

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

        let idx = Self::cache_idx(name);
        let api_key = if let Some(key) = cache_get!(
            ApiKey,
            CACHE_NAME_12HR.to_string(),
            idx.clone(),
            &data.caches.ha_cache_config,
            false
        )
        .await?
        {
            key
        } else {
            let key = Self::find(data, name).await?.into_api_key()?;
            cache_put(
                CACHE_NAME_12HR.to_string(),
                idx,
                &data.caches.ha_cache_config,
                &key,
            )
            .await?;
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AccessRights {
    Read,
    Create,
    Update,
    Delete,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
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
