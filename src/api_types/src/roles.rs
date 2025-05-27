use rauthy_common::regex::RE_GROUPS_ROLES_SCOPES;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(serde::Serialize))]
pub struct RoleRequest {
    /// Validation: `^[a-z0-9-_/,:*]{2,64}$`
    #[validate(regex(path = "*RE_GROUPS_ROLES_SCOPES", code = "^[a-z0-9-_/,:*]{2,64}$"))]
    pub role: String,
}
