use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// The configuration for the Rauthy setup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RauthyConfig {
    /// If this is Some(_), the principal will have a .is_admin field being set correctly, if
    /// this claim matches.
    pub admin_claim: Option<JwtClaim>,
    /// This claim must always exist for every single user. Without this claim, a user would
    /// not have access to this app. This is used, because usually you never want to just have
    /// all your OIDC users to have access to a certain application.
    pub user_claim: JwtClaim,
    /// In almost all cases, this should just match the `client_id`
    pub allowed_audiences: HashSet<String>,
    /// the `client_id` from Rauthy
    pub client_id: String,
    /// If set to 'false', tokens with a non-verified email address will be rejected.
    pub email_verified: bool,
    /// The issuer URL from your Rauthy deployment
    pub iss: String,
    /// The scopes you want to request. The only mandatory which always needs to exist is
    /// `openid`, the rest is optional and depending on your needs.
    pub scope: Vec<String>,
    /// If set to None, the client will be treated as a public client and not provide any
    /// secret to the /token endpoint after the callback. Set a secret for confidential clients.
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
