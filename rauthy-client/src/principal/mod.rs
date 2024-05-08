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

#[cfg(feature = "userinfo")]
#[derive(Debug, Clone, serde::Deserialize)]
pub struct AddressClaim {
    pub formatted: String,
    pub street_address: Option<String>,
    pub locality: Option<String>,
    pub postal_code: Option<i32>,
    pub country: Option<String>,
}

#[cfg(feature = "userinfo")]
#[derive(Debug, serde::Deserialize)]
pub struct Userinfo {
    pub id: String,
    pub sub: String,
    pub name: String,
    pub roles: Vec<String>,
    pub mfa_enabled: bool,

    // scope: address
    pub address: Option<AddressClaim>,

    // scope: email
    pub email: Option<String>,
    pub email_verified: Option<bool>,

    // scope: groups
    pub groups: Option<Vec<String>>,

    // scope: profile
    pub preferred_username: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub birthdate: Option<String>,
    pub locale: Option<String>,

    // scope: phone
    pub phone: Option<String>,

    // scope: webid
    pub webid: Option<String>,
}

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
    #[cfg(feature = "userinfo")]
    access_token: Option<String>,
}

impl PrincipalOidc {
    /// Creates a Principal from a raw Base64 encoded JWT token.
    /// This will also validate the token against the JWK fetched from the issuer.
    pub async fn from_token_validated(token: &str) -> Result<Self, RauthyError> {
        let claims = JwtAccessClaims::from_token_validated(token).await?;

        let config = OidcProvider::config()?;

        let id = claims
            .sub
            .ok_or(RauthyError::InvalidClaims("'sub' claim is mandatory"))?;
        let roles = claims.roles.unwrap_or_default();
        let groups = claims.groups.unwrap_or_default();

        let is_admin = config.admin_claim.matches(roles.deref(), groups.deref());
        let is_user = is_admin || config.user_claim.matches(roles.deref(), groups.deref());

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
            #[cfg(feature = "userinfo")]
            access_token: Some(token.to_string()),
        })
    }

    #[cfg(feature = "userinfo")]
    pub async fn fetch_userinfo(&self) -> Result<Userinfo, RauthyError> {
        use crate::provider::{HTTP_CLIENT, OIDC_CONFIG};
        use std::borrow::Cow;

        let token = match &self.access_token {
            None => {
                return Err(RauthyError::Init(
                    "Cannot fetch userinfo when Principal has not been \
                initialized with an access_token",
                ));
            }
            Some(token) => token,
        };

        let url = match OIDC_CONFIG.get() {
            None => {
                return Err(RauthyError::Init(
                    "OidcProvider::setup_from_config has not been called",
                ));
            }
            Some(cfg) => &cfg.provider.userinfo_endpoint,
        };
        let client = match HTTP_CLIENT.get() {
            None => {
                return Err(RauthyError::Init(
                    "OidcProvider::init_client has not been called",
                ));
            }
            Some(c) => c,
        };

        let res = client
            .get(url)
            .header("Authorization", format!("Bearer {}", token))
            .send()
            .await?;
        let status = res.status();
        if status.is_success() {
            let info = res.json::<Userinfo>().await?;
            Ok(info)
        } else {
            let body = res.text().await?;
            let err = format!("{} {}", status, body);
            Err(RauthyError::Token(Cow::from(err)))
        }
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
