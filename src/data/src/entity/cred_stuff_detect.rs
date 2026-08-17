use crate::database::{Cache, DB};
use crate::entity::ip_blacklist::IpBlacklist;
use crate::events::event::Event;
use crate::rauthy_config::RauthyConfig;
use rauthy_common::sha256;
use rauthy_common::utils::base64_url_no_pad_encode;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::net::IpAddr;
use tracing::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct CredStuffDetect;

impl CredStuffDetect {
    pub async fn trigger(ip: IpAddr, email: &str, password: Option<&str>) {
        if let Err(err) = Self::handle_trigger(ip, email, password.unwrap_or_default()).await {
            error!(
                "Error during credential stuffing detection for IP {}: {}",
                ip, err
            );
        }
    }

    #[inline]
    async fn handle_trigger(ip: IpAddr, email: &str, password: &str) -> Result<(), ErrorResponse> {
        // Each distinct (email, password) pair is stored under its own cache key with a scan-window TTL. This makes the countin...
        let hash = sha256!(format!("{email}/{password}").as_bytes()).to_vec();
        let ip = ip.to_string();
        let key = format!("{ip}#{}", base64_url_no_pad_encode(&hash));

        let vars = &RauthyConfig::get().vars.cred_stuff_detect;

        let client = DB::hql();
        client
            .put(
                Cache::CredStuffDetect,
                key,
                &(),
                Some(vars.scan_window as i64),
            )
            .await?;

        // Count all distinct attempts for this IP within the scan window.
        let snapshot = client.get_snapshot::<_, ()>(Cache::CredStuffDetect).await?;
        let attempts = snapshot
            .keys()
            .filter(|k| k.starts_with(&format!("{ip}#")))
            .count();

        if attempts >= vars.blacklist_threshold as usize {
            Event::cred_stuff(ip.clone()).send().await?;
            IpBlacklist::put(ip.clone(), vars.blacklist_duration as i64).await?;
        }

        Ok(())
    }
}
