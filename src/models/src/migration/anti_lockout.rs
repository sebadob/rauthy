use crate::database::DB;
use crate::entity::clients::Client;
use hiqlite::{params, Param};
use rauthy_common::constants::{
    ADMIN_FORCE_MFA, DEV_MODE, PUB_URL, PUB_URL_WITH_SCHEME, RAUTHY_ADMIN_EMAIL,
};
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use tracing::debug;

pub async fn anti_lockout(issuer: &str) -> Result<(), ErrorResponse> {
    debug!("Executing anti_lockout_check");

    let (redirect_uris, allowed_origins) = if *DEV_MODE {
        let (ip, _) = PUB_URL.split_once(':').expect("PUB_URL must have a port");
        let origin = if ip != "localhost" {
            format!("https://{}:5173", ip)
        } else {
            "http://localhost:5173".to_string()
        };

        (
            format!(
                "{issuer}/oidc/callback,http://localhost:5173/*,https://{}:5173/*",
                ip
            ),
            Some(origin),
        )
    } else {
        (format!("{issuer}/oidc/callback"), None)
    };

    // we will actually skip non-mandatory values in the query below
    let rauthy = Client {
        id: "rauthy".to_string(),
        name: None,
        enabled: true,
        confidential: false,
        secret: None,
        secret_kid: None,
        redirect_uris: redirect_uris.clone(),
        post_logout_redirect_uris: Some(redirect_uris),
        allowed_origins,
        flows_enabled: "authorization_code".to_string(),
        access_token_alg: "EdDSA".to_string(),
        id_token_alg: "EdDSA".to_string(),
        auth_code_lifetime: 10,
        access_token_lifetime: 10, // The token is not used for the Admin UI -> session only
        scopes: "openid".to_string(),
        default_scopes: "openid".to_string(),
        challenge: Some("S256".to_string()),
        force_mfa: *ADMIN_FORCE_MFA,
        client_uri: Some(PUB_URL_WITH_SCHEME.to_string()),
        contacts: RAUTHY_ADMIN_EMAIL.clone(),
    };

    // MUST NOT use `insert or replace` syntax
    // -> SQLite basically re-creates this under the hood, which means the FK restriction
    // from `client_logos` -> `clients` will actually delete the client logo that was
    // saved for `rauthy` before.
    // Update only here and prevent `rauthy` deletion as a special check on DELETE /client

    if is_hiqlite() {
        DB::client()
            .execute(
                r#"
UPDATE clients SET enabled = $1, confidential = $2, redirect_uris = $3,
post_logout_redirect_uris = $4, allowed_origins = $5, flows_enabled = $6, access_token_alg = $7,
id_token_alg = $8, auth_code_lifetime = $9, access_token_lifetime = $10, scopes = $11,
default_scopes = $12, challenge = $13, force_mfa = $14, client_uri = $15, contacts = $16
WHERE id = $17"#,
                params!(
                    rauthy.enabled,
                    rauthy.confidential,
                    rauthy.redirect_uris,
                    rauthy.post_logout_redirect_uris,
                    rauthy.allowed_origins,
                    rauthy.flows_enabled,
                    rauthy.access_token_alg,
                    rauthy.id_token_alg,
                    rauthy.auth_code_lifetime,
                    rauthy.access_token_lifetime,
                    rauthy.scopes,
                    rauthy.default_scopes,
                    rauthy.challenge,
                    rauthy.force_mfa,
                    rauthy.client_uri,
                    rauthy.contacts,
                    rauthy.id
                ),
            )
            .await?;
    } else {
        sqlx::query!(
            r#"
UPDATE clients SET enabled = $1, confidential = $2, redirect_uris = $3,
post_logout_redirect_uris = $4, allowed_origins = $5, flows_enabled = $6, access_token_alg = $7,
id_token_alg = $8, auth_code_lifetime = $9, access_token_lifetime = $10, scopes = $11,
default_scopes = $12, challenge = $13, force_mfa = $14, client_uri = $15, contacts = $16
WHERE id = $17"#,
            rauthy.enabled,
            rauthy.confidential,
            rauthy.redirect_uris,
            rauthy.post_logout_redirect_uris,
            rauthy.allowed_origins,
            rauthy.flows_enabled,
            rauthy.access_token_alg,
            rauthy.id_token_alg,
            rauthy.auth_code_lifetime,
            rauthy.access_token_lifetime,
            rauthy.scopes,
            rauthy.default_scopes,
            rauthy.challenge,
            rauthy.force_mfa,
            rauthy.client_uri,
            rauthy.contacts,
            rauthy.id,
        )
        .execute(DB::conn())
        .await?;
    }

    Ok(())
}
