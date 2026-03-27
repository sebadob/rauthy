use rauthy_common::regex::RE_GROUPS;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Deserialize, ToSchema)]
pub struct KVParams {
    pub limit: Option<u32>,
    pub search: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct KVNamespaceRequest {
    /// Validation: `^[a-zA-Z0-9-_/,:*\\s]{2,64}$`
    #[validate(regex(path = "*RE_GROUPS", code = "^[a-zA-Z0-9-_/,:*\\s]{2,64}$"))]
    pub name: String,
    /// If set to `true`, public READ access will be granted to all key-value pairs in this
    /// namespace. List / GET all values is NOT allowed. Only accessing a single key directly  via
    /// `/kv/ns/{ns}/values` is.
    /// This makes it possible to e.g. use random key names as a kind of pre-signed URL, so you
    /// must know the exact key in advance.
    pub public: Option<bool>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct KVAccessRequest {
    pub enabled: bool,
    /// Validation: `^[a-zA-Z0-9-_/,:*\\s]{2,64}$`
    #[validate(regex(path = "*RE_GROUPS", code = "^[a-zA-Z0-9-_/,:*\\s]{2,64}$"))]
    pub name: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema, Validate)]
pub struct KVValueRequest {
    /// Validation: `^[a-zA-Z0-9-_/,:*\\s]{2,64}$`
    #[validate(regex(path = "*RE_GROUPS", code = "^[a-zA-Z0-9-_/,:*\\s]{2,64}$"))]
    pub key: String,
    /// If set to `true`, the backend will encrypt the value on the application layer.
    /// The database will only contain encrypted data. Requires more resources, but is
    /// probably a good idea for very sensitive information.
    pub encrypted: Option<bool>,
    pub value: serde_json::Value,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct KVNamespaceResponse {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct KVAccessResponse {
    pub id: String,
    pub ns: String,
    pub secret: String,
    pub enabled: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct KVAccessTestResponse {
    pub id: String,
    pub ns: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct KVValueResponse {
    pub key: String,
    pub encrypted: Option<bool>,
    pub value: serde_json::Value,
}
