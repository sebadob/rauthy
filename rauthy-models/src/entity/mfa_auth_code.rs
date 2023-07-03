use crate::app_state::AppState;
use actix_web::web;
use rauthy_common::constants::CACHE_NAME_MFA_APP_REQ;
use rauthy_common::error_response::ErrorResponse;
use redhac::{cache_get, cache_get_from, cache_get_value, cache_insert, cache_remove, AckLevel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MfaAuthCode {
    pub app_id: String,
    pub email: String,
    /// The code used for the MFA App Req Challenge
    pub code_challenge: String,
    /// The code to authenticate in the Browser to the Listen WebSocket to get the "real" AuthCode
    /// upon successful MFA App confirmation.
    pub code_listen: String,
    pub exp: i64,
    pub header_loc: String,
    pub header_origin: Option<String>,
    pub header_session_csrf: String,
    pub req_id: String,
}

/// CRUD
impl MfaAuthCode {
    /// Returns an MFA Authorization code from the cache
    pub async fn find(
        data: &web::Data<AppState>,
        email: String,
    ) -> Result<Option<Self>, ErrorResponse> {
        cache_get!(
            MfaAuthCode,
            CACHE_NAME_MFA_APP_REQ.to_string(),
            email,
            &data.caches.ha_cache_config,
            true
        )
        .await
        .map_err(ErrorResponse::from)
    }

    /// Deletes an MFA Authorization Code from the cache
    pub async fn delete(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        cache_remove(
            CACHE_NAME_MFA_APP_REQ.to_string(),
            self.email.clone(),
            &data.caches.ha_cache_config,
            AckLevel::Quorum,
        )
        .await
        .map_err(ErrorResponse::from)
    }

    /// Saves an MFA Authorization Code in the cache
    pub async fn save(&self, data: &web::Data<AppState>) -> Result<(), ErrorResponse> {
        cache_insert(
            CACHE_NAME_MFA_APP_REQ.to_string(),
            self.email.clone(),
            &data.caches.ha_cache_config,
            self,
            AckLevel::Quorum,
        )
        .await?;
        Ok(())
    }
}
