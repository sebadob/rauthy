use rauthy_common::regex::RE_GROUPS;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(serde::Serialize))]
pub struct GroupRequest {
    /// Validation: `^[a-zA-Z0-9-_/,:*\s]{2,64}$`
    #[validate(regex(path = "*RE_GROUPS", code = "^[a-zA-Z0-9-_/,:*\\s]{2,64}$"))]
    pub group: String,
    pub json_meta: Option<serde_json::Value>,
}

#[derive(Serialize, ToSchema)]
pub struct GroupResponse {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_meta: Option<serde_json::Value>,
}
