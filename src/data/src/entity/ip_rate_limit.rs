use crate::database::{Cache, DB};
use crate::rauthy_config::RauthyConfig;
use chrono::{DateTime, Utc};
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
        let limit_secs = RauthyConfig::get()
            .vars
            .device_grant
            .rate_limit
            .unwrap_or(1) as i64;
        let limit = Utc::now().add(chrono::Duration::seconds(limit_secs));
        DB::hql()
            .put(Cache::IpRateLimit, ip, &limit, Some(limit_secs))
            .await?;

        Ok(())
    }

    pub async fn is_limited(ip: String) -> Result<Option<DateTime<Utc>>, ErrorResponse> {
        let dt = DB::hql().get(Cache::IpRateLimit, ip).await?;
        Ok(dt)
    }
}
