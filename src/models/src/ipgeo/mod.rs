use crate::rauthy_config::RauthyConfig;
use actix_web::HttpRequest;
use rauthy_error::ErrorResponse;
use std::fmt::{Display, Formatter};
use std::net::IpAddr;

pub mod maxmind;

#[derive(Debug)]
pub struct LookupResponse {
    pub alpha_2_code: String,
    pub country: String,
    pub city: Option<String>,
}

impl Display for LookupResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.alpha_2_code, self.country)?;

        if let Some(city) = self.city.as_ref() {
            write!(f, ", {}", city)?;
        }

        Ok(())
    }
}

pub async fn init_geo() {
    let geo = &RauthyConfig::get().vars.geo;
    if geo.maxmind_account_id.is_some() && geo.maxmind_license_key.is_some() {
        let acc = geo.maxmind_account_id.clone().unwrap();
        let key = geo.maxmind_license_key.clone().unwrap();

        maxmind::init(geo.maxmind_db_dir.as_ref(), acc, key)
            .await
            .expect("Error initializing Maxmind IP Geolocation Database");
    }
}

#[inline]
pub fn get_location(req: &HttpRequest, ip: IpAddr) -> Result<Option<String>, ErrorResponse> {
    // TODO maybe check upfront if it's a private IP and skip the lookup?

    if let Some(val) = &RauthyConfig::get().vars.geo.country_header
        && let Some(value) = req.headers().get(val)
    {
        let s = value.to_str().unwrap_or_default();
        if !s.is_empty() {
            return Ok(Some(s.to_string()));
        }
    }

    if maxmind::is_configured() {
        let data = maxmind::get_location(ip)?;
        return Ok(data.map(|d| d.to_string()));
    }

    Ok(None)
}
