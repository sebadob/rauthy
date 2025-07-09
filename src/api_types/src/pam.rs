use crate::users::WebauthnAuthFinishRequest;
use rauthy_common::regex::RE_CLIENT_ID_EPHEMERAL;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

/// Preflight request for PAM authentications to check, if the given user would be allowed to log
/// in to this client and if so, under which conditions.
#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PamPreflightRequest {
    /// Validation: `^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,256}$`
    #[validate(regex(
        path = "*RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,256}$"
    ))]
    pub client_id: String,
    #[validate(length(max = 64))]
    pub client_secret: String,
    /// Validation: `email`
    #[validate(email)]
    pub user_email: String,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PamLoginRequest {
    /// Validation: `^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,256}$`
    #[validate(regex(
        path = "*RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,256}$"
    ))]
    pub client_id: String,
    #[validate(length(max = 64))]
    pub client_secret: String,
    /// Validation: `email`
    #[validate(email)]
    pub user_email: String,
    /// Validation: Applies password policy - max 256 characters
    #[validate(length(max = 256))]
    pub user_password: Option<String>,
    #[validate(length(max = 64))]
    pub webauthn_code: Option<String>,
}

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct PamMfaStartRequest {
    /// Validation: `email`
    #[validate(email)]
    pub user_email: String,
}

#[derive(Deserialize, Validate, ToSchema, Serialize)]
pub struct PamMfaFinishRequest {
    #[validate(length(max = 32))]
    pub user_id: String,
    #[validate(nested)]
    pub data: WebauthnAuthFinishRequest,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PamPreflightResponse {
    pub login_allowed: bool,
    pub mfa_required: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PamLoginResponse {
    pub can_sudo: bool,
}
