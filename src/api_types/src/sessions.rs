use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum SessionState {
    Open,
    Init,
    Auth,
    LoggedOut,
    Unknown,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionResponse<'a> {
    pub id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<&'a str>,
    pub is_mfa: bool,
    pub state: SessionState,
    /// Unix timestamp in seconds
    pub exp: i64,
    /// Unix timestamp in seconds
    pub last_seen: i64,
    pub remote_ip: Option<&'a str>,
}
