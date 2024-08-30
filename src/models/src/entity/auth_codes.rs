use crate::cache::{Cache, DB};
use rauthy_common::constants::CACHE_TTL_AUTH_CODE;
use rauthy_common::utils::get_rand;
use rauthy_error::ErrorResponse;
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
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::client()
            .delete(Cache::AuthCode, self.id.clone())
            .await?;
        Ok(())
    }

    // Returns an Authorization code from the cache
    pub async fn find(id: String) -> Result<Option<Self>, ErrorResponse> {
        Ok(DB::client().get(Cache::AuthCode, id).await?)
    }

    // Saves an Authorization Code
    pub async fn save(&self) -> Result<(), ErrorResponse> {
        DB::client()
            .put(Cache::AuthCode, self.id.clone(), self, *CACHE_TTL_AUTH_CODE)
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
