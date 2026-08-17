use crate::database::{Cache, DB};
use rauthy_error::ErrorResponse;

pub struct FailedLoginCounter;

impl FailedLoginCounter {
    // / Decay window for the per-IP failed-login count. hiqlite counters have no TTL, so the / count is stored as a plain T...
    const TTL_SECS: i64 = 3600;

    fn cache_idx(ip: &str) -> String {
        // separate key namespace: `Cache::IpBlacklist` raw-ip keys hold `IpBlacklist` values
        format!("flc_{ip}")
    }

    #[inline]
    fn next_count(current: Option<i64>) -> i64 {
        current.unwrap_or(0) + 1
    }

    pub async fn get(ip: String) -> Result<Option<i64>, ErrorResponse> {
        Ok(DB::hql().get(Cache::App, Self::cache_idx(&ip)).await?)
    }

    pub async fn increase(ip: String) -> Result<i64, ErrorResponse> {
        // Benign read-modify-write: this is a throttle heuristic, not a security or consistency boundary. A lost concurrent upd...
        let current = DB::hql()
            .get::<_, _, i64>(Cache::App, Self::cache_idx(&ip))
            .await?;
        let count = Self::next_count(current);
        DB::hql()
            .put(
                Cache::App,
                Self::cache_idx(&ip),
                &count,
                Some(Self::TTL_SECS),
            )
            .await?;
        Ok(count)
    }

    pub async fn reset(ip: String) -> Result<(), ErrorResponse> {
        DB::hql().delete(Cache::App, Self::cache_idx(&ip)).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::FailedLoginCounter;

    #[test]
    fn test_next_count() {
        assert_eq!(FailedLoginCounter::next_count(None), 1);
        assert_eq!(FailedLoginCounter::next_count(Some(0)), 1);
        assert_eq!(FailedLoginCounter::next_count(Some(5)), 6);
        // realistic upper bound (tiers cap at 25); no overflow in practice
        assert_eq!(FailedLoginCounter::next_count(Some(30)), 31);
    }
}
