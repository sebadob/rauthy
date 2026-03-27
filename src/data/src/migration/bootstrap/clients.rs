use crate::database::DB;
use crate::entity::clients_scim::ClientScim;
use crate::entity::scopes::Scope;
use crate::migration::bootstrap::bootstrap_data;
use crate::migration::bootstrap::types::{Client, ClientSecret};
use cryptr::{EncKeys, EncValue};
use hiqlite::macros::params;
use itertools::Itertools;
use rauthy_common::constants::SECRET_LEN_CLIENTS;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::base64_decode;
use rauthy_error::ErrorResponse;
use tracing::info;

pub async fn bootstrap() -> Result<(), ErrorResponse> {
    let clients = bootstrap_data!(Client, "clients");

    let scopes = Scope::find_all()
        .await?
        .into_iter()
        .map(|s| s.name)
        .collect::<Vec<_>>();

    let len = clients.len();
    let sql = r#"
INSERT INTO clients (id, name, enabled, confidential, secret, secret_kid, redirect_uris,
post_logout_redirect_uris, allowed_origins, flows_enabled, access_token_alg, id_token_alg,
auth_code_lifetime, access_token_lifetime, scopes, default_scopes, challenge, force_mfa,
client_uri, contacts, backchannel_logout_uri, restrict_group_prefix)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17,
$18, $19, $20, $21, $22)"#;

    for client in clients {
        let (kid, secret) = if let Some(secret) = client.secret {
            let plain = match secret {
                ClientSecret::Plain(s) => s,
                ClientSecret::Encrypted(enc) => {
                    // make sure we can decrypt it
                    let bytes = base64_decode(&enc)
                        .expect("Cannot decode base64 encoded, encrypted client secret");
                    let dec = EncValue::try_from(bytes)
                        .expect("Invalid format for encrypted client secret")
                        .decrypt()
                        .expect("Failed to decrypt encrypted client secret. Make sure Rauthy has access to the used ENC_KEY");
                    String::from_utf8(dec.to_vec()).expect(
                        "Invalid characters in client secret. Cannot convert to lossless String.",
                    )
                }
            };

            if plain.len() < SECRET_LEN_CLIENTS {
                panic!(
                    "Given client secret too short. Expected (at least) {SECRET_LEN_CLIENTS} characters."
                );
            }

            let kid = EncKeys::get_static().enc_key_active.clone();
            let enc = EncValue::encrypt_with_key_id(plain.as_ref(), kid.clone())?
                .into_bytes()
                .to_vec();

            (Some(kid), Some(enc))
        } else {
            (None, None)
        };

        let redirect_uris = opt_vec_to_csv(&Some(client.redirect_uris)).unwrap();
        let post_logout_redirect_uris = opt_vec_to_csv(&client.post_logout_redirect_uris);
        let allowed_origins = opt_vec_to_csv(&client.allowed_origins);

        let flows_enabled = client.flows_enabled.join(",");
        let challenge = if secret.is_none() {
            Some("S256".to_string())
        } else {
            client.challenges.map(|c| c.join(","))
        };
        let contacts = client.contacts.map(|c| c.join(","));

        let default_scopes = "openid, address, email, groups, phone, profile";
        for scope in &client.scopes {
            if !scopes.contains(scope) {
                panic!(
                    "Given client scope '{}' does not exist. Expected one of: {:?}\nor default scopes: {}",
                    scope, scopes, default_scopes
                );
            }
        }
        let scopes = client.scopes.join(",");

        for scope in &client.default_scopes {
            if !scopes.contains(scope) {
                panic!(
                    "Given client scope '{}' does not exist. Expected one of: {:?}\nor default scopes: {}",
                    scope, scopes, default_scopes
                );
            }
        }
        let default_scopes = client.default_scopes.join(",");

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        &client.id,
                        client.name,
                        client.enabled,
                        secret.is_some(),
                        secret,
                        kid,
                        redirect_uris,
                        post_logout_redirect_uris,
                        allowed_origins,
                        flows_enabled,
                        client.access_token_alg.to_string(),
                        client.id_token_alg.to_string(),
                        client.auth_code_lifetime,
                        client.access_token_lifetime,
                        scopes,
                        default_scopes,
                        challenge,
                        client.force_mfa,
                        client.client_uri,
                        contacts,
                        client.backchannel_logout_uri,
                        client.restrict_group_prefix
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &client.id,
                    &client.name,
                    &client.enabled,
                    &secret.is_some(),
                    &secret,
                    &kid,
                    &redirect_uris,
                    &post_logout_redirect_uris,
                    &allowed_origins,
                    &flows_enabled,
                    &client.access_token_alg.to_string(),
                    &client.id_token_alg.to_string(),
                    &client.auth_code_lifetime,
                    &client.access_token_lifetime,
                    &scopes,
                    &default_scopes,
                    &challenge,
                    &client.force_mfa,
                    &client.client_uri,
                    &contacts,
                    &client.backchannel_logout_uri,
                    &client.restrict_group_prefix,
                ],
            )
            .await?;
        }

        if let Some(scim) = client.scim {
            ClientScim::upsert(
                client.id,
                scim.bearer_token.as_str(),
                scim.base_uri,
                scim.sync_groups,
                scim.group_sync_prefix,
            )
            .await?;
        }
    }

    info!("Migrated {len} clients.");

    Ok(())
}

#[cold]
fn opt_vec_to_csv(input: &Option<Vec<String>>) -> Option<String> {
    input.as_ref().map(|v| {
        v.iter()
            .filter_map(|v| {
                let trimmed = v.trim();
                if trimmed.is_empty() {
                    None
                } else {
                    Some(trimmed.to_string())
                }
            })
            .join(",")
    })
}
