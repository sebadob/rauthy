use rauthy_common::regex::RE_ROLES_SCOPES;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(serde::Serialize))]
pub struct RoleRequest {
    /// Validation: `^[a-zA-Z0-9-_/,:*.]{2,64}$`
    #[validate(regex(path = "*RE_ROLES_SCOPES", code = "^[a-zA-Z0-9-_/,:*.]{2,64}$"))]
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
