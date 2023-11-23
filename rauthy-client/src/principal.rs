use crate::oidc_config::JwtClaimTyp;
use crate::provider::OidcProvider;
use crate::token_set::JwtAccessClaims;
use axum::body::Body;
use axum::extract::FromRequestParts;
use axum::headers::authorization::Bearer;
use axum::headers::Authorization;
use axum::http::request::Parts;
use axum::response::Response;
use axum::{async_trait, TypedHeader};
use std::collections::HashMap;
use std::fmt::Display;

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

    pub fn is_admin(&self) -> Result<(), Response<Body>> {
        if self.is_admin {
            Ok(())
        } else {
            Err(Response::builder()
                .status(403)
                .body(Body::from("Admin access only"))
                .unwrap())
        }
    }

    pub fn is_user(&self) -> Result<(), Response<Body>> {
        if self.is_user {
            Ok(())
        } else {
            Err(Response::builder()
                .status(403)
                .body(Body::from("User access only"))
                .unwrap())
        }
    }

    pub fn has_any_group(&self, group: Vec<&str>) -> Result<(), Response<Body>> {
        for g in &self.groups {
            if group.contains(&g.as_str()) {
                return Ok(());
            }
        }
        Err(Response::builder()
            .status(403)
            .body(Body::from("Groups do not match"))
            .unwrap())
    }

    pub fn has_any_role(&self, roles: Vec<&str>) -> Result<(), Response<Body>> {
        for r in &self.roles {
            if roles.contains(&r.as_str()) {
                return Ok(());
            }
        }
        Err(Response::builder()
            .status(403)
            .body(Body::from("Roles do not match"))
            .unwrap())
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

/// Axum trains implementation to extract the Principal early in the reqeust
#[async_trait]
impl<S> FromRequestParts<S> for PrincipalOidc
where
    S: Send + Sync,
{
    type Rejection = Response<Body>;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) =
            TypedHeader::<Authorization<Bearer>>::from_request_parts(parts, state)
                .await
                .map_err(|_| Response::builder().status(401).body(Body::empty()).unwrap())?;

        match PrincipalOidc::from_token_validated(bearer.token()).await {
            Ok(p) => {
                p.is_user()?;
                Ok(p)
            }
            Err(_err) => Err(Response::builder().status(401).body(Body::empty()).unwrap()),
        }
    }
}
