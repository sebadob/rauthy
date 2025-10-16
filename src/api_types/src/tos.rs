use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct ToSRequest {
    pub is_html: bool,
    pub content: String,
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
