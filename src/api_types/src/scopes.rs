use crate::cust_validation::validate_vec_attr;
use rauthy_common::regex::RE_GROUPS_ROLES_SCOPES;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct ScopeRequest {
    /// Validation: `^[a-z0-9-_/,:*]{2,64}$`
    #[validate(regex(path = "*RE_GROUPS_ROLES_SCOPES", code = "^[a-z0-9-_/,:*]{2,64}$"))]
    pub scope: String,
    /// Validation: `^[a-zA-Z0-9-_/]{2,128}$`
    #[validate(custom(function = "validate_vec_attr"))]
    pub attr_include_access: Option<Vec<String>>,
    /// Validation: `^[a-zA-Z0-9-_/]{2,128}$`
    #[validate(custom(function = "validate_vec_attr"))]
    pub attr_include_id: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ScopeResponse {
    pub id: String,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr_include_access: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attr_include_id: Option<Vec<String>>,
}
