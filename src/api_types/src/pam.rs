use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Preflight request for PAM authentications to check, if the given user would be allowed to log
/// in to this client and if so, under which conditions.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PamPreflightRequest {
    pub client_id: String,
    pub client_secret: String,
    pub user_email: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PamLoginRequest {
    pub client_id: String,
    pub client_secret: String,
    pub user_email: String,
    // TODO password or webauthn code may be None, depending on the config later?
    pub user_password: Option<String>,
    pub webauthn_code: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PamPreflightResponse {
    pub login_allowed: bool,
    pub force_mfa: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PamLoginResponse {
    pub can_sudo: bool,
}
