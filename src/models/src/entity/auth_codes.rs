use crate::app_state::AppState;
use actix_web::web;
use rauthy_common::constants::CACHE_NAME_AUTH_CODES;
use rauthy_common::error_response::ErrorResponse;
use rauthy_common::utils::get_rand;
use redhac::{cache_del, cache_get, cache_get_from, cache_get_value, cache_put};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use time::OffsetDateTime;

// Struct for the codes from the 'authorization_code' flow
#[derive(Debug, Deserialize, Serialize)]
pub struct AuthCode {
    pub id: String,
    pub exp: i64,
    pub client_id: String,
    pub user_id: String,
    pub session_id: Option<String>,
    pub challenge: Option<String>,
    pub challenge_method: Option<String>,
    pub nonce: Option<String>,
    pub scopes: Vec<String>,
}

// CRUD
impl AuthCode {
    // Deletes an Authorization Code from the cache
    pub async fn delete(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        cache_del(
            CACHE_NAME_AUTH_CODES.to_string(),
            self.id.clone(),
            &data.caches.ha_cache_config,
        )
        .await
        .map_err(ErrorResponse::from)
    }

    // Returns an Authorization code from the cache
    pub async fn find(
        data: &web::Data<AppState>,
        id: String,
    ) -> Result<Option<Self>, ErrorResponse> {
        cache_get!(
            AuthCode,
            CACHE_NAME_AUTH_CODES.to_string(),
            id,
            &data.caches.ha_cache_config,
            true
        )
        .await
        .map_err(ErrorResponse::from)
    }

    // Saves an Authorization Code
    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        cache_put(
            CACHE_NAME_AUTH_CODES.to_string(),
            self.id.clone(),
            &data.caches.ha_cache_config,
            self,
        )
        .await?;
        Ok(())
    }
}

impl AuthCode {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        user_id: String,
        client_id: String,
        session_id: Option<String>,
        challenge: Option<String>,
        challenge_method: Option<String>,
        nonce: Option<String>,
        scopes: Vec<String>,
        lifetime_secs: i32,
    ) -> Self {
        let id = get_rand(64);
        let exp = OffsetDateTime::now_utc()
            .add(time::Duration::seconds(lifetime_secs as i64))
            .unix_timestamp();
        Self {
            id,
            exp,
            client_id,
            user_id,
            session_id,
            challenge,
            challenge_method,
            nonce,
            scopes,
        }
    }
}
