use rauthy_common::regex::RE_ROLES;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(serde::Serialize))]
pub struct RoleRequest {
    /// Validation: `^[[\p{L}\p{Mn}\p{Mc}\p{N}\-_/,:*.]--[\x{2139}\x{FE0F}]]{2,64}$`
    #[validate(regex(
        path = "*RE_ROLES",
        code = "^[[\\p{L}\\p{Mn}\\p{Mc}\\p{N}\\-_/,:*.]--[\\x{2139}\\x{FE0F}]]{2,64}$"
    ))]
    pub role: String,
    pub meta: Option<serde_json::Value>,
}

#[derive(Serialize, ToSchema)]
pub struct RoleResponse {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub meta: Option<serde_json::Value>,
}
