use crate::entity::jwk::{Jwk, JwkKeyPairAlg, generate_rsa_pkcs8};
use actix_web::web;
use aws_lc_rs::rsa::KeySize;
use cryptr::{EncKeys, EncValue};
use rauthy_common::utils::get_rand;
use rauthy_error::ErrorResponse;
use time::OffsetDateTime;
use tracing::info;

pub async fn bootstrap() -> Result<(), ErrorResponse> {
    let enc_key_active = &EncKeys::get_static().enc_key_active;

    info!("Generating new JWKs - this might take a few seconds");
    let mut entities = Vec::with_capacity(4);

    // RSA256
    let jwk_plain = web::block(|| generate_rsa_pkcs8(KeySize::Rsa2048)).await??;
    let jwk = EncValue::encrypt(&jwk_plain)?.into_bytes().to_vec();
    entities.push(Jwk {
        kid: get_rand(24),
        created_at: OffsetDateTime::now_utc().unix_timestamp(),
        signature: JwkKeyPairAlg::RS256,
        enc_key_id: enc_key_active.clone(),
        jwk,
    });

    // RS384
    let jwk_plain = web::block(|| generate_rsa_pkcs8(KeySize::Rsa3072)).await??;
    let jwk = EncValue::encrypt(&jwk_plain)?.into_bytes().to_vec();
    entities.push(Jwk {
        kid: get_rand(24),
        created_at: OffsetDateTime::now_utc().unix_timestamp(),
        signature: JwkKeyPairAlg::RS384,
        enc_key_id: enc_key_active.clone(),
        jwk,
    });

    // RSA512
    let jwk_plain = web::block(|| generate_rsa_pkcs8(KeySize::Rsa4096)).await??;
    let jwk = EncValue::encrypt(&jwk_plain)?.into_bytes().to_vec();
    entities.push(Jwk {
        kid: get_rand(24),
        created_at: OffsetDateTime::now_utc().unix_timestamp(),
        signature: JwkKeyPairAlg::RS512,
        enc_key_id: enc_key_active.clone(),
        jwk,
    });

    // Ed25519
    let jwk_plain = web::block(ed25519_compact::KeyPair::generate).await?;
    let jwk = EncValue::encrypt(jwk_plain.sk.to_der().as_slice())?
        .into_bytes()
        .to_vec();
    entities.push(Jwk {
        kid: get_rand(24),
        created_at: OffsetDateTime::now_utc().unix_timestamp(),
        signature: JwkKeyPairAlg::EdDSA,
        enc_key_id: enc_key_active.clone(),
        jwk,
    });

    for e in entities {
        e.save().await?;
    }

    Ok(())
}
