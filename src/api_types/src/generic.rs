use rauthy_common::constants::{RE_ALNUM, RE_SEARCH};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct EncKeyMigrateRequest {
    #[validate(regex(path = "*RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub key_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct I18nRequest {
    pub content: I18nContent,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum I18nContent {
    Account,
    Authorize,
    Device,
    EmailChangeConfirm,
    Error,
    Index,
    Logout,
    PasswordReset,
    Register,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    En,
    De,
    ZhHans,
}

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct PaginationParams {
    pub page_size: Option<u16>,
    pub offset: Option<u16>,
    pub backwards: Option<bool>,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "*RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub continuation_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct PasswordHashTimesRequest {
    #[validate(range(min = 500))]
    pub target_time: u32,
    #[validate(range(min = 32768))]
    pub m_cost: Option<u32>,
    #[validate(range(min = 2))]
    pub p_cost: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct PasswordPolicyRequest {
    /// Validation: `8 <= length_min <= 128`
    #[validate(range(min = 8, max = 128))]
    pub length_min: i32,
    /// Validation: `8 <= length_max <= 128`
    #[validate(range(min = 8, max = 128))]
    pub length_max: i32,
    /// Validation: `1 <= include_lower_case <= 32`
    #[validate(range(min = 1, max = 32))]
    pub include_lower_case: Option<i32>,
    /// Validation: `1 <= include_upper_case <= 32`
    #[validate(range(min = 1, max = 32))]
    pub include_upper_case: Option<i32>,
    /// Validation: `1 <= include_digits <= 32`
    #[validate(range(min = 1, max = 32))]
    pub include_digits: Option<i32>,
    /// Validation: `1 <= include_special <= 32`
    #[validate(range(min = 1, max = 32))]
    pub include_special: Option<i32>,
    /// Validation: `1 <= valid_days <= 3650`
    #[validate(range(min = 1, max = 3650))]
    pub valid_days: Option<i32>,
    /// Validation: `1 <= not_recently_used <= 10`
    #[validate(range(min = 1, max = 10))]
    pub not_recently_used: Option<i32>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct SearchParams {
    /// Data type
    pub ty: SearchParamsType,
    /// Index
    pub idx: SearchParamsIdx,
    /// The actual search query - validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%@]+`
    #[validate(regex(path = "*RE_SEARCH", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%@]+"))]
    pub q: String,
    pub limit: Option<u16>,
}

#[derive(Debug, PartialEq, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum SearchParamsIdx {
    // user params
    Id,
    Email,
    // session params
    UserId,
    SessionId,
    Ip,
}

#[derive(Debug, PartialEq, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum SearchParamsType {
    User,
    Session,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AppVersionResponse {
    pub current: String,
    pub last_check: Option<i64>,
    pub latest: Option<String>,
    pub latest_url: Option<String>,
    pub update_available: bool,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct Argon2ParamsResponse {
    pub m_cost: u32,
    pub t_cost: u32,
    pub p_cost: u32,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct EncKeysResponse<'a> {
    pub active: &'a str,
    pub keys: Vec<&'a str>,
}

#[derive(Debug, Default, Serialize, ToSchema)]
pub struct HealthResponse {
    pub db_healthy: bool,
    pub cache_healthy: bool,
}

#[derive(Debug, Clone, Serialize, ToSchema)]
pub struct LoginTimeResponse {
    pub argon2_params: Argon2ParamsResponse,
    pub login_time: u32,
    pub num_cpus: usize,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct PasswordPolicyResponse {
    pub length_min: i32,
    pub length_max: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_lower_case: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_upper_case: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_digits: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_special: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_days: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_recently_used: Option<i32>,
}
