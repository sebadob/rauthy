use crate::database::{Cache, DB};
use rauthy_error::ErrorResponse;

pub struct FailedLoginCounter;

impl FailedLoginCounter {
    pub async fn get(ip: String) -> Result<Option<i64>, ErrorResponse> {
        Ok(DB::hql().counter_get(Cache::IpBlacklist, ip).await?)
    }

    pub async fn increase(ip: String) -> Result<i64, ErrorResponse> {
        Ok(DB::hql().counter_add(Cache::IpBlacklist, ip, 1).await?)
    }

    pub async fn reset(ip: String) -> Result<(), ErrorResponse> {
        DB::hql().counter_del(Cache::IpBlacklist, ip).await?;
        Ok(())
    }
}
