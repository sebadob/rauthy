use crate::database::{Cache, DB};
use crate::rauthy_config::RauthyConfig;
use chrono::Utc;
use rauthy_common::utils::get_rand;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::fmt::Write;
use std::fmt::{Debug, Formatter};
use std::ops::Add;
use utoipa::ToSchema;

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
    pub async fn save(&self, ttl: i32) -> Result<(), ErrorResponse> {
        DB::hql()
            .put(Cache::AuthCode, self.id.clone(), self, Some(ttl as i64))
            .await?;
        Ok(())
    }
}

impl AuthCode {
    #[inline]
    pub fn build_location_header(
        &self,
        redirect_uri: &str,
        state: Option<&str>,
    ) -> Result<String, ErrorResponse> {
        let append_char = if redirect_uri.contains('?') { '&' } else { '?' };
        let mut loc = format!("{}{}code={}", redirect_uri, append_char, self.id);
        if let Some(state) = state {
            write!(loc, "&state={state}")?;
        };
        Ok(loc)
    }

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
        let exp = Utc::now()
            .add(chrono::Duration::seconds(lifetime_secs as i64))
            .timestamp();
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

    /// CAUTION: DO NOT use this reset in any other case than after accepting updated ToS!
    pub async fn reset_exp(&mut self, auth_code_lifetime: i32) -> Result<(), ErrorResponse> {
        self.exp = Utc::now()
            .add(chrono::Duration::seconds(auth_code_lifetime as i64))
            .timestamp();

        self.save(auth_code_lifetime).await
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, ToSchema)]
pub struct AuthCodeToSAwait {
    pub auth_code: String,
    pub await_code: String,
    pub auth_code_lifetime: i32,
    pub header_loc: String,
    pub header_origin: Option<String>,
    pub needs_user_update: bool,
}

// CRUD
impl AuthCodeToSAwait {
    pub async fn delete(&self) -> Result<(), ErrorResponse> {
        DB::hql()
            .delete(Cache::AuthCode, Self::cache_idx(&self.await_code))
            .await?;
        Ok(())
    }

    pub async fn find(code: &str) -> Result<Option<Self>, ErrorResponse> {
        Ok(DB::hql()
            .get(Cache::AuthCode, Self::cache_idx(code))
            .await?)
    }

    pub async fn save(&self) -> Result<(), ErrorResponse> {
        DB::hql()
            .put(
                Cache::AuthCode,
                Self::cache_idx(&self.await_code),
                &self,
                Some(RauthyConfig::get().vars.tos.accept_timeout as i64),
            )
            .await?;

        Ok(())
    }
}

impl AuthCodeToSAwait {
    #[inline]
    fn cache_idx(await_code: &str) -> String {
        // Note: We don't want to simply index the await
        // codes by id to never have an accidental misuse.
        format!("tos_aw_{await_code}")
    }

    #[inline]
    pub fn generate_code() -> String {
        get_rand(64)
    }
}
