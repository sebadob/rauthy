// Copyright 2025 Sebastian Dobe <sebastiandobe@mailbox.org>

#![forbid(unsafe_code)]

use actix_web::HttpRequest;
use rauthy_common::utils::real_ip_from_req;
use rauthy_error::ErrorResponse;
use rauthy_models::rauthy_config::RauthyConfig;
use std::fmt::{Display, Formatter};

mod maxmind;

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

pub async fn init() {
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
pub fn get_location(req: &HttpRequest) -> Result<Option<String>, ErrorResponse> {
    if let Some(val) = &RauthyConfig::get().vars.geo.country_header
        && let Some(value) = req.headers().get(val)
    {
        let s = value.to_str().unwrap_or_default();
        if !s.is_empty() {
            return Ok(Some(s.to_string()));
        }
    }

    if maxmind::is_configured() {
        let ip = real_ip_from_req(req)?;
        let data = maxmind::get_location(ip)?;
        return Ok(data.map(|d| d.to_string()));
    }

    Ok(None)
}
