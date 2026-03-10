use crate::entity::jwk::{Jwk, JwkKeyPairAlg};
use actix_web::web;
use cryptr::{EncKeys, EncValue};
use rauthy_common::utils::get_rand;
use rauthy_error::ErrorResponse;
use rsa::pkcs8::EncodePrivateKey;
use time::OffsetDateTime;
use tracing::info;

pub async fn bootstrap() -> Result<(), ErrorResponse> {
    let enc_key_active = &EncKeys::get_static().enc_key_active;

    info!("Generating new JWKs - this might take a few seconds");
    let mut entities = Vec::with_capacity(4);

    // RSA256
    let jwk_plain = web::block(|| {
        let mut rng = rand_08::thread_rng();
        rsa::RsaPrivateKey::new(&mut rng, 2048)
            .unwrap()
            .to_pkcs8_der()
            .unwrap()
    })
    .await?;
    let jwk = EncValue::encrypt(jwk_plain.to_bytes().as_slice())?
        .into_bytes()
        .to_vec();
    entities.push(Jwk {
        kid: get_rand(24),
        created_at: OffsetDateTime::now_utc().unix_timestamp(),
        signature: JwkKeyPairAlg::RS256,
        enc_key_id: enc_key_active.clone(),
        jwk,
    });

    // RS384
    let jwk_plain = web::block(|| {
        let mut rng = rand_08::thread_rng();
        rsa::RsaPrivateKey::new(&mut rng, 3072)
            .unwrap()
            .to_pkcs8_der()
            .unwrap()
    })
    .await?;
    let jwk = EncValue::encrypt(jwk_plain.to_bytes().as_slice())?
        .into_bytes()
        .to_vec();
    entities.push(Jwk {
        kid: get_rand(24),
        created_at: OffsetDateTime::now_utc().unix_timestamp(),
        signature: JwkKeyPairAlg::RS384,
        enc_key_id: enc_key_active.clone(),
        jwk,
    });

    // RSA512
    let jwk_plain = web::block(|| {
        let mut rng = rand_08::thread_rng();
        rsa::RsaPrivateKey::new(&mut rng, 4096)
            .unwrap()
            .to_pkcs8_der()
            .unwrap()
    })
    .await?;
    let jwk = EncValue::encrypt(jwk_plain.to_bytes().as_slice())?
        .into_bytes()
        .to_vec();
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
