use crate::entity::clients::Client;
use crate::entity::colors::ColorEntity;
use crate::entity::groups::Group;
use crate::entity::jwk::{Jwk, JwkKeyPairType};
use crate::entity::magic_links::MagicLinkPassword;
use crate::entity::password::RecentPasswordsEntity;
use crate::entity::refresh_tokens::RefreshToken;
use crate::entity::roles::Role;
use crate::entity::scopes::Scope;
use crate::entity::sessions::Session;
use crate::entity::user_attr::{UserAttrConfigEntity, UserAttrValueEntity};
use crate::entity::users::User;
use crate::entity::webauthn::PasskeyEntity;
use actix_web::web;
use argon2::password_hash::SaltString;
use argon2::{Algorithm, Argon2, Params, PasswordHasher, Version};
use jwt_simple::algorithms::{
    Ed25519KeyPair, EdDSAKeyPairLike, RS256KeyPair, RS384KeyPair, RS512KeyPair, RSAKeyPairLike,
};
use rand_core::OsRng;
use rauthy_common::constants::{DB_TYPE, DEV_MODE};
use rauthy_common::error_response::ErrorResponse;
use rauthy_common::utils::{encrypt, get_rand};
use rauthy_common::DbType;
use sqlx::Row;
use time::OffsetDateTime;
use tracing::{debug, info};

pub async fn anti_lockout(db: &sqlx::Pool<sqlx::Any>, issuer: &str) -> Result<(), ErrorResponse> {
    debug!("Executing anti_lockout_check");

    let (redirect_uris, allowed_origins) = if *DEV_MODE {
        (
            format!("{issuer}/oidc/*,http://localhost:5173/*"),
            Some("http://localhost:5173".to_string()),
        )
    } else {
        (format!("{issuer}/*"), None)
    };

    let rauthy = Client {
        id: "rauthy".to_string(),
        name: Some("Rauthy".to_string()),
        enabled: true,
        confidential: false,
        secret: None,
        secret_kid: None,
        redirect_uris: redirect_uris.clone(),
        post_logout_redirect_uris: Some(redirect_uris),
        allowed_origins,
        flows_enabled: "authorization_code,password".to_string(),
        access_token_alg: "EdDSA".to_string(),
        id_token_alg: "EdDSA".to_string(),
        refresh_token: false,
        auth_code_lifetime: 60,
        access_token_lifetime: 1800,
        scopes: "openid".to_string(),
        default_scopes: "openid".to_string(),
        challenge: Some("S256".to_string()),
    };

    match *DB_TYPE {
        DbType::Sqlite => {
            sqlx::query(r#"insert or replace into clients (id, name, enabled, confidential,
            secret, secret_kid, redirect_uris, post_logout_redirect_uris, allowed_origins,
            flows_enabled, access_token_alg, id_token_alg, refresh_token, auth_code_lifetime,
            access_token_lifetime, scopes, default_scopes, challenge)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)"#)
        }
        DbType::Postgres => {
            sqlx::query(r#"insert into clients (id, name, enabled, confidential, secret, secret_kid,
            redirect_uris, post_logout_redirect_uris, allowed_origins, flows_enabled,
            access_token_alg, id_token_alg, refresh_token, auth_code_lifetime,
            access_token_lifetime, scopes, default_scopes, challenge)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            on conflict(id) do update set name = $2, enabled = $3, confidential = $4, secret = $5,
            secret_kid = $6, redirect_uris = $7, post_logout_redirect_uris = $8, allowed_origins = $9,
            flows_enabled = $10, access_token_alg = $11, id_token_alg = $12, refresh_token = $13,
            auth_code_lifetime = $14, access_token_lifetime = $15, scopes = $16, default_scopes = $17,
            challenge = $18"#)
        }
    }
        .bind(&rauthy.id)
        .bind(&rauthy.name)
        .bind(rauthy.enabled)
        .bind(rauthy.confidential)
        .bind(&rauthy.secret)
        .bind(&rauthy.secret_kid)
        .bind(&rauthy.redirect_uris)
        .bind(&rauthy.post_logout_redirect_uris)
        .bind(&rauthy.allowed_origins)
        .bind(&rauthy.flows_enabled)
        .bind(&rauthy.access_token_alg)
        .bind(&rauthy.id_token_alg)
        .bind(rauthy.refresh_token)
        .bind(rauthy.auth_code_lifetime)
        .bind(rauthy.access_token_lifetime)
        .bind(&rauthy.scopes)
        .bind(&rauthy.default_scopes)
        .bind(&rauthy.challenge)
        .execute(db)
        .await?;

    Ok(())
}

/// Initializes an empty production database for a new deployment
pub async fn migrate_init_prod(
    db: &sqlx::Pool<sqlx::Any>,
    enc_key_active: String,
    enc_key: &[u8],
    argon2_params: Params,
    issuer: &str,
) -> Result<(), ErrorResponse> {
    // check if the database is un-initialized by looking at the jwks table, which should be empty
    let jwks = sqlx::query_as::<_, Jwk>("select * from jwks")
        .fetch_all(db)
        .await?;
    if jwks.is_empty() {
        info!("Initializing empty production database");

        // For initializing a prod database, we need to:
        // - delete init_admin and client
        // - set new random password for admin and log to console with the first startup
        // - generate a new set of JWKs

        // cleanup
        sqlx::query("delete from clients where id = 'init_client'")
            .execute(db)
            .await?;

        sqlx::query(
            "delete from users where email in ('init_admin@localhost.de', 'test_admin@localhost.de')",
        )
        .execute(db)
        .await?;

        // generate new admin password
        let plain = get_rand(24);
        info!(
            r#"

    First-Time setup - new random password for 'admin@localhost.de':

    {plain}

    Please change it immediately: {issuer}/account
    You will never see this message again!
        "#
        );

        let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, argon2_params);
        let salt = SaltString::generate(&mut OsRng);
        let hash = argon2
            .hash_password(plain.as_bytes(), &salt)
            .expect("Error hashing the Password")
            .to_string();

        sqlx::query("update users set password = $1 where email = 'admin@localhost.de'")
            .bind(hash)
            .execute(db)
            .await?;

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
        let jwk = encrypt(jwk_plain.to_der().unwrap().as_slice(), enc_key)?;
        entities.push(Jwk {
            kid: jwk_plain.key_id().as_ref().unwrap().clone(),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            signature: JwkKeyPairType::RS256,
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
        let jwk = encrypt(jwk_plain.to_der().unwrap().as_slice(), enc_key)?;
        entities.push(Jwk {
            kid: jwk_plain.key_id().as_ref().unwrap().clone(),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            signature: JwkKeyPairType::RS384,
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
        let jwk = encrypt(jwk_plain.to_der().unwrap().as_slice(), enc_key)?;
        entities.push(Jwk {
            kid: jwk_plain.key_id().as_ref().unwrap().clone(),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            signature: JwkKeyPairType::RS512,
            enc_key_id: enc_key_active.clone(),
            jwk,
        });

        // Ed25519
        let jwk_plain =
            web::block(|| Ed25519KeyPair::generate().with_key_id(&get_rand(24))).await?;
        let jwk = encrypt(jwk_plain.to_der().as_slice(), enc_key)?;
        entities.push(Jwk {
            kid: jwk_plain.key_id().as_ref().unwrap().clone(),
            created_at: OffsetDateTime::now_utc().unix_timestamp(),
            signature: JwkKeyPairType::EdDSA,
            enc_key_id: enc_key_active,
            jwk,
        });

        for e in entities {
            e.save(db).await?;
        }

        info!("Production database initialized successfully");
    }

    Ok(())
}

/// Migrates `MIGRATE_DB_FROM` to `DATABASE_URL`
pub async fn migrate(
    db_from: &sqlx::Pool<sqlx::Any>,
    db_to: &sqlx::Pool<sqlx::Any>,
) -> Result<(), ErrorResponse> {
    info!("Starting migration to another DB");

    // WEBNAUTHN
    let before = sqlx::query_as::<_, PasskeyEntity>("select * from webauthn")
        .fetch_all(db_from)
        .await?;
    sqlx::query("delete from webauthn").execute(db_to).await?;
    for b in before {
        sqlx::query("insert into webauthn (id, passkey) values ($1, $2)")
            .bind(b.id)
            .bind(b.passkey)
            .execute(db_to)
            .await?;
    }

    // USERS
    let before = sqlx::query_as::<_, User>("select * from users")
        .fetch_all(db_from)
        .await?;
    sqlx::query("delete from users").execute(db_to).await?;
    for b in before {
        sqlx::query(
            r#"insert into users(id, email, given_name, family_name, password, roles, groups,
            enabled, email_verified, password_expires, created_at, last_login, last_failed_login,
            failed_login_attempts, mfa_app, sec_key_1, sec_key_2)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)"#,
        )
        .bind(b.id)
        .bind(b.email)
        .bind(b.given_name)
        .bind(b.family_name)
        .bind(b.password)
        .bind(b.roles)
        .bind(b.groups)
        .bind(b.enabled)
        .bind(b.email_verified)
        .bind(b.password_expires)
        .bind(b.created_at)
        .bind(b.last_login)
        .bind(b.last_failed_login)
        .bind(b.failed_login_attempts)
        .bind(b.mfa_app)
        .bind(b.sec_key_1)
        .bind(b.sec_key_2)
        .execute(db_to)
        .await?;
    }

    // CLIENTS
    let before = sqlx::query_as::<_, Client>("select * from clients")
        .fetch_all(db_from)
        .await?;
    sqlx::query("delete from clients").execute(db_to).await?;
    for b in before {
        sqlx::query(
            r#"insert into clients (id, name, enabled, confidential, secret, secret_kid,
            redirect_uris, post_logout_redirect_uris, allowed_origins, flows_enabled, access_token_alg,
            id_token_alg, refresh_token, auth_code_lifetime, access_token_lifetime, scopes, default_scopes,
            challenge)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)"#)
            .bind(&b.id)
            .bind(&b.name)
            .bind(b.enabled)
            .bind(b.confidential)
            .bind(&b.secret)
            .bind(&b.secret_kid)
            .bind(&b.redirect_uris)
            .bind(&b.post_logout_redirect_uris)
            .bind(&b.allowed_origins)
            .bind(&b.flows_enabled)
            .bind(&b.access_token_alg)
            .bind(&b.id_token_alg)
            .bind(b.refresh_token)
            .bind(b.auth_code_lifetime)
            .bind(b.access_token_lifetime)
            .bind(&b.scopes)
            .bind(&b.default_scopes)
            .bind(&b.challenge)
            .execute(db_to)
            .await?;
    }

    // COLORS
    let before = sqlx::query_as::<_, ColorEntity>("select * from colors")
        .fetch_all(db_from)
        .await?;
    sqlx::query("delete from colors").execute(db_to).await?;
    for b in before {
        sqlx::query("insert into colors (client_id, data) values ($1, $2)")
            .bind(b.client_id)
            .bind(b.data)
            .execute(db_to)
            .await?;
    }

    // LOGOS
    let before = sqlx::query("select * from logos")
        .fetch_all(db_from)
        .await?;
    sqlx::query("delete from logos").execute(db_to).await?;
    for b in before {
        let id: String = b.get("client_id");
        let data: String = b.get("data");
        sqlx::query("insert into logos (client_id, data) values ($1, $2)")
            .bind(id)
            .bind(data)
            .execute(db_to)
            .await?;
    }

    // GROUPS
    let before = sqlx::query_as::<_, Group>("select * from groups")
        .fetch_all(db_from)
        .await?;
    sqlx::query("delete from groups").execute(db_to).await?;
    for b in before {
        sqlx::query("insert into groups (id, name) values ($1, $2)")
            .bind(b.id)
            .bind(b.name)
            .execute(db_to)
            .await?;
    }

    // JWKS
    let before = sqlx::query_as::<_, Jwk>("select * from jwks")
        .fetch_all(db_from)
        .await?;
    sqlx::query("delete from jwks").execute(db_to).await?;
    for b in before {
        sqlx::query(
            r#"insert into jwks (kid, created_at, signature, enc_key_id, jwk)
            values ($1, $2, $3, $4, $5)"#,
        )
        .bind(&b.kid)
        .bind(b.created_at)
        .bind(b.signature.as_str())
        .bind(&b.enc_key_id)
        .bind(&b.jwk)
        .execute(db_to)
        .await?;
    }

    // MAGIC LINKS
    let before = sqlx::query_as::<_, MagicLinkPassword>("select * from magic_links")
        .fetch_all(db_from)
        .await?;
    sqlx::query("delete from magic_links")
        .execute(db_to)
        .await?;
    for b in before {
        sqlx::query(
            r#"insert into magic_links (id, user_id, csrf_token, cookie, exp, used)
            values ($1, $2, $3, $4, $5, $6)"#,
        )
        .bind(&b.id)
        .bind(&b.user_id)
        .bind(&b.csrf_token)
        .bind(&b.cookie)
        .bind(b.exp)
        .bind(b.used)
        .execute(db_to)
        .await?;
    }

    // PASSWORD POLICY
    let res = sqlx::query("select data from config where id = 'password_policy'")
        .fetch_one(db_from)
        .await?;
    let bytes: Vec<u8> = res.get("data");
    sqlx::query("update config set data = $1 where id = 'password_policy'")
        .bind(bytes)
        .execute(db_to)
        .await?;

    // REFRESH TOKENS
    let before = sqlx::query_as::<_, RefreshToken>("select * from refresh_tokens")
        .fetch_all(db_from)
        .await?;
    sqlx::query("delete from refresh_tokens")
        .execute(db_to)
        .await?;
    for b in before {
        sqlx::query(
            r#"insert into refresh_tokens (id, user_id, nbf, exp, scope)
            values ($1, $2, $3, $4, $5)"#,
        )
        .bind(&b.id)
        .bind(&b.user_id)
        .bind(b.nbf)
        .bind(b.exp)
        .bind(&b.scope)
        .execute(db_to)
        .await?;
    }

    // ROLES
    let before = sqlx::query_as::<_, Role>("select * from roles")
        .fetch_all(db_from)
        .await?;
    sqlx::query("delete from roles").execute(db_to).await?;
    for b in before {
        sqlx::query("insert into roles (id, name) values ($1, $2)")
            .bind(b.id)
            .bind(b.name)
            .execute(db_to)
            .await?;
    }

    // SCOPES
    let before = sqlx::query_as::<_, Scope>("select * from scopes")
        .fetch_all(db_from)
        .await?;
    sqlx::query("delete from scopes").execute(db_to).await?;
    for b in before {
        sqlx::query("insert into scopes (id, name) values ($1, $2)")
            .bind(b.id)
            .bind(b.name)
            .execute(db_to)
            .await?;
    }

    // USER ATTR CONFIG
    let before = sqlx::query_as::<_, UserAttrConfigEntity>("select * from user_attr_config")
        .fetch_all(db_from)
        .await?;
    sqlx::query("delete from user_attr_config")
        .execute(db_to)
        .await?;
    for b in before {
        match *DB_TYPE {
            DbType::Sqlite => {
                sqlx::query("insert into user_attr_config (name, desc) values ($1, $2)")
            }
            DbType::Postgres => {
                sqlx::query("insert into user_attr_config (name, \"desc\") values ($1, $2)")
            }
        }
        .bind(b.name)
        .bind(b.desc)
        .execute(db_to)
        .await?;
    }

    // USER ATTR VALUES
    let before = sqlx::query_as::<_, UserAttrValueEntity>("select * from user_attr_values")
        .fetch_all(db_from)
        .await?;
    sqlx::query("delete from user_attr_values")
        .execute(db_to)
        .await?;
    for b in before {
        sqlx::query("insert into user_attr_values (user_id, key, value) values ($1, $2, $3)")
            .bind(b.user_id)
            .bind(b.key)
            .bind(b.value)
            .execute(db_to)
            .await?;
    }

    // SESSIONS
    let before = sqlx::query_as::<_, Session>("select * from sessions")
        .fetch_all(db_from)
        .await?;
    sqlx::query("delete from sessions").execute(db_to).await?;
    for b in before {
        sqlx::query(
            r#"insert into
            sessions (id, csrf_token, user_id, roles, groups, is_mfa, state, exp, last_seen)
            values ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
        )
        .bind(&b.id)
        .bind(&b.csrf_token)
        .bind(&b.user_id)
        .bind(&b.roles)
        .bind(&b.groups)
        .bind(b.is_mfa)
        .bind(b.state.as_str())
        .bind(b.exp)
        .bind(b.last_seen)
        .execute(db_to)
        .await?;
    }

    // RECENT PASSWORDS
    let before = sqlx::query_as::<_, RecentPasswordsEntity>("select * from recent_passwords")
        .fetch_all(db_from)
        .await?;
    sqlx::query("delete from recent_passwords")
        .execute(db_to)
        .await?;
    for b in before {
        sqlx::query("insert into recent_passwords (user_id, passwords) values ($1, $2)")
            .bind(b.user_id)
            .bind(b.passwords)
            .execute(db_to)
            .await?;
    }

    Ok(())
}
