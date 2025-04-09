use crate::database::DB;
use crate::entity::api_keys::ApiKeyEntity;
use crate::entity::jwk::{Jwk, JwkKeyPairAlg};
use actix_web::web;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use cryptr::{EncKeys, EncValue};
use hiqlite::{Param, params};
use jwt_simple::algorithms::{
    Ed25519KeyPair, EdDSAKeyPairLike, RS256KeyPair, RS384KeyPair, RS512KeyPair, RSAKeyPairLike,
};
use rand_core::OsRng;
use rauthy_api_types::api_keys::ApiKeyRequest;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::{base64_decode, get_rand};
use rauthy_error::ErrorResponse;
use ring::digest;
use std::env;
use time::OffsetDateTime;
use tracing::{debug, info};
use validator::Validate;

/// Initializes an empty production database for a new deployment
pub async fn migrate_init_prod(argon2_params: Params, issuer: &str) -> Result<(), ErrorResponse> {
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
        if is_hiqlite() {
            DB::hql()
                .execute("DELETE FROM clients WHERE id = 'init_client'", params!())
                .await?;
            DB::hql().execute("DELETE FROM users WHERE email IN ('init_admin@localhost.de', 'test_admin@localhost.de')", params!())
                .await?;
        } else {
            sqlx::query!("DELETE FROM clients WHERE id = 'init_client'")
                .execute(DB::conn_sqlx())
                .await?;

            sqlx::query!(
                "DELETE FROM users WHERE email IN ('init_admin@localhost.de', 'test_admin@localhost.de')",
            )
                .execute(DB::conn_sqlx())
                .await?;
        }

        // check if we should use manually provided bootstrap values
        let email =
            env::var("BOOTSTRAP_ADMIN_EMAIL").unwrap_or_else(|_| "admin@localhost.de".to_string());
        let hash = match env::var("BOOTSTRAP_ADMIN_PASSWORD_ARGON2ID") {
            Ok(hash) => {
                info!(
                    r#"

    First-Time setup - an already hashed bootstrap password has been given for '{email}'

    Please change it immediately: {issuer}/account
    You will never see this message again!
        "#
                );
                hash
            }
            Err(_) => {
                let plain = match env::var("BOOTSTRAP_ADMIN_PASSWORD_PLAIN") {
                    Ok(plain) => {
                        info!(
                            r#"

    First-Time setup - a bootstrap password has been given for '{email}'

    Please change it immediately: {issuer}/account
    You will never see this message again!
        "#
                        );
                        plain
                    }
                    Err(_) => {
                        let plain = get_rand(24);
                        info!(
                            r#"

    First-Time setup - new random password for '{email}':

    {plain}

    Please change it immediately: {issuer}/account
    You will never see this message again!
        "#
                        );
                        plain
                    }
                };

                let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, argon2_params);
                let salt = SaltString::generate(&mut OsRng);
                argon2
                    .hash_password(plain.as_bytes(), &salt)
                    .expect("Error hashing the Password")
                    .to_string()
            }
        };

        if is_hiqlite() {
            // TODO we could grab a possibly existing `RAUTHY_ADMIN_EMAIL` and initialize a custom
            // admin
            DB::hql()
                .execute(
                    "UPDATE users SET email = $1, password = $2 WHERE email = 'admin@localhost.de'",
                    params!(email, hash),
                )
                .await?;
        } else {
            sqlx::query!(
                "UPDATE users SET email = $1, password = $2 WHERE email = 'admin@localhost.de'",
                email,
                hash
            )
            .execute(DB::conn_sqlx())
            .await?;
        }

        // now check if we should bootstrap an API Key
        if let Ok(api_key_raw) = env::var("BOOTSTRAP_API_KEY") {
            // this is expected to be base64 encoded
            let bytes = base64_decode(&api_key_raw)?;
            // ... and then a JSON string
            let s = String::from_utf8_lossy(&bytes);
            let req = serde_json::from_str::<ApiKeyRequest>(&s)?;
            req.validate()
                .expect("Invalid API Key in BOOTSTRAP_API_KEY");

            debug!("Bootstrapping API Key:\n{:?}", req);
            let key_name = req.name.clone();
            let _ = ApiKeyEntity::create(
                req.name,
                req.exp,
                req.access.into_iter().map(|a| a.into()).collect(),
            )
            .await?;

            if let Ok(secret_plain) = env::var("BOOTSTRAP_API_KEY_SECRET") {
                assert!(secret_plain.len() >= 64);
                let secret_hash = digest::digest(&digest::SHA256, secret_plain.as_bytes());
                let secret_enc = EncValue::encrypt(secret_hash.as_ref())?
                    .into_bytes()
                    .to_vec();

                if is_hiqlite() {
                    DB::hql()
                        .execute(
                            "UPDATE api_keys SET secret = $1 WHERE name = $2",
                            params!(secret_enc, key_name),
                        )
                        .await?;
                } else {
                    sqlx::query!(
                        "UPDATE api_keys SET secret = $1 WHERE name = $2",
                        secret_enc,
                        key_name,
                    )
                    .execute(DB::conn_sqlx())
                    .await?;
                }
            }
        }

        let enc_key_active = &EncKeys::get_static().enc_key_active;

        // generate JWKs
        info!("Generating new JWKs - this might take a few seconds");
        let mut entities = Vec::with_capacity(4);

        // RSA256
        let jwk_plain = web::block(|| {
            RS256KeyPair::generate(2048)
                .unwrap()
                .with_key_id(&get_rand(24))
        })
        .await?;
        let jwk = EncValue::encrypt(jwk_plain.to_der().unwrap().as_slice())?
            .into_bytes()
            .to_vec();
        entities.push(Jwk {
            kid: jwk_plain.key_id().as_ref().unwrap().clone(),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            signature: JwkKeyPairAlg::RS256,
            enc_key_id: enc_key_active.clone(),
            jwk,
        });

        // RS384
        let jwk_plain = web::block(|| {
            RS384KeyPair::generate(3072)
                .unwrap()
                .with_key_id(&get_rand(24))
        })
        .await?;
        let jwk = EncValue::encrypt(jwk_plain.to_der().unwrap().as_slice())?
            .into_bytes()
            .to_vec();
        entities.push(Jwk {
            kid: jwk_plain.key_id().as_ref().unwrap().clone(),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            signature: JwkKeyPairAlg::RS384,
            enc_key_id: enc_key_active.clone(),
            jwk,
        });

        // RSA512
        let jwk_plain = web::block(|| {
            RS512KeyPair::generate(4096)
                .unwrap()
                .with_key_id(&get_rand(24))
        })
        .await?;
        let jwk = EncValue::encrypt(jwk_plain.to_der().unwrap().as_slice())?
            .into_bytes()
            .to_vec();
        entities.push(Jwk {
            kid: jwk_plain.key_id().as_ref().unwrap().clone(),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            signature: JwkKeyPairAlg::RS512,
            enc_key_id: enc_key_active.clone(),
            jwk,
        });

        // Ed25519
        let jwk_plain =
            web::block(|| Ed25519KeyPair::generate().with_key_id(&get_rand(24))).await?;
        let jwk = EncValue::encrypt(jwk_plain.to_der().as_slice())?
            .into_bytes()
            .to_vec();
        entities.push(Jwk {
            kid: jwk_plain.key_id().as_ref().unwrap().clone(),
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
