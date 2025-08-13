use crate::sleep_schedule_next;
use rauthy_data::ipgeo::maxmind;
use rauthy_data::rauthy_config::RauthyConfig;
use std::str::FromStr;
use tracing::{error, info};

pub async fn update_ip_geo_db() {
    if RauthyConfig::get().vars.geo.maxmind_account_id.is_none()
        || RauthyConfig::get().vars.geo.maxmind_license_key.is_none()
    {
        info!("No MaxMind IpGeo DB configured - not starting Update Cron Scheduler");
        return;
    }

    let schedule =
        cron::Schedule::from_str(RauthyConfig::get().vars.geo.maxmind_update_cron.as_ref())
            .unwrap();
    loop {
        sleep_schedule_next(&schedule).await;

        if let Err(err) = maxmind::update_db().await {
            error!(?err, "Updating MaxMind IP Geo DB");
        }
    }
}
