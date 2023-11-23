use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// The configuration for the Rauthy setup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RauthyConfig {
    pub admin_claim: Option<JwtClaim>,
    pub user_claim: JwtClaim,
    pub allowed_audiences: HashSet<String>,
    pub client_id: String,
    pub email_verified: bool,
    pub iss: String,
    pub scope: Vec<String>,
    pub secret: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaim {
    pub typ: JwtClaimTyp,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JwtClaimTyp {
    Roles,
    Groups,
}
