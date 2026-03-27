use crate::database::DB;
use crate::entity::api_keys::ApiKeyEntity;
use crate::rauthy_config::RauthyConfig;
use cryptr::EncValue;
use hiqlite::macros::params;
use rauthy_api_types::api_keys::ApiKeyRequest;
use rauthy_common::utils::base64_decode;
use rauthy_common::{is_hiqlite, sha256};
use rauthy_error::ErrorResponse;
use tracing::debug;
use validator::Validate;

pub async fn bootstrap() -> Result<(), ErrorResponse> {
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
    let _ = ApiKeyEntity::create(
        req.name,
        req.exp,
        req.access.into_iter().map(|a| a.into()).collect(),
    )
    .await?;

    if let Some(secret_plain) = RauthyConfig::get().vars.bootstrap.api_key_secret.as_ref() {
        assert!(secret_plain.len() >= 64);
        let secret_enc = EncValue::encrypt(sha256!(secret_plain.as_bytes()))?
            .into_bytes()
            .to_vec();

        let sql = "UPDATE api_keys SET secret = $1 WHERE name = $2";
        if is_hiqlite() {
            DB::hql()
                .execute(sql, params!(secret_enc, key_name))
                .await?;
        } else {
            DB::pg_execute(sql, &[&secret_enc, &key_name]).await?;
        }
    }

    Ok(())
}
