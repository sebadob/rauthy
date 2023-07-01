use crate::auth;
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::{decrypt, encrypt};
use rauthy_models::app_state::AppState;
use rauthy_models::entity::clients::Client;
use tracing::info;

/// Migrates encrypted data in the backend to a new key.
/// JWKS's are just rotated and a new set will be created.
pub async fn migrate_encryption_alg(
    data: &actix_web::web::Data<AppState>,
    new_kid: &str,
) -> Result<(), ErrorResponse> {
    let new_enc_key = data.enc_keys.get(new_kid).ok_or_else(|| {
        ErrorResponse::new(
            ErrorResponseType::BadRequest,
            format!("Encryption key id {} does not exist", new_kid),
        )
    })?;

    let start = tokio::time::Instant::now();
    info!("Starting secrets migration to key id: {}", new_kid);

    let mut modified = 0;

    // migrate clients
    info!("Starting client secrets migration to key id: {}", new_kid);
    let clients = Client::find_all(data).await?;
    for mut client in clients {
        if !client.confidential {
            continue;
        }

        let key = data
            .enc_keys
            .get(client.secret_kid.as_ref().expect("client secret_kid"))
            .expect("client secret_kid in AppState");
        let dec = decrypt(client.secret.as_ref().unwrap(), key)?;
        let enc = encrypt(&dec, new_enc_key).unwrap();

        client.secret = Some(enc);
        client.secret_kid = Some(new_kid.to_string());
        client.save(data, None).await?;
        modified += 1;
    }
    info!("Finished clients secrets migration to key id: {}", new_kid);

    // JWKS will just be rotated, which is better for security anyways
    auth::rotate_jwks(data).await?;

    info!(
        "Finished secrets migration to key id: {} after {} ms. Modified {} rows",
        new_kid,
        start.elapsed().as_millis(),
        modified,
    );

    Ok(())
}
