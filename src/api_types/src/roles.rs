use rauthy_common::regex::RE_ROLES_SCOPES;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(serde::Serialize))]
pub struct RoleRequest {
    /// Validation: `^[a-zA-Z0-9-_/,:*.]{2,64}$`
    #[validate(regex(path = "*RE_ROLES_SCOPES", code = "^[a-zA-Z0-9-_/,:*.]{2,64}$"))]
    pub role: String,
}
