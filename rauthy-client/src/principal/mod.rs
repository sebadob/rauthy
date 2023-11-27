use crate::oidc_config::JwtClaimTyp;
use crate::provider::OidcProvider;
use crate::token_set::JwtAccessClaims;
use std::collections::HashMap;
use std::fmt::Display;

#[cfg(feature = "actix-web")]
mod actix_web;
#[cfg(feature = "axum")]
mod axum;

/// The AuthorizedUser making requests to the API
#[derive(Debug)]
pub struct PrincipalOidc {
    pub id: String,
    pub expires_at_ts: Option<u64>,
    pub preferred_username: Option<String>,
    pub sub: String,
    pub roles: Vec<String>,
    pub groups: Vec<String>,
    pub scope: String,
    pub is_admin: bool,
    pub is_user: bool,
    pub custom_claims: Option<HashMap<String, serde_json::Value>>,
}

impl PrincipalOidc {
    /// Creates a Principal from a raw Base64 encoded JWT token.
    pub async fn from_token_validated(token: &str) -> anyhow::Result<Self> {
        let claims = JwtAccessClaims::from_token_validated(token).await?;

        let config = OidcProvider::config()?;

        let id = claims
            .uid
            .ok_or_else(|| anyhow::Error::msg("'uid' claim is mandatory"))?;
        let sub = claims
            .sub
            .ok_or_else(|| anyhow::Error::msg("'sub' claim is mandatory"))?;
        let roles = claims.roles.unwrap_or_default();
        let groups = claims.groups.unwrap_or_default();

        let is_admin = if let Some(claim) = &config.admin_claim {
            match claim.typ {
                JwtClaimTyp::Roles => roles.contains(&claim.value),
                JwtClaimTyp::Groups => groups.contains(&claim.value),
            }
        } else {
            false
        };
        let is_user = match config.user_claim.typ {
            JwtClaimTyp::Roles => roles.contains(&config.user_claim.value),
            JwtClaimTyp::Groups => groups.contains(&config.user_claim.value),
        };

        Ok(Self {
            id,
            expires_at_ts: claims.expires_at_ts,
            preferred_username: claims.preferred_username,
            sub,
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
            "Id: {}\nSubject: {}\nUsername: {:?}\nRoles: {:?}\nGroups: {:?}\nScope: {}",
            self.id, self.sub, self.preferred_username, self.roles, self.groups, self.scope,
        )
    }
}
