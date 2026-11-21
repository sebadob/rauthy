use chrono::Utc;
use rauthy_common::constants::IDX_LOGIN_TIME;
use rauthy_data::database::{Cache, DB};
use rauthy_data::entity::failed_login_counter::FailedLoginCounter;
use rauthy_data::entity::ip_blacklist::IpBlacklist;
use rauthy_data::events::event::Event;
use rauthy_data::html::templates::TooManyRequestsHtml;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::cmp::{max, min};
use std::net::IpAddr;
use std::ops::{Add, Sub};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tracing::{debug, warn};

/**
Handles the login delay.

With every successful login, a new average login time is calculated for how
long it took for a successful login. If a login failed though, the answer will be delayed by the
current average for a successful login, to prevent things like username enumeration.

With `ip_checks = false`, the per-IP failed login counter is neither read nor touched and the IP is
never blacklisted. The delay then only grows with `user_failed_logins`.
 */
pub async fn handle_login_delay<T>(
    peer_ip: IpAddr,
    start: Duration,
    res: Result<T, ErrorResponse>,
    has_password_been_hashed: bool,
    user_failed_logins: Option<i64>,
    ip_checks: bool,
) -> Result<T, ErrorResponse> {
    let client = DB::hql();
    let success_time: i64 = client
        .get(Cache::App, IDX_LOGIN_TIME)
        .await?
        .unwrap_or(2000);

    let end = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    let delta = end - start;

    match res {
        Ok(resp) => {
            if ip_checks {
                FailedLoginCounter::reset(peer_ip.to_string()).await?;
            }

            if has_password_been_hashed {
                let new_time = (success_time + delta.as_millis() as i64) / 2;
                client
                    .put(Cache::App, IDX_LOGIN_TIME, &new_time, Some(i64::MAX))
                    .await?;
                debug!("New login_success_time: {new_time}");
            }

            Ok(resp)
        }
        Err(err) => {
            let counter = if ip_checks {
                FailedLoginCounter::increase(peer_ip.to_string()).await?
            } else {
                0
            };
            // When we combine the fails per IP with the fails from the user and use the higher
            // value, we not only get the limiting for brute-force from a single IP, but also for
            // multiple small tries from different IPs for the same user / email.
            let combined = max(counter, user_failed_logins.unwrap_or(0));
            let failed_logins = min(combined, u32::MAX as i64) as u32;
            warn!("Failed Logins from {peer_ip}: {failed_logins}");

            RauthyConfig::get()
                .tx_events
                .send_async(Event::invalid_login(failed_logins, peer_ip.to_string()))
                .await
                .unwrap();

            let sleep_time_median = {
                let time_taken = end.sub(start).as_millis() as u64;
                let mut sleep_time_median = 0;
                let su64 = success_time as u64;
                if time_taken < su64 {
                    sleep_time_median = su64 - time_taken;
                }
                sleep_time_median
            };

            let sleep_time = match sleep_time(failed_logins as u64, sleep_time_median, ip_checks) {
                Ok(ms) => ms,
                Err(nbf_seconds) => return build_send_event(&peer_ip, nbf_seconds).await,
            };

            debug!("Failed login - sleeping for {sleep_time} ms now");
            tokio::time::sleep(Duration::from_millis(sleep_time)).await;

            Err(err)
        }
    }
}

/// Returns the delay in ms for `failed_logins`, or `Err(seconds)` to blacklist the IP for. Without
/// `blacklist`, the blacklist steps fall through to the next lower delay step.
fn sleep_time(failed_logins: u64, sleep_time_median: u64, blacklist: bool) -> Result<u64, u64> {
    let ms = match failed_logins {
        // n-th blacklist -> blocks for 24h with each invalid request
        t if t >= 25 && blacklist => return Err(86400),
        t if t > 20 => sleep_time_median + min(t, 24) * 20_000,
        20 if blacklist => return Err(3600),
        t if t > 15 => sleep_time_median + t * 15_000,
        15 if blacklist => return Err(900),
        t if t > 10 => sleep_time_median + t * 10_000,
        10 if blacklist => return Err(600),
        t if t > 7 => sleep_time_median + t * 5_000,
        7 if blacklist => return Err(60),
        t if t >= 5 => sleep_time_median + t * 3_000,
        t if t >= 3 => sleep_time_median + t * 2_000,
        _ => sleep_time_median,
    };
    Ok(ms)
}

async fn build_send_event<T>(peer_ip: &IpAddr, nbf_seconds: u64) -> Result<T, ErrorResponse> {
    let nbf = Duration::from_secs(nbf_seconds);
    let not_before = Utc::now().add(nbf);
    let ts = not_before.timestamp();
    let html = TooManyRequestsHtml::build(peer_ip.to_string(), ts);

    RauthyConfig::get()
        .tx_events
        .send_async(Event::ip_blacklisted(not_before, peer_ip.to_string()))
        .await
        .unwrap();
    IpBlacklist::put(peer_ip.to_string(), nbf).await?;

    Err(ErrorResponse::new(
        ErrorResponseType::TooManyRequests(ts),
        html,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sleep_time_blacklist_steps() {
        for (failed, nbf) in [
            (7, 60),
            (10, 600),
            (15, 900),
            (20, 3600),
            (25, 86400),
            (99, 86400),
        ] {
            assert_eq!(sleep_time(failed, 0, true), Err(nbf));
        }
        assert_eq!(sleep_time(2, 100, true), Ok(100));
        assert_eq!(sleep_time(8, 0, true), Ok(40_000));
    }

    #[test]
    fn sleep_time_without_blacklist() {
        assert_eq!(sleep_time(7, 0, false), Ok(21_000));
        assert_eq!(sleep_time(15, 0, false), Ok(150_000));
        assert_eq!(sleep_time(25, 0, false), Ok(480_000));
        assert_eq!(sleep_time(u32::MAX as u64, 0, false), Ok(480_000));
        for failed in 0..100 {
            assert!(sleep_time(failed, 0, false).is_ok());
        }
    }
}
