//! This file combines all types incl validation that are used for bootstrapping additional data
//! from the `bootstrap.bootstrap_dir`. They are combined here instead of inside their separate
//! modules with the bootstrapping logic to have an easy to use single source of truth when looking
//! up information.
//!
//! All listed types will be expected as a JSON array inside the files. The API request types are
//! specifically NOT re-used here, because the bootstrapping process must accept different data in
//! some places. To keep everything consistent, some types, even though they match the API types,
//! are defined here again on purpose.
//!
//! NOTE: Most database IDs here are all optional. If you provide them, the given, static ID will be
//! used. If it is not given, a random ID will be generated.

use crate::language::Language;
use rauthy_api_types::api_keys::{AccessGroup, AccessRights};
use rauthy_api_types::clients::ScimClientRequestResponse;
use rauthy_api_types::cust_validation::*;
use rauthy_api_types::oidc::JwkKeyPairAlg;
use rauthy_api_types::users::{UserAttrValueRequest, UserValuesRequest};
use rauthy_common::regex::*;
use regex::Regex;
use serde::{Deserialize, Deserializer};
use std::fmt::{Debug, Formatter};
use std::sync::LazyLock;
use validator::{Validate, ValidationError, ValidationErrors};

pub static RE_DB_ID: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9]{24}$").unwrap());

#[derive(Debug, Deserialize, Validate)]
pub struct Group {
    /// Validation: UNIQUE `^[a-zA-Z0-9]{24}$`
    #[validate(regex(path = "*RE_DB_ID", code = "^[a-zA-Z0-9]{24}$"))]
    pub id: Option<String>,
    /// Validation: `^[a-zA-Z0-9-_/,:*\s]{2,64}$`
    #[validate(regex(path = "*RE_GROUPS", code = "^[a-zA-Z0-9-_/,:*\\s]{2,64}$"))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct Role {
    /// Validation: UNIQUE `^[a-zA-Z0-9]{24}$`
    #[validate(regex(path = "*RE_DB_ID", code = "^[a-zA-Z0-9]{24}$"))]
    pub id: Option<String>,
    /// Validation: `^[a-zA-Z0-9-_/,:*\s]{2,64}$`
    #[validate(regex(path = "*RE_GROUPS", code = "^[a-zA-Z0-9-_/,:*\\s]{2,64}$"))]
    pub name: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UserAttribute {
    /// Validation: `^[a-zA-Z0-9-_/]{2,32}$`
    #[validate(regex(path = "*RE_ATTR", code = "^[a-z0-9-_/]{2,32}$"))]
    pub name: String,
    /// Validation: `^[a-zA-Z0-9-_/]{0,128}$`
    #[validate(regex(path = "*RE_ATTR_DESC", code = "[a-zA-Z0-9À-ÿ-\\s]{2,128}"))]
    pub desc: Option<String>,
    pub default_value: Option<serde_json::Value>,
    pub user_editable: Option<bool>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct Scope {
    /// Validation: UNIQUE `^[a-zA-Z0-9]{24}$`
    #[validate(regex(path = "*RE_DB_ID", code = "^[a-zA-Z0-9]{24}$"))]
    pub id: Option<String>,
    /// Validation: `^[a-zA-Z0-9-_/,:*\s]{2,64}$`
    #[validate(regex(path = "*RE_GROUPS", code = "^[a-zA-Z0-9-_/,:*\\s]{2,64}$"))]
    pub name: String,
    /// `[UserAttribute.name]`s that should be included in the `access_token` if this scope is
    /// requested.
    pub attr_include_access: Option<Vec<String>>,
    /// `[UserAttribute.name]`s that should be included in the `id_token` if this scope is
    /// requested.
    pub attr_include_id: Option<Vec<String>>,
    /// If `true`, this scope's mapped custom attributes are emitted at the token root
    /// (flattened) instead of nested under `custom`. Only honored for custom scopes.
    /// Defaults to `false`.
    pub claims_at_root: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct ApiKey {
    /// Validation: `^[a-zA-Z0-9_-/]{2,24}$`
    pub name: String,
    /// Unix timestamp in seconds.
    pub exp: Option<i64>,
    pub secret: ApiKeySecret,
    pub access: Vec<ApiKeyAccess>,
}

impl Validate for ApiKey {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if !RE_API_KEY.is_match(&self.name) {
            let mut err = ValidationError::new("^[a-zA-Z0-9_-/]{2,24}$");
            err.add_param(std::borrow::Cow::from("value"), &self.name);
            errors.add("name", err);
        }

        if let Some(exp) = self.exp
            && exp < 1719784800
        {
            let mut err = ValidationError::new("range");
            err.add_param(std::borrow::Cow::from("min"), &1719784800);
            err.add_param(std::borrow::Cow::from("value"), &exp);
            errors.add("exp", err);
        }

        if self.access.is_empty() {
            let mut err = ValidationError::new("length");
            err.add_param(std::borrow::Cow::from("min"), &1);
            errors.add("access", err);
        } else {
            errors.merge_self("access", self.access.validate());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

pub enum ApiKeySecret {
    // Plain text secret
    Plain(String),
    // Must be encrypted using [cryptr](https://github.com/sebadob/cryptr) with an `ENC_KEY` that
    // the Rauthy instance must have available. The encrypted data is expected as **base64**.
    Encrypted(String),
    // Generate a new API-key token on first bootstrap and store the full
    // `name$secret` token in the encrypted generated-secret container before
    // the DB row is inserted.
    Generate,
}

impl Debug for ApiKeySecret {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("<hidden>")
    }
}

impl<'de> Deserialize<'de> for ApiKeySecret {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_secret(
            deserializer,
            "API-key secret",
            "Encrypted",
            |s| Ok(Self::Plain(s)),
            |s| Ok(Self::Encrypted(s)),
            || Ok(Self::Generate),
        )
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct ApiKeyAccess {
    pub group: AccessGroup,
    #[validate(length(min = 1))]
    pub access_rights: Vec<AccessRights>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct Client {
    /// Validation: `^[a-zA-Z0-9._\-]{2,256}$`
    #[validate(regex(path = "*RE_CLIENT_ID_STRICT", code = "^[a-zA-Z0-9._\\-]{2,256}$"))]
    pub id: String,
    /// Validation: `[a-zA-Z0-9À-ÿ-\\s]{2,128}`
    #[validate(regex(path = "*RE_CLIENT_NAME", code = "[a-zA-Z0-9À-ɏ-\\s]{2,128}"))]
    pub name: Option<String>,
    /// If none given, the client will automatically be public and require S256 PKCE.
    /// The length will be validated against `src/common/src/constants.rs` -> `SECRET_LEN_CLIENTS`,
    /// which is 64 characters at the time of writing. Do **NOT** include special characters to
    /// avoid potential issues in different situations. This is also why it's 64 characters long.
    /// Stick to alphanumeric unless you really have to do something else for a good reason.
    pub secret: Option<ClientSecret>,
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
    /// Must match any `Scope.name` (not the `id`) or OIDC default scopes.
    ///
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_scopes"))]
    pub scopes: Vec<String>,
    /// Must match any `Scope.name` (not the `id`) or OIDC default scopes.
    ///
    /// Validation: `Vec<^[a-z0-9-_/:\s]{0,64}$>`
    #[validate(custom(function = "validate_vec_scopes"))]
    pub default_scopes: Vec<String>,
    /// If no secret is given, `S256` will be enforced here.
    ///
    /// Validation: `Vec<^(plain|S256)$>`
    #[validate(custom(function = "validate_vec_challenge"))]
    pub challenges: Option<Vec<String>>,
    pub force_mfa: bool,
    /// Validation: `[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$`
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub client_uri: Option<String>,
    /// Validation: `Vec<^[a-zA-Z0-9\+.@/-]{0,48}$>`
    #[validate(custom(function = "validate_vec_contact"))]
    pub contacts: Option<Vec<String>>,
    #[validate(regex(path = "*RE_URI", code = "[a-zA-Z0-9,.:/_-&?=~#!$'()*+%@]+$"))]
    pub backchannel_logout_uri: Option<String>,
    /// Restricts the login to this client by membership to a group prefix. The comparison under
    /// the hood will be `group.name.starts_with(client.restrict_group_prefix)`.
    ///
    /// Validation: `^[a-zA-Z0-9-_/,:*\\s]{2,64}$`
    #[validate(regex(path = "*RE_GROUPS", code = "^[a-zA-Z0-9-_/,:*\\s]{2,64}$"))]
    pub restrict_group_prefix: Option<String>,
    #[validate(nested)]
    pub scim: Option<ScimClientRequestResponse>,
}

#[derive(Debug)]
pub enum ClientSecret {
    // Plain text secret
    Plain(String),
    // Must be encrypted using [cryptr](https://github.com/sebadob/cryptr) with an `ENC_KEY` that
    // the Rauthy instance must have available. This is a bit tricky right now. A rough explanation
    // can be found in an old discussion:
    // https://github.com/sebadob/rauthy/discussions/853#discussioncomment-12869750
    // The encrypted data is expected as **base64** (not as HEX as mentioned in the comment). This
    // is the default output format when you encrypt with `cryptr` from the command line.
    //
    // For future versions, the idea is to add CLI capabilities to the `rauthy` binary and provide
    // a much easier way to achieve this.
    Encrypted(String),
    // Generate a new confidential-client secret on first bootstrap and store
    // the cleartext in the encrypted generated-secret container before the DB
    // row is inserted.
    Generate,
}

impl<'de> Deserialize<'de> for ClientSecret {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_secret(
            deserializer,
            "client secret",
            "Encrypted",
            |s| Ok(Self::Plain(s)),
            |s| Ok(Self::Encrypted(s)),
            || Ok(Self::Generate),
        )
    }
}

#[derive(Debug, Deserialize, Validate)]
pub struct User {
    /// Validation: UNIQUE `^[a-zA-Z0-9]{24}$`
    #[validate(regex(path = "*RE_DB_ID", code = "^[a-zA-Z0-9]{24}$"))]
    pub id: Option<String>,
    /// Validation: `email`
    #[validate(email)]
    pub email: String,
    /// Validation: `[user_values.preferred_username] -> regex_rust`
    #[validate(regex(path = "RE_PREFERRED_USERNAME"))]
    pub preferred_username: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-'\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ɏ-'\\s]{1,32}"))]
    pub given_name: Option<String>,
    /// Validation: `[a-zA-Z0-9À-ÿ-'\\s]{1,32}`
    #[validate(regex(path = "*RE_USER_NAME", code = "[a-zA-Z0-9À-ɏ-'\\s]{1,32}"))]
    pub family_name: Option<String>,
    pub language: Option<Language>,
    pub password: UserPassword,
    /// Must match any `Role.name` (not the `id`) or existing default roles:
    /// - `rauthy_admin`
    /// - `admin`
    /// - `user`
    ///
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_roles"))]
    pub roles: Vec<String>,
    /// Must match any `Group.name` (not the `id`) or existing default groups:
    /// - `admin`
    /// - `user`
    ///
    /// Validation: `Vec<^[a-z0-9-_/,:*]{2,64}$>`
    #[validate(custom(function = "validate_vec_groups"))]
    pub groups: Option<Vec<String>>,
    pub enabled: bool,
    pub email_verified: bool,
    /// Unix timestamp in seconds
    #[validate(range(min = 1719784800))]
    pub user_expires: Option<i64>,
    #[validate(nested)]
    pub user_values: Option<UserValuesRequest>,
    pub attributes: Option<Vec<UserAttrValueRequest>>,
}

#[derive(Debug)]
pub enum UserPassword {
    Plain(String),
    Argon2ID(String),
    Generate,
}

impl<'de> Deserialize<'de> for UserPassword {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_secret(
            deserializer,
            "user password",
            "Argon2ID",
            |s| Ok(Self::Plain(s)),
            |s| Ok(Self::Argon2ID(s)),
            || Ok(Self::Generate),
        )
    }
}

fn deserialize_secret<'de, D, T, Plain, Encrypted, Generate>(
    deserializer: D,
    name: &str,
    encrypted_variant: &str,
    plain: Plain,
    encrypted: Encrypted,
    generate: Generate,
) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    Plain: FnOnce(String) -> Result<T, D::Error>,
    Encrypted: FnOnce(String) -> Result<T, D::Error>,
    Generate: FnOnce() -> Result<T, D::Error>,
{
    let value = serde_json::Value::deserialize(deserializer)?;
    match value {
        serde_json::Value::String(s) if s == "generate" => generate(),
        serde_json::Value::String(s) => Err(serde::de::Error::custom(format!(
            "invalid {name} string '{s}', expected 'generate'"
        ))),
        serde_json::Value::Object(mut obj) if obj.len() == 1 => {
            if let Some(v) = obj.remove("Plain") {
                let s = serde_json::from_value(v).map_err(serde::de::Error::custom)?;
                plain(s)
            } else if let Some(v) = obj.remove(encrypted_variant) {
                let s = serde_json::from_value(v).map_err(serde::de::Error::custom)?;
                encrypted(s)
            } else {
                Err(serde::de::Error::custom(format!(
                    "invalid {name} object variant"
                )))
            }
        }
        _ => Err(serde::de::Error::custom(format!(
            "invalid {name}, expected {{\"Plain\": ...}}, {{\"{encrypted_variant}\": ...}}, \
             or \"generate\""
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use validator::Validate;

    #[test]
    fn parses_api_keys_bootstrap_example() {
        let api_keys = serde_json::from_str::<Vec<ApiKey>>(include_str!(
            "../../../../../bootstrap/api_keys.json"
        ))
        .expect("api_keys bootstrap example should parse");

        assert_eq!(api_keys.len(), 1);
        assert_eq!(api_keys[0].name, "bootstrap");
        for api_key in &api_keys {
            api_key
                .validate()
                .expect("api_keys bootstrap example should validate");
        }
    }

    #[test]
    fn parses_generated_api_key_secret_string() {
        let api_key = serde_json::from_str::<ApiKey>(
            r#"{
                "name": "provision",
                "secret": "generate",
                "access": [
                    {
                        "group": "Clients",
                        "access_rights": ["read"]
                    }
                ]
            }"#,
        )
        .unwrap();

        assert!(matches!(api_key.secret, ApiKeySecret::Generate));
        api_key.validate().unwrap();
    }

    #[test]
    fn parses_generated_client_secret_string() {
        let client = serde_json::from_str::<Client>(
            r#"{
                "id": "generated-client",
                "secret": "generate",
                "redirect_uris": ["https://localhost/callback"],
                "enabled": true,
                "flows_enabled": ["authorization_code"],
                "access_token_alg": "EdDSA",
                "id_token_alg": "EdDSA",
                "auth_code_lifetime": 10,
                "access_token_lifetime": 900,
                "scopes": ["openid", "profile"],
                "default_scopes": ["openid"],
                "force_mfa": false
            }"#,
        )
        .unwrap();

        assert!(matches!(client.secret, Some(ClientSecret::Generate)));
    }

    #[test]
    fn absent_or_null_client_secret_stays_public() {
        let without_secret = serde_json::from_str::<Client>(
            r#"{
                "id": "public-client",
                "redirect_uris": ["https://localhost/callback"],
                "enabled": true,
                "flows_enabled": ["authorization_code"],
                "access_token_alg": "EdDSA",
                "id_token_alg": "EdDSA",
                "auth_code_lifetime": 10,
                "access_token_lifetime": 900,
                "scopes": ["openid", "profile"],
                "default_scopes": ["openid"],
                "force_mfa": false
            }"#,
        )
        .unwrap();
        let null_secret = serde_json::from_str::<Client>(
            r#"{
                "id": "public-client-null",
                "secret": null,
                "redirect_uris": ["https://localhost/callback"],
                "enabled": true,
                "flows_enabled": ["authorization_code"],
                "access_token_alg": "EdDSA",
                "id_token_alg": "EdDSA",
                "auth_code_lifetime": 10,
                "access_token_lifetime": 900,
                "scopes": ["openid", "profile"],
                "default_scopes": ["openid"],
                "force_mfa": false
            }"#,
        )
        .unwrap();

        assert!(without_secret.secret.is_none());
        assert!(null_secret.secret.is_none());
    }

    #[test]
    fn parses_generated_user_password_string() {
        let user = serde_json::from_str::<User>(
            r#"{
                "email": "generated@example.com",
                "password": "generate",
                "roles": ["user"],
                "enabled": true,
                "email_verified": true
            }"#,
        )
        .unwrap();

        assert!(matches!(user.password, UserPassword::Generate));
    }
}
