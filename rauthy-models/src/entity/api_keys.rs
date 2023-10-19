use crate::app_state::AppState;
use actix_web::web;
use chrono::Utc;
use rauthy_common::constants::{API_KEY_LENGTH, CACHE_NAME_12HR};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::{decrypt, encrypt, get_rand};
use redhac::{cache_get, cache_get_from, cache_get_value, cache_put};
use ring::digest;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use tracing::error;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
        data: &web::Data<AppState>,
        name: String,
        expires: Option<i64>,
        access: Vec<ApiKeyAccess>,
    ) -> Result<String, ErrorResponse> {
        let enc_key = data.enc_keys.get(&data.enc_key_active).unwrap();

        let created = Utc::now().timestamp();
        let secret_plain = get_rand(API_KEY_LENGTH);
        let secret_hash = digest::digest(&digest::SHA256, secret_plain.as_bytes());
        let secret_enc =
            encrypt(secret_hash.as_ref(), enc_key).expect("Encryption Keys not set up correctly");

        let access_bytes = bincode::serialize(&access)?;
        let access_enc =
            encrypt(&access_bytes, enc_key).expect("Encryption Keys not set up correctly");

        query!(
            r#"INSERT INTO
            api_keys (name, secret, created, expires, enc_key_id, access)
            VALUES ($1, $2, $3, $4, $5, $6)"#,
            name,
            secret_enc,
            created,
            expires,
            data.enc_key_active,
            access_enc,
        )
        .execute(&data.db)
        .await?;

        Ok(secret_plain)
    }

    pub async fn delete(data: &web::Data<AppState>, name: &str) -> Result<(), ErrorResponse> {
        query!("DELETE FROM api_keys WHERE name = $1", name)
            .fetch_one(&data.db)
            .await?;
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
        let slf = Self::find(data, name).await?;

        let enc_key = data.enc_keys.get(&data.enc_key_active).unwrap();

        let secret_plain = get_rand(API_KEY_LENGTH);
        let secret_hash = digest::digest(&digest::SHA256, secret_plain.as_bytes());
        let secret_enc =
            encrypt(secret_hash.as_ref(), enc_key).expect("Encryption Keys not set up correctly");

        let access_bytes =
            bincode::deserialize::<Vec<u8>>(&slf.access).expect("deserializing AccessRights");
        let access_enc =
            encrypt(&access_bytes, enc_key).expect("Encryption Keys not set up correctly");

        query!(
            "UPDATE api_keys SET secret = $1, enc_key_id = $2, access = $3 WHERE name = $4",
            secret_enc,
            data.enc_key_active,
            access_enc,
            name,
        )
        .fetch_one(&data.db)
        .await?;

        Ok(secret_plain)
    }
}

impl ApiKeyEntity {
    #[inline(always)]
    pub async fn api_key_from_token_validated(
        data: &web::Data<AppState>,
        token: &str,
    ) -> Result<ApiKey, ErrorResponse> {
        let (name, secret) = token.split_once('$').ok_or_else(|| {
            ErrorResponse::new(
                ErrorResponseType::BadRequest,
                "Malformed API-Key".to_string(),
            )
        })?;

        let idx = format!("api_key_{}", name);
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
            let key = Self::find(data, name).await?.into_api_key(data)?;
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

    pub fn into_api_key(self, data: &web::Data<AppState>) -> Result<ApiKey, ErrorResponse> {
        let enc_key = data.enc_keys.get(&self.enc_key_id).ok_or_else(|| {
            error!("Cannot get encryption key {} from config", self.enc_key_id);
            ErrorResponse::new(
                ErrorResponseType::Internal,
                "Cannot decrypt API-Key".to_string(),
            )
        })?;

        let secret = decrypt(self.secret.as_slice(), enc_key.as_slice())?;

        let access_dec = decrypt(self.access.as_slice(), enc_key.as_slice())?;
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
#[serde(rename_all = "lowercase")]
pub enum AccessGroup {
    Attributes,
    Events,
    Groups,
    Sessions,
    Scopes,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyAccess {
    pub group: AccessGroup,
    pub access_rights: AccessRights,
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
    pub fn has_access(
        &self,
        group: AccessGroup,
        access_rights: AccessRights,
    ) -> Result<(), ErrorResponse> {
        for a in &self.access {
            if a.group == group {
                return if a.access_rights == access_rights {
                    Ok(())
                } else {
                    Err(ErrorResponse::new(
                        ErrorResponseType::Forbidden,
                        "Access denied".to_string(),
                    ))
                };
            }
        }
        Err(ErrorResponse::new(
            ErrorResponseType::Forbidden,
            "Access denied".to_string(),
        ))
    }

    pub fn validate_secret(&self, secret: &str) -> Result<(), ErrorResponse> {
        let hash = digest::digest(&digest::SHA256, secret.as_bytes());
        if hash.as_ref() == self.secret.as_slice() {
            Ok(())
        } else {
            Err(ErrorResponse::new(
                ErrorResponseType::Unauthorized,
                "Invalid API-Key".to_string(),
            ))
        }
    }
}
