use crate::rauthy_config::RauthyConfig;
use actix_web::HttpRequest;
use actix_web::http::header::HeaderMap;
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
            write!(f, ", {city}")?;
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

    if let Some(loc) = location_from_header(req.headers()) {
        return Ok(Some(loc));
    }

    if maxmind::is_configured() {
        let data = maxmind::get_location(ip)?;
        return Ok(data.map(|d| d.to_string()));
    }

    Ok(None)
}

/// Returns only the Alpha2-ISO code for the country, or the raw value from the configured
/// `country_header`.
#[inline]
pub fn get_location_alpha2(
    headers: &HeaderMap,
    ip: IpAddr,
) -> Result<Option<String>, ErrorResponse> {
    // TODO maybe check upfront if it's a private IP and skip the lookup?

    if let Some(loc) = location_from_header(headers) {
        return Ok(Some(loc));
    }

    if maxmind::is_configured() {
        let data = maxmind::get_location(ip)?;
        return Ok(data.map(|d| d.alpha_2_code));
    }

    Ok(None)
}

#[inline]
fn location_from_header(headers: &HeaderMap) -> Option<String> {
    let vars = &RauthyConfig::get().vars;

    if !vars.server.proxy_mode {
        return None;
    }

    // Just extracting from the header alone is unsafe, because we don't check if the
    // direct peer IP is a trusted proxy here. However, this function is only called from
    // the other 2 `get_location` fns here, which requires the `IpAddr` to be given as well.
    // This ip is extracted in other parts of the code, but no matter from where it comes,
    // it's always extracted via the `real_ip_from_req()` fns, which validate the trusted proxy,
    // if `proxy_mode` is enabled. This prevents spoofing this header, if an attacker would
    // know it somehow.
    // Doing another trusted proxy check here would be a waste of resources.
    if let Some(val) = &vars.geo.country_header
        && let Some(value) = headers.get(val)
    {
        let s = value.to_str().unwrap_or_default();
        if !s.is_empty() {
            return Some(s.to_string());
        }
    }

    None
}
