//! With `consent.enable`, the authorization_code grant (FedCM included) and the refresh tokens
//! it issued only yield tokens while the user's consent grant (`user_consents`) for the client
//! and RFC 8707 resource exists. The password and device_code grants have no consent step: their
//! tokens are `ConsentCheck::Exempt` and not checked. Without `consent.enable`, everything here
//! is a no-op.
//!
//! The checks here are plain reads that fail early. What a concurrent revoke cannot miss is
//! decided by the write: a `ConsentCheck::Required` refresh token is only inserted while the
//! grant exists (`RefreshTokenGrant::Required` of `RefreshToken::create`).

use crate::token_set::ConsentCheck;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::user_consents::UserConsent;
use rauthy_data::rauthy_config::{RauthyConfig, VarsConsent};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::future::Future;
use tracing::warn;

/// The `user_consents.resource` key: the RFC 8707 resource, or `''` without one.
#[inline]
fn grant_resource(resource: Option<&str>) -> &str {
    resource.unwrap_or_default()
}

/// The [`ConsentCheck`] of a refresh token from its claims. `consent_exempt` marks the tokens
/// of a grant without a consent step. The signed `did` only covers device tokens issued before
/// `consent_exempt` existed that have not been rotated yet. A rotation re-issues the token
/// without a `did`, but with `consent_exempt` set from the returned check.
pub fn refresh_consent_check(consent_exempt: bool, has_did: bool) -> ConsentCheck {
    if consent_exempt || has_did {
        ConsentCheck::Exempt
    } else {
        ConsentCheck::Required
    }
}

/// Whether a token is checked against the user's consent grant.
pub(crate) fn needs_check(enable: bool, consent: ConsentCheck) -> bool {
    enable && consent == ConsentCheck::Required
}

fn invalid_grant(msg: &'static str) -> ErrorResponse {
    ErrorResponse::new(ErrorResponseType::InvalidGrant, msg)
}

/// Authorization code exchange: the grant the code was issued under must still exist. It may have
/// been revoked between code issue and exchange. This read only fails early: a revoke that
/// commits after it is covered by the insert of the refresh token, which requires the grant
/// (`ConsentCheck::Required`). FedCM checks the grant with its own `require_consent`.
pub async fn check_code_exchange(
    user_id: &str,
    client_id: &str,
    resource: Option<&str>,
) -> Result<(), ErrorResponse> {
    check_code_exchange_with(
        RauthyConfig::get().vars.consent.enable,
        user_id,
        client_id,
        || UserConsent::find(user_id, client_id, grant_resource(resource)),
    )
    .await
}

/// [`check_code_exchange`] with `consent.enable` and the grant lookup passed in.
async fn check_code_exchange_with<F, Fut>(
    enable: bool,
    user_id: &str,
    client_id: &str,
    find: F,
) -> Result<(), ErrorResponse>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<Option<UserConsent>, ErrorResponse>>,
{
    if !enable {
        return Ok(());
    }

    if find().await?.is_none() {
        warn!(user_id, client_id, "no consent grant for the client");
        return Err(invalid_grant("no consent grant for this client"));
    }
    Ok(())
}

/// The consent grant a refresh is checked against. Looked up via [`RefreshGrant::lookup`]
/// after the refresh token row checks and before the rotation. Without the grant,
/// [`RefreshGrant::require`] discards the source refresh token, so a token that escaped a
/// revocation cannot be refreshed again after a new consent.
#[derive(Debug)]
pub enum RefreshGrant {
    /// `consent.enable` is off, or the refresh token is exempt from the consent check
    Unchecked,
    Granted(UserConsent),
}

impl RefreshGrant {
    /// Checks the `resource` and looks up the grant for (user, client, resource). `None` without
    /// a grant, see [`Self::require`]. A stale `content_hash` on the grant does not block a
    /// refresh.
    pub async fn lookup(
        consent: ConsentCheck,
        client: &Client,
        user_id: &str,
        resource: Option<&str>,
    ) -> Result<Option<Self>, ErrorResponse> {
        let vars = &RauthyConfig::get().vars.consent;
        if !needs_check(vars.enable, consent) {
            return Ok(Some(Self::Unchecked));
        }

        validate_refresh_resource(client, resource, vars)?;
        let grant = UserConsent::find(user_id, &client.id, grant_resource(resource)).await?;
        Ok(grant.map(Self::Granted))
    }

    /// The grant from [`Self::lookup`]. Without one, `discard_source` deletes the refresh token
    /// the refresh was made with and revokes its access token, in one transaction
    /// (`RefreshToken::discard`, in the order `PG_DISCARD_STEPS`
    /// and `DiscardSql::hql` give per database), and the refresh fails with
    /// `invalid_grant`. The token may have escaped a revocation, and must not come back once the
    /// user consents again. If the discard fails, nothing is gone and the error is returned: the
    /// next refresh with the token discards it again, but only while the grant is still missing.
    /// Device refresh tokens are always `ConsentCheck::Exempt`, so they are never discarded here.
    pub async fn require<F, Fut>(
        grant: Option<Self>,
        discard_source: F,
    ) -> Result<Self, ErrorResponse>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<(), ErrorResponse>>,
    {
        match grant {
            Some(grant) => Ok(grant),
            None => {
                warn!("no consent grant for the refresh token");
                discard_source().await?;
                Err(invalid_grant("no consent grant for this client"))
            }
        }
    }

    /// The scope for the new token set from `rt_scope` of the refresh token, see
    /// [`granted_scope`].
    pub fn scope(
        &self,
        rt_scope: Option<String>,
        default_scopes: &[String],
    ) -> Result<Option<String>, ErrorResponse> {
        match self {
            Self::Unchecked => Ok(rt_scope),
            Self::Granted(grant) => granted_scope(grant, rt_scope, default_scopes),
        }
    }
}

/// A refresh token without a `resource` for a client that must request one at the authorization
/// request (see [`Client::validate_consent_resource`]) cannot be matched to its consent source.
fn validate_refresh_resource(
    client: &Client,
    resource: Option<&str>,
    vars: &VarsConsent,
) -> Result<(), ErrorResponse> {
    client
        .validate_consent_resource(resource, vars)
        .map_err(|_| {
            warn!("refresh token without a `resource` for a client that requires one");
            invalid_grant("a `resource` is required")
        })
}

/// `rt_scope` from the refresh token (space separated), `None` meaning the client's
/// `default_scopes`, see [`effective_scope`]. `None` stays `None`: the client defaults are never
/// narrowed by the grant.
fn granted_scope(
    grant: &UserConsent,
    rt_scope: Option<String>,
    default_scopes: &[String],
) -> Result<Option<String>, ErrorResponse> {
    let Some(rt_scope) = rt_scope else {
        return Ok(None);
    };
    let scope = effective_scope(&rt_scope, &grant.scopes_vec(), default_scopes);
    if scope.is_empty() {
        warn!("no scope of the refresh token is covered by the consent grant");
        return Err(invalid_grant(
            "the consent grant covers no scope of the refresh token",
        ));
    }
    Ok(Some(scope))
}

/// `rt_scope ∩ (granted ∪ default_scopes)`, space separated, in the order of `rt_scope`. Never
/// wider than the refresh token (RFC 6749 §6).
fn effective_scope(rt_scope: &str, granted: &[String], default_scopes: &[String]) -> String {
    let mut res: Vec<&str> = Vec::new();
    for s in rt_scope.split_whitespace() {
        let covered = granted.iter().chain(default_scopes).any(|g| g == s);
        if covered && !res.contains(&s) {
            res.push(s);
        }
    }
    res.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use rauthy_data::rauthy_config::VarsConsentSource;
    use std::cell::RefCell;

    fn grant(scopes: &str) -> UserConsent {
        UserConsent {
            user_id: "u1".to_string(),
            client_id: "c1".to_string(),
            resource: String::new(),
            scopes: scopes.to_string(),
            content_hash: Some("stale".to_string()),
            created: 1,
            updated: 1,
            ip: "127.0.0.1".to_string(),
            user_agent: None,
        }
    }

    fn strings(v: &[&str]) -> Vec<String> {
        v.iter().map(|s| s.to_string()).collect()
    }

    #[test]
    fn test_effective_scope() {
        let granted = strings(&["openid", "email", "profile"]);

        // narrowed to the grant and the defaults, never widened by a default
        assert_eq!(
            effective_scope(
                "openid email groups",
                &granted,
                &strings(&["openid", "address"])
            ),
            "openid email"
        );
        // a default in the refresh token is kept even if not granted, no duplicates
        assert_eq!(
            effective_scope(
                "profile groups profile",
                &granted,
                &strings(&["groups", "openid"])
            ),
            "profile groups"
        );
        // nothing granted
        assert_eq!(
            effective_scope("email openid", &[], &strings(&["openid"])),
            "openid"
        );
        assert_eq!(effective_scope("email", &[], &strings(&["openid"])), "");
        assert_eq!(effective_scope("email", &[], &[]), "");
        // an empty default (`default_scopes` of `''`) does not match anything
        assert_eq!(effective_scope("email", &[], &strings(&[""])), "");
        // extra whitespace
        assert_eq!(
            effective_scope("  openid   email ", &granted, &[]),
            "openid email"
        );
    }

    /// The grant is gone (revoked, maybe after this refresh token escaped the revocation): the
    /// source refresh token is deleted, so a new consent cannot bring it back.
    #[tokio::test]
    async fn test_refresh_without_grant_discards_source() {
        let discarded = RefCell::new(0);
        let discard = || {
            *discarded.borrow_mut() += 1;
            async { Ok(()) }
        };
        let err = RefreshGrant::require(None, discard).await.unwrap_err();
        assert_eq!(err.error, ErrorResponseType::InvalidGrant);
        assert_eq!(*discarded.borrow(), 1);

        // with a grant, or unchecked, the source is kept for the rotation
        let discard = || {
            *discarded.borrow_mut() += 1;
            async { Ok(()) }
        };
        let g = RefreshGrant::require(Some(RefreshGrant::Granted(grant("openid"))), discard)
            .await
            .unwrap();
        assert!(matches!(g, RefreshGrant::Granted(_)));
        let discard = || {
            *discarded.borrow_mut() += 1;
            async { Ok(()) }
        };
        let g = RefreshGrant::require(Some(RefreshGrant::Unchecked), discard)
            .await
            .unwrap();
        assert!(matches!(g, RefreshGrant::Unchecked));
        assert_eq!(*discarded.borrow(), 1);

        // a failed delete is reported instead of `invalid_grant`
        let err = RefreshGrant::require(None, || async {
            Err(ErrorResponse::new(ErrorResponseType::Database, "down"))
        })
        .await
        .unwrap_err();
        assert_eq!(err.error, ErrorResponseType::Database);
    }

    #[tokio::test]
    async fn test_check_code_exchange() {
        let lookups = RefCell::new(0);
        let find = |found: Option<UserConsent>| {
            *lookups.borrow_mut() += 1;
            async { Ok(found) }
        };

        // no grant -> `invalid_grant`
        let err = check_code_exchange_with(true, "u1", "c1", || find(None))
            .await
            .unwrap_err();
        assert_eq!(err.error, ErrorResponseType::InvalidGrant);
        assert_eq!(*lookups.borrow(), 1);

        // with a grant
        check_code_exchange_with(true, "u1", "c1", || find(Some(grant("openid"))))
            .await
            .unwrap();
        assert_eq!(*lookups.borrow(), 2);

        // `consent.enable` off -> no lookup at all
        check_code_exchange_with(false, "u1", "c1", || find(None))
            .await
            .unwrap();
        assert_eq!(*lookups.borrow(), 2);

        // a lookup error is returned as is
        let err = check_code_exchange_with(true, "u1", "c1", || async {
            Err(ErrorResponse::new(ErrorResponseType::Database, "down"))
        })
        .await
        .unwrap_err();
        assert_eq!(err.error, ErrorResponseType::Database);
    }

    #[test]
    fn test_refresh_grant() {
        let defaults = strings(&["openid"]);
        let unchecked = RefreshGrant::Unchecked;
        assert_eq!(
            unchecked
                .scope(Some("openid groups".to_string()), &defaults)
                .unwrap(),
            Some("openid groups".to_string())
        );
        assert_eq!(unchecked.scope(None, &defaults).unwrap(), None);

        // a stale `content_hash` does not matter
        let g = RefreshGrant::Granted(grant("openid,email"));
        assert_eq!(
            g.scope(Some("openid email profile".to_string()), &defaults)
                .unwrap(),
            Some("openid email".to_string())
        );
        // the client defaults
        assert_eq!(g.scope(None, &defaults).unwrap(), None);
        // no overlap -> no token with an empty scope
        let err = g
            .scope(Some("profile".to_string()), &strings(&["groups"]))
            .unwrap_err();
        assert_eq!(err.error, ErrorResponseType::InvalidGrant);
    }

    #[test]
    fn test_validate_refresh_resource() {
        let vars = VarsConsent {
            enable: true,
            content_cache_ttl: 3600,
            danger_allow_private_addresses: false,
            sources: [(
                "https://api.example.com".to_string(),
                VarsConsentSource::new(
                    "https://api.example.com".to_string(),
                    "local",
                    "API".to_string(),
                )
                .unwrap(),
            )]
            .into(),
        };
        let ephemeral = Client {
            id: "https://cimd.example/client".to_string(),
            ..Default::default()
        };
        assert!(ephemeral.is_ephemeral());
        let client = Client {
            id: "client".to_string(),
            ..Default::default()
        };

        let err = validate_refresh_resource(&ephemeral, None, &vars).unwrap_err();
        assert_eq!(err.error, ErrorResponseType::InvalidGrant);
        assert!(
            validate_refresh_resource(&ephemeral, Some("https://api.example.com"), &vars).is_ok()
        );
        // registered clients may log in without a `resource` -> the grant for `''` decides
        assert!(validate_refresh_resource(&client, None, &vars).is_ok());

        let no_sources = VarsConsent {
            sources: Default::default(),
            ..vars
        };
        assert!(validate_refresh_resource(&ephemeral, None, &no_sources).is_ok());
    }

    #[test]
    fn test_refresh_consent_check() {
        // (consent_exempt, has_did) -> check
        for (exempt, did, expected) in [
            (false, false, ConsentCheck::Required),
            (true, false, ConsentCheck::Exempt),
            (false, true, ConsentCheck::Exempt),
            (true, true, ConsentCheck::Exempt),
        ] {
            assert_eq!(
                refresh_consent_check(exempt, did),
                expected,
                "consent_exempt: {exempt}, did: {did}"
            );
        }
    }

    #[test]
    fn test_needs_check() {
        for (enable, consent, expected) in [
            (false, ConsentCheck::Required, false),
            (false, ConsentCheck::Exempt, false),
            (true, ConsentCheck::Required, true),
            (true, ConsentCheck::Exempt, false),
        ] {
            assert_eq!(
                needs_check(enable, consent),
                expected,
                "enable: {enable}, {consent:?}"
            );
        }
    }

    #[test]
    fn test_grant_resource() {
        assert_eq!(grant_resource(None), "");
        assert_eq!(
            grant_resource(Some("https://api.example.com")),
            "https://api.example.com"
        );
    }
}
