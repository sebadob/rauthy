use crate::database::{Cache, DB};
use chrono::{DateTime, Utc};
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ops::Add;

#[derive(Debug, Serialize, Deserialize)]
pub struct IpBlacklist {
    pub ip: String,
    pub exp: DateTime<Utc>,
}

impl IpBlacklist {
    pub async fn put(ip: String, ttl_seconds: i64) -> Result<(), ErrorResponse> {
        let exp = Utc::now().add(chrono::Duration::seconds(ttl_seconds));
        let slf = Self {
            ip: ip.clone(),
            exp,
        };

        // make sure to reset TTL properly
        DB::hql().delete(Cache::IpBlacklist, ip.clone()).await?;
        DB::hql()
            .put(Cache::IpBlacklist, ip, &slf, Some(ttl_seconds))
            .await?;

        Ok(())
    }

    pub async fn get(ip: String) -> Result<Option<Self>, ErrorResponse> {
        Ok(DB::hql().get(Cache::IpBlacklist, ip).await?)
    }

    pub async fn get_all() -> Result<BTreeMap<String, Self>, ErrorResponse> {
        Ok(DB::hql().get_snapshot(Cache::IpBlacklist).await?)
    }

    pub async fn delete(ip: String) -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::IpBlacklist, ip).await?;
        Ok(())
    }
}
