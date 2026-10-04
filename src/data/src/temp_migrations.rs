use crate::entity::clients::Client;
use rauthy_common::validation::validate_redirect_uri;
use rauthy_error::ErrorResponse;
use semver::Version;
use tracing::{error, warn};

pub async fn apply_temp_migrations(
    _previous_db_version: Option<Version>,
) -> Result<(), ErrorResponse> {
    warn_invalid_redirect_uris().await;

    Ok(())
}

/// Redirect URIs with a fragment, a `,` or a reserved query key (`code`, `state`, `iss`, ...) are
/// rejected since v0.37 (RFC 6749 §3.1.2, RFC 9207), and so are post-logout redirect URIs with a
/// fragment, a `,` or a `state` query key. A client stored before may still contain one: an
/// authorization or logout request with it is rejected, and the client can only be updated once
/// the URI is replaced. Only logs and never fails the startup.
async fn warn_invalid_redirect_uris() {
    let clients = match Client::find_all().await {
        Ok(clients) => clients,
        Err(err) => {
            error!("Cannot load clients to check their redirect URIs: {err}");
            return;
        }
    };

    for client in clients {
        for uri in client.get_redirect_uris() {
            if let Err(err) = validate_redirect_uri(&uri, true, true) {
                warn!(
                    "Client '{}' has an invalid redirect URI '{}': {}. Authorization \
                requests with it are rejected and the client cannot be updated until the URI is \
                replaced (breaking change, RFC 6749 §3.1.2).",
                    client.id, uri, err.message
                );
            }
        }

        if let Some(post_logout) = client.get_post_logout_uris() {
            for uri in post_logout {
                if let Err(err) = validate_redirect_uri(&uri, true, true) {
                    warn!(
                        "Client '{}' has an invalid post-logout redirect URI '{}': {}. Authorization \
                requests with it are rejected and the client cannot be updated until the URI is \
                replaced (breaking change, RFC 6749 §3.1.2).",
                        client.id, uri, err.message
                    );
                }
            }
        }
    }
}
