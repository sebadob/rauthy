use crate::provider::OidcProvider;
use crate::rauthy_error::RauthyError;
use crate::token_set::JwtAccessClaims;
use std::collections::HashMap;
use std::fmt::Display;
use std::ops::Deref;

#[cfg(feature = "actix-web")]
mod actix_web;
#[cfg(feature = "axum")]
mod axum;

/// The AuthorizedUser making requests to the API
#[derive(Debug)]
pub struct PrincipalOidc {
    /// Matches the `sub` token claim
    pub id: String,
    /// Matches the `expires_at` token claim -> UNIX timestamp in seconds
    pub expires_at_ts: Option<u64>,
    /// Rauthy always set's the users email as the `preferred_username`
    pub preferred_username: Option<String>,
    /// Matches the `roles` token claim
    pub roles: Vec<String>,
    /// Matches the `groups` token claim
    pub groups: Vec<String>,
    /// Matches the `scope` token claim
    pub scope: String,
    /// Will be true, if the access token matches the `RauthyConfig.admin_claim`
    pub is_admin: bool,
    /// Will be true, if the access token matches the `RauthyConfig.user_claim`
    pub is_user: bool,
    /// Contains all custom scopes that are configured inside Rauthy and are mapped into the
    /// `access_token` for the given user
    pub custom_claims: Option<HashMap<String, serde_json::Value>>,
}

impl PrincipalOidc {
    /// Creates a Principal from a raw Base64 encoded JWT token.
    /// This will also validate the token against the JWK fetched from the issuer.
    pub async fn from_token_validated(token: &str) -> Result<Self, RauthyError> {
        let claims = JwtAccessClaims::from_token_validated(token).await?;

        let config = OidcProvider::config()?;

        let id = claims
            .sub
            .ok_or_else(|| RauthyError::InvalidClaims("'sub' claim is mandatory"))?;
        let roles = claims.roles.unwrap_or_default();
        let groups = claims.groups.unwrap_or_default();

        let is_admin = config.admin_claim.matches(roles.deref(), groups.deref());
        let is_user = config.user_claim.matches(roles.deref(), groups.deref());

        Ok(Self {
            id,
            expires_at_ts: claims.expires_at_ts,
            preferred_username: claims.preferred_username,
            roles,
            groups,
            scope: claims.scope,
            is_admin,
            is_user,
            custom_claims: claims.custom,
        })
    }
}

impl Display for PrincipalOidc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Id: {}\nUsername: {:?}\nRoles: {:?}\nGroups: {:?}\nScope: {}",
            self.id, self.preferred_username, self.roles, self.groups, self.scope,
        )
    }
}
