use rauthy_common::regex::RE_GROUPS_ROLES_SCOPES;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct GroupRequest {
    /// Validation: `^[a-z0-9-_/,:*]{2,64}$`
    #[validate(regex(path = "*RE_GROUPS_ROLES_SCOPES", code = "^[a-z0-9-_/,:*]{2,64}$"))]
    pub group: String,
}
