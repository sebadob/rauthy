use crate::database::DB;
use crate::entity::clients::Client;
use deadpool_postgres::GenericClient;
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
                "{issuer}/oidc/callback,http://localhost:5173/auth/v1/oidc/callback,https://{}:5173/auth/v1/oidc/callback",
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
        name: Some("Rauthy".to_string()),
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
        force_mfa: *ADMIN_FORCE_MFA,
        client_uri: Some(PUB_URL_WITH_SCHEME.to_string()),
        contacts: RAUTHY_ADMIN_EMAIL.clone(),
        backchannel_logout_uri: None,
        restrict_group_prefix: None,
    };
    debug!("Rauthy client anti-lockout: {:?}", rauthy);

    // we are using a txn h ere to be able to re-use the already written update queries for the client
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
