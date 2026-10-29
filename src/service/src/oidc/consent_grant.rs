//! With `consent.enable`, the authorization_code grant (FedCM included) and the refresh tokens
//! it issued only yield tokens while the user's consent grant (`user_consents`) for the client
//! and RFC 8707 resource exists. The password and device_code grants have no consent step: their
//! tokens are `ConsentCheck::Exempt` and not checked. Without `consent.enable`, everything here
//! is a no-op.

use crate::token_set::ConsentCheck;
use rauthy_common::constants::REFRESH_TOKEN_VALIDATION_LEN;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::refresh_tokens::RefreshToken;
use rauthy_data::entity::user_consents::UserConsent;
use rauthy_data::rauthy_config::{RauthyConfig, VarsConsent};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::future::Future;
use tracing::{error, warn};

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

/// Whether a refresh is checked against the user's consent grant.
fn needs_check(enable: bool, consent: ConsentCheck) -> bool {
    enable && consent == ConsentCheck::Required
}

fn invalid_grant(msg: &'static str) -> ErrorResponse {
    ErrorResponse::new(ErrorResponseType::InvalidGrant, msg)
}

/// Authorization code exchange: the grant the code was issued under must still exist. It may have
/// been revoked between code issue and exchange. FedCM checks the grant before issuance with its
/// own `require_consent`, it shares only the post-issuance fence [`fence_code_exchange`].
pub async fn check_code_exchange(
    user_id: &str,
    client_id: &str,
    resource: Option<&str>,
) -> Result<(), ErrorResponse> {
    if !RauthyConfig::get().vars.consent.enable {
        return Ok(());
    }

    let grant = UserConsent::find(user_id, client_id, grant_resource(resource)).await?;
    if grant.is_none() {
        warn!("no consent grant for the authorization code");
        return Err(invalid_grant("no consent grant for this client"));
    }
    Ok(())
}

/// Authorization code exchange and FedCM, after `TokenSet::from_user`: the grant must still
/// exist.
/// [`check_code_exchange`] runs before the refresh token is inserted, so a revocation that
/// commits in between cannot see that token. Without the grant, the access token of the new
/// refresh token is revoked, the refresh token deleted ([`RefreshToken::discard`]), and the
/// exchange fails with `invalid_grant`. The grant is read with [`UserConsent::exists_fenced`],
/// which does not pass while a revocation that could have missed the new refresh token is
/// still in flight.
pub async fn fence_code_exchange(
    user_id: &str,
    client_id: &str,
    resource: Option<&str>,
    refresh_token: Option<&str>,
) -> Result<(), ErrorResponse> {
    if !RauthyConfig::get().vars.consent.enable {
        return Ok(());
    }

    let granted = UserConsent::exists_fenced(user_id, client_id, grant_resource(resource)).await;
    // an id that is already gone, e.g. removed by the revocation itself, is fine
    fence_issued(granted, refresh_token, async |id| {
        RefreshToken::discard(&id).await
    })
    .await
}

/// [`discard_unless_granted`] with the result of the grant lookup. If the lookup failed, the
/// exchange fails with that error, and the refresh token is discarded on a best-effort basis: the
/// grant may be gone. A failed discard is logged, the lookup error is returned.
async fn fence_issued<F, Fut>(
    granted: Result<bool, ErrorResponse>,
    refresh_token: Option<&str>,
    discard: F,
) -> Result<(), ErrorResponse>
where
    F: FnOnce(String) -> Fut,
    Fut: Future<Output = Result<(), ErrorResponse>>,
{
    let err = match granted {
        Ok(granted) => return discard_unless_granted(granted, refresh_token, discard).await,
        Err(err) => err,
    };

    error!(
        ?err,
        "consent grant lookup failed after the tokens were issued"
    );
    discard_logged(refresh_token, discard).await;
    Err(err)
}

/// `discard`s the refresh token (by its validation id), if any. A failure, of the discard or of
/// the id, is logged with its cause: the exchange fails with its own error anyway.
async fn discard_logged<F, Fut>(refresh_token: Option<&str>, discard: F)
where
    F: FnOnce(String) -> Fut,
    Fut: Future<Output = Result<(), ErrorResponse>>,
{
    let Some(token) = refresh_token else {
        return;
    };
    let discarded = match refresh_token_id(token) {
        Ok(id) => discard(id.to_string()).await,
        Err(err) => Err(err),
    };
    if let Err(discard_err) = discarded {
        error!(
            ?discard_err,
            "could not discard the refresh token of a failed code exchange"
        );
    }
}

/// The validation id of a refresh token, its `refresh_tokens.id`.
fn refresh_token_id(token: &str) -> Result<&str, ErrorResponse> {
    token
        .len()
        .checked_sub(REFRESH_TOKEN_VALIDATION_LEN)
        .and_then(|start| token.get(start..))
        .ok_or_else(|| ErrorResponse::new(ErrorResponseType::Internal, "Invalid refresh token"))
}

/// Without the grant, `discard`s the refresh token (by its validation id) and fails with
/// `invalid_grant`. A failed discard is logged, like in [`fence_issued`]: the exchange still
/// fails with `invalid_grant`, which FedCM maps to its consent-required denial.
async fn discard_unless_granted<F, Fut>(
    granted: bool,
    refresh_token: Option<&str>,
    discard: F,
) -> Result<(), ErrorResponse>
where
    F: FnOnce(String) -> Fut,
    Fut: Future<Output = Result<(), ErrorResponse>>,
{
    if granted {
        return Ok(());
    }

    warn!("consent grant revoked while the tokens were issued");
    discard_logged(refresh_token, discard).await;
    Err(invalid_grant("no consent grant for this client"))
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
    /// ([`RefreshToken::discard`], `RefreshTokenDevice::discard`, in the order `PG_DISCARD_STEPS`
    /// and `DiscardSql::hql` give per database), and the refresh fails with
    /// `invalid_grant`. If that fails, nothing is gone and the error is returned: the next
    /// refresh with the token finds the grant still missing and discards it again. A missing grant is
    /// permanent for that token: it may have escaped a revocation, and must not come back once
    /// the user consents again.
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
        .validate_consent_resource_with(resource, vars)
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
    use std::collections::HashMap;

    fn grant(scopes: &str) -> UserConsent {
        UserConsent {
            user_id: "u1".to_string(),
            client_id: "c1".to_string(),
            resource: String::new(),
            scopes: scopes.to_string(),
            content_hash: "stale".to_string(),
            created: 1,
            updated: 1,
            ip: "127.0.0.1".to_string(),
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

    /// In-memory stand-in for `user_consents`, `refresh_tokens` and `issued_tokens`.
    #[derive(Default)]
    struct Db {
        grant: bool,
        // refresh token id -> `access_token_jti`
        refresh_tokens: HashMap<String, String>,
        revoked: Vec<String>,
    }

    fn discard(db: &RefCell<Db>, id: String) -> impl Future<Output = Result<(), ErrorResponse>> {
        let mut db = db.borrow_mut();
        if let Some(jti) = db.refresh_tokens.remove(&id) {
            db.revoked.push(jti);
        }
        async { Ok(()) }
    }

    /// The grant is revoked after the code exchange check passed, but before `TokenSet::from_user`
    /// inserted the new refresh token. The revocation could not see that token, so the re-check
    /// after issuance must remove it again.
    #[tokio::test]
    async fn test_code_exchange_revoked_before_issue() {
        let rt_id = "r".repeat(REFRESH_TOKEN_VALIDATION_LEN);
        let refresh_token = format!("header.claims.{rt_id}");
        let db = RefCell::new(Db {
            grant: true,
            ..Default::default()
        });

        // `check_code_exchange` passes
        assert!(db.borrow().grant);
        // `UserConsent::revoke` commits: grant and the (so far empty) set of tokens are gone
        db.borrow_mut().grant = false;
        // `TokenSet::from_user` inserts the new refresh token
        db.borrow_mut()
            .refresh_tokens
            .insert(rt_id.clone(), "jti1".to_string());

        let granted = db.borrow().grant;
        let err = discard_unless_granted(granted, Some(&refresh_token), |id| discard(&db, id))
            .await
            .unwrap_err();
        assert_eq!(err.error, ErrorResponseType::InvalidGrant);
        assert!(db.borrow().refresh_tokens.is_empty());
        assert_eq!(db.borrow().revoked, vec!["jti1".to_string()]);
    }

    #[tokio::test]
    async fn test_code_exchange_still_granted() {
        let rt_id = "r".repeat(REFRESH_TOKEN_VALIDATION_LEN);
        let refresh_token = format!("header.claims.{rt_id}");
        let db = RefCell::new(Db {
            grant: true,
            ..Default::default()
        });
        db.borrow_mut()
            .refresh_tokens
            .insert(rt_id.clone(), "jti1".to_string());

        let granted = db.borrow().grant;
        discard_unless_granted(granted, Some(&refresh_token), |id| discard(&db, id))
            .await
            .unwrap();
        assert_eq!(db.borrow().refresh_tokens.len(), 1);
        assert!(db.borrow().revoked.is_empty());

        // without a refresh token, a missing grant still fails the exchange
        let err = discard_unless_granted(false, None, |id| discard(&db, id))
            .await
            .unwrap_err();
        assert_eq!(err.error, ErrorResponseType::InvalidGrant);
        assert_eq!(db.borrow().refresh_tokens.len(), 1);
    }

    /// The grant is gone and the discard of the new refresh token fails: the exchange still fails
    /// with `invalid_grant` (FedCM: `401` consent-required), not with the error of the discard.
    #[tokio::test]
    async fn test_code_exchange_discard_fails() {
        let rt_id = "r".repeat(REFRESH_TOKEN_VALIDATION_LEN);
        let refresh_token = format!("header.claims.{rt_id}");
        let called = RefCell::new(Vec::new());
        let err = discard_unless_granted(false, Some(&refresh_token), |id| {
            called.borrow_mut().push(id);
            async { Err(ErrorResponse::new(ErrorResponseType::Database, "down")) }
        })
        .await
        .unwrap_err();
        assert_eq!(err.error, ErrorResponseType::InvalidGrant);
        assert_eq!(*called.borrow(), vec![rt_id]);

        // the same through `fence_issued`, the path of `fence_code_exchange`
        let err = fence_issued(Ok(false), Some(&refresh_token), |_| async {
            Err(ErrorResponse::new(ErrorResponseType::Database, "down"))
        })
        .await
        .unwrap_err();
        assert_eq!(err.error, ErrorResponseType::InvalidGrant);
    }

    /// A refresh token shorter than its validation id cannot be discarded: nothing is touched,
    /// and the exchange fails with `invalid_grant` without a grant, or with the lookup error. One
    /// of exactly that length is its own id.
    #[tokio::test]
    async fn test_code_exchange_short_refresh_token() {
        let rt_id = "r".repeat(REFRESH_TOKEN_VALIDATION_LEN);
        let short = &rt_id[1..];
        let db = RefCell::new(Db::default());
        db.borrow_mut()
            .refresh_tokens
            .insert(short.to_string(), "jti1".to_string());

        for granted in [
            Ok(false),
            Err(ErrorResponse::new(ErrorResponseType::Database, "down")),
        ] {
            let expected = match &granted {
                Ok(_) => ErrorResponseType::InvalidGrant,
                Err(err) => err.error.clone(),
            };
            let err = fence_issued(granted, Some(short), |id| discard(&db, id))
                .await
                .unwrap_err();
            assert_eq!(err.error, expected);
            assert_eq!(db.borrow().refresh_tokens.len(), 1);
            assert!(db.borrow().revoked.is_empty());
        }

        db.borrow_mut()
            .refresh_tokens
            .insert(rt_id.clone(), "jti2".to_string());
        let err = fence_issued(Ok(false), Some(&rt_id), |id| discard(&db, id))
            .await
            .unwrap_err();
        assert_eq!(err.error, ErrorResponseType::InvalidGrant);
        assert_eq!(db.borrow().revoked, vec!["jti2".to_string()]);
    }

    /// The grant lookup of the fence fails: the exchange fails with that error, and the issued
    /// refresh token is discarded, since the grant may be gone. A failed discard does not mask
    /// the lookup error.
    #[tokio::test]
    async fn test_code_exchange_lookup_error() {
        let rt_id = "r".repeat(REFRESH_TOKEN_VALIDATION_LEN);
        let refresh_token = format!("header.claims.{rt_id}");
        let lookup_err = || Err(ErrorResponse::new(ErrorResponseType::Database, "down"));
        let db = RefCell::new(Db::default());
        db.borrow_mut()
            .refresh_tokens
            .insert(rt_id.clone(), "jti1".to_string());

        let err = fence_issued(lookup_err(), Some(&refresh_token), |id| discard(&db, id))
            .await
            .unwrap_err();
        assert_eq!(err.error, ErrorResponseType::Database);
        assert!(db.borrow().refresh_tokens.is_empty());
        assert_eq!(db.borrow().revoked, vec!["jti1".to_string()]);

        // the discard fails as well
        let called = RefCell::new(Vec::new());
        let err = fence_issued(lookup_err(), Some(&refresh_token), |id| {
            called.borrow_mut().push(id);
            async { Err(ErrorResponse::new(ErrorResponseType::Internal, "discard")) }
        })
        .await
        .unwrap_err();
        assert_eq!(err.error, ErrorResponseType::Database);
        assert_eq!(*called.borrow(), vec![rt_id.clone()]);

        // an invalid refresh token cannot be discarded, the lookup error stays
        let err = fence_issued(lookup_err(), Some("short"), |id| {
            called.borrow_mut().push(id);
            async { Ok(()) }
        })
        .await
        .unwrap_err();
        assert_eq!(err.error, ErrorResponseType::Database);
        assert_eq!(called.borrow().len(), 1);

        // without a refresh token, only the error
        let err = fence_issued(lookup_err(), None, |id| {
            called.borrow_mut().push(id);
            async { Ok(()) }
        })
        .await
        .unwrap_err();
        assert_eq!(err.error, ErrorResponseType::Database);
        assert_eq!(called.borrow().len(), 1);

        // a successful lookup goes through `discard_unless_granted`
        fence_issued(Ok(true), Some(&refresh_token), |id| {
            called.borrow_mut().push(id);
            async { Ok(()) }
        })
        .await
        .unwrap();
        let err = fence_issued(Ok(false), Some(&refresh_token), |id| {
            called.borrow_mut().push(id);
            async { Ok(()) }
        })
        .await
        .unwrap_err();
        assert_eq!(err.error, ErrorResponseType::InvalidGrant);
        assert_eq!(*called.borrow(), vec![rt_id.clone(), rt_id]);
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
