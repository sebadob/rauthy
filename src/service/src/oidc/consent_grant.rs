//! The consent grant check of the authorization_code exchange and FedCM, the mapping of its
//! `ConsentRequired` to `invalid_grant`, and the narrowing of a refresh token's scope to the
//! consent grant.

use rauthy_data::entity::clients::Client;
use rauthy_data::entity::user_consents::UserConsent;
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_error::{ErrorResponse, ErrorResponseType};

/// With `consent.enable`, fails with `ConsentRequired` when the user has no consent grant for the
/// client and `resource`. The caller maps it to its own denial, see [`consent_as_invalid_grant`].
pub async fn exists_consent_grant(
    user_id: &str,
    client_id: &str,
    resource: Option<&str>,
) -> Result<(), ErrorResponse> {
    if !RauthyConfig::get().vars.consent.enable {
        return Ok(());
    }

    if UserConsent::find(user_id, client_id, resource)
        .await?
        .is_none()
    {
        tracing::warn!(user_id, client_id, "no consent grant for the client");
        return Err(ErrorResponse::new(
            ErrorResponseType::ConsentRequired,
            "no consent grant for this client",
        ));
    }
    Ok(())
}

/// A missing consent grant (`ConsentRequired`) as the `invalid_grant` of the token endpoint. Any
/// other error is returned as is.
pub fn consent_as_invalid_grant(err: ErrorResponse) -> ErrorResponse {
    if err.error == ErrorResponseType::ConsentRequired {
        ErrorResponse::new(ErrorResponseType::InvalidGrant, err.message)
    } else {
        err
    }
}

// never wider than the refresh token (RFC 6749 §6)
pub(crate) fn effective_scope(rt_scope: &str, grant: &UserConsent, client: &Client) -> String {
    let mut res = String::with_capacity(rt_scope.len());
    for s in rt_scope.split_whitespace() {
        let covered = grant
            .scopes()
            .chain(client.default_scopes.split(',').map(str::trim))
            .any(|g| g == s);
        if covered && !res.split(' ').any(|r| r == s) {
            if !res.is_empty() {
                res.push(' ');
            }
            res.push_str(s);
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn client(default_scopes: &str) -> Client {
        Client {
            default_scopes: default_scopes.to_string(),
            ..Default::default()
        }
    }

    #[test]
    fn test_effective_scope() {
        let granted = grant("openid,email,profile");

        // narrowed to the grant and the defaults, never widened by a default
        assert_eq!(
            effective_scope("openid email groups", &granted, &client("openid, address")),
            "openid email"
        );
        // a default in the refresh token is kept even if not granted, no duplicates
        assert_eq!(
            effective_scope("profile groups profile", &granted, &client("groups,openid")),
            "profile groups"
        );
        // nothing granted
        assert_eq!(
            effective_scope("email openid", &grant(""), &client("openid")),
            "openid"
        );
        assert_eq!(effective_scope("email", &grant(""), &client("openid")), "");
        // `default_scopes` items are trimmed
        assert_eq!(
            effective_scope("address", &grant(""), &client("openid, address")),
            "address"
        );
        // an empty `default_scopes` does not match anything
        assert_eq!(effective_scope("email", &grant(""), &client("")), "");
        // extra whitespace
        assert_eq!(
            effective_scope("  openid   email ", &granted, &client("")),
            "openid email"
        );
    }

    #[test]
    fn test_consent_as_invalid_grant() {
        let err = consent_as_invalid_grant(ErrorResponse::new(
            ErrorResponseType::ConsentRequired,
            "no consent grant for this client",
        ));
        assert_eq!(err.error, ErrorResponseType::InvalidGrant);
        assert_eq!(err.message, "no consent grant for this client");
        for typ in [ErrorResponseType::Database, ErrorResponseType::InvalidGrant] {
            assert_eq!(
                consent_as_invalid_grant(ErrorResponse::new(typ.clone(), "x")).error,
                typ
            );
        }
    }
}
