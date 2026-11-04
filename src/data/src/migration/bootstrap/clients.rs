use crate::database::DB;
use crate::entity::clients::{Client as ClientEntity, SQL_INSERT};
use crate::entity::clients_scim::ClientScim;
use crate::entity::scopes::Scope;
use crate::migration::bootstrap::bootstrap_data;
use crate::migration::bootstrap::generated_secrets::{GeneratedSecretEntry, GeneratedSecretKey};
use crate::migration::bootstrap::types::{Client, ClientSecret};
use crate::rauthy_config::RauthyConfig;
use chrono::Utc;
use cryptr::utils::secure_random_alnum;
use cryptr::{EncKeys, EncValue};
use hiqlite::macros::params;
use itertools::Itertools;
use rauthy_api_types::oidc::GrantType;
use rauthy_common::constants::SECRET_LEN_CLIENTS;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::base64_decode;
use rauthy_common::validation::validate_redirect_uri;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::collections::HashMap;
use tracing::info;
use zeroize::Zeroize;

/// Validates the `redirect_uris` and `post_logout_redirect_uris` of all bootstrap clients. This
/// must run before anything is written during bootstrap, so that an invalid config fails cleanly
/// and can be fixed.
pub async fn validate() -> Result<(), ErrorResponse> {
    load().await.map(|_| ())
}

fn validate_client_uris(clients: &[Client]) -> Result<(), String> {
    for client in clients {
        for uri in &client.redirect_uris {
            if let Err(err) = validate_redirect_uri(uri, true, true) {
                return Err(format!(
                    "client '{}' has an invalid redirect_uri '{uri}': {}",
                    client.id, err.message
                ));
            }
        }
        for uri in client.post_logout_redirect_uris.iter().flatten() {
            if let Err(err) = validate_redirect_uri(uri, true, true) {
                return Err(format!(
                    "client '{}' has an invalid post_logout_redirect_uri '{uri}': {}",
                    client.id, err.message
                ));
            }
        }
    }
    Ok(())
}

/// Loads `clients.json` and validates its URIs.
pub async fn load() -> Result<Vec<Client>, ErrorResponse> {
    let clients = bootstrap_data!(Client, "clients");
    if let Err(err) = validate_client_uris(&clients) {
        panic!("Validation error when bootstrapping clients: {err}");
    }
    Ok(clients)
}

pub async fn bootstrap() -> Result<(), ErrorResponse> {
    let clients = load().await?;
    let scopes = scope_names().await?;

    let len = clients.len();
    for client in clients {
        let entity = to_entity(&client, &scopes, None).await?;
        insert(&entity).await?;
        upsert_scim(client).await?;
    }

    info!("Migrated {len} clients.");

    Ok(())
}

/// Creates missing and updates changed clients. Never deletes anything.
pub async fn reconcile(clients: Vec<Client>) -> Result<(), ErrorResponse> {
    let existing = ClientEntity::find_all().await?;
    let existing: HashMap<&str, &ClientEntity> =
        existing.iter().map(|e| (e.id.as_str(), e)).collect();
    let scopes = scope_names().await?;
    let mut created = 0;
    let mut updated = 0;

    for client in clients {
        match existing.get(client.id.as_str()).copied() {
            Some(e) => {
                let entity = to_entity(&client, &scopes, Some(e)).await?;
                if &entity != e {
                    entity.save().await?;
                    updated += 1;
                }
            }
            None => {
                let entity = to_entity(&client, &scopes, None).await?;
                insert(&entity).await?;
                created += 1;
            }
        }
        upsert_scim(client).await?;
    }

    info!("Reconciled clients: {created} created, {updated} updated.");

    Ok(())
}

async fn scope_names() -> Result<Vec<String>, ErrorResponse> {
    Ok(Scope::find_all()
        .await?
        .into_iter()
        .map(|s| s.name)
        .collect())
}

async fn upsert_scim(client: Client) -> Result<(), ErrorResponse> {
    let Some(scim) = client.scim else {
        return Ok(());
    };
    let new = ClientScim {
        client_id: client.id,
        bearer_token: scim.bearer_token,
        base_uri: scim.base_uri,
        sync_groups: scim.sync_groups,
        group_sync_prefix: scim.group_sync_prefix,
    };
    // the token is encrypted with a new nonce on every write -> skip unchanged values
    if ClientScim::find_opt(new.client_id.clone()).await?.as_ref() == Some(&new) {
        return Ok(());
    }
    ClientScim::upsert(
        new.client_id,
        new.bearer_token.as_str(),
        new.base_uri,
        new.sync_groups,
        new.group_sync_prefix,
    )
    .await
}

/// Converts a bootstrap client into its entity. With `existing`, the values the bootstrap format
/// does not cover are kept, and so is the secret when it did not change, or when it is
/// `generate`, which is never rotated.
async fn to_entity(
    client: &Client,
    scopes: &[String],
    existing: Option<&ClientEntity>,
) -> Result<ClientEntity, ErrorResponse> {
    let existing_secret = existing.filter(|e| e.secret.is_some());

    let (kid, secret) = match &client.secret {
        None => (None, None),
        Some(ClientSecret::Generate) if existing_secret.is_some() => {
            let e = existing_secret.unwrap();
            (e.secret_kid.clone(), e.secret.clone())
        }
        Some(secret) => {
            let mut plain = match secret {
                ClientSecret::Plain(s) => s.clone(),
                ClientSecret::Encrypted(enc) => {
                    // make sure we can decrypt it
                    let bytes = base64_decode(enc)
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

            if plain.len() != SECRET_LEN_CLIENTS {
                panic!(
                    "Given client secret too short. Expected exactly {SECRET_LEN_CLIENTS} characters."
                );
            }

            // keep the stored value when the secret did not change, so an unchanged client
            // compares equal and is not written again
            let unchanged = match existing_secret {
                Some(e) => {
                    let mut current = e.get_secret_cleartext()?.unwrap_or_default();
                    let eq = current == plain;
                    current.zeroize();
                    eq
                }
                None => false,
            };

            let res = if unchanged {
                let e = existing_secret.unwrap();
                (e.secret_kid.clone(), e.secret.clone())
            } else {
                let kid = EncKeys::get_static().enc_key_active.clone();
                let enc = EncValue::encrypt_with_key_id(plain.as_ref(), kid.clone())?
                    .into_bytes()
                    .to_vec();
                (Some(kid), Some(enc))
            };
            plain.zeroize();
            res
        }
    };

    for uri in &client.redirect_uris {
        if let Err(err) = validate_redirect_uri(uri, true, true) {
            return Err(ErrorResponse::new(
                ErrorResponseType::BadRequest,
                format!(
                    "Bootstrap client '{}' has an invalid redirect_uri '{uri}': {}",
                    client.id, err.message
                ),
            ));
        }
    }
    let redirect_uris = opt_vec_to_csv(&Some(client.redirect_uris.clone())).unwrap();
    // empty values are stored as `NULL`, like `ClientEntity::save()` does
    let post_logout_redirect_uris =
        opt_vec_to_csv(&client.post_logout_redirect_uris).filter(|v| !v.is_empty());
    let allowed_origins = opt_vec_to_csv(&client.allowed_origins).filter(|v| !v.is_empty());

    let flows_enabled = GrantType::csv(&client.flows_enabled);
    let challenge = if secret.is_none() {
        Some("S256".to_string())
    } else {
        client.challenges.as_ref().map(|c| c.join(","))
    };
    let contacts = client
        .contacts
        .as_ref()
        .map(|c| c.join(","))
        .filter(|v| !v.is_empty());

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

    Ok(ClientEntity {
        id: client.id.clone(),
        name: client.name.clone(),
        enabled: client.enabled,
        confidential: secret.is_some(),
        secret,
        secret_kid: kid,
        redirect_uris,
        post_logout_redirect_uris,
        allowed_origins,
        flows_enabled,
        access_token_alg: client.access_token_alg.to_string(),
        id_token_alg: client.id_token_alg.to_string(),
        auth_code_lifetime: client.auth_code_lifetime,
        access_token_lifetime: client.access_token_lifetime,
        scopes,
        default_scopes,
        challenge,
        force_mfa: client.force_mfa,
        client_uri: client.client_uri.clone(),
        contacts,
        backchannel_logout_uri: client
            .backchannel_logout_uri
            .clone()
            .filter(|v| !v.is_empty()),
        restrict_group_prefix: client.restrict_group_prefix.clone(),
        // not covered by the bootstrap format -> keep what exists
        claims: existing.and_then(|e| e.claims.clone()),
        claims_at_root: existing.is_some_and(|e| e.claims_at_root),
        allowed_resources: existing.and_then(|e| e.allowed_resources.clone()),
        default_aud: existing.and_then(|e| e.default_aud.clone()),
        created_micros: existing
            .map(|e| e.created_micros)
            .unwrap_or_else(|| Some(Utc::now().timestamp_micros())),
    })
}

async fn insert(c: &ClientEntity) -> Result<(), ErrorResponse> {
    if is_hiqlite() {
        DB::hql()
            .execute(
                SQL_INSERT,
                params!(
                    &c.id,
                    c.name.clone(),
                    c.enabled,
                    c.confidential,
                    c.secret.clone(),
                    c.secret_kid.clone(),
                    c.redirect_uris.clone(),
                    c.post_logout_redirect_uris.clone(),
                    c.allowed_origins.clone(),
                    c.flows_enabled.clone(),
                    c.access_token_alg.clone(),
                    c.id_token_alg.clone(),
                    c.auth_code_lifetime,
                    c.access_token_lifetime,
                    c.scopes.clone(),
                    c.default_scopes.clone(),
                    c.challenge.clone(),
                    c.force_mfa,
                    c.client_uri.clone(),
                    c.contacts.clone(),
                    c.backchannel_logout_uri.clone(),
                    c.restrict_group_prefix.clone(),
                    c.allowed_resources.clone(),
                    c.default_aud.clone(),
                    c.created_micros
                ),
            )
            .await?;
    } else {
        DB::pg_execute(
            SQL_INSERT,
            &[
                &c.id,
                &c.name,
                &c.enabled,
                &c.confidential,
                &c.secret,
                &c.secret_kid,
                &c.redirect_uris,
                &c.post_logout_redirect_uris,
                &c.allowed_origins,
                &c.flows_enabled,
                &c.access_token_alg,
                &c.id_token_alg,
                &c.auth_code_lifetime,
                &c.access_token_lifetime,
                &c.scopes,
                &c.default_scopes,
                &c.challenge,
                &c.force_mfa,
                &c.client_uri,
                &c.contacts,
                &c.backchannel_logout_uri,
                &c.restrict_group_prefix,
                &c.allowed_resources,
                &c.default_aud,
                &c.created_micros,
            ],
        )
        .await?;
    }
    ClientEntity::delete_cache_for(&c.id).await?;

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

    #[tokio::test]
    async fn test_to_entity_keeps_existing_values() {
        let scopes = vec!["openid".to_string()];
        let mut existing = to_entity(&client("app", "https://localhost/cb"), &scopes, None)
            .await
            .unwrap();
        existing.secret = Some(vec![1, 2, 3]);
        existing.secret_kid = Some("kid".to_string());
        existing.claims = Some(b"{}".to_vec());
        existing.default_aud = Some("aud".to_string());

        // `generate` must never rotate an existing secret
        let mut c = client("app", "https://localhost/changed");
        c.secret = Some(ClientSecret::Generate);
        let e = to_entity(&c, &scopes, Some(&existing)).await.unwrap();

        assert_eq!(e.secret, existing.secret);
        assert_eq!(e.secret_kid, existing.secret_kid);
        assert!(e.confidential);
        // values outside the bootstrap format are kept, the rest comes from the file
        assert_eq!(e.claims, existing.claims);
        assert_eq!(e.default_aud, existing.default_aud);
        assert_eq!(e.created_micros, existing.created_micros);
        assert_eq!(
            e.redirect_uris,
            "https://localhost/callback,https://localhost/changed"
        );
    }

    #[tokio::test]
    async fn test_to_entity_fixed_secret() {
        let _ = EncKeys::generate_with_id("test".to_string())
            .unwrap()
            .init();
        let scopes = vec!["openid".to_string()];
        let secret = |s: &str| ClientSecret::Plain(s.repeat(SECRET_LEN_CLIENTS));

        let mut c = client("app", "https://localhost/cb");
        c.secret = Some(secret("a"));
        let existing = to_entity(&c, &scopes, None).await.unwrap();
        assert!(existing.confidential);

        // unchanged -> the stored ciphertext is kept, so the client compares equal
        let e = to_entity(&c, &scopes, Some(&existing)).await.unwrap();
        assert_eq!(e, existing);

        // changed -> re-encrypted with the new value
        c.secret = Some(secret("b"));
        let e = to_entity(&c, &scopes, Some(&existing)).await.unwrap();
        assert_ne!(e.secret, existing.secret);
        assert_eq!(
            e.get_secret_cleartext().unwrap(),
            Some("b".repeat(SECRET_LEN_CLIENTS))
        );
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

        let valid = || with_post_logout("valid", "https://localhost/bye?foo=bar");
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
