use crate::database::{Cache, DB};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use chrono::Utc;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::ops::Add;

#[derive(Debug, Serialize, Deserialize)]
pub struct IpBlacklist;
impl IpBlacklist {
    pub async fn put(ip: String, ttl_seconds: i64) -> Result<(), ErrorResponse> {
        let exp = Utc::now().add(chrono::Duration::seconds(ttl_seconds));
        let mut buf = Vec::with_capacity(8);
        buf.write_i64::<BigEndian>(exp.timestamp())?;

        DB::hql()
            .put(Cache::IpBlacklist, ip, &exp, Some(ttl_seconds))
            .await?;

        Ok(())
    }

    /// Returns the timestamp of the expiry
    pub async fn get(ip: String) -> Result<Option<i64>, ErrorResponse> {
        if let Some(bytes) = DB::hql().get_bytes(Cache::IpBlacklist, ip).await? {
            Ok(Some((&bytes[..8]).read_i64::<BigEndian>()?))
        } else {
            Ok(None)
        }
    }

    /// Returns `<ip, expiry ts>`
    pub async fn get_all() -> Result<BTreeMap<String, i64>, ErrorResponse> {
        let mut res = BTreeMap::new();
        let snapshot = DB::hql().get_snapshot(Cache::IpBlacklist).await?;
        for (ip, bytes_exp) in snapshot {
            let exp = (&bytes_exp[..8]).read_i64::<BigEndian>()?;
            res.insert(ip, exp);
        }
        Ok(res)
    }
}
