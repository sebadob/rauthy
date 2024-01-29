use crate::app_state::{AppState, DbTxn};
use crate::entity::clients::Client;
use actix_web::web;
use chrono::Utc;
use cryptr::EncValue;
use rauthy_common::constants::CACHE_NAME_12HR;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::cache_entry_client;
use redhac::{cache_get, cache_get_from, cache_get_value, cache_insert, AckLevel};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as, FromRow};

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct ClientDyn {
    pub id: String,
    pub created: i64,
    pub last_used: Option<i64>,
    pub registration_token: Vec<u8>,
    pub token_endpoint_auth_method: String,
}

impl ClientDyn {
    pub async fn create(
        txn: &mut DbTxn<'_>,
        id: String,
        token_endpoint_auth_method: String,
    ) -> Result<Self, ErrorResponse> {
        let (_secret_plain, registration_token) = Client::generate_new_secret()?;
        let created = Utc::now().timestamp();

        query!(
            r#"INSERT INTO
            clients_dyn (id, created, registration_token, token_endpoint_auth_method)
            VALUES ($1, $2, $3, $4)"#,
            id,
            created,
            registration_token,
            token_endpoint_auth_method,
        )
        .execute(&mut **txn)
        .await?;

        Ok(Self {
            id,
            created,
            last_used: None,
            registration_token,
            token_endpoint_auth_method,
        })
    }

    pub async fn find(data: &web::Data<AppState>, id: String) -> Result<Self, ErrorResponse> {
        if let Some(slf) = cache_get!(
            Self,
            CACHE_NAME_12HR.to_string(),
            cache_entry_client(&id),
            &data.caches.ha_cache_config,
            false
        )
        .await?
        {
            return Ok(slf);
        }

        let slf = query_as!(Self, "SELECT * FROM clients_dyn WHERE id = $1", id)
            .fetch_one(&data.db)
            .await?;

        cache_insert(
            CACHE_NAME_12HR.to_string(),
            Client::get_cache_entry(&slf.id),
            &data.caches.ha_cache_config,
            &slf,
            AckLevel::Leader,
        )
        .await?;

        Ok(slf)
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
    pub fn registration_client_uri(data: &web::Data<AppState>, id: &str) -> String {
        format!("{}/clients_dyn/{}", data.issuer, id)
    }

    pub fn registration_token_plain(&self) -> Result<String, ErrorResponse> {
        let bytes = EncValue::try_from(self.registration_token.clone())?.decrypt()?;
        Ok(String::from_utf8_lossy(bytes.as_ref()).to_string())
    }

    pub fn validate_token(&self, bearer: &str) -> Result<(), ErrorResponse> {
        if self.registration_token_plain()? != bearer {
            Err(ErrorResponse::new(
                ErrorResponseType::WWWAuthenticate(
                    r#"error="invalid_token",
                    error_description="Invalid registration_token"#
                        .to_string(),
                ),
                "Invalid registration_token".to_string(),
            ))
        } else {
            Ok(())
        }
    }
}
