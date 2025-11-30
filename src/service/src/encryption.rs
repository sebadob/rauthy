use cryptr::{EncKeys, EncValue};
use rauthy_data::entity::api_keys::ApiKeyEntity;
use rauthy_data::entity::auth_providers::AuthProvider;
use rauthy_data::entity::clients::Client;
use rauthy_data::entity::clients_scim::ClientScim;
use rauthy_data::entity::jwk::JWKS;
use rauthy_error::ErrorResponse;
use tracing::{error, info};

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

        let dec = EncValue::try_from(client.secret.unwrap())?.decrypt()?;
        let enc = EncValue::encrypt_with_key_id(dec.as_ref(), new_kid.to_string())?
            .into_bytes()
            .to_vec();

        client.secret = Some(enc);
        client.secret_kid = Some(new_kid.to_string());
        client.save().await?;
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
        // secret
        let dec = EncValue::try_from(api_key.secret)?.decrypt()?;
        api_key.secret = EncValue::encrypt_with_key_id(dec.as_ref(), new_kid.to_string())?
            .into_bytes()
            .to_vec();

        // access rights
        let dec = EncValue::try_from(api_key.access)?.decrypt()?;
        api_key.access = EncValue::encrypt_with_key_id(dec.as_ref(), new_kid.to_string())?
            .into_bytes()
            .to_vec();

        api_key.enc_key_id = new_kid.to_string();

        api_key.save().await?;
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
            client.bearer_token.as_bytes(),
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

    info!(
        "Finished secrets migration to key id: {new_kid} after {} ms. Modified {modified} \
        encryption's",
        start.elapsed().as_millis(),
    );

    Ok(())
}
