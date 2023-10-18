use crate::app_state::AppState;
use actix_web::web;
use chrono::Utc;
use rauthy_common::error_response::ErrorResponse;
use rauthy_common::password_hasher::HashPassword;
use rauthy_common::utils::{encrypt, get_rand};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyEntity {
    pub name: String,
    pub secret: String,
    pub is_hash_uptodate: bool,
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
        let created = Utc::now().timestamp();
        let secret_plain = format!("{}${}", name, get_rand(48));
        let secret_hash = HashPassword::hash_password(secret_plain.clone()).await?;
        let access_bytes = bincode::serialize(&access)?;
        let enc_key = data.enc_keys.get(&data.enc_key_active).unwrap();
        let access_enc = encrypt(&access_bytes, enc_key).unwrap();

        query!(
            r#"INSERT INTO
            api_keys (name, secret, created, expires, enc_key_id, access)
            VALUES ($1, $2, $3, $4, $5, $6)"#,
            name,
            secret_hash,
            created,
            expires,
            data.enc_key_active,
            access_enc,
        )
        .execute(&data.db)
        .await?;

        Ok(secret_plain)
    }

    pub async fn find_all(data: &web::Data<AppState>) -> Result<Vec<Self>, ErrorResponse> {
        let res = query_as!(Self, "SELECT * FROM api_keys")
            .fetch_all(&data.db)
            .await?;
        Ok(res)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub name: String,
    pub created: i64,
    pub expires: Option<i64>,
    pub access: Vec<ApiKeyAccess>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiKeyGroup {
    Events,
    Groups,
    Sessions,
    Scopes,
    Users,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiKeyAccessRights {
    Read,
    Create,
    Update,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKeyAccess {
    pub group: ApiKeyGroup,
    pub access_rights: ApiKeyAccessRights,
}
