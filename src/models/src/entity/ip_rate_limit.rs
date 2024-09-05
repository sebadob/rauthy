use crate::cache::{Cache, DB};
use chrono::{DateTime, Utc};
use rauthy_common::constants::DEVICE_GRANT_RATE_LIMIT;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::ops::Add;

/// Caution: The `exp` on this struct does not define the timeout. It is only used
/// to return information back to the limited client when it is allowed to poll again.
/// The timeout is defined via `DEVICE_GRANT_RATE_LIMIT`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceIpRateLimit {
    pub ip: String,
    pub exp: DateTime<Utc>,
}

impl DeviceIpRateLimit {
    pub async fn insert(ip: String) -> Result<(), ErrorResponse> {
        let limit_secs = DEVICE_GRANT_RATE_LIMIT.unwrap_or(1) as i64;
        let limit = Utc::now().add(chrono::Duration::seconds(limit_secs));
        DB::client()
            .put(Cache::IPRateLimit, ip, &limit, Some(limit_secs))
            .await?;

        Ok(())
    }

    pub async fn is_limited(ip: String) -> Result<Option<DateTime<Utc>>, ErrorResponse> {
        let dt = DB::client().get(Cache::IPRateLimit, ip).await?;
        Ok(dt)
    }
}
