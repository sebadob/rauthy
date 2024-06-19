use jwt_simple::prelude::Deserialize;
use serde::Serialize;
use std::fmt::{Display, Formatter};
use utoipa::ToSchema;

pub mod api_keys;
pub mod auth_providers;
pub mod blacklist;
pub mod clients;
mod cust_validation;
pub mod events;
pub mod fed_cm;
pub mod generic;
pub mod groups;
pub mod oidc;
pub mod roles;
pub mod scopes;
pub mod sessions;
pub mod users;

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum AccessGroup {
    Blacklist,
    Clients,
    Events,
    Generic,
    Groups,
    Roles,
    Secrets,
    Sessions,
    Scopes,
    UserAttributes,
    Users,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AccessRights {
    Read,
    Create,
    Update,
    Delete,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct AddressClaim {
    pub formatted: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locality: Option<String>,
    // pub region: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct ApiKeyAccess {
    pub group: AccessGroup,
    pub access_rights: Vec<AccessRights>,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum AuthProviderType {
    Custom,
    Github,
    Google,
    OIDC,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct JktClaim {
    pub jkt: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum JwkKeyPairAlg {
    RS256,
    RS384,
    RS512,
    EdDSA,
}

impl Default for JwkKeyPairAlg {
    fn default() -> Self {
        Self::EdDSA
    }
}

impl Display for JwkKeyPairAlg {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            JwkKeyPairAlg::RS256 => "RS256",
            JwkKeyPairAlg::RS384 => "RS384",
            JwkKeyPairAlg::RS512 => "RS512",
            JwkKeyPairAlg::EdDSA => "EdDSA",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub enum JwkKeyPairType {
    RSA,
    OKP,
}

impl Default for JwkKeyPairType {
    fn default() -> Self {
        Self::OKP
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum Language {
    En,
    De,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum SessionState {
    Open,
    Init,
    Auth,
    LoggedOut,
    Unknown,
}
