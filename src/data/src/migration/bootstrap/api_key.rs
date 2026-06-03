use crate::database::DB;
use crate::entity::api_keys::ApiKeyEntity;
use crate::migration::bootstrap::bootstrap_data;
use crate::migration::bootstrap::types::{ApiKey as BootstrapApiKey, ApiKeySecret};
use crate::rauthy_config::RauthyConfig;
use cryptr::EncValue;
use hiqlite::macros::params;
use rauthy_api_types::api_keys::ApiKeyRequest;
use rauthy_common::constants::API_KEY_LENGTH;
use rauthy_common::utils::base64_decode;
use rauthy_common::{is_hiqlite, sha256};
use rauthy_error::ErrorResponse;
use tracing::{debug, info};
use validator::Validate;
use zeroize::Zeroize;

pub async fn bootstrap() -> Result<(), ErrorResponse> {
    bootstrap_legacy_config_api_key().await?;
    bootstrap_api_keys_json().await?;

    Ok(())
}

async fn bootstrap_legacy_config_api_key() -> Result<(), ErrorResponse> {
    let Some(api_key_raw) = RauthyConfig::get().vars.bootstrap.api_key.as_ref() else {
        return Ok(());
    };

    // this is expected to be base64 encoded
    let bytes = base64_decode(api_key_raw)?;
    // ... and then a JSON string
    let s = String::from_utf8_lossy(&bytes);
    let req = serde_json::from_str::<ApiKeyRequest>(&s)?;
    req.validate()
        .expect("Invalid API Key in BOOTSTRAP_API_KEY");

    debug!("Bootstrapping API Key:\n{req:?}");
    let key_name = req.name.clone();
    let mut generated_secret = ApiKeyEntity::create(
        req.name,
        req.exp,
        req.access.into_iter().map(|a| a.into()).collect(),
    )
    .await?;
    generated_secret.zeroize();

    if let Some(secret_plain) = RauthyConfig::get().vars.bootstrap.api_key_secret.as_ref() {
        set_api_key_secret(&key_name, secret_plain).await?;
    }

    Ok(())
}

async fn bootstrap_api_keys_json() -> Result<(), ErrorResponse> {
    let api_keys = bootstrap_data!(BootstrapApiKey, "api_keys");

    let len = api_keys.len();
    for api_key in api_keys {
        let key_name = api_key.name.clone();
        debug!("Bootstrapping API Key: {key_name}");

        let mut generated_secret = ApiKeyEntity::create(
            api_key.name,
            api_key.exp,
            api_key.access.into_iter().map(|a| a.into()).collect(),
        )
        .await?;
        generated_secret.zeroize();

        let mut secret_plain = api_key_secret_plain(api_key.secret);
        set_api_key_secret(&key_name, &secret_plain).await?;
        secret_plain.zeroize();
    }

    info!("Migrated {len} API keys.");

    Ok(())
}

async fn set_api_key_secret(name: &str, secret_plain: &str) -> Result<(), ErrorResponse> {
    assert!(
        secret_plain.len() >= API_KEY_LENGTH,
        "Given API Key secret too short. Expected at least {API_KEY_LENGTH} characters."
    );

    let secret_enc = EncValue::encrypt(sha256!(secret_plain.as_bytes()))?
        .into_bytes()
        .to_vec();

    let sql = "UPDATE api_keys SET secret = $1 WHERE name = $2";
    if is_hiqlite() {
        DB::hql()
            .execute(sql, params!(secret_enc, name.to_string()))
            .await?;
    } else {
        DB::pg_execute(sql, &[&secret_enc, &name]).await?;
    }

    Ok(())
}

fn api_key_secret_plain(secret: ApiKeySecret) -> String {
    match secret {
        ApiKeySecret::Plain(s) => s,
        ApiKeySecret::Encrypted(enc) => {
            let bytes = base64_decode(&enc)
                .expect("Cannot decode base64 encoded, encrypted API Key secret");
            let dec = EncValue::try_from(bytes)
                .expect("Invalid format for encrypted API Key secret")
                .decrypt()
                .expect("Failed to decrypt encrypted API Key secret. Make sure Rauthy has access to the used ENC_KEY");
            String::from_utf8(dec.to_vec())
                .expect("Invalid characters in API Key secret. Cannot convert to lossless String.")
        }
    }
}

impl From<crate::migration::bootstrap::types::ApiKeyAccess>
    for crate::entity::api_keys::ApiKeyAccess
{
    fn from(value: crate::migration::bootstrap::types::ApiKeyAccess) -> Self {
        Self {
            group: value.group.into(),
            access_rights: value
                .access_rights
                .into_iter()
                .map(|ar| ar.into())
                .collect(),
        }
    }
}
