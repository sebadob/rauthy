use jwt_simple::prelude::{Deserialize, Serialize};

use std::net::Ipv4Addr;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct IpBlacklistRequest {
    /// Validation: Ipv4Addr
    #[schema(value_type = str)]
    pub ip: Ipv4Addr,
    /// Unix timestamp in seconds in the future (max year 2099)
    #[validate(range(min = 1672527600, max = 4070905200))]
    pub exp: i64,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct BlacklistResponse {
    pub ips: Vec<BlacklistedIp>,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct BlacklistedIp {
    pub ip: String,
    pub exp: i64,
}
