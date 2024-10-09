use crate::cust_validation::*;
use crate::oidc::JwkKeyPairAlg;
use css_color::Srgb;
use rauthy_common::constants::{
    RE_CLIENT_ID_EPHEMERAL, RE_CLIENT_NAME, RE_LOWERCASE, RE_SCOPE_SPACE,
    RE_TOKEN_ENDPOINT_AUTH_METHOD, RE_URI,
};
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, Validate, ToSchema)]
pub struct ColorsRequest {
    #[validate(length(min = 2, max = 32))]
    pub act1: String,
    #[validate(length(min = 2, max = 32))]
    pub act1a: String,
    #[validate(length(min = 2, max = 32))]
    pub act2: String,
    #[validate(length(min = 2, max = 32))]
    pub act2a: String,
    #[validate(length(min = 2, max = 32))]
    pub acnt: String,
    #[validate(length(min = 2, max = 32))]
    pub acnta: String,
    #[validate(length(min = 2, max = 32))]
    pub ok: String,
    #[validate(length(min = 2, max = 32))]
    pub err: String,
    #[validate(length(min = 2, max = 32))]
    pub glow: String,
    #[validate(length(min = 2, max = 32))]
    pub gmid: String,
    #[validate(length(min = 2, max = 32))]
    pub ghigh: String,
    #[validate(length(min = 2, max = 32))]
    pub text: String,
    #[validate(length(min = 2, max = 32))]
    pub bg: String,
}

impl ColorsRequest {
    pub fn validate_css(&self) -> Result<(), ErrorResponse> {
        Srgb::from_str(&self.act1)?;
        Srgb::from_str(&self.act1a)?;
        Srgb::from_str(&self.act2)?;
        Srgb::from_str(&self.act2a)?;
        Srgb::from_str(&self.acnt)?;
        Srgb::from_str(&self.acnta)?;
        Srgb::from_str(&self.ok)?;
        Srgb::from_str(&self.err)?;
        Srgb::from_str(&self.glow)?;
        Srgb::from_str(&self.gmid)?;
        Srgb::from_str(&self.ghigh)?;
        Srgb::from_str(&self.text)?;
        Srgb::from_str(&self.bg)?;
        Ok(())
    }
}

// https://openid.net/specs/openid-connect-registration-1_0.html#ClientMetadata
#[derive(Debug, Validate, Serialize, Deserialize, ToSchema)]
pub struct DynamicClientRequest {
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    #[validate(custom(function = "validate_vec_uri"))]
    pub redirect_uris: Vec<String>,
    /// Validation: `Vec<^(authorization_code|client_credentials|password|refresh_token)$>`
    #[validate(custom(function = "validate_vec_grant_types"))]
    pub grant_types: Vec<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "*RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub client_name: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub client_uri: Option<String>,
    /// Validation: `Vec<^[a-zA-Z0-9\+.@/]{0,48}$>`
    #[validate(custom(function = "validate_vec_contact"))]
    pub contacts: Option<Vec<String>>,
    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    pub id_token_signed_response_alg: Option<JwkKeyPairAlg>,
    /// Validation: `^(client_secret_post|client_secret_basic|none)$`
    #[validate(regex(
        path = "*RE_TOKEN_ENDPOINT_AUTH_METHOD",
        code = "client_secret_post|client_secret_basic|none"
    ))]
    pub token_endpoint_auth_method: Option<String>,
    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    pub token_endpoint_auth_signing_alg: Option<JwkKeyPairAlg>,
    // Rauthy will only accept the following defaults
    // `response_type=code`
    // `subject_type=public`
    // `require_auth_time=true` (always added by Rauthy anyway)
    //
    // The following must never be accepted for security reasons,
    // because the registration may be unauthenticated:
    // - logo_uri
    // - client_uri
    // - policy_uri
    // - tos_uri
    //
    // Unsupported values:
    // - application_type (may come in the future)
    // - contacts (may come in the future)
    // - jwks_uri
    // - jwks
    // - sector_identifier_uri
    // - id_token_encrypted_response_alg
    // - id_token_encrypted_response_enc
    // - userinfo_signed_response_alg
    // - userinfo_encrypted_response_alg
    // - userinfo_encrypted_response_enc
    // - request_object_signing_alg
    // - request_object_encryption_alg
    // - request_object_encryption_enc
    // - default_max_age (can be specified during auth init with `max_age`)
    // - default_acr_values
    // - initiate_login_uri
    // - request_uris (may come in the future with `request_uri` during login)
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub post_logout_redirect_uri: Option<String>,
}

/// This request is used for ephemeral clients, which are needed for Solid OIDC for instance.
#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct EphemeralClientRequest {
    /// Validation: `^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,256}$`
    #[validate(regex(
        path = "*RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,256}$"
    ))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "*RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub client_name: Option<String>,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub client_uri: Option<String>,
    /// Validation: `Vec<^[a-zA-Z0-9\+.@/]{0,48}$>`
    #[validate(custom(function = "validate_vec_contact"))]
    pub contacts: Option<Vec<String>>,
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    #[validate(custom(function = "validate_vec_uri"))]
    pub redirect_uris: Vec<String>,
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    #[validate(custom(function = "validate_vec_uri"))]
    pub post_logout_redirect_uris: Option<Vec<String>>,
    /// Validation: `Vec<^(authorization_code|client_credentials|password|refresh_token)$>`
    #[validate(custom(function = "validate_vec_grant_type"))]
    pub grant_types: Option<Vec<String>>,
    /// Validation: `60 <= access_token_lifetime <= 86400`
    #[validate(range(min = 60, max = 86400))]
    pub default_max_age: Option<i32>,
    /// Validation: `[a-z0-9-_/:\s*]{0,512}`
    #[validate(regex(path = "*RE_SCOPE_SPACE", code = "[a-z0-9-_/:\\s*]{0,512}"))]
    pub scope: Option<String>,
    pub require_auth_time: Option<bool>,

    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    pub access_token_signed_response_alg: Option<JwkKeyPairAlg>,
    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    pub id_token_signed_response_alg: Option<JwkKeyPairAlg>,
}

#[derive(Debug, Validate, Serialize, Deserialize, ToSchema)]
pub struct NewClientRequest {
    /// Validation: `^[a-z0-9-_/]{2,128}$`
    #[validate(regex(path = "*RE_LOWERCASE", code = "^[a-z0-9-_/]{2,128}$"))]
    pub id: String,
    /// Validation: None - will not be deserialized
    #[serde(skip_deserializing)]
    pub secret: Option<Vec<u8>>,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "*RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub name: Option<String>,
    /// Validation: bool
    pub confidential: bool,
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    #[validate(custom(function = "validate_vec_uri"))]
    pub redirect_uris: Vec<String>,
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    #[validate(custom(function = "validate_vec_uri"))]
    pub post_logout_redirect_uris: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Validate, ToSchema)]
pub struct UpdateClientRequest {
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,256}$`
    #[validate(regex(
        path = "*RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,256}$"
    ))]
    pub id: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "*RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub name: Option<String>,
    pub confidential: bool,
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    #[validate(custom(function = "validate_vec_uri"))]
    pub redirect_uris: Vec<String>,
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    #[validate(custom(function = "validate_vec_uri"))]
    pub post_logout_redirect_uris: Option<Vec<String>>,
    /// Validation: `Vec<^(http|https)://[a-z0-9.:-]+$>`
    #[validate(custom(function = "validate_vec_origin"))]
    pub allowed_origins: Option<Vec<String>>,
    pub enabled: bool,
    /// Validation: `Vec<^(authorization_code|client_credentials|password|refresh_token)$>`
    #[validate(custom(function = "validate_vec_grant_types"))]
    pub flows_enabled: Vec<String>,
    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    pub access_token_alg: JwkKeyPairAlg,
    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    pub id_token_alg: JwkKeyPairAlg,
    /// Validation: `10 <= auth_code_lifetime <= 300`
    #[validate(range(min = 10, max = 300))]
    pub auth_code_lifetime: i32,
    /// Validation: `10 <= access_token_lifetime <= 86400`
    #[validate(range(min = 10, max = 86400))]
    pub access_token_lifetime: i32,
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_scopes"))]
    pub scopes: Vec<String>,
    /// Validation: `Vec<^[a-z0-9-_/:\s]{0,64}$>`
    #[validate(custom(function = "validate_vec_scopes"))]
    pub default_scopes: Vec<String>,
    /// Validation: `Vec<^(plain|S256)$>`
    #[validate(custom(function = "validate_vec_challenge"))]
    pub challenges: Option<Vec<String>>,
    pub force_mfa: bool,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub client_uri: Option<String>,
    /// Validation: `Vec<^[a-zA-Z0-9\+.@/]{0,48}$>`
    #[validate(custom(function = "validate_vec_contact"))]
    pub contacts: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ClientResponse {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub enabled: bool,
    pub confidential: bool,
    pub redirect_uris: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_logout_redirect_uris: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    pub flows_enabled: Vec<String>,
    pub access_token_alg: JwkKeyPairAlg,
    pub id_token_alg: JwkKeyPairAlg,
    pub auth_code_lifetime: i32,
    pub access_token_lifetime: i32,
    pub scopes: Vec<String>,
    pub default_scopes: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub challenges: Option<Vec<String>>,
    pub force_mfa: bool,
    pub client_uri: Option<String>,
    pub contacts: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct ClientSecretResponse {
    pub id: String,
    pub confidential: bool,
    pub secret: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct DynamicClientResponse {
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    // TODO can we "trust" in a client doing a PUT on Self before en expiry to
    // implement proper forced secret rotation from time to time? -> not mentioned in RFC
    /// Unix timestamp in seconds
    pub client_secret_expires_at: i64,

    pub redirect_uris: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_logout_redirect_uri: Option<String>,

    // only Some(_) after new token has been issued
    // TODO rotate on PUT
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_access_token: Option<String>,
    // This is the uri for PUT requests from Self -> only provide if `registration_access_token`
    // has been updated as well
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_client_uri: Option<String>,

    pub grant_types: Vec<String>,
    pub id_token_signed_response_alg: String,
    pub token_endpoint_auth_method: String,
    pub token_endpoint_auth_signing_alg: String,
}

#[cfg(test)]
mod tests {
    use crate::clients::ColorsRequest;
    use css_color::Srgb;
    use std::str::FromStr;

    #[test]
    pub fn test_css_color_validation() {
        let req = ColorsRequest {
            act1: "#000".to_string(),
            act1a: "#123456".to_string(),
            act2: "rgb(1,2,3)".to_string(),
            act2a: "rgba(123, 223, 121, .3)".to_string(),
            acnt: "hsl(360 100% 50%)".to_string(),
            acnta: "hsl(123 13% 23%)".to_string(),
            ok: "red".to_string(),
            err: "blue".to_string(),
            glow: "green".to_string(),
            gmid: "aliceblue".to_string(),
            ghigh: "antiquewhite".to_string(),
            text: "hsla(30,20%,60%,0.9)".to_string(),
            bg: "rgb(127  ,    211 , 200    )".to_string(),
        };
        assert!(req.validate_css().is_ok());

        assert!(Srgb::from_str("#00").is_err());
        assert!(Srgb::from_str("#12345").is_err());
        assert!(Srgb::from_str("hsl(360 100%)").is_err());
        assert!(Srgb::from_str(" ").is_err());
    }
}
