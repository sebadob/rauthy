use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct IpBlacklistRequest {
    /// Validation: Ipv4Addr
    #[schema(value_type = str)]
    pub ip: Ipv4Addr,
    // TODO max validation for inner i64 is broken in the macro in v0.18.1
    // #[validate(range(min = 1719784800, max = 4070905200))]
    /// Unix timestamp in seconds
    #[validate(range(min = 1719784800))]
    pub exp: i64,
}

#[derive(Serialize, ToSchema)]
pub struct BlacklistResponse {
    pub ips: Vec<BlacklistedIp>,
}

#[derive(Serialize, ToSchema)]
pub struct BlacklistedIp {
    pub ip: String,
    /// Unix timestamp in seconds
    pub exp: i64,
}
