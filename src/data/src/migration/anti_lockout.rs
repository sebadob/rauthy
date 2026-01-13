use crate::database::DB;
use crate::entity::clients::Client;
use crate::rauthy_config::RauthyConfig;
use deadpool_postgres::GenericClient;
use rauthy_common::is_hiqlite;
use rauthy_error::ErrorResponse;
use tracing::debug;

pub async fn anti_lockout() -> Result<(), ErrorResponse> {
    debug!("Executing anti_lockout_check");

    let vars = &RauthyConfig::get().vars;
    let issuer = &RauthyConfig::get().issuer;
    let (redirect_uris, allowed_origins) = if vars.dev.dev_mode {
        let (ip, _) = RauthyConfig::get()
            .vars
            .server
            .pub_url
            .split_once(':')
            .expect("PUB_URL must have a port");
        let origin = if ip != "localhost" {
            format!("https://{ip}:5173")
        } else {
            "http://localhost:5173".to_string()
        };

        (
            format!(
                "{issuer}/oidc/callback,http://localhost:5173/auth/v1/oidc/callback,\
                https://{ip}:5173/auth/v1/oidc/callback"
            ),
            Some(origin),
        )
    } else {
        (format!("{issuer}/oidc/callback"), None)
    };

    let cl = Client::find("rauthy".to_string()).await?;

    let rauthy = Client {
        id: "rauthy".to_string(),
        name: Some(cl.name.unwrap_or_else(|| "Rauthy".to_string())),
        enabled: true,
        confidential: false,
        secret: None,
        secret_kid: None,
        redirect_uris,
        post_logout_redirect_uris: None,
        allowed_origins,
        flows_enabled: "authorization_code".to_string(),
        access_token_alg: "EdDSA".to_string(),
        id_token_alg: "EdDSA".to_string(),
        auth_code_lifetime: 10,
        access_token_lifetime: 10, // The token is not used for the Admin UI -> session only
        scopes: "openid".to_string(),
        default_scopes: "openid".to_string(),
        challenge: Some("S256".to_string()),
        force_mfa: RauthyConfig::get().vars.mfa.admin_force_mfa,
        client_uri: Some(RauthyConfig::get().pub_url_with_scheme.clone()),
        contacts: vars.email.rauthy_admin_email.clone(),
        backchannel_logout_uri: None,
        restrict_group_prefix: None,
    };
    debug!(client = ?rauthy, "Rauthy client anti-lockout");

    // we are using a txn here to be able to re-use the already written update queries for the client
    if is_hiqlite() {
        let mut txn = Vec::with_capacity(1);
        rauthy.save_txn_append(&mut txn);
        DB::hql().txn(txn).await?;
    } else {
        let mut cl = DB::pg().await?;
        let txn = cl.transaction().await?;
        rauthy.save_txn(&txn).await?;
        txn.commit().await?;
    }

    Ok(())
}
