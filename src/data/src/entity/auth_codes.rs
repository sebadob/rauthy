use crate::database::{Cache, DB};
use crate::rauthy_config::RauthyConfig;
use rauthy_common::utils::get_rand;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use time::OffsetDateTime;

#[derive(Deserialize, Serialize)]
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

impl Debug for AuthCode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "AuthCode {{ id: {}(...), exp: {}, client_id: {}, user_id: {}, scopes: {:?} }}",
            &self.id[..5],
            self.exp,
            self.client_id,
            self.user_id,
            self.scopes
        )
    }
}

// CRUD
impl AuthCode {
    // Deletes an Authorization Code from the cache
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::AuthCode, self.id.clone()).await?;
        Ok(())
    }

    // Returns an Authorization code from the cache
    pub async fn find(id: String) -> Result<Option<Self>, ErrorResponse> {
        Ok(DB::hql().get(Cache::AuthCode, id).await?)
    }

    // Saves an Authorization Code
    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let ttl = (300 + RauthyConfig::get().vars.webauthn.req_exp) as i64;
        DB::hql()
            .put(Cache::AuthCode, self.id.clone(), self, Some(ttl))
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

#[derive(Deserialize, Serialize)]
pub struct AuthCodeToSAwait {
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
impl AuthCodeToSAwait {
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::hql()
            .delete(Cache::AuthCode, Self::cache_idx(&self.id))
            .await?;
        Ok(())
    }

    pub async fn find(id: &str) -> Result<Option<Self>, ErrorResponse> {
        Ok(DB::hql().get(Cache::AuthCode, Self::cache_idx(id)).await?)
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        let ttl = (300 + RauthyConfig::get().vars.webauthn.req_exp) as i64;
        DB::hql()
            .put(Cache::AuthCode, Self::cache_idx(&self.id), self, Some(ttl))
            .await?;
        Ok(())
    }
}

impl AuthCodeToSAwait {
    #[inline]
    fn cache_idx(id: &str) -> String {
        // Note: We don't want to simply index the await
        // codes by id to never have an accidental misuse.
        format!("await_{id}")
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn new(
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
