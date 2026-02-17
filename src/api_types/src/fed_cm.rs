use rauthy_common::regex::{RE_ALNUM, RE_CLIENT_ID, RE_URI};
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Deserialize, Validate, ToSchema)]
pub struct FedCMAssertionRequest {
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    #[validate(regex(
        path = "*RE_CLIENT_ID",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,128}$"
    ))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub nonce: Option<String>,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "*RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub account_id: String,
    /// Whether the user agent has explicitly shown to the user what specific information the
    /// IDP intends to share with the RP (e.g. "idp.example will share your name, email... with
    /// rp.example"), used by the request permission to sign-up algorithm for new users. It is
    /// used as an assurance by the user agent to the IDP that it has indeed shown the terms of
    /// service and privacy policy to the user in the cases where it is required to do so.
    pub disclosure_text_shown: bool,
}

#[derive(Deserialize, Validate, ToSchema, IntoParams)]
pub struct FedCMClientMetadataRequest {
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    #[validate(regex(
        path = "*RE_CLIENT_ID",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,128}$"
    ))]
    pub client_id: String,
}
