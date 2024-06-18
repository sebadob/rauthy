use crate::app_state::AppState;
use actix_web::web;
use chrono::{DateTime, Utc};
use rauthy_common::constants::CACHE_NAME_IP_RATE_LIMIT;
use rauthy_common::constants::DEVICE_GRANT_RATE_LIMIT;
use rauthy_error::ErrorResponse;
use redhac::{cache_get, cache_get_from, cache_get_value, cache_insert, AckLevel};
use serde::{Deserialize, Serialize};

/// Caution: The `exp` on this struct does not define the timeout. It is only used
/// to return information back to the limited client when it is allowed to poll again.
/// The timeout is defined via `DEVICE_GRANT_RATE_LIMIT`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceIpRateLimit {
    pub ip: String,
    pub exp: DateTime<Utc>,
}

impl DeviceIpRateLimit {
    pub async fn insert(data: &web::Data<AppState>, ip: String) -> Result<(), ErrorResponse> {
        cache_insert(
            CACHE_NAME_IP_RATE_LIMIT.to_string(),
            ip.clone(),
            &data.caches.ha_cache_config,
            &*DEVICE_GRANT_RATE_LIMIT,
            AckLevel::Quorum,
        )
        .await?;
        Ok(())
    }

    pub async fn is_limited(data: &web::Data<AppState>, ip: String) -> Option<DateTime<Utc>> {
        cache_get!(
            DateTime<Utc>,
            CACHE_NAME_IP_RATE_LIMIT.to_string(),
            ip,
            &data.caches.ha_cache_config,
            true
        )
        .await
        .unwrap_or_default()
    }
}
