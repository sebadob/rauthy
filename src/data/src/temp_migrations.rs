use crate::entity::clients::{Client, validate_redirect_uri_shape};
use crate::entity::db_version::DbVersion;
use rauthy_error::ErrorResponse;
use semver::Version;
use tracing::{error, info, warn};

pub async fn apply_temp_migrations(
    previous_db_version: Option<Version>,
) -> Result<(), ErrorResponse> {
    warn_invalid_redirect_uris().await;

    let Some(previous) = previous_db_version else {
        return Ok(());
    };
    let app = DbVersion::app_version();
    if !needs_temp_migration(&previous, &app) {
        info!("Cache WAL state was reset before startup for upgrade from v{previous}");
    }

    Ok(())
}

/// Redirect URIs with a fragment, a `,` or a reserved query key (`code`, `state`, `iss`, ...) are
/// rejected since v0.37 (RFC 6749 §3.1.2, RFC 9207). A client stored before may still contain
/// one: an authorization request with it is rejected, and the client can only be updated once the
/// URI is replaced. Only logs and never fails the startup.
async fn warn_invalid_redirect_uris() {
    let clients = match Client::find_all().await {
        Ok(clients) => clients,
        Err(err) => {
            error!("Cannot load clients to check their redirect URIs: {err}");
            return;
        }
    };

    for client in clients {
        for (uri, reason) in invalid_redirect_uris(&client) {
            warn!(
                "Client '{}' has an invalid redirect URI '{uri}': {reason}. Authorization \
                requests with it are rejected and the client cannot be updated until the URI is \
                replaced (breaking change, RFC 6749 §3.1.2).",
                client.id
            );
        }
    }
}

/// Returns each stored redirect URI of `client` that fails `validate_redirect_uri_shape()`,
/// together with the violated rule.
fn invalid_redirect_uris(client: &Client) -> Vec<(String, String)> {
    client
        .get_redirect_uris()
        .into_iter()
        .filter_map(|uri| {
            validate_redirect_uri_shape(&uri)
                .err()
                .map(|err| (uri, err.message.into_owned()))
        })
        .collect()
}

fn needs_temp_migration(previous: &Version, app: &Version) -> bool {
    previous.major != app.major || previous.minor < app.minor
}

#[cfg(test)]
mod tests {
    use super::*;

    fn v(s: &str) -> Version {
        Version::parse(s).unwrap()
    }

    #[test]
    fn invalid_redirect_uris_lists_each_violation() {
        let client = Client {
            id: "legacy".to_string(),
            redirect_uris: [
                "https://app.example.com/cb",
                "https://app.example.com/cb#/x",
                "https://app.example.com/cb?iss=x",
                "https://app.example.com/*",
            ]
            .join(","),
            ..Default::default()
        };

        let invalid = invalid_redirect_uris(&client);
        assert_eq!(invalid.len(), 2, "{invalid:?}");
        assert_eq!(invalid[0].0, "https://app.example.com/cb#/x");
        assert!(invalid[0].1.contains("fragment"), "{}", invalid[0].1);
        assert_eq!(invalid[1].0, "https://app.example.com/cb?iss=x");
        assert!(invalid[1].1.contains("'iss'"), "{}", invalid[1].1);

        let client = Client {
            redirect_uris: "https://app.example.com/*".to_string(),
            ..Default::default()
        };
        assert!(invalid_redirect_uris(&client).is_empty());
    }

    #[test]
    fn cache_wal_cleanup_runs_on_minor_upgrade() {
        assert!(needs_temp_migration(&v("0.36.2"), &v("0.37.0-20260819")));
        assert!(needs_temp_migration(&v("0.36.0"), &v("0.37.0")));
    }

    #[test]
    fn cache_wal_cleanup_skips_same_or_newer_minor() {
        assert!(!needs_temp_migration(
            &v("0.37.0-20260819"),
            &v("0.37.0-20260820"),
        ));
        assert!(!needs_temp_migration(&v("0.37.0"), &v("0.37.0")));
        assert!(!needs_temp_migration(&v("0.37.0"), &v("0.36.2")));
    }
}
