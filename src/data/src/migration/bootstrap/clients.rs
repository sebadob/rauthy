use crate::database::DB;
use crate::entity::clients::{
    validate_post_logout_redirect_uri_shape, validate_redirect_uri_shape,
};
use crate::entity::clients_scim::ClientScim;
use crate::entity::scopes::Scope;
use crate::migration::bootstrap::bootstrap_data;
use crate::migration::bootstrap::generated_secrets::{GeneratedSecretEntry, GeneratedSecretKey};
use crate::migration::bootstrap::types::{Client, ClientSecret};
use crate::rauthy_config::RauthyConfig;
use cryptr::utils::secure_random_alnum;
use cryptr::{EncKeys, EncValue};
use hiqlite::macros::params;
use itertools::Itertools;
use rauthy_api_types::oidc::GrantType;
use rauthy_common::constants::SECRET_LEN_CLIENTS;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::base64_decode;
use rauthy_error::ErrorResponse;
use tracing::info;
use zeroize::Zeroize;

/// Validates the `redirect_uris` and `post_logout_redirect_uris` of all bootstrap clients. This
/// must run before anything is written during bootstrap, so that an invalid config fails cleanly
/// and can be fixed.
pub async fn validate() -> Result<(), ErrorResponse> {
    let clients = bootstrap_data!(Client, "clients");
    if let Err(err) = validate_client_uris(&clients) {
        panic!("Validation error when bootstrapping clients: {err}");
    }
    Ok(())
}

fn validate_client_uris(clients: &[Client]) -> Result<(), String> {
    for client in clients {
        for uri in &client.redirect_uris {
            if let Err(err) = validate_redirect_uri_shape(uri) {
                return Err(format!(
                    "client '{}' has an invalid redirect_uri '{uri}': {}",
                    client.id, err.message
                ));
            }
        }
        for uri in client.post_logout_redirect_uris.iter().flatten() {
            if let Err(err) = validate_post_logout_redirect_uri_shape(uri) {
                return Err(format!(
                    "client '{}' has an invalid post_logout_redirect_uri '{uri}': {}",
                    client.id, err.message
                ));
            }
        }
    }
    Ok(())
}

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
            let mut plain = match secret {
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
                ClientSecret::Generate => {
                    let mut plain = secure_random_alnum(SECRET_LEN_CLIENTS);
                    let bootstrap = &RauthyConfig::get().vars.bootstrap;
                    if let Err(err) = crate::migration::bootstrap::generated_secrets::upsert_secret(
                        bootstrap.generated_secrets_file.as_ref(),
                        bootstrap.generated_secrets_ttl,
                        GeneratedSecretEntry::new(
                            GeneratedSecretKey::new("client", &client.id, "secret"),
                            plain.clone(),
                        ),
                    )
                    .await
                    {
                        plain.zeroize();
                        return Err(err);
                    }
                    plain
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
            plain.zeroize();

            (Some(kid), Some(enc))
        } else {
            (None, None)
        };

        let redirect_uris = opt_vec_to_csv(&Some(client.redirect_uris)).unwrap();
        let post_logout_redirect_uris = opt_vec_to_csv(&client.post_logout_redirect_uris);
        let allowed_origins = opt_vec_to_csv(&client.allowed_origins);

        let flows_enabled = GrantType::csv(&client.flows_enabled);
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

#[cfg(test)]
mod tests {
    use super::*;

    fn client(id: &str, redirect_uri: &str) -> Client {
        serde_json::from_value(serde_json::json!({
            "id": id,
            "redirect_uris": ["https://localhost/callback", redirect_uri],
            "enabled": true,
            "flows_enabled": ["authorization_code"],
            "access_token_alg": "EdDSA",
            "id_token_alg": "EdDSA",
            "auth_code_lifetime": 10,
            "access_token_lifetime": 900,
            "scopes": ["openid"],
            "default_scopes": ["openid"],
            "force_mfa": false
        }))
        .unwrap()
    }

    #[test]
    fn test_validate_redirect_uris() {
        let valid = || client("valid", "https://localhost/other?foo=bar");
        assert!(validate_client_uris(&[valid()]).is_ok());

        // an invalid client anywhere in the list fails the whole set before any write
        for uri in [
            "https://localhost/#/cb",
            "https://localhost/cb?iss=x",
            // would be split into `https://localhost/cb?a=` and `iss=x` once stored
            "https://localhost/cb?a=,iss=x",
            "https://localhost/cb,https://localhost/cb?iss=x",
        ] {
            let clients = [valid(), client("invalid", uri)];
            let err = validate_client_uris(&clients).unwrap_err();
            assert!(err.contains("'invalid'"), "{err}");
            assert!(err.contains(uri), "{err}");
        }
    }

    #[test]
    fn test_validate_post_logout_redirect_uris() {
        let with_post_logout = |id: &str, uri: &str| {
            let mut c = client(id, "https://localhost/other");
            c.post_logout_redirect_uris =
                Some(vec!["https://localhost/".to_string(), uri.to_string()]);
            c
        };

        // only `state` is reserved for a post-logout redirect URI
        let valid = || with_post_logout("valid", "https://localhost/bye?foo=bar&code=x&iss=y");
        assert!(validate_client_uris(&[valid()]).is_ok());

        for uri in [
            "https://localhost/#/bye",
            "https://localhost/bye?state=x",
            "https://localhost/bye?STATE%5B%5D=x",
            // would be split into `https://localhost/bye?a=` and `state=x` once stored
            "https://localhost/bye?a=,state=x",
        ] {
            let clients = [valid(), with_post_logout("invalid", uri)];
            let err = validate_client_uris(&clients).unwrap_err();
            assert!(err.contains("'invalid'"), "{err}");
            assert!(err.contains("post_logout_redirect_uri"), "{err}");
            assert!(err.contains(uri), "{err}");
        }
    }
}
