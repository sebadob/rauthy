use crate::SessionState;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
pub struct SessionResponse<'a> {
    pub id: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<&'a str>,
    pub is_mfa: bool,
    pub state: SessionState,
    pub exp: i64,
    pub last_seen: i64,
    pub remote_ip: Option<&'a str>,
}
