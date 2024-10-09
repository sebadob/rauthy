use rauthy_common::constants::RE_API_KEY;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum AccessGroup {
    Blacklist,
    Clients,
    Events,
    Generic,
    Groups,
    Roles,
    Secrets,
    Sessions,
    Scopes,
    UserAttributes,
    Users,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AccessRights {
    Read,
    Create,
    Update,
    Delete,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiKeyAccess {
    pub group: AccessGroup,
    pub access_rights: Vec<AccessRights>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct ApiKeyRequest {
    /// Validation: `^[a-zA-Z0-9_-/]{2,24}$`
    #[validate(regex(path = "*RE_API_KEY", code = "^[a-zA-Z0-9_-/]{2,24}$"))]
    pub name: String,
    // TODO max validation for inner i64 is broken in the macro in v0.18.1
    // #[validate(range(min = 1719784800, max = 4070905200))]
    /// Unix timestamp in seconds
    #[validate(range(min = 1719784800))]
    pub exp: Option<i64>,
    pub access: Vec<ApiKeyAccess>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiKeysResponse {
    pub keys: Vec<ApiKeyResponse>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ApiKeyResponse {
    pub name: String,
    /// Unix timestamp in seconds
    pub created: i64,
    /// Unix timestamp in seconds
    pub expires: Option<i64>,
    pub access: Vec<ApiKeyAccess>,
}
