use crate::entity::user_attr::UserAttrValueEntity;
use crate::entity::users::User;
use crate::entity::users_values::UserValues;
use rauthy_error::ErrorResponse;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::collections::HashMap;
use tracing::{debug, error};

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

impl ScimAddress {
    pub fn try_build(values: &UserValues) -> Option<ScimAddress> {
        let mut slf = ScimAddress {
            street_address: None,
            locality: None,
            region: None,
            postal_code: None,
            country: None,
        };

        if let Some(street) = &values.street {
            slf.street_address = Some(street.clone());
        }

        if let Some(zip) = &values.zip {
            slf.postal_code = Some(zip.to_string());

            if let Some(city) = &values.city {
                slf.locality = Some(city.clone());
            }
        }

        if let Some(country) = &values.country {
            slf.country = Some(country.clone());
        }

        if slf.street_address.is_some()
            || slf.locality.is_some()
            || slf.postal_code.is_some()
            || slf.country.is_some()
        {
            Some(slf)
        } else {
            None
        }
    }
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
/// Some implementations like e.g. aws have the following mandatory values:
///   - `user_name`
///   - `name.given_name`
///   - `name.family_name`
///   - `display_name`
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimUser {
    // Note: This is not really optional, but there a terrible client implementations that do
    // not send this field, even when they should.
    pub schemas: Option<Vec<Cow<'static, str>>>,
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
    /// `groups` are read-only and changes MUST be applied via the "Group Resource"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ScimGroupValue>>,
    /// RFC does not specify how this value should look, although it is "expected" to be a String
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<ScimValue>>,
    /// Rauthy custom attributes for the client scope + user attributes combination.
    /// Will be ignored by implementations that don't understand them.-
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom: Option<HashMap<String, serde_json::Value>>,
}

impl Default for ScimUser {
    fn default() -> Self {
        Self {
            schemas: Some(vec!["urn:ietf:params:scim:schemas:core:2.0:User".into()]),
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

impl ScimUser {
    /// Respects the `client_scopes` as much as possible. Some values though are necessary for most
    /// SCIM Service Providers, like `email` and the users' names, even though they should only
    /// be given with the `profile` scope.
    pub async fn from_user_values(
        user: User,
        values: UserValues,
        client_scopes: &str,
    ) -> Result<Self, ErrorResponse> {
        let roles = user
            .roles_iter()
            .map(|r| ScimValue {
                value: r.to_string(),
                display: None,
                primary: None,
            })
            .collect::<Vec<_>>();
        let address = if client_scopes.contains("address") {
            ScimAddress::try_build(&values)
        } else {
            None
        };
        let (picture_uri, preferred_language, locale, timezone) =
            if client_scopes.contains("profile") {
                (
                    user.picture_uri(),
                    Some(user.language.as_str().to_string()),
                    Some(user.language.as_str().to_string()),
                    values.tz,
                )
            } else {
                (None, None, None, None)
            };
        let phone_numbers = if client_scopes.contains("phone") {
            values.phone.map(|no| {
                vec![ScimValue {
                    value: no,
                    display: None,
                    primary: Some(true),
                }]
            })
        } else {
            None
        };

        let custom =
            UserAttrValueEntity::find_key_value_by_scope(client_scopes, user.id.clone()).await?;

        Ok(Self {
            schemas: Some(vec!["urn:ietf:params:scim:schemas:core:2.0:User".into()]),
            // must be None, set by the service provider
            id: None,
            external_id: Some(user.id),
            user_name: user.email.clone(),
            name: Some(ScimName {
                // even though this is optional, we want to have at least an empty string to
                // avoid errors with some service providers like AWS
                family_name: Some(user.family_name.unwrap_or_default()),
                given_name: Some(user.given_name),
            }),
            display_name: Some(user.email.clone()),
            preferred_language,
            locale,
            timezone,
            active: Some(user.enabled),
            emails: Some(vec![ScimValue {
                value: user.email,
                display: None,
                primary: Some(true),
            }]),
            phone_numbers,
            photos: picture_uri.map(|uri| {
                vec![ScimValue {
                    value: uri,
                    display: None,
                    primary: Some(true),
                }]
            }),
            addresses: address.map(|addr| vec![addr]),
            // read-only
            groups: None,
            roles: Some(roles),
            custom,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScimGroup {
    // Note: This is not really optional, but there a terrible client implementations that do
    // not send this field, even when they should.
    pub schemas: Option<Vec<Cow<'static, str>>>,
    /// Managed by the Service Provider
    #[serde(skip_serializing_if = "Option::is_none")]
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
            schemas: Some(vec!["urn:ietf:params:scim:schemas:core:2.0:Group".into()]),
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
    // Note: This is not really optional, but there a terrible client implementations that do
    // not send this field, even when they should.
    pub schemas: Option<Vec<Cow<'static, str>>>,
    pub items_per_page: i64,
    pub total_results: i64,
    pub start_index: Option<i64>,
    #[serde(rename = "Resources")]
    pub resources: Vec<ScimResource>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScimError {
    // Note: This is not really optional, but there a terrible client implementations that do
    // not send this field, even when they should.
    /// `["urn:ietf:params:scim:api:messages:2.0:Error"]`
    pub schemas: Option<Vec<Cow<'static, str>>>,
    pub detail: Option<String>,
    pub status: u16,
}

impl ScimError {
    pub async fn extract_from_res(res: reqwest::Response) -> Self {
        debug!("SCIM request error raw: {res:?}");
        let status = res.status().as_u16();
        // we deserialize the body first because of clients not setting back RFC-valid errors
        let body = res.text().await.unwrap_or_default();
        match serde_json::from_str(&body) {
            Ok(slf) => slf,
            Err(err) => {
                error!(
                    "Error deserializing ScimError from client: {err:?}\nExtracting raw text instead"
                );
                Self {
                    schemas: None,
                    detail: Some(body),
                    status,
                }
            }
        }
    }
}

// Note:
// The Patch Operations are correctly typed, but out-commented on purpose for now.
// As a SCIM client with the current implementation, we will always use 3 different,
// static JSONs that are known in advance and only differ in at most 2 values.
// It is much more efficient to build up these JSONs as Strings manually.
// This is a bit ugly, but reduces the number of memory allocation by at least 75%
// per loop iteration.

// #[derive(Serialize, Debug)]
// pub struct ScimPatchOp {
//     /// `urn:ietf:params:scim:api:messages:2.0:PatchOp`
//     pub schemas: Vec<Cow<'static, str>>,
//     #[serde(rename = "Operations")]
//     pub operations: Vec<ScimPatchOperations>,
// }
//
// impl Default for ScimPatchOp {
//     fn default() -> Self {
//         Self {
//             schemas: vec!["urn:ietf:params:scim:api:messages:2.0:PatchOp".into()],
//             operations: Vec::default(),
//         }
//     }
// }
//
// #[derive(Serialize, Debug)]
// pub struct ScimPatchOpWithPath {
//     /// `urn:ietf:params:scim:api:messages:2.0:PatchOp`
//     pub schemas: Vec<Cow<'static, str>>,
//     #[serde(rename = "Operations")]
//     pub operations: Vec<ScimPatchOperationsWithPath>,
// }
//
// impl Default for ScimPatchOpWithPath {
//     fn default() -> Self {
//         Self {
//             schemas: vec!["urn:ietf:params:scim:api:messages:2.0:PatchOp".into()],
//             operations: Vec::default(),
//         }
//     }
// }
//
// #[derive(Debug, Serialize)]
// #[serde(rename_all = "lowercase")]
// pub enum ScimOp {
//     Add,
//     Remove,
//     Replace,
// }
//
// #[derive(Serialize, Debug)]
// pub struct ScimPatchOperations {
//     pub op: ScimOp,
//     pub value: HashMap<Cow<'static, str>, serde_json::Value>,
// }
//
// #[derive(Serialize, Debug)]
// pub struct ScimPatchOperationsWithPath {
//     pub op: ScimOp,
//     pub path: Cow<'static, str>,
//     pub value: Vec<HashMap<Cow<'static, str>, serde_json::Value>>,
// }
