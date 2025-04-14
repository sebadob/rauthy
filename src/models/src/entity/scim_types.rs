use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimName {
    // Note: We don't even want to send `formatted`, because it's duplicate data and therefore
    // wasted bandwidth. The `formatted` can be built from all other existing values -> pointless!
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub formatted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimAddress {
    // Note: We don't even want to send `formatted`, because it's duplicate data and therefore
    // wasted bandwidth. The `formatted` can be built from all other existing values -> pointless!
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub formatted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// When specified, the value
    /// MUST be in ISO 3166-1 "alpha-2" code format [ISO3166]; e.g.,
    /// the United States and Sweden are "US" and "SE", respectively.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    // #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    // pub _type: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ScimGroupValue {
    /// `value` MUST be the `id` from the Service Provider
    pub value: String,
    /// `_ref` MUST be the URI of the corresponding "Group"
    /// resources to which the user belongs, basically the PUT URI.
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub _ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ScimValue {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

/// A SCIM v2 User - values that Rauthy does not support are not added at all.
/// Some implementations like e.g. aws have the follwoing mandatory values:
///   - `user_name`
///   - `name.given_name`
///   - `name.family_name`
///   - `display_name`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimUser {
    pub schemas: Vec<Cow<'static, str>>,
    /// The `id` MUST be handled and set by the Service Provider, so it MUST be
    /// `None` for Create requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Rauthy's User ID
    pub external_id: Option<String>,
    pub user_name: String,
    /// Optional by RFC by mandatory in a few Service Providers
    pub name: Option<ScimName>,
    /// Optional by RFC by mandatory in a few Service Providers
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<ScimValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_numbers: Option<Vec<ScimValue>>,
    /// For a Photo, the inner `value` should contain the `picture_uri`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photos: Option<Vec<ScimValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<ScimAddress>>,
    /// `groups` are read-only and changes MUST be applied via the "Group Resource"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ScimGroupValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<ScimValue>>,
}

impl Default for ScimUser {
    fn default() -> Self {
        Self {
            schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:User".into()],
            id: None,
            external_id: None,
            user_name: String::default(),
            name: Default::default(),
            display_name: String::default(),
            preferred_language: None,
            locale: None,
            active: None,
            emails: None,
            phone_numbers: None,
            photos: None,
            addresses: None,
            groups: None,
            roles: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimGroup {
    pub schemas: Vec<Cow<'static, str>>,
    /// Managed by the Service Provider
    pub id: Option<String>,
    /// Rauthy's Group ID
    pub external_id: Option<String>,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<ScimGroupValue>>,
}

impl Default for ScimGroup {
    fn default() -> Self {
        ScimGroup {
            schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:Group".into()],
            id: None,
            external_id: None,
            display_name: String::default(),
            members: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimSearchRequest {
    pub schemas: Vec<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    excluded_attributes: Option<Vec<String>>,
    pub filter: String,
    pub start_index: i64,
    pub count: i64,
}

impl Default for ScimSearchRequest {
    fn default() -> Self {
        ScimSearchRequest {
            schemas: vec!["urn:ietf:params:scim:api:messages:2.0:SearchRequest".into()],
            attributes: None,
            excluded_attributes: None,
            filter: String::default(),
            start_index: 1,
            count: 100,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attributes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_attributes: Option<String>,
}

impl Default for ScimListQuery {
    fn default() -> Self {
        ScimListQuery {
            filter: None,
            start_index: Some(1),
            count: Some(100),
            attributes: None,
            excluded_attributes: None,
        }
    }
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct ScimSchema {
//     pub id: String,
//     pub name: String,
//     pub description: String,
//     pub attributes: Vec<Attributes>,
//     pub meta: Meta,
// }

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ScimResource {
    User(Box<ScimUser>),
    // Schema(Box<ScimSchema>),
    Group(Box<ScimGroup>),
    // ResourceType(Box<ScimResourceType>),
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimListResponse {
    /// `["urn:ietf:params:scim:api:messages:2.0:ListResponse"]`
    pub schemas: Vec<Cow<'static, str>>,
    pub items_per_page: i64,
    pub total_results: i64,
    pub start_index: i64,
    #[serde(rename = "Resources")]
    pub resources: Vec<ScimResource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScimError {
    /// `["urn:ietf:params:scim:api:messages:2.0:Error"]`
    pub schemas: Vec<Cow<'static, str>>,
    pub detail: Option<String>,
    pub status: u16,
}

#[derive(Serialize, Debug)]
pub struct ScimPatchOp {
    /// `urn:ietf:params:scim:api:messages:2.0:PatchOp`
    pub schemas: Vec<Cow<'static, str>>,
    #[serde(rename = "Operations")]
    pub operations: Vec<ScimPatchOperations>,
}

impl Default for ScimPatchOp {
    fn default() -> Self {
        Self {
            schemas: vec!["urn:ietf:params:scim:api:messages:2.0:PatchOp".into()],
            operations: Vec::default(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ScimOp {
    Add,
    Remove,
    Replace,
}

#[derive(Serialize, Debug)]
pub struct ScimPatchOperations {
    pub op: ScimOp,
    pub value: HashMap<Cow<'static, str>, serde_json::Value>,
}
