use rauthy_common::regex::RE_GROUPS;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct EmailJobRequest {
    /// Validation: Unix TS in the future
    pub scheduled: Option<i64>,
    pub filter_type: EmailJobFilterType,
    /// Validation: `^[a-zA-Z0-9-_/,:*\s]{2,64}$`
    #[validate(regex(path = "*RE_GROUPS", code = "^[a-zA-Z0-9-_/,:*\\s]{2,64}$"))]
    pub filter_value: Option<String>,
    pub content_type: EmailContentType,
    #[validate(length(max = 1024))]
    pub subject: String,
    #[validate(length(max = 32768))]
    pub body: String,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum EmailContentType {
    Text,
    Markdown,
    HTML,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum EmailJobFilterType {
    None,
    InGroup,
    NotInGroup,
    HasRole,
    HasNotRole,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EmailJobResponse {
    pub id: i64,
    pub scheduled: Option<i64>,
    pub status: EmailJobStatus,
    pub updated: i64,
    pub last_user_ts: i64,
    pub filter_type: EmailJobFilterType,
    pub filter_value: Option<String>,
    pub content_type: EmailContentType,
    pub subject: String,
    pub body: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub enum EmailJobStatus {
    Open = 0,
    Finished,
    Canceled,
}
