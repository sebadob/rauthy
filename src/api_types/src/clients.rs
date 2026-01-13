use crate::cust_validation::*;
use crate::oidc::JwkKeyPairAlg;
use rauthy_common::regex::{
    RE_CLIENT_ID_EPHEMERAL, RE_CLIENT_NAME, RE_GROUPS, RE_LOWERCASE, RE_SCOPE_SPACE,
    RE_TOKEN_ENDPOINT_AUTH_METHOD, RE_URI,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

// https://openid.net/specs/openid-connect-registration-1_0.html#ClientMetadata
#[derive(Validate, Deserialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct DynamicClientRequest {
    /// Validation: `Vec<^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]+$>`
    #[validate(custom(function = "validate_vec_uri"))]
    pub redirect_uris: Vec<String>,
    /// Validation: `Vec<^(authorization_code|client_credentials|urn:ietf:params:oauth:grant-type:device_code|password|refresh_token)$>`
    #[validate(custom(function = "validate_vec_grant_types"))]
    pub grant_types: Vec<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "*RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ɏ-\\s]{2,128}"))]
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
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub backchannel_logout_uri: Option<String>,
}

/// This request is used for ephemeral clients, which are needed for Solid OIDC for instance.
#[derive(Serialize, Deserialize, Validate, ToSchema)]
pub struct EphemeralClientRequest {
    /// Validation: `^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,256}$`
    #[validate(regex(
        path = "*RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,256}$"
    ))]
    pub client_id: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "*RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ɏ-\\s]{2,128}"))]
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
    /// Validation: `Vec<^(authorization_code|client_credentials|urn:ietf:params:oauth:grant-type:device_code|password|refresh_token)$>`
    #[validate(custom(function = "validate_vec_grant_type"))]
    pub grant_types: Option<Vec<String>>,
    /// Validation: `60 <= access_token_lifetime <= 86400`
    #[validate(range(min = 60, max = 86400))]
    pub default_max_age: Option<i32>,
    /// Validation: `[a-zA-Z0-9-_/:\s*]{0,512}`
    #[validate(regex(path = "*RE_SCOPE_SPACE", code = "[a-zA-Z0-9-_/:\\s*]{0,512}"))]
    pub scope: Option<String>,
    pub require_auth_time: Option<bool>,

    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    pub access_token_signed_response_alg: Option<JwkKeyPairAlg>,
    /// Validation: `^(RS256|RS384|RS512|EdDSA)$`
    pub id_token_signed_response_alg: Option<JwkKeyPairAlg>,
}

#[derive(Validate, Deserialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct NewClientRequest {
    /// Validation: `^[a-z0-9-_/]{2,128}$`
    #[validate(regex(path = "*RE_LOWERCASE", code = "^[a-z0-9-_/]{2,128}$"))]
    pub id: String,
    /// Validation: None - will not be deserialized
    #[serde(skip_deserializing)]
    pub secret: Option<Vec<u8>>,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "*RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ɏ-\\s]{2,128}"))]
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

#[derive(Deserialize, Validate, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct UpdateClientRequest {
    /// Validation: `^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,256}$`
    #[validate(regex(
        path = "*RE_CLIENT_ID_EPHEMERAL",
        code = "^[a-zA-Z0-9,.:/_\\-&?=~#!$'()*+%]{2,256}$"
    ))]
    pub id: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "*RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ɏ-\\s]{2,128}"))]
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
    /// Validation: `Vec<^(authorization_code|client_credentials|urn:ietf:params:oauth:grant-type:device_code|password|refresh_token)$>`
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
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub backchannel_logout_uri: Option<String>,
    /// Validation: `^[a-zA-Z0-9-_/,:*\\s]{2,64}$`
    #[validate(regex(path = "*RE_GROUPS", code = "^[a-zA-Z0-9-_/,:*\\s]{2,64}$"))]
    pub restrict_group_prefix: Option<String>,
    #[validate(nested)]
    pub scim: Option<ScimClientRequestResponse>,
}

#[derive(Default, Validate, Deserialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Serialize))]
pub struct ClientSecretRequest {
    /// Validation: Value between 1 and 24
    #[validate(range(min = 1, max = 24))]
    pub cache_current_hours: Option<u8>,
}

#[derive(Serialize, Deserialize, ToSchema, Validate)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct ScimClientRequestResponse {
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub bearer_token: String,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%]+$"))]
    pub base_uri: String,
    pub sync_groups: bool,
    /// Validation: `^[a-zA-Z0-9-_/,:*\\s]{2,64}$`
    #[validate(regex(path = "*RE_GROUPS", code = "^[a-zA-Z0-9-_/,:*\\s]{2,64}$"))]
    pub group_sync_prefix: Option<String>,
}

#[derive(Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Debug, Deserialize))]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backchannel_logout_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrict_group_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scim: Option<ScimClientRequestResponse>,
}

#[derive(Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Deserialize))]
pub struct ClientSecretResponse {
    pub id: String,
    pub confidential: bool,
    pub secret: Option<String>,
}

#[derive(PartialEq, Serialize, ToSchema)]
#[cfg_attr(debug_assertions, derive(Debug, Deserialize))]
pub struct DynamicClientResponse {
    pub client_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backchannel_logout_uri: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    // TODO can we "trust" in a client doing a PUT on Self before en expiry to
    //  implement proper forced secret rotation from time to time? -> not mentioned in RFC
    /// Unix timestamp in seconds
    pub client_secret_expires_at: i64,

    pub redirect_uris: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_logout_redirect_uri: Option<String>,

    // only Some(_) after new token has been issued
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
