use crate::database::{Cache, DB};
use crate::entity::ip_blacklist::IpBlacklist;
use crate::events::event::Event;
use crate::rauthy_config::RauthyConfig;
use rauthy_common::sha256;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
use std::net::IpAddr;
use tracing::error;

#[derive(Debug, Serialize, Deserialize)]
pub struct CredStuffDetect;

impl CredStuffDetect {
    pub async fn trigger(ip: IpAddr, email: &str, password: Option<&str>) {
        tracing::warn!(
            "Credential stuffing attempt detected for IP: {:?} - {:?}",
            ip,
            password
        );
        if let Err(err) = Self::handle_trigger(ip, email, password.unwrap_or_default()).await {
            error!(
                "Error during credential stuffing detection for IP {}: {}",
                ip, err
            );
        }
    }

    #[inline]
    async fn handle_trigger(ip: IpAddr, email: &str, password: &str) -> Result<(), ErrorResponse> {
        let hash = sha256!(format!("{email}/{password}").as_bytes()).to_vec();
        let ip = ip.to_string();

        let mut hashes = DB::hql()
            .get::<_, _, BTreeSet<Vec<u8>>>(Cache::CredStuffDetect, ip.clone())
            .await?
            .unwrap_or_default();

        tracing::warn!("Hashes for CredStuff {} -> {:?}", ip, hashes);

        let vars = &RauthyConfig::get().vars.cred_stuff_detect;

        hashes.insert(hash);
        if hashes.len() >= vars.blacklist_threshold as usize {
            Event::cred_stuff(ip.clone()).send().await?;
            IpBlacklist::put(ip.clone(), vars.blacklist_duration as i64).await?;
        }

        DB::hql()
            .put(
                Cache::CredStuffDetect,
                ip,
                &hashes,
                Some(vars.scan_window as i64),
            )
            .await?;

        Ok(())
    }
}
