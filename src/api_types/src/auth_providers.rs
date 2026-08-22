use crate::cust_validation::validate_vec_scopes;
use rauthy_common::regex::{
    RE_ALNUM, RE_ATPROTO_HANDLE, RE_CLIENT_ID, RE_CLIENT_NAME, RE_CODE_CHALLENGE, RE_RESOURCE,
    RE_SCOPE_SPACE, RE_URI,
};
use rauthy_derive::FromPgRow;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AuthProviderType {
    Custom,
    GitHub,
    Google,
    OIDC,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct ProviderRequest {
    /// Validation: `[a-zA-Z0-9À-ÿ-\s]{2,128}]`
    #[validate(regex(path = "*RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ɏ-\\s]{2,128}"))]
    pub name: String,
    pub typ: AuthProviderType,
    pub enabled: bool,

    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub issuer: String,
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub authorization_endpoint: String,
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub token_endpoint: String,
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub userinfo_endpoint: String,
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub jwks_endpoint: Option<String>,

    pub use_pkce: bool,
    pub client_secret_basic: bool,
    pub client_secret_post: bool,
    pub auto_onboarding: bool,
    pub auto_link: bool,

    // This validation is pretty loose, but if we make it too strict,
    // we will most probably get into compatibility issues.
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub client_id: String,
    /// Validation: max length is 256
    #[validate(length(max = 256))]
    pub client_secret: Option<String>,
    /// Validation: `[a-zA-Z0-9-_/:\s*]{0,512}`
    #[validate(regex(path = "*RE_SCOPE_SPACE", code = "[a-zA-Z0-9-_/:\\s*]{0,512}"))]
    pub scope: String,

    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub admin_claim_path: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub admin_claim_value: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub mfa_claim_path: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub mfa_claim_value: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct ProviderCallbackRequest {
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub state: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub code: String,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "*RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub xsrf_token: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub pkce_verifier: String,

    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub iss_atproto: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct ProviderLoginRequest {
    // values for the downstream client
    /// Validation: `email`
    #[validate(email)]
    pub email: Option<String>,
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    #[validate(regex(
        path = "*RE_CLIENT_ID",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,128}$"
    ))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub redirect_uri: String,
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_scopes"))]
    pub scopes: Option<Vec<String>>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub state: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub nonce: Option<String>,
    /// Validation: `[a-zA-Z0-9-._~]{43,128}`
    #[validate(regex(path = "*RE_CODE_CHALLENGE", code = "[a-zA-Z0-9-._~]{43,128}"))]
    pub code_challenge: Option<String>,
    /// Validation: `[a-zA-Z0-9]`
    #[validate(regex(path = "*RE_ALNUM", code = "[a-zA-Z0-9]"))]
    pub code_challenge_method: Option<String>,
    /// RFC 8707 resource indicator from the authorization request. Carried
    /// through the upstream-provider round trip and into the auth code, exactly
    /// like `LoginRequest.resource`, so the token request may repeat it.
    ///
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~!$'()*+%@]+$` (no `#`; RFC 8707 forbids a fragment)
    #[validate(regex(path = "*RE_RESOURCE", code = "[a-zA-Z0-9,.:/_-&?=~!$'()*+%@]+$"))]
    pub resource: Option<String>,
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,128}$`
    #[validate(regex(
        path = "*RE_CLIENT_ID",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,128}$"
    ))]
    pub pow: String,

    // values for the callback from upstream
    /// Validation: `[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub provider_id: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub pkce_challenge: String,

    /// Validation:
    /// `^(did:[a-z]+:[a-zA-Z0-9._:%-]*[a-zA-Z0-9._-]|([a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\\.)+[a-zA-Z]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)$`
    #[validate(regex(
        path = "*RE_ATPROTO_HANDLE",
        code = "^(did:[a-z]+:[a-zA-Z0-9._:%-]*[a-zA-Z0-9._-]|([a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\\.)+[a-zA-Z]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)$"
    ))]
    pub handle: Option<String>,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct ProviderLookupRequest {
    /// Validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub issuer: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]"))]
    pub metadata_url: Option<String>,
}

#[derive(Serialize, ToSchema)]
pub struct ProviderResponse {
    pub id: String,
    pub name: String,
    pub typ: AuthProviderType,
    pub enabled: bool,

    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub jwks_endpoint: Option<String>,

    pub client_id: String,
    pub client_secret: Option<String>,
    pub scope: String,

    pub admin_claim_path: Option<String>,
    pub admin_claim_value: Option<String>,
    pub mfa_claim_path: Option<String>,
    pub mfa_claim_value: Option<String>,

    pub use_pkce: bool,
    pub client_secret_basic: bool,
    pub client_secret_post: bool,
    pub auto_onboarding: bool,
    pub auto_link: bool,
}

#[derive(Serialize, Deserialize, FromPgRow, ToSchema)]
pub struct ProviderLinkedUserResponse {
    pub id: String,
    pub email: String,
}

#[derive(Serialize, ToSchema)]
pub struct ProviderLookupResponse {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub jwks_endpoint: Option<String>,
    pub scope: String,
    pub use_pkce: bool,
    pub client_secret_basic: bool,
    pub client_secret_post: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    // Regression tests for RFC 8707 `resource` being dropped on upstream-provider
    // logins (#1702). The provider login request must carry a `resource` just like
    // `LoginRequest` / `LoginRefreshRequest`, so the auth code minted after the
    // upstream round trip is audience-bound and the token request may repeat it.
    #[test]
    fn provider_login_request_carries_valid_resource() {
        let json = r#"{
            "client_id": "my-client",
            "redirect_uri": "https://app.example.com/callback",
            "resource": "https://mcp.example.com/mcp",
            "pow": "abc123",
            "provider_id": "google",
            "pkce_challenge": "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM"
        }"#;

        let req: ProviderLoginRequest =
            serde_json::from_str(json).expect("must deserialize the resource field");
        assert_eq!(req.resource.as_deref(), Some("https://mcp.example.com/mcp"));
        assert!(
            req.validate().is_ok(),
            "a valid absolute-URI resource must pass validation"
        );
    }

    #[test]
    fn provider_login_request_rejects_invalid_resource() {
        let json = r#"{
            "client_id": "my-client",
            "redirect_uri": "https://app.example.com/callback",
            "resource": "https://mcp.example.com/mcp#fragment",
            "pow": "abc123",
            "provider_id": "google",
            "pkce_challenge": "E9Melhoa2OwvFrEMTJguCHaoeK1t8URWbuGJSstw-cM"
        }"#;

        let req: ProviderLoginRequest = serde_json::from_str(json).expect("must deserialize");
        assert!(
            req.validate().is_err(),
            "RFC 8707 forbids a fragment in `resource`; validation must reject it"
        );
    }
}
