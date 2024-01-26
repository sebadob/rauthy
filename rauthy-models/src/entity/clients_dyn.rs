use crate::app_state::AppState;
use crate::entity::clients::Client;
use actix_web::web;
use chrono::Utc;
use cryptr::EncValue;
use rauthy_common::error_response::ErrorResponse;
use sqlx::{query, FromRow};

#[derive(Debug, FromRow)]
pub struct ClientDyn {
    pub id: String,
    pub created: i64,
    pub last_used: Option<i64>,
    pub registration_token: Vec<u8>,
}

impl ClientDyn {
    pub async fn create(data: &web::Data<AppState>, id: String) -> Result<String, ErrorResponse> {
        let (secret_plain, registration_token) = Client::generate_new_secret()?;
        let created = Utc::now().timestamp();

        query!(
            "INSERT INTO clients_dyn (id, created, registration_token) VALUES ($1, $2, $3)",
            id,
            created,
            registration_token,
        )
        .execute(&data.db)
        .await?;

        Ok(secret_plain)
    }

    pub async fn update_used(data: &web::Data<AppState>, id: &str) -> Result<(), ErrorResponse> {
        let now = Utc::now().timestamp();

        query!(
            "UPDATE clients_dyn SET last_used = $1 WHERE id = $2",
            now,
            id
        )
        .execute(&data.db)
        .await?;

        Ok(())
    }
}

impl ClientDyn {
    pub fn registration_token_plain(&self) -> Result<String, ErrorResponse> {
        let bytes = EncValue::try_from(self.registration_token.clone())?.decrypt()?;
        Ok(String::from_utf8_lossy(bytes.as_ref()).to_string())
    }
}
