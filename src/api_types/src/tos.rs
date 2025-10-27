use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Deserialize, ToSchema, Validate)]
pub struct ToSRequest {
    pub is_html: bool,
    #[validate(length(min = 1, max = 8192))]
    pub content: String,
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct ToSAcceptRequest {
    pub tos_ts: i64,
    #[validate(length(min = 64, max = 64))]
    pub accept_code: String,
}

#[derive(Serialize, ToSchema)]
pub struct ToSResponse {
    pub ts: i64,
    pub author: String,
    pub is_html: bool,
    pub content: String,
}

#[derive(Serialize, ToSchema)]
pub struct ToSLatestResponse {
    pub ts: i64,
    pub is_html: bool,
    pub content: String,
}

#[derive(Serialize, ToSchema)]
pub struct ToSUserAcceptResponse {
    pub user_id: String,
    pub tos_ts: i64,
    pub accept_ts: i64,
    pub location: String,
}
