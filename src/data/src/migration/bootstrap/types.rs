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
//! NOTE: The database IDs here are all optional. If you provide them, the given, static ID will be
//! used. If it is not given, a random ID will be generated.

use rauthy_common::regex::{RE_ATTR, RE_ATTR_DESC, RE_GROUPS};
use regex::Regex;
use serde::Deserialize;
use std::sync::LazyLock;
use validator::Validate;

pub static RE_DB_ID: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9]{24}$").unwrap());

// TODO
//  - clients
//  - users

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
}
