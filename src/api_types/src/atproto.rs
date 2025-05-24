use rauthy_common::regex::{RE_ALNUM, RE_CODE_CHALLENGE_METHOD, RE_URI};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct LoginRequest {
    /// Validation:
    /// `^(did:[a-z]+:[a-zA-Z0-9._:%-]*[a-zA-Z0-9._-]|([a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)$`
    pub at_id: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub redirect_uri: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub state: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub nonce: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub code_challenge: Option<String>,
    /// Validation: `plain|S256`
    #[validate(regex(path = "*RE_CODE_CHALLENGE_METHOD", code = "plain|S256"))]
    pub code_challenge_method: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub pkce_challenge: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Validate, ToSchema)]
pub struct CallbackRequest {
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "*RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub state: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub code: String,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "*RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub iss: Option<String>,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "*RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub xsrf_token: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub pkce_verifier: String,
}
