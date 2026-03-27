use crate::database::DB;
use crate::rauthy_config::RauthyConfig;
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Algorithm, Argon2, PasswordHasher, Version};
use hiqlite::macros::params;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::{get_rand, new_store_id};
use rauthy_error::ErrorResponse;
use tracing::info;

pub async fn bootstrap() -> Result<(), ErrorResponse> {
    // check if we should use manually provided bootstrap values
    let issuer = &RauthyConfig::get().issuer;
    let bootstrap = &RauthyConfig::get().vars.bootstrap;
    let email = &bootstrap.admin_email;
    let hash = match bootstrap.pasword_argon2id.clone() {
        Some(hash) => {
            info!(
                r#"

    First-Time setup - an already hashed bootstrap password has been given for '{email}'

    Please change it immediately: {issuer}/account
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

    Please change it immediately: {issuer}/account
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

    Please change it immediately: {issuer}/account
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

    Ok(())
}
