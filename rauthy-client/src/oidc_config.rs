use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// The configuration for the Rauthy setup.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RauthyConfig {
    /// Sets the .is_admin field for the principal based on the `ClaimMapping`.
    pub admin_claim: ClaimMapping,
    /// Sets the .is_user field for the principal based on the `ClaimMapping`.
    /// Without this claim, a user would not have access to this app. This is
    /// used, because usually you never want to just have all your OIDC users to
    /// have access to a certain application.
    pub user_claim: ClaimMapping,
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

/// The claim selector which decides how a claim should be evaluated, based on the roles and groups.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClaimMapping {
    /// It doesn't matter which role or group a user have, it returns always `true`.
    Any,
    /// Return true if one of the `JwtClaim` is in the role or group, based on the `JwtClaimTyp`.
    Or(Vec<JwtClaim>),
    /// Return true if all the `JwtClaim`s are in the role or group, based on the `JwtClaimTyp`.
    And(Vec<JwtClaim>),
    /// It doesn't matter which role or group a user have, it returns always `false`.
    None,
}

impl ClaimMapping {
    pub fn matches(&self, roles: &[String], groups: &[String]) -> bool {
        match self {
            ClaimMapping::Any => true,
            ClaimMapping::Or(claims) => claims.iter().any(|claim| claim.matches(roles, groups)),
            ClaimMapping::And(claims) => claims.iter().all(|claim| claim.matches(roles, groups)),
            ClaimMapping::None => false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaim {
    pub typ: JwtClaimTyp,
    pub value: String,
}

impl JwtClaim {
    pub fn matches(&self, roles: &[String], groups: &[String]) -> bool {
        match &self.typ {
            JwtClaimTyp::Roles => roles.contains(&self.value),
            JwtClaimTyp::Groups => groups.contains(&self.value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum JwtClaimTyp {
    Roles,
    Groups,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rauthy_error::RauthyError;
    use std::ops::Deref;

    #[test]
    fn test_claim_mapping() -> Result<(), RauthyError> {
        let mapping = ClaimMapping::None;
        // no matter which roles / groupe we have - None should always deny access
        assert!(!mapping.matches(&vec!["".to_string()], &vec!["".to_string()]));

        let mapping = ClaimMapping::Any;
        // no matter which roles / groupe we have - Any should always allow access
        assert!(mapping.matches(&vec!["".to_string()], &vec!["".to_string()]));

        let test_roles = vec![
            "role1".to_string(),
            "role2".to_string(),
            "role3".to_string(),
        ];
        let test_groups = vec![
            "group1".to_string(),
            "group2".to_string(),
            "group3".to_string(),
        ];

        // AND: these should be allowed
        let mapping = ClaimMapping::And(vec![
            JwtClaim {
                typ: JwtClaimTyp::Roles,
                value: "role1".to_string(),
            },
            JwtClaim {
                typ: JwtClaimTyp::Groups,
                value: "group1".to_string(),
            },
        ]);
        assert!(mapping.matches(test_roles.deref(), test_groups.deref()));

        let mapping = ClaimMapping::And(vec![
            JwtClaim {
                typ: JwtClaimTyp::Roles,
                value: "role1".to_string(),
            },
            JwtClaim {
                typ: JwtClaimTyp::Roles,
                value: "role2".to_string(),
            },
            JwtClaim {
                typ: JwtClaimTyp::Groups,
                value: "group3".to_string(),
            },
        ]);
        assert!(mapping.matches(test_roles.deref(), test_groups.deref()));

        // AND: these should not be allowed
        let mapping = ClaimMapping::And(vec![
            JwtClaim {
                typ: JwtClaimTyp::Roles,
                value: "role1".to_string(),
            },
            JwtClaim {
                typ: JwtClaimTyp::Groups,
                value: "group4".to_string(),
            },
        ]);
        assert!(!mapping.matches(test_roles.deref(), test_groups.deref()));

        let mapping = ClaimMapping::And(vec![
            JwtClaim {
                typ: JwtClaimTyp::Roles,
                value: "role4".to_string(),
            },
            JwtClaim {
                typ: JwtClaimTyp::Groups,
                value: "group2".to_string(),
            },
        ]);
        assert!(!mapping.matches(test_roles.deref(), test_groups.deref()));

        // OR: should be allowed
        let mapping = ClaimMapping::Or(vec![
            JwtClaim {
                typ: JwtClaimTyp::Roles,
                value: "role1".to_string(),
            },
            JwtClaim {
                typ: JwtClaimTyp::Roles,
                value: "role4".to_string(),
            },
            JwtClaim {
                typ: JwtClaimTyp::Groups,
                value: "group1".to_string(),
            },
            JwtClaim {
                typ: JwtClaimTyp::Groups,
                value: "group5".to_string(),
            },
        ]);
        assert!(mapping.matches(test_roles.deref(), test_groups.deref()));

        // OR: should not be allowed
        let mapping = ClaimMapping::Or(vec![
            JwtClaim {
                typ: JwtClaimTyp::Roles,
                value: "role5".to_string(),
            },
            JwtClaim {
                typ: JwtClaimTyp::Groups,
                value: "group4".to_string(),
            },
        ]);
        assert!(!mapping.matches(test_roles.deref(), test_groups.deref()));

        Ok(())
    }
}
