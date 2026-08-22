use cryptr::{EncKeys, EncValue};
use rauthy_data::entity::api_keys::ApiKeyEntity;
use rauthy_data::entity::auth_providers::AuthProvider;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::clients_scim::ClientScim;
use rauthy_data::entity::jwk::JWKS;
use rauthy_data::entity::kv::{KVAccess, KVValue};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use tracing::{error, info};

/// Maximum retries for a concurrent per-row migration update.
const MIGRATION_ATTEMPTS: u8 = 5;

/// Migrates encrypted data in the backend to a new key.
/// JWKS's are just rotated and a new set will be created.
pub async fn migrate_encryption_alg(new_kid: &str) -> Result<(), ErrorResponse> {
    // check that the requested Key ID exists
    EncKeys::get_static_key(new_kid)?;

    let start = tokio::time::Instant::now();
    info!("Starting secrets migration to key id: {new_kid}");

    let mut modified = 0;

    // migrate clients
    info!("Starting client secrets migration to key id: {new_kid}");
    let clients = Client::find_all()
        .await?
        .into_iter()
        // filter out all clients that already use the new key
        .filter(|c| c.secret_kid.as_deref() != Some(new_kid))
        .collect::<Vec<Client>>();
    for mut client in clients {
        if !client.confidential {
            continue;
        }

        let mut migrated = false;
        for _ in 0..MIGRATION_ATTEMPTS {
            let dec = EncValue::try_from(client.secret.clone().unwrap())?.decrypt()?;
            let enc = EncValue::encrypt_with_key_id(dec.as_ref(), new_kid.to_string())?
                .into_bytes()
                .to_vec();

            let old_kid = client.secret_kid.clone().unwrap_or_default();
            client.secret = Some(enc);
            client.secret_kid = Some(new_kid.to_string());
            if client
                .update_secret_migrated(
                    client.secret.clone().unwrap(),
                    new_kid.to_string(),
                    &old_kid,
                )
                .await?
            {
                migrated = true;
                break;
            }
            client = Client::find(client.id.clone()).await?;
        }
        if !migrated {
            return Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                format!(
                    "Secret migration failed for client '{}': the secret changed \
                     concurrently on every attempt. Please re-run the migration.",
                    client.id
                ),
            ));
        }
        modified += 1;
    }
    info!("Finished clients secrets migration to key id: {new_kid}");

    // JWKS will just be rotated, which is better for security anyway
    JWKS::rotate().await?;

    // migrate ApiKey's
    info!("Starting ApiKeys migration to key id: {new_kid}");
    let api_keys = ApiKeyEntity::find_all()
        .await?
        .into_iter()
        // filter out all keys that already use the new key
        .filter(|k| k.enc_key_id != new_kid)
        .collect::<Vec<ApiKeyEntity>>();
    for mut api_key in api_keys {
        let mut migrated = false;
        for _ in 0..MIGRATION_ATTEMPTS {
            let dec = EncValue::try_from(api_key.secret.clone())?.decrypt()?;
            let secret_enc = EncValue::encrypt_with_key_id(dec.as_ref(), new_kid.to_string())?
                .into_bytes()
                .to_vec();

            let dec = EncValue::try_from(api_key.access.clone())?.decrypt()?;
            let access_enc = EncValue::encrypt_with_key_id(dec.as_ref(), new_kid.to_string())?
                .into_bytes()
                .to_vec();

            let old_kid = api_key.enc_key_id.clone();
            if api_key
                .save_migrated(secret_enc, access_enc, new_kid.to_string(), &old_kid)
                .await?
            {
                migrated = true;
                break;
            }
            api_key = ApiKeyEntity::find(&api_key.name).await?;
        }
        if !migrated {
            return Err(ErrorResponse::new(
                ErrorResponseType::Internal,
                format!(
                    "Secret migration failed for API key '{}': the key changed \
                     concurrently on every attempt. Please re-run the migration.",
                    api_key.name
                ),
            ));
        }
        modified += 1;
    }
    info!("Finished ApiKeys migration to key id: {new_kid}");

    // migrate auth providers
    let providers = AuthProvider::find_all().await?;
    for mut provider in providers {
        match AuthProvider::secret_cleartext(&provider.secret) {
            Ok(plaintext_opt) => {
                if let Some(plaintext) = plaintext_opt {
                    provider.secret = Some(
                        EncValue::encrypt_with_key_id(plaintext.as_ref(), new_kid.to_string())?
                            .into_bytes()
                            .to_vec(),
                    );
                    provider.save().await?;

                    modified += 1;
                };
            }
            Err(err) => {
                error!(
                    ?err,
                    "decrypting AuthProvider secret, this should never happen!",
                );
            }
        }
    }
    info!(
        "Finished auth provider secrets migration to key id: {}",
        new_kid
    );

    // migrate SCIM clients
    for client in ClientScim::find_all().await? {
        // the upsert just re-encrypts the bearer token
        // and otherwise does not update anything else
        ClientScim::upsert(
            client.client_id,
            &client.bearer_token,
            client.base_uri,
            client.sync_groups,
            client.group_sync_prefix,
        )
        .await?;
    }
    info!(
        "Finished SCIM clients secrets migration to key id: {}",
        new_kid
    );

    // migrate KV Access keys
    for access in KVAccess::find_all_no_ns().await? {
        access.save_re_encrypt_secret().await?;
    }
    info!(
        "Finished KV Access Keys secrets migration to key id: {}",
        new_kid
    );

    // migrate KV Values
    for value in KVValue::find_all_encrypted().await? {
        value.save_re_encrypted().await?;
    }
    info!(
        "Finished KV Values secrets migration to key id: {}",
        new_kid
    );

    info!(
        "Finished secrets migration to key id: {new_kid} after {} ms. Modified {modified} \
        encryption's",
        start.elapsed().as_millis(),
    );

    Ok(())
}
