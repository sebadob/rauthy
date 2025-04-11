use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimName {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
}

#[derive(Debug, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatted: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    // When specified, the value
    // MUST be in ISO 3166-1 "alpha-2" code format [ISO3166]; e.g.,
    // the United States and Sweden are "US" and "SE", respectively.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct ScimGroupValue {
    // `value` MUST be the `id` from the Service Provider
    pub value: String,
    // `_ref` MUST be the URI of the corresponding "Group"
    // resources to which the user belongs, basically the PUT URI.
    #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    pub _ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

#[derive(Debug, Default, Serialize)]
pub struct ScimValue {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

/// A SCIM v2 User - values that Rauthy does not support are out-commented.
/// Some implementations like e.g. aws have the follwoing mandatory values:
///   - `user_name`
///   - `name.given_name`
///   - `name.family_name`
///   - `display_name`
///
/// Because of this, we set these to be non-Optional values, even if the RFC
/// defines almost everything as optional. Also, to make sure we never forget
/// about if, we define `external_id` as mandatory as well, as it would break
/// some logic if it does not exist.
/// The `id` MUST be handled and set by the Service Provider, so it MUST be
/// `None` for Create requests.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimUser {
    pub schemas: Vec<Cow<'static, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub external_id: String,
    pub user_name: String,
    pub name: ScimName,
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
    // For a Photo, the inner `value` should contain the `picture_uri`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub photos: Option<Vec<ScimValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<ScimAddress>>,
    // `groups` are read-only and changes MUST be applied via the "Group Resource"
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
            external_id: String::default(),
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

#[derive(Debug, Default, Serialize)]
pub struct ScimMember {
    pub value: String,
    #[serde(rename = "$ref")]
    pub _ref: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimGroup {
    pub schemas: Vec<Cow<'static, str>>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<ScimMember>>,
}

impl Default for ScimGroup {
    fn default() -> Self {
        ScimGroup {
            schemas: vec!["urn:ietf:params:scim:schemas:core:2.0:Group".into()],
            id: String::default(),
            external_id: None,
            display_name: String::default(),
            members: None,
        }
    }
}
