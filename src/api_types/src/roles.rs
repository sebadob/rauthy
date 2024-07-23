use rauthy_common::constants::RE_GROUPS;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct NewRoleRequest {
    /// Validation: `^[a-z0-9-_/,:*]{2,64}$`
    #[validate(regex(path = "*RE_GROUPS", code = "^[a-z0-9-_/,:*]{2,64}$"))]
    pub role: String,
}
