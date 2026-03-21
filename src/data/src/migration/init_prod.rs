use crate::database::DB;
use crate::entity::api_keys::ApiKeyEntity;
use crate::entity::jwk::{Jwk, JwkKeyPairAlg};
use crate::rauthy_config::RauthyConfig;
use actix_web::web;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Algorithm, Argon2, PasswordHasher, Version};
use cryptr::{EncKeys, EncValue};
use hiqlite_macros::params;
use rauthy_api_types::api_keys::ApiKeyRequest;
use rauthy_common::utils::{base64_decode, get_rand, new_store_id};
use rauthy_common::{is_hiqlite, sha256};
use rauthy_error::ErrorResponse;
use rsa::pkcs8::EncodePrivateKey;
use time::OffsetDateTime;
use tracing::{debug, info};
use validator::Validate;

/// Initializes an empty production database for a new deployment
pub async fn migrate_init_prod() -> Result<(), ErrorResponse> {
    // check if the database is un-initialized by looking at the jwks table, which should be empty
    let sql = "SELECT * FROM JWKS";
    let jwks: Vec<Jwk> = if is_hiqlite() {
        DB::hql().query_as(sql, params!()).await?
    } else {
        DB::pg_query(sql, &[], 8).await?
    };

    if jwks.is_empty() {
        info!("Initializing empty production database");

        // For initializing a prod database, we need to:
        // - delete init_admin and client
        // - set new random password for admin and log to console with the first startup
        // - generate a new set of JWKs

        // cleanup
        let sql_1 = "DELETE FROM clients WHERE id = 'init_client'";
        let sql_2 =
            "DELETE FROM users WHERE email IN ('init_admin@localhost', 'test_admin@localhost')";

        if is_hiqlite() {
            DB::hql().execute(sql_1, params!()).await?;
            DB::hql().execute(sql_2, params!()).await?;
        } else {
            DB::pg_execute(sql_1, &[]).await?;
            DB::pg_execute(sql_2, &[]).await?;
        }

        // check if we should use manually provided bootstrap values
        let issuer = &RauthyConfig::get().issuer;
        let bootstrap = &RauthyConfig::get().vars.bootstrap;
        let email = &bootstrap.admin_email;
        let hash = match bootstrap.pasword_argon2id.clone() {
            Some(hash) => {
                info!(
                    r#"

    First-Time setup - an already hashed bootstrap password has been given for '{email}'

    Please change it immediately: {issuer}account
    You will never see this message again!
        "#
                );
                hash
            }
            None => {
                let plain = match bootstrap.password_plain.clone() {
                    Some(plain) => {
                        info!(
                            r#"

    First-Time setup - a bootstrap password has been given for '{email}'

    Please change it immediately: {issuer}account
    You will never see this message again!
        "#
                        );
                        plain
                    }
                    None => {
                        let plain = get_rand(24);
                        info!(
                            r#"

    First-Time setup - new random password for '{email}':

    {plain}

    Please change it immediately: {issuer}account
    You will never see this message again!
        "#
                        );
                        plain
                    }
                };

                let params = RauthyConfig::get().argon2_params.clone();
                let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
                let salt = SaltString::generate(&mut OsRng);
                argon2
                    .hash_password(plain.as_bytes(), &salt)
                    .expect("Error hashing the Password")
                    .to_string()
            }
        };

        let sql = r#"
UPDATE users
SET id = $1, email = $2, password = $3
WHERE email = 'admin@localhost'"#;

        // We want the same user ID in tests all the time, but generate a new one for a fresh prod
        // init to reduce the possible attack surface even further. As long as a user provides a
        // `BOOTSTRAP_ADMIN_EMAIL` (maybe force the existence in the future?), the default admin
        // for new instances is impossible to guess.
        let new_id = new_store_id();
        if is_hiqlite() {
            DB::hql().execute(sql, params!(new_id, email, hash)).await?;
        } else {
            DB::pg_execute(sql, &[&new_id, &email, &hash]).await?;
        }

        // now check if we should bootstrap an API Key
        if let Some(api_key_raw) = RauthyConfig::get().vars.bootstrap.api_key.as_ref() {
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
        }

        let enc_key_active = &EncKeys::get_static().enc_key_active;

        // generate JWKs
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

        info!("Production database initialized successfully");
    }

    Ok(())
}
