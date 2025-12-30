use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;

pub static SCIM_SCHEMA_ERROR: &str = "urn:ietf:params:scim:api:messages:2.0:Error";
pub static SCIM_SCHEMA_GROUP: &str = "urn:ietf:params:scim:schemas:core:2.0:Group";
pub static SCIM_SCHEMA_LIST_RESPONSE: &str = "urn:ietf:params:scim:api:messages:2.0:ListResponse";
pub static SCIM_SCHEMA_PATCH_OP: &str = "urn:ietf:params:scim:api:messages:2.0:PatchOp";
pub static SCIM_SCHEMA_USER: &str = "urn:ietf:params:scim:schemas:core:2.0:User";

/// Empty struct, exists only for ease of use in SCIM endpoints for automatic SCIM token validation,
/// if no other `Scim*` type is being used.
#[derive(Debug)]
pub struct ScimToken;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
    /// MUST be in ISO 3166-1 "alpha-2" code format ISO3166; e.g.,
    /// the United States and Sweden are "US" and "SE", respectively.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    // #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    // pub _type: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScimGroupValue {
    /// Our group ID
    pub value: String,
    /// `_ref` MUST be the URI of the corresponding "Group"
    /// resources to which the user belongs, basically the PUT URI.
    // #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    // pub _ref: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // Optional by RFC, but Rauthy requests it
    pub display: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ScimValue {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<bool>,
}

/// A SCIM v2 User - values that Rauthy does not support are not added at all.
/// Some implementations like e.g. aws have the following mandatory values:
///   - `user_name`
///   - `name.given_name`
///   - `name.family_name`
///   - `display_name`
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimUser {
    pub schemas: Vec<Cow<'static, str>>,
    /// Our ID. Will be `None` for `Create` requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Rauthy's User ID
    pub external_id: Option<String>,
    pub user_name: String,
    pub name: Option<ScimName>,
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
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
    /// `groups` are read-only and changes MUST be applied via the "Group Resource".
    /// Any given groups via `/Users` requests MUST be ignored!
    ///
    /// Provide the current assignment when the User is returned from us though.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ScimGroupValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<ScimValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<String, serde_json::Value>>,
}

impl Default for ScimUser {
    fn default() -> Self {
        Self {
            schemas: vec![SCIM_SCHEMA_USER.into()],
            id: None,
            external_id: None,
            user_name: String::default(),
            name: Default::default(),
            display_name: None,
            preferred_language: None,
            locale: None,
            timezone: None,
            active: None,
            emails: None,
            phone_numbers: None,
            photos: None,
            addresses: None,
            groups: None,
            roles: None,
            custom: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimGroup {
    pub schemas: Vec<Cow<'static, str>>,
    /// Our Group ID. Will be `None` for `Create` requests.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Rauthy's Group ID
    pub external_id: Option<String>,
    pub display_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<ScimGroupMember>>,
}

impl Default for ScimGroup {
    fn default() -> Self {
        ScimGroup {
            schemas: vec![SCIM_SCHEMA_GROUP.into()],
            id: None,
            external_id: None,
            display_name: String::default(),
            members: None,
        }
    }
}

/// We only allow Users to be members of groups, and not groups as group members.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimGroupMember {
    /// Must be the local user ID, which returns this resource on `/Users/{id}`
    pub value: String,
    // /// `_ref` MUST be the URI of the corresponding "User"
    // #[serde(rename = "$ref", skip_serializing_if = "Option::is_none")]
    // pub _ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<String>,
}

#[derive(Debug, PartialEq)]
pub enum ScimFilterBy<'a> {
    ExternalId(&'a str),
    UserName(&'a str),
    DisplayName(&'a str),
    None,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimListQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filter: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_index: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<u32>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub attributes: Option<String>,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub excluded_attributes: Option<String>,
}

impl Default for ScimListQuery {
    fn default() -> Self {
        ScimListQuery {
            filter: None,
            start_index: Some(1),
            count: Some(100),
            // attributes: None,
            // excluded_attributes: None,
        }
    }
}

impl ScimListQuery {
    pub fn filter_by(&self) -> ScimFilterBy<'_> {
        if self.filter.is_none() {
            ScimFilterBy::None
        } else {
            let filter = self.filter.as_deref().unwrap_or_default();

            if let Some(v) = filter.strip_prefix("externalId eq \"") {
                ScimFilterBy::ExternalId(&v[..v.len() - 1])
            } else if let Some(v) = filter.strip_prefix("userName eq \"") {
                let stripped = &v[..v.len() - 1];
                ScimFilterBy::UserName(stripped)
            } else if let Some(v) = filter.strip_prefix("displayName eq \"") {
                let stripped = &v[..v.len() - 1];
                ScimFilterBy::DisplayName(stripped)
            } else {
                panic!("invalid filter type: {filter}");
            }
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(untagged)]
pub enum ScimResource {
    User(Box<ScimUser>),
    // Schema(Box<ScimSchema>),
    Group(Box<ScimGroup>),
    // ResourceType(Box<ScimResourceType>),
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimListResponse {
    /// `["urn:ietf:params:scim:api:messages:2.0:ListResponse"]`
    pub schemas: Vec<Cow<'static, str>>,
    pub items_per_page: u32,
    pub total_results: u32,
    pub start_index: u32,
    #[serde(rename = "Resources")]
    pub resources: Vec<ScimResource>,
}

impl Default for ScimListResponse {
    fn default() -> Self {
        Self {
            schemas: vec![SCIM_SCHEMA_LIST_RESPONSE.into()],
            items_per_page: 0,
            total_results: 0,
            start_index: 0,
            resources: vec![],
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ScimError {
    /// `["urn:ietf:params:scim:api:messages:2.0:Error"]`
    pub schemas: Vec<Cow<'static, str>>,
    pub detail: Option<Cow<'static, str>>,
    pub status: u16,
}

impl ScimError {
    pub fn new(status: u16, detail: Option<Cow<'static, str>>) -> Self {
        Self {
            schemas: vec![SCIM_SCHEMA_ERROR.into()],
            detail,
            status,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct ScimPatchOp {
    /// `urn:ietf:params:scim:api:messages:2.0:PatchOp`
    pub schemas: Vec<Cow<'static, str>>,
    #[serde(rename = "Operations")]
    pub operations: Vec<ScimPatchOperations>,
}

impl Default for ScimPatchOp {
    fn default() -> Self {
        Self {
            schemas: vec![SCIM_SCHEMA_PATCH_OP.into()],
            operations: Vec::default(),
        }
    }
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ScimOp {
    Add,
    Remove,
    Replace,
}

#[derive(Debug, Deserialize)]
pub struct ScimPatchOperations {
    pub op: ScimOp,
    pub path: Option<String>,
    pub value: serde_json::Value,
}

impl ScimPatchOperations {
    pub fn try_as_add_member(&self) -> Result<Vec<ScimOpAddMember<'_>>, ScimError> {
        if self.op != ScimOp::Add || self.path.as_deref() != Some("members") {
            return Err(ScimError::new(
                400,
                Some("Invalid input for ScimOp::Add".into()),
            ));
        }

        if let serde_json::Value::Array(arr) = &self.value {
            let mut res = Vec::with_capacity(arr.len());
            for val in arr {
                let Some(uid) = val.get("value") else {
                    return Err(ScimError::new(
                        400,
                        Some("Missing `value` for ScimPatchOp".into()),
                    ));
                };
                let Some(email) = val.get("display") else {
                    return Err(ScimError::new(
                        400,
                        Some("Missing `display` for ScimPatchOp".into()),
                    ));
                };

                res.push(ScimOpAddMember {
                    user_id: uid.as_str().unwrap_or_default(),
                    user_email: email.as_str().unwrap_or_default(),
                })
            }
            return Ok(res);
        }
        Err(ScimError::new(
            400,
            Some("Cannot deserialize ScimPatchOp".into()),
        ))
    }

    pub fn try_as_replace_name(&self) -> Result<ScimOpReplaceName<'_>, ScimError> {
        if self.op != ScimOp::Replace {
            return Err(ScimError::new(
                400,
                Some("Invalid input for ScimOp::Replace".into()),
            ));
        }

        if let serde_json::Value::Object(map) = &self.value {
            let Some(name) = map.get("displayName") else {
                return Err(ScimError::new(
                    400,
                    Some("Missing `displayName` for ScimPatchOp".into()),
                ));
            };
            let Some(ext_id) = map.get("externalId") else {
                return Err(ScimError::new(
                    400,
                    Some("Missing `externalId` for ScimPatchOp".into()),
                ));
            };

            return Ok(ScimOpReplaceName {
                group_name: name.as_str().unwrap_or_default(),
                external_id: ext_id.as_str().unwrap_or_default(),
            });
        }
        Err(ScimError::new(
            400,
            Some("Cannot deserialize ScimPatchOp".into()),
        ))
    }

    pub fn try_as_remove_member(&self) -> Result<Vec<ScimOpRemoveMember<'_>>, ScimError> {
        if self.op != ScimOp::Remove || self.path.as_deref() != Some("members") {
            return Err(ScimError::new(
                400,
                Some("Invalid input for ScimOp::Remove".into()),
            ));
        }

        if let serde_json::Value::Array(arr) = &self.value {
            let mut res = Vec::with_capacity(arr.len());
            for val in arr {
                let Some(uid) = val.get("value") else {
                    return Err(ScimError::new(
                        400,
                        Some("Missing `value` for ScimPatchOp".into()),
                    ));
                };

                res.push(ScimOpRemoveMember {
                    user_id: uid.as_str().unwrap_or_default(),
                })
            }
            return Ok(res);
        }
        Err(ScimError::new(
            400,
            Some("Cannot deserialize ScimPatchOp".into()),
        ))
    }
}

#[derive(Debug, Deserialize)]
pub struct ScimOpAddMember<'a> {
    pub user_id: &'a str,
    pub user_email: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct ScimOpReplaceName<'a> {
    pub group_name: &'a str,
    pub external_id: &'a str,
}

#[derive(Debug, Deserialize)]
pub struct ScimOpRemoveMember<'a> {
    pub user_id: &'a str,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filter_extract() {
        let q = ScimListQuery {
            filter: Some("externalId eq \"id123\"".to_string()),
            start_index: None,
            count: None,
        };
        assert_eq!(q.filter_by(), ScimFilterBy::ExternalId("id123"));

        let q = ScimListQuery {
            filter: Some("userName eq \"Batman\"".to_string()),
            start_index: None,
            count: None,
        };
        assert_eq!(q.filter_by(), ScimFilterBy::UserName("Batman"));

        let q = ScimListQuery {
            filter: Some("displayName eq \"Alfred\"".to_string()),
            start_index: None,
            count: None,
        };
        assert_eq!(q.filter_by(), ScimFilterBy::DisplayName("Alfred"));
    }
}
