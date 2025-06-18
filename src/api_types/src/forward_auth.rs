use rauthy_common::regex::RE_ALNUM;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

/// Params to configure the `/clients/{id}/forward_auth` endpoint.
///
/// Default values:
/// - danger_cookie_insecure: false
/// - redirect_state: 401
#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct ForwardAuthParams {
    #[serde(default)]
    pub danger_cookie_insecure: bool,
    #[serde(default = "redirect_state")]
    pub redirect_state: u16,
}

#[derive(Debug, Deserialize, Validate, ToSchema, IntoParams)]
pub struct ForwardAuthCallbackParams {
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "*RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub code: String,
    /// Validation: max length 2048
    #[validate(length(max = 2048))]
    pub state: String,
}

#[inline]
fn redirect_state() -> u16 {
    401
}
