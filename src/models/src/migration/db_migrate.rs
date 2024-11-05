use crate::app_state::DbPool;
use crate::database::DB;
use crate::entity::api_keys::ApiKeyEntity;
use crate::entity::auth_providers::AuthProvider;
use crate::entity::clients::Client;
use crate::entity::clients_dyn::ClientDyn;
use crate::entity::colors::ColorEntity;
use crate::entity::config::ConfigEntity;
use crate::entity::devices::DeviceEntity;
use crate::entity::groups::Group;
use crate::entity::jwk::Jwk;
use crate::entity::logos::Logo;
use crate::entity::magic_links::MagicLink;
use crate::entity::password::RecentPasswordsEntity;
use crate::entity::refresh_tokens::RefreshToken;
use crate::entity::refresh_tokens_devices::RefreshTokenDevice;
use crate::entity::roles::Role;
use crate::entity::scopes::Scope;
use crate::entity::sessions::Session;
use crate::entity::user_attr::{UserAttrConfigEntity, UserAttrValueEntity};
use crate::entity::users::User;
use crate::entity::users_values::UserValues;
use crate::entity::webauthn::PasskeyEntity;
use crate::entity::webids::WebId;
use crate::events::event::Event;
use crate::migration::inserts;
use hiqlite::{params, Param};
use rauthy_common::constants::DB_TYPE;
use rauthy_common::DbType;
use rauthy_error::ErrorResponse;
use sqlx::Row;
use tracing::{debug, info};

// NOTE:
// All the queries in the migrations here cannot be compile-time checked.
// This is a limitation of `sqlx`, because we would be referencing 2 different database
// types at the same time.
// Be careful when you update these!

/// This function will only exist temporarily until sqlx::Sqlite has been dropped.
/// This is for the automatic migration possibility from existing SQLite databases to
/// the new Hiqlite.
pub async fn migrate_sqlite_to_hiqlite(db_from: sqlx::SqlitePool) -> Result<(), ErrorResponse> {
    info!("Starting migration from existing SQLite to Hiqlite");
    let hql = DB::client();

    // CONFIG
    debug!("Migrating table: config");
    let before = sqlx::query_as::<_, ConfigEntity>("SELECT * FROM config")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM config", params!()).await?;
    for b in before {
        hql.execute(
            "INSERT INTO config (id, data) VALUES ($1, $2)",
            params!(b.id, b.data),
        )
        .await?;
    }

    // API KEYS
    debug!("Migrating table: api_keys");
    let before = sqlx::query_as::<_, ApiKeyEntity>("SELECT * FROM api_keys")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM api_keys", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO
api_keys (name, secret, created, expires, enc_key_id, access)
VALUES ($1, $2, $3, $4, $5, $6)"#,
            params!(
                b.name,
                b.secret,
                b.created,
                b.expires,
                b.enc_key_id,
                b.access
            ),
        )
        .await?;
    }

    // The users table has a FK to auth_providers - the order is important here!
    // AUTH PROVIDERS
    debug!("Migrating table: auth_providers");
    let before = sqlx::query_as::<_, AuthProvider>("select * from auth_providers")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM auth_providers", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO
auth_providers (id, enabled, name, typ, issuer, authorization_endpoint, token_endpoint,
userinfo_endpoint, client_id, secret, scope, admin_claim_path, admin_claim_value, mfa_claim_path,
mfa_claim_value, allow_insecure_requests, use_pkce, root_pem)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)"#,
            params!(
                b.id,
                b.enabled,
                b.name,
                b.typ.as_str(),
                b.issuer,
                b.authorization_endpoint,
                b.token_endpoint,
                b.userinfo_endpoint,
                b.client_id,
                b.secret,
                b.scope,
                b.admin_claim_path,
                b.admin_claim_value,
                b.mfa_claim_path,
                b.mfa_claim_value,
                b.allow_insecure_requests,
                b.use_pkce,
                b.root_pem
            ),
        )
        .await?;
    }

    // AUTH PROVIDER LOGOS
    debug!("Migrating table: auth_provider_logos");
    let before = sqlx::query(
        "select auth_provider_id as id, res, content_type, data from auth_provider_logos",
    )
    .fetch_all(&db_from)
    .await?;
    hql.execute("DELETE FROM auth_provider_logos", params!())
        .await?;
    for b in before {
        let id: String = b.get("id");
        let res: String = b.get("res");
        let content_type: String = b.get("content_type");
        let data: Vec<u8> = b.get("data");

        hql.execute(
            r#"
INSERT INTO auth_provider_logos (auth_provider_id, res, content_type, data)
VALUES ($1, $2, $3, $4)
ON CONFLICT(auth_provider_id, res) DO UPDATE
SET content_type = $3, data = $4"#,
            params!(id, res, content_type, data),
        )
        .await?;
    }

    // USERS
    debug!("Migrating table: users");
    let before = sqlx::query_as::<_, User>("select * from users")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM users", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO users
(id, email, given_name, family_name, password, roles, groups, enabled, email_verified,
password_expires, created_at, last_login, last_failed_login, failed_login_attempts, language,
webauthn_user_id, user_expires, auth_provider_id, federation_uid)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)"#,
            params!(
                b.id,
                b.email,
                b.given_name,
                b.family_name,
                b.password,
                b.roles,
                b.groups,
                b.enabled,
                b.email_verified,
                b.password_expires,
                b.created_at,
                b.last_login,
                b.last_failed_login,
                b.failed_login_attempts,
                b.language.as_str(),
                b.webauthn_user_id,
                b.user_expires,
                b.auth_provider_id,
                b.federation_uid
            ),
        )
        .await?;
    }

    // PASSKEYS
    debug!("Migrating table: passkeys");
    let before = sqlx::query_as::<_, PasskeyEntity>("SELECT * FROM passkeys")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM passkeys", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO passkeys
(user_id, name, passkey_user_id, passkey, credential_id, registered, last_used, user_verified)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            params!(
                b.user_id,
                b.name,
                b.passkey_user_id,
                b.passkey,
                b.credential_id,
                b.registered,
                b.last_used,
                b.user_verified
            ),
        )
        .await?;
    }

    // Do not change the order - tables below have FKs to clients
    // CLIENTS
    debug!("Migrating table: clients");
    let before = sqlx::query_as::<_, Client>("select * from clients")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM clients", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO clients
(id, name, enabled, confidential, secret, secret_kid, redirect_uris, post_logout_redirect_uris,
allowed_origins, flows_enabled, access_token_alg, id_token_alg, auth_code_lifetime,
access_token_lifetime, scopes, default_scopes, challenge, force_mfa, client_uri, contacts)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)"#,
        params!(
            b.id,
            b.name,
            b.enabled,
            b.confidential,
            b.secret,
            b.secret_kid,
            b.redirect_uris,
            b.post_logout_redirect_uris,
            b.allowed_origins,
            b.flows_enabled,
            b.access_token_alg,
            b.id_token_alg,
            b.auth_code_lifetime,
            b.access_token_lifetime,
            b.scopes,
            b.default_scopes,
            b.challenge,
            b.force_mfa,
            b.client_uri,
            b.contacts
        )).await?;
    }

    // CLIENTS DYN
    debug!("Migrating table: clients_dyn");
    let before = sqlx::query_as::<_, ClientDyn>("select * from clients_dyn")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM clients_dyn", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO
clients_dyn (id, created, registration_token, token_endpoint_auth_method)
VALUES ($1, $2, $3, $4)"#,
            params!(
                b.id,
                b.created,
                b.registration_token,
                b.token_endpoint_auth_method
            ),
        )
        .await?;
    }

    // CLIENT LOGOS
    debug!("Migrating table: client_logos");
    let before = sqlx::query("select * from client_logos")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM client_logos", params!()).await?;
    for b in before {
        let id: String = b.get("client_id");
        let res: String = b.get("res");
        let content_type: String = b.get("content_type");
        let data: Vec<u8> = b.get("data");

        hql.execute(
            r#"
INSERT INTO client_logos (client_id, res, content_type, data)
VALUES ($1, $2, $3, $4)"#,
            params!(id, res, content_type, data),
        )
        .await?;
    }

    // COLORS
    debug!("Migrating table: colors");
    let before = sqlx::query_as::<_, ColorEntity>("select * from colors")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM colors", params!()).await?;
    for b in before {
        hql.execute(
            "INSERT INTO colors (client_id, data) VALUES ($1, $2)",
            params!(b.client_id, b.data),
        )
        .await?;
    }

    // GROUPS
    debug!("Migrating table: groups");
    let before = sqlx::query_as::<_, Group>("select * from groups")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM groups", params!()).await?;
    for b in before {
        hql.execute(
            "INSERT INTO groups (id, name) VALUES ($1, $2)",
            params!(b.id, b.name),
        )
        .await?;
    }

    // JWKS
    debug!("Migrating table: jwks");
    let before = sqlx::query_as::<_, Jwk>("select * from jwks")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM jwks", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO jwks (kid, created_at, signature, enc_key_id, jwk)
VALUES ($1, $2, $3, $4, $5)"#,
            params!(
                b.kid,
                b.created_at,
                b.signature.as_str(),
                &b.enc_key_id,
                b.jwk
            ),
        )
        .await?;
    }

    // MAGIC LINKS
    debug!("Migrating table: magic_links");
    let before = sqlx::query_as::<_, MagicLink>("select * from magic_links")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM magic_links", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO magic_links
(id, user_id, csrf_token, cookie, exp, used, usage)
VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
            params!(
                b.id,
                b.user_id,
                b.csrf_token,
                b.cookie,
                b.exp,
                b.used,
                b.usage
            ),
        )
        .await?;
    }

    // PASSWORD POLICY
    debug!("Migrating table: password_policy");
    let res = sqlx::query("select data from config where id = 'password_policy'")
        .fetch_one(&db_from)
        .await?;
    let bytes: Vec<u8> = res.get("data");
    hql.execute(
        "UPDATE config SET data = $1 WHERE id = 'password_policy'",
        params!(bytes),
    )
    .await?;

    // REFRESH TOKENS
    debug!("Migrating table: refresh_tokens");
    let before = sqlx::query_as::<_, RefreshToken>("select * from refresh_tokens")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM refresh_tokens", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO refresh_tokens (id, user_id, nbf, exp, scope)
VALUES ($1, $2, $3, $4, $5)"#,
            params!(b.id, b.user_id, b.nbf, b.exp, b.scope),
        )
        .await?;
    }

    // ROLES
    debug!("Migrating table: roles");
    let before = sqlx::query_as::<_, Role>("select * from roles")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM roles", params!()).await?;
    for b in before {
        hql.execute(
            "INSERT INTO roles (id, name) VALUES ($1, $2)",
            params!(b.id, b.name),
        )
        .await?;
    }

    // SCOPES
    debug!("Migrating table: scopes");
    let before = sqlx::query_as::<_, Scope>("select * from scopes")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM scopes", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO scopes (id, name, attr_include_access, attr_include_id)
VALUES ($1, $2, $3, $4)"#,
            params!(b.id, b.name, b.attr_include_access, b.attr_include_id),
        )
        .await?;
    }

    // EVENTS
    let before = sqlx::query("select * from events")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM events", params!()).await?;
    for b in before {
        let id: String = b.get("id");
        let timestamp: i64 = b.get("timestamp");
        let level: i16 = b.get("level");
        let typ: i16 = b.get("typ");
        let ip: Option<String> = b.get("ip");
        let data: Option<i64> = b.get("data");
        let text: Option<String> = b.get("text");

        hql.execute(
            r#"
INSERT INTO events (id, timestamp, level, typ, ip, data, text)
VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
            params!(id, timestamp, level, typ, ip, data, text),
        )
        .await?;
    }

    // USER ATTR CONFIG
    debug!("Migrating table: user_attr_config");
    let before = sqlx::query_as::<_, UserAttrConfigEntity>("select * from user_attr_config")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM user_attr_config", params!())
        .await?;
    for b in before {
        hql.execute(
            "INSERT INTO user_attr_config (name, desc) VALUES ($1, $2)",
            params!(b.name, b.desc),
        )
        .await?;
    }

    // USER ATTR VALUES
    debug!("Migrating table: user_attr_values");
    let before = sqlx::query_as::<_, UserAttrValueEntity>("select * from user_attr_values")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM user_attr_values", params!())
        .await?;
    for b in before {
        hql.execute(
            "INSERT INTO user_attr_values (user_id, key, value) VALUES ($1, $2, $3)",
            params!(b.user_id, b.key, b.value),
        )
        .await?;
    }

    // USERS VALUES
    debug!("Migrating table: users_values");
    let before = sqlx::query_as::<_, UserValues>("select * from users_values")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM users_values", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO
users_values (id, birthdate, phone, street, zip, city, country)
VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
            params!(
                b.id,
                b.birthdate,
                b.phone,
                b.street,
                b.zip,
                b.city,
                b.country
            ),
        )
        .await?;
    }

    // DEVICES
    debug!("Migrating table: devices");
    let before = sqlx::query_as::<_, DeviceEntity>("select * from devices")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM devices", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO devices
(id, client_id, user_id, created, access_exp, refresh_exp, peer_ip, name)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            params!(
                b.id,
                b.client_id,
                b.user_id,
                b.created,
                b.access_exp,
                b.refresh_exp,
                b.peer_ip,
                b.name
            ),
        )
        .await?;
    }

    // REFRESH TOKENS DEVICES
    debug!("Migrating table: devices");
    let before = sqlx::query_as::<_, RefreshTokenDevice>("select * from refresh_tokens_devices")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM refresh_tokens_devices", params!())
        .await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO refresh_tokens_devices
(id, device_id, user_id, nbf, exp, scope)
VALUES ($1, $2, $3, $4, $5, $6)"#,
            params!(b.id, b.device_id, b.user_id, b.nbf, b.exp, b.scope),
        )
        .await?;
    }

    // SESSIONS
    debug!("Migrating table: sessions");
    let before = sqlx::query_as::<_, Session>("select * from sessions")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM sessions", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO
sessions (id, csrf_token, user_id, roles, groups, is_mfa, state, exp, last_seen)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
            params!(
                b.id,
                b.csrf_token,
                b.user_id,
                b.roles,
                b.groups,
                b.is_mfa,
                b.state,
                b.exp,
                b.last_seen
            ),
        )
        .await?;
    }

    // RECENT PASSWORDS
    debug!("Migrating table: recent_passwords");
    let before = sqlx::query_as::<_, RecentPasswordsEntity>("select * from recent_passwords")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM recent_passwords", params!())
        .await?;
    for b in before {
        hql.execute(
            "INSERT INTO recent_passwords (user_id, passwords) VALUES ($1, $2)",
            params!(b.user_id, b.passwords),
        )
        .await?;
    }

    // WEBIDS
    debug!("Migrating table: webids");
    let before = sqlx::query_as::<_, WebId>("select * from webids")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM webids", params!()).await?;
    for b in before {
        hql.execute(
            "INSERT INTO webids (user_id, custom_triples, expose_email) VALUES ($1, $2, $3)",
            params!(b.user_id, b.custom_triples, b.expose_email),
        )
        .await?;
    }

    Ok(())
}

pub async fn migrate_sqlx_to_hiqlite(db_from: DbPool) -> Result<(), ErrorResponse> {
    info!("Starting migration from existing SQLite to Hiqlite");
    let hql = DB::client();

    // CONFIG
    debug!("Migrating table: config");
    let before = sqlx::query_as::<_, ConfigEntity>("SELECT * FROM config")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM config", params!()).await?;
    for b in before {
        hql.execute(
            "INSERT INTO config (id, data) VALUES ($1, $2)",
            params!(b.id, b.data),
        )
        .await?;
    }

    // API KEYS
    debug!("Migrating table: api_keys");
    let before = sqlx::query_as::<_, ApiKeyEntity>("SELECT * FROM api_keys")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM api_keys", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO
api_keys (name, secret, created, expires, enc_key_id, access)
VALUES ($1, $2, $3, $4, $5, $6)"#,
            params!(
                b.name,
                b.secret,
                b.created,
                b.expires,
                b.enc_key_id,
                b.access
            ),
        )
        .await?;
    }

    // The users table has a FK to auth_providers - the order is important here!
    // AUTH PROVIDERS
    debug!("Migrating table: auth_providers");
    let before = sqlx::query_as::<_, AuthProvider>("select * from auth_providers")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM auth_providers", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO
auth_providers (id, enabled, name, typ, issuer, authorization_endpoint, token_endpoint,
userinfo_endpoint, client_id, secret, scope, admin_claim_path, admin_claim_value, mfa_claim_path,
mfa_claim_value, allow_insecure_requests, use_pkce, root_pem)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)"#,
            params!(
                b.id,
                b.enabled,
                b.name,
                b.typ.as_str(),
                b.issuer,
                b.authorization_endpoint,
                b.token_endpoint,
                b.userinfo_endpoint,
                b.client_id,
                b.secret,
                b.scope,
                b.admin_claim_path,
                b.admin_claim_value,
                b.mfa_claim_path,
                b.mfa_claim_value,
                b.allow_insecure_requests,
                b.use_pkce,
                b.root_pem
            ),
        )
        .await?;
    }

    // AUTH PROVIDER LOGOS
    debug!("Migrating table: auth_provider_logos");
    let before = sqlx::query(
        "select auth_provider_id as id, res, content_type, data from auth_provider_logos",
    )
    .fetch_all(&db_from)
    .await?;
    hql.execute("DELETE FROM auth_provider_logos", params!())
        .await?;
    for b in before {
        let id: String = b.get("id");
        let res: String = b.get("res");
        let content_type: String = b.get("content_type");
        let data: Vec<u8> = b.get("data");

        hql.execute(
            r#"
INSERT INTO auth_provider_logos (auth_provider_id, res, content_type, data)
VALUES ($1, $2, $3, $4)
ON CONFLICT(auth_provider_id, res) DO UPDATE
SET content_type = $3, data = $4"#,
            params!(id, res, content_type, data),
        )
        .await?;
    }

    // USERS
    debug!("Migrating table: users");
    let before = sqlx::query_as::<_, User>("select * from users")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM users", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO users
(id, email, given_name, family_name, password, roles, groups, enabled, email_verified,
password_expires, created_at, last_login, last_failed_login, failed_login_attempts, language,
webauthn_user_id, user_expires, auth_provider_id, federation_uid)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)"#,
            params!(
                b.id,
                b.email,
                b.given_name,
                b.family_name,
                b.password,
                b.roles,
                b.groups,
                b.enabled,
                b.email_verified,
                b.password_expires,
                b.created_at,
                b.last_login,
                b.last_failed_login,
                b.failed_login_attempts,
                b.language.as_str(),
                b.webauthn_user_id,
                b.user_expires,
                b.auth_provider_id,
                b.federation_uid
            ),
        )
        .await?;
    }

    // PASSKEYS
    debug!("Migrating table: passkeys");
    let before = sqlx::query_as::<_, PasskeyEntity>("SELECT * FROM passkeys")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM passkeys", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO passkeys
(user_id, name, passkey_user_id, passkey, credential_id, registered, last_used, user_verified)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            params!(
                b.user_id,
                b.name,
                b.passkey_user_id,
                b.passkey,
                b.credential_id,
                b.registered,
                b.last_used,
                b.user_verified
            ),
        )
        .await?;
    }

    // Do not change the order - tables below have FKs to clients
    // CLIENTS
    debug!("Migrating table: clients");
    let before = sqlx::query_as::<_, Client>("select * from clients")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM clients", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO clients
(id, name, enabled, confidential, secret, secret_kid, redirect_uris, post_logout_redirect_uris,
allowed_origins, flows_enabled, access_token_alg, id_token_alg, auth_code_lifetime,
access_token_lifetime, scopes, default_scopes, challenge, force_mfa, client_uri, contacts)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)"#,
            params!(
            b.id,
            b.name,
            b.enabled,
            b.confidential,
            b.secret,
            b.secret_kid,
            b.redirect_uris,
            b.post_logout_redirect_uris,
            b.allowed_origins,
            b.flows_enabled,
            b.access_token_alg,
            b.id_token_alg,
            b.auth_code_lifetime,
            b.access_token_lifetime,
            b.scopes,
            b.default_scopes,
            b.challenge,
            b.force_mfa,
            b.client_uri,
            b.contacts
        )).await?;
    }

    // CLIENTS DYN
    debug!("Migrating table: clients_dyn");
    let before = sqlx::query_as::<_, ClientDyn>("select * from clients_dyn")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM clients_dyn", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO
clients_dyn (id, created, registration_token, token_endpoint_auth_method)
VALUES ($1, $2, $3, $4)"#,
            params!(
                b.id,
                b.created,
                b.registration_token,
                b.token_endpoint_auth_method
            ),
        )
        .await?;
    }

    // CLIENT LOGOS
    debug!("Migrating table: client_logos");
    let before = sqlx::query("select * from client_logos")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM client_logos", params!()).await?;
    for b in before {
        let id: String = b.get("client_id");
        let res: String = b.get("res");
        let content_type: String = b.get("content_type");
        let data: Vec<u8> = b.get("data");

        hql.execute(
            r#"
INSERT INTO client_logos (client_id, res, content_type, data)
VALUES ($1, $2, $3, $4)"#,
            params!(id, res, content_type, data),
        )
        .await?;
    }

    // COLORS
    debug!("Migrating table: colors");
    let before = sqlx::query_as::<_, ColorEntity>("select * from colors")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM colors", params!()).await?;
    for b in before {
        hql.execute(
            "INSERT INTO colors (client_id, data) VALUES ($1, $2)",
            params!(b.client_id, b.data),
        )
        .await?;
    }

    // GROUPS
    debug!("Migrating table: groups");
    let before = sqlx::query_as::<_, Group>("select * from groups")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM groups", params!()).await?;
    for b in before {
        hql.execute(
            "INSERT INTO groups (id, name) VALUES ($1, $2)",
            params!(b.id, b.name),
        )
        .await?;
    }

    // JWKS
    debug!("Migrating table: jwks");
    let before = sqlx::query_as::<_, Jwk>("select * from jwks")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM jwks", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO jwks (kid, created_at, signature, enc_key_id, jwk)
VALUES ($1, $2, $3, $4, $5)"#,
            params!(
                b.kid,
                b.created_at,
                b.signature.as_str(),
                &b.enc_key_id,
                b.jwk
            ),
        )
        .await?;
    }

    // MAGIC LINKS
    debug!("Migrating table: magic_links");
    let before = sqlx::query_as::<_, MagicLink>("select * from magic_links")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM magic_links", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO magic_links
(id, user_id, csrf_token, cookie, exp, used, usage)
VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
            params!(
                b.id,
                b.user_id,
                b.csrf_token,
                b.cookie,
                b.exp,
                b.used,
                b.usage
            ),
        )
        .await?;
    }

    // PASSWORD POLICY
    debug!("Migrating table: password_policy");
    let res = sqlx::query("select data from config where id = 'password_policy'")
        .fetch_one(&db_from)
        .await?;
    let bytes: Vec<u8> = res.get("data");
    hql.execute(
        "UPDATE config SET data = $1 WHERE id = 'password_policy'",
        params!(bytes),
    )
    .await?;

    // REFRESH TOKENS
    debug!("Migrating table: refresh_tokens");
    let before = sqlx::query_as::<_, RefreshToken>("select * from refresh_tokens")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM refresh_tokens", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO refresh_tokens (id, user_id, nbf, exp, scope)
VALUES ($1, $2, $3, $4, $5)"#,
            params!(b.id, b.user_id, b.nbf, b.exp, b.scope),
        )
        .await?;
    }

    // ROLES
    debug!("Migrating table: roles");
    let before = sqlx::query_as::<_, Role>("select * from roles")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM roles", params!()).await?;
    for b in before {
        hql.execute(
            "INSERT INTO roles (id, name) VALUES ($1, $2)",
            params!(b.id, b.name),
        )
        .await?;
    }

    // SCOPES
    debug!("Migrating table: scopes");
    let before = sqlx::query_as::<_, Scope>("select * from scopes")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM scopes", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO scopes (id, name, attr_include_access, attr_include_id)
VALUES ($1, $2, $3, $4)"#,
            params!(b.id, b.name, b.attr_include_access, b.attr_include_id),
        )
        .await?;
    }

    // EVENTS
    let before = sqlx::query("select * from events")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM events", params!()).await?;
    for b in before {
        let id: String = b.get("id");
        let timestamp: i64 = b.get("timestamp");
        let level: i16 = b.get("level");
        let typ: i16 = b.get("typ");
        let ip: Option<String> = b.get("ip");
        let data: Option<i64> = b.get("data");
        let text: Option<String> = b.get("text");

        hql.execute(
            r#"
INSERT INTO events (id, timestamp, level, typ, ip, data, text)
VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
            params!(id, timestamp, level, typ, ip, data, text),
        )
        .await?;
    }

    // USER ATTR CONFIG
    debug!("Migrating table: user_attr_config");
    let before = sqlx::query_as::<_, UserAttrConfigEntity>("select * from user_attr_config")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELET FROM user_attr_config", params!())
        .await?;
    for b in before {
        hql.execute(
            "INSERT INTO user_attr_config (name, desc) VALUES ($1, $2)",
            params!(b.name, b.desc),
        )
        .await?;
    }

    // USER ATTR VALUES
    debug!("Migrating table: user_attr_values");
    let before = sqlx::query_as::<_, UserAttrValueEntity>("select * from user_attr_values")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM user_attr_values", params!())
        .await?;
    for b in before {
        hql.execute(
            "INSERT INTO user_attr_values (user_id, key, value) VALUES ($1, $2, $3)",
            params!(b.user_id, b.key, b.value),
        )
        .await?;
    }

    // USERS VALUES
    debug!("Migrating table: users_values");
    let before = sqlx::query_as::<_, UserValues>("select * from users_values")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM users_values", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO
users_values (id, birthdate, phone, street, zip, city, country)
VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
            params!(
                b.id,
                b.birthdate,
                b.phone,
                b.street,
                b.zip,
                b.city,
                b.country
            ),
        )
        .await?;
    }

    // DEVICES
    debug!("Migrating table: devices");
    let before = sqlx::query_as::<_, DeviceEntity>("select * from devices")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM devices", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO devices
(id, client_id, user_id, created, access_exp, refresh_exp, peer_ip, name)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            params!(
                b.id,
                b.client_id,
                b.user_id,
                b.created,
                b.access_exp,
                b.refresh_exp,
                b.peer_ip,
                b.name
            ),
        )
        .await?;
    }

    // REFRESH TOKENS DEVICES
    debug!("Migrating table: devices");
    let before = sqlx::query_as::<_, RefreshTokenDevice>("select * from refresh_tokens_devices")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM refresh_tokens_devices", params!())
        .await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO refresh_tokens_devices
(id, device_id, user_id, nbf, exp, scope)
VALUES ($1, $2, $3, $4, $5, $6)"#,
            params!(b.id, b.device_id, b.user_id, b.nbf, b.exp, b.scope),
        )
        .await?;
    }

    // SESSIONS
    debug!("Migrating table: sessions");
    let before = sqlx::query_as::<_, Session>("select * from sessions")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM sessions", params!()).await?;
    for b in before {
        hql.execute(
            r#"
INSERT INTO
sessions (id, csrf_token, user_id, roles, groups, is_mfa, state, exp, last_seen)
VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
            params!(
                b.id,
                b.csrf_token,
                b.user_id,
                b.roles,
                b.groups,
                b.is_mfa,
                b.state,
                b.exp,
                b.last_seen
            ),
        )
        .await?;
    }

    // RECENT PASSWORDS
    debug!("Migrating table: recent_passwords");
    let before = sqlx::query_as::<_, RecentPasswordsEntity>("select * from recent_passwords")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM recent_passwords", params!())
        .await?;
    for b in before {
        hql.execute(
            "INSERT INTO recent_passwords (user_id, passwords) VALUES ($1, $2)",
            params!(b.user_id, b.passwords),
        )
        .await?;
    }

    // WEBIDS
    debug!("Migrating table: webids");
    let before = sqlx::query_as::<_, WebId>("select * from webids")
        .fetch_all(&db_from)
        .await?;
    hql.execute("DELETE FROM webids", params!()).await?;
    for b in before {
        hql.execute(
            "INSERT INTO webids (user_id, custom_triples, expose_email) VALUES ($1, $2, $3)",
            params!(b.user_id, b.custom_triples, b.expose_email),
        )
        .await?;
    }

    Ok(())
}

pub async fn migrate_hiqlite_to_sqlx() -> Result<(), ErrorResponse> {
    info!("Starting migration from Hqilite");
    let hql = DB::client();

    // CONFIG
    debug!("Migrating table: config");
    let before: Vec<ConfigEntity> = hql.query_as("SELECT * FROM config", params!()).await?;
    sqlx::query("DELETE FROM config")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query!(
            "INSERT INTO config (id, data) VALUES ($1, $2)",
            b.id,
            b.data,
        )
        .execute(DB::conn())
        .await?;
    }

    // API KEYS
    debug!("Migrating table: api_keys");
    let before: Vec<ApiKeyEntity> = hql.query_as("SELECT * FROM api_keys", params!()).await?;
    sqlx::query!("DELETE FROM api_keys")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query(
            r#"INSERT INTO
            api_keys (name, secret, created, expires, enc_key_id, access)
            VALUES ($1, $2, $3, $4, $5, $6)"#,
        )
        .bind(b.name)
        .bind(b.secret)
        .bind(b.created)
        .bind(b.expires)
        .bind(b.enc_key_id)
        .bind(b.access)
        .execute(DB::conn())
        .await?;
    }

    // The users table has a FK to auth_providers - the order is important here!
    // AUTH PROVIDERS
    debug!("Migrating table: auth_providers");
    let before: Vec<AuthProvider> = hql
        .query_as("SELECT * FROM auth_providers", params!())
        .await?;
    sqlx::query("delete from auth_providers")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query(
            r#"INSERT INTO
            auth_providers (id, enabled, name, typ, issuer, authorization_endpoint, token_endpoint,
            userinfo_endpoint, client_id, secret, scope, admin_claim_path, admin_claim_value,
            mfa_claim_path, mfa_claim_value, allow_insecure_requests, use_pkce, root_pem)
            VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)"#,
        )
        .bind(b.id)
        .bind(b.enabled)
        .bind(b.name)
        .bind(b.typ.as_str())
        .bind(b.issuer)
        .bind(b.authorization_endpoint)
        .bind(b.token_endpoint)
        .bind(b.userinfo_endpoint)
        .bind(b.client_id)
        .bind(b.secret)
        .bind(b.scope)
        .bind(b.admin_claim_path)
        .bind(b.admin_claim_value)
        .bind(b.mfa_claim_path)
        .bind(b.mfa_claim_value)
        .bind(b.allow_insecure_requests)
        .bind(b.use_pkce)
        .bind(b.root_pem)
        .execute(DB::conn())
        .await?;
    }

    // AUTH PROVIDER LOGOS
    debug!("Migrating table: auth_provider_logos");
    let before: Vec<Logo> = hql
        .query_as(
            "SELECT auth_provider_id AS id, res, content_type, data FROM auth_provider_logos",
            params!(),
        )
        .await?;
    sqlx::query("delete from auth_provider_logos")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query(
            r#"INSERT INTO auth_provider_logos (auth_provider_id, res, content_type, data)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT(auth_provider_id, res) DO UPDATE SET content_type = $3, data = $4"#,
        )
        .bind(b.id)
        .bind(b.res.as_str())
        .bind(b.content_type)
        .bind(b.data)
        .execute(DB::conn())
        .await?;
    }

    // USERS
    debug!("Migrating table: users");
    let before: Vec<User> = hql.query_as("SELECT * FROM users", params!()).await?;
    sqlx::query("delete from users").execute(DB::conn()).await?;
    for b in before {
        sqlx::query(
            r#"insert into users
            (id, email, given_name, family_name, password, roles, groups, enabled, email_verified,
            password_expires, created_at, last_login, last_failed_login, failed_login_attempts,
            language, webauthn_user_id, user_expires, auth_provider_id, federation_uid)
            values
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)"#,
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
            .bind(b.language)
            .bind(b.webauthn_user_id)
            .bind(b.user_expires)
            .bind(b.auth_provider_id)
            .bind(b.federation_uid)
            .execute(DB::conn())
            .await?;
    }

    // PASSKEYS
    debug!("Migrating table: passkeys");
    let before: Vec<PasskeyEntity> = hql.query_as("SELECT * FROM passkeys", params!()).await?;
    sqlx::query("DELETE FROM passkeys")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query!(
            r#"INSERT INTO passkeys
            (user_id, name, passkey_user_id, passkey, credential_id, registered, last_used, user_verified)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            b.user_id,
            b.name,
            b.passkey_user_id,
            b.passkey,
            b.credential_id,
            b.registered,
            b.last_used,
            b.user_verified,
        )
            .execute(DB::conn())
            .await?;
    }

    // Do not change the order - tables below have FKs to clients
    // CLIENTS
    debug!("Migrating table: clients");
    let before: Vec<Client> = hql.query_as("SELECT * FROM clients", params!()).await?;
    sqlx::query("delete from clients")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query(
            r#"insert into clients (id, name, enabled, confidential, secret, secret_kid,
            redirect_uris, post_logout_redirect_uris, allowed_origins, flows_enabled, access_token_alg,
            id_token_alg, auth_code_lifetime, access_token_lifetime, scopes, default_scopes,
            challenge, force_mfa, client_uri, contacts)
            values
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)"#)
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
            .bind(b.auth_code_lifetime)
            .bind(b.access_token_lifetime)
            .bind(&b.scopes)
            .bind(&b.default_scopes)
            .bind(&b.challenge)
            .bind(b.force_mfa)
            .bind(b.client_uri)
            .bind(b.contacts)
            .execute(DB::conn())
            .await?;
    }

    // CLIENTS DYN
    debug!("Migrating table: clients_dyn");
    let before: Vec<ClientDyn> = hql.query_as("SELECT * FROM clients_dyn", params!()).await?;
    sqlx::query("delete from clients_dyn")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query(
            r#"INSERT INTO
            clients_dyn (id, created, registration_token, token_endpoint_auth_method)
            VALUES ($1, $2, $3, $4)"#,
        )
        .bind(&b.id)
        .bind(b.created)
        .bind(&b.registration_token)
        .bind(&b.token_endpoint_auth_method)
        .execute(DB::conn())
        .await?;
    }

    // CLIENT LOGOS
    debug!("Migrating table: client_logos");
    let before: Vec<Logo> = hql
        .query_map(
            "SELECT client_id AS id, res, content_type, data FROM client_logos",
            params!(),
        )
        .await?;
    sqlx::query("delete from client_logos")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query(
            r#"INSERT INTO client_logos (client_id, res, content_type, data)
                VALUES ($1, $2, $3, $4)"#,
        )
        .bind(b.id)
        .bind(b.res.as_str())
        .bind(b.content_type)
        .bind(b.data)
        .execute(DB::conn())
        .await?;
    }

    // COLORS
    debug!("Migrating table: colors");
    let before: Vec<ColorEntity> = hql.query_as("SELECT * FROM colors", params!()).await?;
    sqlx::query("delete from colors")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query("insert into colors (client_id, data) values ($1, $2)")
            .bind(b.client_id)
            .bind(b.data)
            .execute(DB::conn())
            .await?;
    }

    // GROUPS
    debug!("Migrating table: groups");
    let before: Vec<Group> = hql.query_as("SELECT * FROM groups", params!()).await?;
    sqlx::query("delete from groups")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query("insert into groups (id, name) values ($1, $2)")
            .bind(b.id)
            .bind(b.name)
            .execute(DB::conn())
            .await?;
    }

    // JWKS
    debug!("Migrating table: jwks");
    let before: Vec<Jwk> = hql.query_as("SELECT * FROM jwks", params!()).await?;
    sqlx::query("delete from jwks").execute(DB::conn()).await?;
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
        .execute(DB::conn())
        .await?;
    }

    // MAGIC LINKS
    debug!("Migrating table: magic_links");
    let before: Vec<MagicLink> = hql.query_as("SELECT * FROM magic_links", params!()).await?;
    sqlx::query("delete from magic_links")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query(
            r#"insert into magic_links
            (id, user_id, csrf_token, cookie, exp, used, usage)
            values ($1, $2, $3, $4, $5, $6, $7)"#,
        )
        .bind(&b.id)
        .bind(&b.user_id)
        .bind(&b.csrf_token)
        .bind(&b.cookie)
        .bind(b.exp)
        .bind(b.used)
        .bind(b.usage)
        .execute(DB::conn())
        .await?;
    }

    // PASSWORD POLICY
    debug!("Migrating table: password_policy");
    let mut res = hql
        .query_raw_one(
            "SELECT data FROM config WHERE id = 'password_policy'",
            params!(),
        )
        .await?;
    let bytes: Vec<u8> = res.get("data");
    sqlx::query("update config set data = $1 where id = 'password_policy'")
        .bind(bytes)
        .execute(DB::conn())
        .await?;

    // REFRESH TOKENS
    debug!("Migrating table: refresh_tokens");
    let before: Vec<RefreshToken> = hql
        .query_as("SELECT * FROM refresh_tokens", params!())
        .await?;
    sqlx::query("delete from refresh_tokens")
        .execute(DB::conn())
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
        .execute(DB::conn())
        .await?;
    }

    // ROLES
    debug!("Migrating table: roles");
    let before: Vec<Role> = hql.query_as("SELECT * FROM roles", params!()).await?;
    sqlx::query("delete from roles").execute(DB::conn()).await?;
    for b in before {
        sqlx::query("insert into roles (id, name) values ($1, $2)")
            .bind(b.id)
            .bind(b.name)
            .execute(DB::conn())
            .await?;
    }

    // SCOPES
    debug!("Migrating table: scopes");
    let before: Vec<Scope> = hql.query_as("SELECT * FROM scopes", params!()).await?;
    sqlx::query("delete from scopes")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query(
            r#"insert into scopes (id, name, attr_include_access, attr_include_id)
            values ($1, $2, $3, $4)"#,
        )
        .bind(b.id)
        .bind(b.name)
        .bind(b.attr_include_access)
        .bind(b.attr_include_id)
        .execute(DB::conn())
        .await?;
    }

    // EVENTS
    let before: Vec<Event> = hql.query_map("SELECT * FROM events", params!()).await?;
    sqlx::query("delete from events")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query(
            r#"INSERT INTO events (id, timestamp, level, typ, ip, data, text)
            VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        )
        .bind(b.id)
        .bind(b.timestamp)
        .bind(b.level.value())
        .bind(b.typ.value())
        .bind(b.ip)
        .bind(b.data)
        .bind(b.text)
        .execute(DB::conn())
        .await?;
    }

    // USER ATTR CONFIG
    debug!("Migrating table: user_attr_config");
    let before: Vec<UserAttrConfigEntity> = hql
        .query_as("SELECT * FROM user_attr_config", params!())
        .await?;
    sqlx::query("delete from user_attr_config")
        .execute(DB::conn())
        .await?;
    for b in before {
        match *DB_TYPE {
            DbType::Sqlite => {
                sqlx::query("insert into user_attr_config (name, desc) values ($1, $2)")
            }
            DbType::Postgres => {
                sqlx::query("insert into user_attr_config (name, \"desc\") values ($1, $2)")
            }
            DbType::Hiqlite => {
                todo!("Migration from Hiqlite has not been implemented yet");
            }
        }
        .bind(b.name)
        .bind(b.desc)
        .execute(DB::conn())
        .await?;
    }

    // USER ATTR VALUES
    debug!("Migrating table: user_attr_values");
    let before: Vec<UserAttrValueEntity> = hql
        .query_as("SELECT * FROM user_attr_values", params!())
        .await?;
    sqlx::query("delete from user_attr_values")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query("insert into user_attr_values (user_id, key, value) values ($1, $2, $3)")
            .bind(b.user_id)
            .bind(b.key)
            .bind(b.value)
            .execute(DB::conn())
            .await?;
    }

    // USERS VALUES
    debug!("Migrating table: users_values");
    let before: Vec<UserValues> = hql
        .query_as("SELECT * FROM users_values", params!())
        .await?;
    sqlx::query("delete from users_values")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query(
            r#"INSERT INTO
            users_values (id, birthdate, phone, street, zip, city, country)
            VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        )
        .bind(b.id)
        .bind(b.birthdate)
        .bind(b.phone)
        .bind(b.street)
        .bind(b.zip)
        .bind(b.city)
        .bind(b.country)
        .execute(DB::conn())
        .await?;
    }

    // DEVICES
    debug!("Migrating table: devices");
    let before: Vec<DeviceEntity> = hql.query_as("SELECT * FROM devices", params!()).await?;
    sqlx::query("delete from devices")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query(
            r#"INSERT INTO devices
            (id, client_id, user_id, created, access_exp, refresh_exp, peer_ip, name)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
        )
        .bind(b.id)
        .bind(b.client_id)
        .bind(b.user_id)
        .bind(b.created)
        .bind(b.access_exp)
        .bind(b.refresh_exp)
        .bind(b.peer_ip)
        .bind(b.name)
        .execute(DB::conn())
        .await?;
    }

    // REFRESH TOKENS DEVICES
    debug!("Migrating table: devices");
    let before: Vec<RefreshTokenDevice> = hql
        .query_as("SELECT * FROM refresh_tokens_devices", params!())
        .await?;
    sqlx::query("delete from refresh_tokens_devices")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query(
            r#"INSERT INTO refresh_tokens_devices
                (id, device_id, user_id, nbf, exp, scope)
                VALUES ($1, $2, $3, $4, $5, $6)"#,
        )
        .bind(b.id)
        .bind(b.device_id)
        .bind(b.user_id)
        .bind(b.nbf)
        .bind(b.exp)
        .bind(b.scope)
        .execute(DB::conn())
        .await?;
    }

    // SESSIONS
    debug!("Migrating table: sessions");
    let before: Vec<Session> = hql.query_as("SELECT * FROM sessions", params!()).await?;
    sqlx::query("delete from sessions")
        .execute(DB::conn())
        .await?;
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
        .execute(DB::conn())
        .await?;
    }

    // RECENT PASSWORDS
    debug!("Migrating table: recent_passwords");
    let before: Vec<RecentPasswordsEntity> = hql
        .query_as("SELECT * FROM recent_passwords", params!())
        .await?;
    sqlx::query("delete from recent_passwords")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query("insert into recent_passwords (user_id, passwords) values ($1, $2)")
            .bind(b.user_id)
            .bind(b.passwords)
            .execute(DB::conn())
            .await?;
    }

    // WEBIDS
    debug!("Migrating table: webids");
    let before: Vec<WebId> = hql.query_as("SELECT * FROM webids", params!()).await?;
    sqlx::query("delete from webids")
        .execute(DB::conn())
        .await?;
    for b in before {
        sqlx::query(
            "INSERT INTO webids (user_id, custom_triples, expose_email) VALUES ($1, $2, $3)",
        )
        .bind(b.user_id)
        .bind(b.custom_triples)
        .bind(b.expose_email)
        .execute(DB::conn())
        .await?;
    }

    Ok(())
}

// /// Migrates `MIGRATE_DB_FROM` to `DATABASE_URL`
// pub async fn migrate_from_sqlite(
//     db_from: sqlx::SqlitePool,
//     db_to: &DbPool,
// ) -> Result<(), ErrorResponse> {
//     info!("Starting migration to another DB");
//
//     // CONFIG
//     debug!("Migrating table: config");
//     let before = sqlx::query_as::<_, ConfigEntity>("SELECT * FROM config")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("DELETE FROM config").execute(db_to).await?;
//     for b in before {
//         sqlx::query!(
//             "INSERT INTO config (id, data) VALUES ($1, $2)",
//             b.id,
//             b.data,
//         )
//         .execute(db_to)
//         .await?;
//     }
//
//     // API KEYS
//     debug!("Migrating table: api_keys");
//     let before = sqlx::query_as::<_, ApiKeyEntity>("SELECT * FROM api_keys")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query!("DELETE FROM api_keys").execute(db_to).await?;
//     for b in before {
//         sqlx::query(
//             r#"INSERT INTO
//             api_keys (name, secret, created, expires, enc_key_id, access)
//             VALUES ($1, $2, $3, $4, $5, $6)"#,
//         )
//         .bind(b.name)
//         .bind(b.secret)
//         .bind(b.created)
//         .bind(b.expires)
//         .bind(b.enc_key_id)
//         .bind(b.access)
//         .execute(db_to)
//         .await?;
//     }
//
//     // The users table has a FK to auth_providers - the order is important here!
//     // AUTH PROVIDERS
//     debug!("Migrating table: auth_providers");
//     let before = sqlx::query_as::<_, AuthProvider>("select * from auth_providers")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from auth_providers")
//         .execute(db_to)
//         .await?;
//     for b in before {
//         sqlx::query(
//             r#"INSERT INTO
//             auth_providers (id, enabled, name, typ, issuer, authorization_endpoint, token_endpoint,
//             userinfo_endpoint, client_id, secret, scope, admin_claim_path, admin_claim_value,
//             mfa_claim_path, mfa_claim_value, allow_insecure_requests, use_pkce, root_pem)
//             VALUES
//             ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)"#,
//         )
//         .bind(b.id)
//         .bind(b.enabled)
//         .bind(b.name)
//         .bind(b.typ.as_str())
//         .bind(b.issuer)
//         .bind(b.authorization_endpoint)
//         .bind(b.token_endpoint)
//         .bind(b.userinfo_endpoint)
//         .bind(b.client_id)
//         .bind(b.secret)
//         .bind(b.scope)
//         .bind(b.admin_claim_path)
//         .bind(b.admin_claim_value)
//         .bind(b.mfa_claim_path)
//         .bind(b.mfa_claim_value)
//         .bind(b.allow_insecure_requests)
//         .bind(b.use_pkce)
//         .bind(b.root_pem)
//         .execute(db_to)
//         .await?;
//     }
//
//     // AUTH PROVIDER LOGOS
//     debug!("Migrating table: auth_provider_logos");
//     let before = sqlx::query(
//         "select auth_provider_id as id, res, content_type, data from auth_provider_logos",
//     )
//     .fetch_all(&db_from)
//     .await?;
//     sqlx::query("delete from auth_provider_logos")
//         .execute(db_to)
//         .await?;
//     for b in before {
//         let id: String = b.get("id");
//         let res: String = b.get("res");
//         let content_type: String = b.get("content_type");
//         let data: Vec<u8> = b.get("data");
//
//         sqlx::query(
//             r#"INSERT INTO auth_provider_logos (auth_provider_id, res, content_type, data)
//                 VALUES ($1, $2, $3, $4)
//                 ON CONFLICT(auth_provider_id, res) DO UPDATE SET content_type = $3, data = $4"#,
//         )
//         .bind(id)
//         .bind(res)
//         .bind(content_type)
//         .bind(data)
//         .execute(db_to)
//         .await?;
//     }
//
//     // USERS
//     debug!("Migrating table: users");
//     let before = sqlx::query_as::<_, User>("select * from users")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from users").execute(db_to).await?;
//     for b in before {
//         sqlx::query(
//             r#"insert into users
//             (id, email, given_name, family_name, password, roles, groups, enabled, email_verified,
//             password_expires, created_at, last_login, last_failed_login, failed_login_attempts,
//             language, webauthn_user_id, user_expires, auth_provider_id, federation_uid)
//             values
//             ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)"#,
//         )
//         .bind(b.id)
//         .bind(b.email)
//         .bind(b.given_name)
//         .bind(b.family_name)
//         .bind(b.password)
//         .bind(b.roles)
//         .bind(b.groups)
//         .bind(b.enabled)
//         .bind(b.email_verified)
//         .bind(b.password_expires)
//         .bind(b.created_at)
//         .bind(b.last_login)
//         .bind(b.last_failed_login)
//         .bind(b.failed_login_attempts)
//         .bind(b.language)
//         .bind(b.webauthn_user_id)
//         .bind(b.user_expires)
//         .bind(b.auth_provider_id)
//         .bind(b.federation_uid)
//         .execute(db_to)
//         .await?;
//     }
//
//     // PASSKEYS
//     debug!("Migrating table: passkeys");
//     let before = sqlx::query_as::<_, PasskeyEntity>("SELECT * FROM passkeys")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("DELETE FROM passkeys").execute(db_to).await?;
//     for b in before {
//         sqlx::query!(
//             r#"INSERT INTO passkeys
//             (user_id, name, passkey_user_id, passkey, credential_id, registered, last_used, user_verified)
//             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
//             b.user_id,
//             b.name,
//             b.passkey_user_id,
//             b.passkey,
//             b.credential_id,
//             b.registered,
//             b.last_used,
//             b.user_verified,
//         )
//             .execute(db_to)
//             .await?;
//     }
//
//     // Do not change the order - tables below have FKs to clients
//     // CLIENTS
//     debug!("Migrating table: clients");
//     let before = sqlx::query_as::<_, Client>("select * from clients")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from clients").execute(db_to).await?;
//     for b in before {
//         sqlx::query(
//             r#"insert into clients (id, name, enabled, confidential, secret, secret_kid,
//             redirect_uris, post_logout_redirect_uris, allowed_origins, flows_enabled, access_token_alg,
//             id_token_alg, auth_code_lifetime, access_token_lifetime, scopes, default_scopes,
//             challenge, force_mfa, client_uri, contacts)
//             values
//             ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)"#)
//             .bind(&b.id)
//             .bind(&b.name)
//             .bind(b.enabled)
//             .bind(b.confidential)
//             .bind(&b.secret)
//             .bind(&b.secret_kid)
//             .bind(&b.redirect_uris)
//             .bind(&b.post_logout_redirect_uris)
//             .bind(&b.allowed_origins)
//             .bind(&b.flows_enabled)
//             .bind(&b.access_token_alg)
//             .bind(&b.id_token_alg)
//             .bind(b.auth_code_lifetime)
//             .bind(b.access_token_lifetime)
//             .bind(&b.scopes)
//             .bind(&b.default_scopes)
//             .bind(&b.challenge)
//             .bind(b.force_mfa)
//             .bind(b.client_uri)
//             .bind(b.contacts)
//             .execute(db_to)
//             .await?;
//     }
//
//     // CLIENTS DYN
//     debug!("Migrating table: clients_dyn");
//     let before = sqlx::query_as::<_, ClientDyn>("select * from clients_dyn")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from clients_dyn")
//         .execute(db_to)
//         .await?;
//     for b in before {
//         sqlx::query(
//             r#"INSERT INTO
//             clients_dyn (id, created, registration_token, token_endpoint_auth_method)
//             VALUES ($1, $2, $3, $4)"#,
//         )
//         .bind(&b.id)
//         .bind(b.created)
//         .bind(&b.registration_token)
//         .bind(&b.token_endpoint_auth_method)
//         .execute(db_to)
//         .await?;
//     }
//
//     // CLIENT LOGOS
//     debug!("Migrating table: client_logos");
//     let before = sqlx::query("select * from client_logos")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from client_logos")
//         .execute(db_to)
//         .await?;
//     for b in before {
//         let id: String = b.get("client_id");
//         let res: String = b.get("res");
//         let content_type: String = b.get("content_type");
//         let data: Vec<u8> = b.get("data");
//
//         sqlx::query(
//             r#"INSERT INTO client_logos (client_id, res, content_type, data)
//                 VALUES ($1, $2, $3, $4)"#,
//         )
//         .bind(id)
//         .bind(res)
//         .bind(content_type)
//         .bind(data)
//         .execute(db_to)
//         .await?;
//     }
//
//     // COLORS
//     debug!("Migrating table: colors");
//     let before = sqlx::query_as::<_, ColorEntity>("select * from colors")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from colors").execute(db_to).await?;
//     for b in before {
//         sqlx::query("insert into colors (client_id, data) values ($1, $2)")
//             .bind(b.client_id)
//             .bind(b.data)
//             .execute(db_to)
//             .await?;
//     }
//
//     // GROUPS
//     debug!("Migrating table: groups");
//     let before = sqlx::query_as::<_, Group>("select * from groups")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from groups").execute(db_to).await?;
//     for b in before {
//         sqlx::query("insert into groups (id, name) values ($1, $2)")
//             .bind(b.id)
//             .bind(b.name)
//             .execute(db_to)
//             .await?;
//     }
//
//     // JWKS
//     debug!("Migrating table: jwks");
//     let before = sqlx::query_as::<_, Jwk>("select * from jwks")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from jwks").execute(db_to).await?;
//     for b in before {
//         sqlx::query(
//             r#"insert into jwks (kid, created_at, signature, enc_key_id, jwk)
//             values ($1, $2, $3, $4, $5)"#,
//         )
//         .bind(&b.kid)
//         .bind(b.created_at)
//         .bind(b.signature.as_str())
//         .bind(&b.enc_key_id)
//         .bind(&b.jwk)
//         .execute(db_to)
//         .await?;
//     }
//
//     // MAGIC LINKS
//     debug!("Migrating table: magic_links");
//     let before = sqlx::query_as::<_, MagicLink>("select * from magic_links")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from magic_links")
//         .execute(db_to)
//         .await?;
//     for b in before {
//         sqlx::query(
//             r#"insert into magic_links
//             (id, user_id, csrf_token, cookie, exp, used, usage)
//             values ($1, $2, $3, $4, $5, $6, $7)"#,
//         )
//         .bind(&b.id)
//         .bind(&b.user_id)
//         .bind(&b.csrf_token)
//         .bind(&b.cookie)
//         .bind(b.exp)
//         .bind(b.used)
//         .bind(b.usage)
//         .execute(db_to)
//         .await?;
//     }
//
//     // PASSWORD POLICY
//     debug!("Migrating table: password_policy");
//     let res = sqlx::query("select data from config where id = 'password_policy'")
//         .fetch_one(&db_from)
//         .await?;
//     let bytes: Vec<u8> = res.get("data");
//     sqlx::query("update config set data = $1 where id = 'password_policy'")
//         .bind(bytes)
//         .execute(db_to)
//         .await?;
//
//     // REFRESH TOKENS
//     debug!("Migrating table: refresh_tokens");
//     let before = sqlx::query_as::<_, RefreshToken>("select * from refresh_tokens")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from refresh_tokens")
//         .execute(db_to)
//         .await?;
//     for b in before {
//         sqlx::query(
//             r#"insert into refresh_tokens (id, user_id, nbf, exp, scope)
//             values ($1, $2, $3, $4, $5)"#,
//         )
//         .bind(&b.id)
//         .bind(&b.user_id)
//         .bind(b.nbf)
//         .bind(b.exp)
//         .bind(&b.scope)
//         .execute(db_to)
//         .await?;
//     }
//
//     // ROLES
//     debug!("Migrating table: roles");
//     let before = sqlx::query_as::<_, Role>("select * from roles")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from roles").execute(db_to).await?;
//     for b in before {
//         sqlx::query("insert into roles (id, name) values ($1, $2)")
//             .bind(b.id)
//             .bind(b.name)
//             .execute(db_to)
//             .await?;
//     }
//
//     // SCOPES
//     debug!("Migrating table: scopes");
//     let before = sqlx::query_as::<_, Scope>("select * from scopes")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from scopes").execute(db_to).await?;
//     for b in before {
//         sqlx::query(
//             r#"insert into scopes (id, name, attr_include_access, attr_include_id)
//             values ($1, $2, $3, $4)"#,
//         )
//         .bind(b.id)
//         .bind(b.name)
//         .bind(b.attr_include_access)
//         .bind(b.attr_include_id)
//         .execute(db_to)
//         .await?;
//     }
//
//     // EVENTS
//     let before = sqlx::query("select * from events")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from events").execute(db_to).await?;
//     for b in before {
//         let id: String = b.get("id");
//         let timestamp: i64 = b.get("timestamp");
//         let level: i16 = b.get("level");
//         let typ: i16 = b.get("typ");
//         let ip: Option<String> = b.get("ip");
//         let data: Option<i64> = b.get("data");
//         let text: Option<String> = b.get("text");
//
//         sqlx::query(
//             r#"INSERT INTO events (id, timestamp, level, typ, ip, data, text)
//             VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
//         )
//         .bind(id)
//         .bind(timestamp)
//         .bind(level)
//         .bind(typ)
//         .bind(ip)
//         .bind(data)
//         .bind(text)
//         .execute(db_to)
//         .await?;
//     }
//
//     // USER ATTR CONFIG
//     debug!("Migrating table: user_attr_config");
//     let before = sqlx::query_as::<_, UserAttrConfigEntity>("select * from user_attr_config")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from user_attr_config")
//         .execute(db_to)
//         .await?;
//     for b in before {
//         match *DB_TYPE {
//             DbType::Sqlite => {
//                 sqlx::query("insert into user_attr_config (name, desc) values ($1, $2)")
//             }
//             DbType::Postgres => {
//                 sqlx::query("insert into user_attr_config (name, \"desc\") values ($1, $2)")
//             }
//             DbType::Hiqlite => {
//                 todo!("Migration from Hiqlite has not been implemented yet");
//             }
//         }
//         .bind(b.name)
//         .bind(b.desc)
//         .execute(db_to)
//         .await?;
//     }
//
//     // USER ATTR VALUES
//     debug!("Migrating table: user_attr_values");
//     let before = sqlx::query_as::<_, UserAttrValueEntity>("select * from user_attr_values")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from user_attr_values")
//         .execute(db_to)
//         .await?;
//     for b in before {
//         sqlx::query("insert into user_attr_values (user_id, key, value) values ($1, $2, $3)")
//             .bind(b.user_id)
//             .bind(b.key)
//             .bind(b.value)
//             .execute(db_to)
//             .await?;
//     }
//
//     // USERS VALUES
//     debug!("Migrating table: users_values");
//     let before = sqlx::query_as::<_, UserValues>("select * from users_values")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from users_values")
//         .execute(db_to)
//         .await?;
//     for b in before {
//         sqlx::query(
//             r#"INSERT INTO
//             users_values (id, birthdate, phone, street, zip, city, country)
//             VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
//         )
//         .bind(b.id)
//         .bind(b.birthdate)
//         .bind(b.phone)
//         .bind(b.street)
//         .bind(b.zip)
//         .bind(b.city)
//         .bind(b.country)
//         .execute(db_to)
//         .await?;
//     }
//
//     // DEVICES
//     debug!("Migrating table: devices");
//     let before = sqlx::query_as::<_, DeviceEntity>("select * from devices")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from devices").execute(db_to).await?;
//     for b in before {
//         sqlx::query(
//             r#"INSERT INTO devices
//             (id, client_id, user_id, created, access_exp, refresh_exp, peer_ip, name)
//             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
//         )
//         .bind(b.id)
//         .bind(b.client_id)
//         .bind(b.user_id)
//         .bind(b.created)
//         .bind(b.access_exp)
//         .bind(b.refresh_exp)
//         .bind(b.peer_ip)
//         .bind(b.name)
//         .execute(db_to)
//         .await?;
//     }
//
//     // REFRESH TOKENS DEVICES
//     debug!("Migrating table: devices");
//     let before = sqlx::query_as::<_, RefreshTokenDevice>("select * from refresh_tokens_devices")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from refresh_tokens_devices")
//         .execute(db_to)
//         .await?;
//     for b in before {
//         sqlx::query(
//             r#"INSERT INTO refresh_tokens_devices
//                 (id, device_id, user_id, nbf, exp, scope)
//                 VALUES ($1, $2, $3, $4, $5, $6)"#,
//         )
//         .bind(b.id)
//         .bind(b.device_id)
//         .bind(b.user_id)
//         .bind(b.nbf)
//         .bind(b.exp)
//         .bind(b.scope)
//         .execute(db_to)
//         .await?;
//     }
//
//     // SESSIONS
//     debug!("Migrating table: sessions");
//     let before = sqlx::query_as::<_, Session>("select * from sessions")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from sessions").execute(db_to).await?;
//     for b in before {
//         sqlx::query(
//             r#"insert into
//             sessions (id, csrf_token, user_id, roles, groups, is_mfa, state, exp, last_seen)
//             values ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
//         )
//         .bind(&b.id)
//         .bind(&b.csrf_token)
//         .bind(&b.user_id)
//         .bind(&b.roles)
//         .bind(&b.groups)
//         .bind(b.is_mfa)
//         .bind(b.state.as_str())
//         .bind(b.exp)
//         .bind(b.last_seen)
//         .execute(db_to)
//         .await?;
//     }
//
//     // RECENT PASSWORDS
//     debug!("Migrating table: recent_passwords");
//     let before = sqlx::query_as::<_, RecentPasswordsEntity>("select * from recent_passwords")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from recent_passwords")
//         .execute(db_to)
//         .await?;
//     for b in before {
//         sqlx::query("insert into recent_passwords (user_id, passwords) values ($1, $2)")
//             .bind(b.user_id)
//             .bind(b.passwords)
//             .execute(db_to)
//             .await?;
//     }
//
//     // WEBIDS
//     debug!("Migrating table: webids");
//     let before = sqlx::query_as::<_, WebId>("select * from webids")
//         .fetch_all(&db_from)
//         .await?;
//     sqlx::query("delete from webids").execute(db_to).await?;
//     for b in before {
//         sqlx::query(
//             "INSERT INTO webids (user_id, custom_triples, expose_email) VALUES ($1, $2, $3)",
//         )
//         .bind(b.user_id)
//         .bind(b.custom_triples)
//         .bind(b.expose_email)
//         .execute(db_to)
//         .await?;
//     }
//
//     Ok(())
// }

/// Migrates `MIGRATE_DB_FROM` to `DATABASE_URL`
pub async fn migrate_from_sqlite(db_from: sqlx::SqlitePool) -> Result<(), ErrorResponse> {
    info!("Starting migration to another DB");

    // CONFIG
    debug!("Migrating table: config");
    let before = sqlx::query_as::<_, ConfigEntity>("SELECT * FROM config")
        .fetch_all(&db_from)
        .await?;
    inserts::config(before).await?;

    // API KEYS
    debug!("Migrating table: api_keys");
    let before = sqlx::query_as::<_, ApiKeyEntity>("SELECT * FROM api_keys")
        .fetch_all(&db_from)
        .await?;
    inserts::api_keys(before).await?;

    // The users table has a FK to auth_providers - the order is important here!
    // AUTH PROVIDERS
    debug!("Migrating table: auth_providers");
    let before = sqlx::query_as::<_, AuthProvider>("select * from auth_providers")
        .fetch_all(&db_from)
        .await?;
    inserts::auth_providers(before).await?;

    // AUTH PROVIDER LOGOS
    debug!("Migrating table: auth_provider_logos");
    let before = sqlx::query_as::<_, Logo>(
        "select auth_provider_id as id, res, content_type, data from auth_provider_logos",
    )
    .fetch_all(&db_from)
    .await?;
    inserts::auth_provider_logos(before).await?;

    // USERS
    debug!("Migrating table: users");
    let before = sqlx::query_as::<_, User>("select * from users")
        .fetch_all(&db_from)
        .await?;
    inserts::users(before).await?;

    // PASSKEYS
    debug!("Migrating table: passkeys");
    let before = sqlx::query_as::<_, PasskeyEntity>("SELECT * FROM passkeys")
        .fetch_all(&db_from)
        .await?;
    inserts::passkeys(before).await?;

    // Do not change the order - tables below have FKs to clients
    // CLIENTS
    debug!("Migrating table: clients");
    let before = sqlx::query_as::<_, Client>("select * from clients")
        .fetch_all(&db_from)
        .await?;
    inserts::clients(before).await?;

    // CLIENTS DYN
    debug!("Migrating table: clients_dyn");
    let before = sqlx::query_as::<_, ClientDyn>("select * from clients_dyn")
        .fetch_all(&db_from)
        .await?;
    inserts::clients_dyn(before).await?;

    // CLIENT LOGOS
    debug!("Migrating table: client_logos");
    let before = sqlx::query_as::<_, Logo>("select * from client_logos")
        .fetch_all(&db_from)
        .await?;
    inserts::client_logos(before).await?;

    // COLORS
    debug!("Migrating table: colors");
    let before = sqlx::query_as::<_, ColorEntity>("select * from colors")
        .fetch_all(&db_from)
        .await?;
    inserts::colors(before).await?;

    // GROUPS
    debug!("Migrating table: groups");
    let before = sqlx::query_as::<_, Group>("select * from groups")
        .fetch_all(&db_from)
        .await?;
    inserts::groups(before).await?;

    // JWKS
    debug!("Migrating table: jwks");
    let before = sqlx::query_as::<_, Jwk>("select * from jwks")
        .fetch_all(&db_from)
        .await?;
    inserts::jwks(before).await?;

    // MAGIC LINKS
    debug!("Migrating table: magic_links");
    let before = sqlx::query_as::<_, MagicLink>("select * from magic_links")
        .fetch_all(&db_from)
        .await?;
    inserts::magic_links(before).await?;

    // PASSWORD POLICY
    debug!("Migrating table: password_policy");
    let res = sqlx::query("select data from config where id = 'password_policy'")
        .fetch_one(&db_from)
        .await?;
    let bytes: Vec<u8> = res.get("data");
    inserts::password_policy(bytes).await?;

    // REFRESH TOKENS
    debug!("Migrating table: refresh_tokens");
    let before = sqlx::query_as::<_, RefreshToken>("select * from refresh_tokens")
        .fetch_all(&db_from)
        .await?;
    inserts::refresh_tokens(before).await?;

    // ROLES
    debug!("Migrating table: roles");
    let before = sqlx::query_as::<_, Role>("select * from roles")
        .fetch_all(&db_from)
        .await?;
    inserts::roles(before).await?;

    // SCOPES
    debug!("Migrating table: scopes");
    let before = sqlx::query_as::<_, Scope>("select * from scopes")
        .fetch_all(&db_from)
        .await?;
    inserts::scopes(before).await?;

    // EVENTS
    let before = sqlx::query_as::<_, Event>("select * from events")
        .fetch_all(&db_from)
        .await?;
    inserts::events(before).await?;

    // USER ATTR CONFIG
    debug!("Migrating table: user_attr_config");
    let before = sqlx::query_as::<_, UserAttrConfigEntity>("select * from user_attr_config")
        .fetch_all(&db_from)
        .await?;
    inserts::user_attr_config(before).await?;

    // USER ATTR VALUES
    debug!("Migrating table: user_attr_values");
    let before = sqlx::query_as::<_, UserAttrValueEntity>("select * from user_attr_values")
        .fetch_all(&db_from)
        .await?;
    inserts::user_attr_values(before).await?;

    // USERS VALUES
    debug!("Migrating table: users_values");
    let before = sqlx::query_as::<_, UserValues>("select * from users_values")
        .fetch_all(&db_from)
        .await?;
    inserts::users_values(before).await?;

    // DEVICES
    debug!("Migrating table: devices");
    let before = sqlx::query_as::<_, DeviceEntity>("select * from devices")
        .fetch_all(&db_from)
        .await?;
    inserts::devices(before).await?;

    // REFRESH TOKENS DEVICES
    debug!("Migrating table: devices");
    let before = sqlx::query_as::<_, RefreshTokenDevice>("select * from refresh_tokens_devices")
        .fetch_all(&db_from)
        .await?;
    inserts::refresh_tokens_devices(before).await?;

    // SESSIONS
    debug!("Migrating table: sessions");
    let before = sqlx::query_as::<_, Session>("select * from sessions")
        .fetch_all(&db_from)
        .await?;
    inserts::sessions(before).await?;

    // RECENT PASSWORDS
    debug!("Migrating table: recent_passwords");
    let before = sqlx::query_as::<_, RecentPasswordsEntity>("select * from recent_passwords")
        .fetch_all(&db_from)
        .await?;
    inserts::recent_passwords(before).await?;

    // WEBIDS
    debug!("Migrating table: webids");
    let before = sqlx::query_as::<_, WebId>("select * from webids")
        .fetch_all(&db_from)
        .await?;
    inserts::webids(before).await?;

    Ok(())
}

pub async fn migrate_from_postgres(
    db_from: sqlx::PgPool,
    db_to: &DbPool,
) -> Result<(), ErrorResponse> {
    info!("Starting migration to another DB");

    // CONFIG
    debug!("Migrating table: config");
    let before = sqlx::query_as::<_, ConfigEntity>("SELECT * FROM rauthy.config")
        .fetch_all(&db_from)
        .await?;
    sqlx::query("DELETE FROM config").execute(db_to).await?;
    for b in before {
        sqlx::query!(
            "INSERT INTO config (id, data) VALUES ($1, $2)",
            b.id,
            b.data,
        )
        .execute(db_to)
        .await?;
    }

    // API KEYS
    debug!("Migrating table: api_keys");
    let before = sqlx::query_as::<_, ApiKeyEntity>("SELECT * FROM rauthy.api_keys")
        .fetch_all(&db_from)
        .await?;
    sqlx::query!("DELETE FROM api_keys").execute(db_to).await?;
    for b in before {
        sqlx::query(
            r#"INSERT INTO
            api_keys (name, secret, created, expires, enc_key_id, access)
            VALUES ($1, $2, $3, $4, $5, $6)"#,
        )
        .bind(b.name)
        .bind(b.secret)
        .bind(b.created)
        .bind(b.expires)
        .bind(b.enc_key_id)
        .bind(b.access)
        .execute(db_to)
        .await?;
    }

    // The users table has a FK to auth_providers - the order is important here!
    // AUTH PROVIDERS
    debug!("Migrating table: auth_providers");
    let before = sqlx::query_as::<_, AuthProvider>("select * from rauthy.auth_providers")
        .fetch_all(&db_from)
        .await?;
    sqlx::query("delete from auth_providers")
        .execute(db_to)
        .await?;
    for b in before {
        sqlx::query(
            r#"INSERT INTO
            auth_providers (id, enabled, name, typ, issuer, authorization_endpoint, token_endpoint,
            userinfo_endpoint, client_id, secret, scope, admin_claim_path, admin_claim_value,
            mfa_claim_path, mfa_claim_value, allow_insecure_requests, use_pkce, root_pem)
            VALUES
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)"#,
        )
        .bind(b.id)
        .bind(b.enabled)
        .bind(b.name)
        .bind(b.typ.as_str())
        .bind(b.issuer)
        .bind(b.authorization_endpoint)
        .bind(b.token_endpoint)
        .bind(b.userinfo_endpoint)
        .bind(b.client_id)
        .bind(b.secret)
        .bind(b.scope)
        .bind(b.admin_claim_path)
        .bind(b.admin_claim_value)
        .bind(b.mfa_claim_path)
        .bind(b.mfa_claim_value)
        .bind(b.allow_insecure_requests)
        .bind(b.use_pkce)
        .bind(b.root_pem)
        .execute(db_to)
        .await?;
    }

    // AUTH PROVIDER LOGOS
    debug!("Migrating table: auth_provider_logos");
    let before = sqlx::query(
        "select auth_provider_id as id, res, content_type, data from rauthy.auth_provider_logos",
    )
    .fetch_all(&db_from)
    .await?;
    sqlx::query("delete from auth_provider_logos")
        .execute(db_to)
        .await?;
    for b in before {
        let id: String = b.get("id");
        let res: String = b.get("res");
        let content_type: String = b.get("content_type");
        let data: Vec<u8> = b.get("data");

        sqlx::query(
            r#"INSERT OR REPLACE INTO
            auth_provider_logos (auth_provider_id, res, content_type, data)
            VALUES ($1, $2, $3, $4)"#,
        )
        .bind(id)
        .bind(res)
        .bind(content_type)
        .bind(data)
        .execute(db_to)
        .await?;
    }

    // USERS
    debug!("Migrating table: users");
    let before = sqlx::query_as::<_, User>("select * from rauthy.users")
        .fetch_all(&db_from)
        .await?;
    sqlx::query("delete from users").execute(db_to).await?;
    for b in before {
        sqlx::query(
            r#"insert into users
            (id, email, given_name, family_name, password, roles, groups, enabled, email_verified,
            password_expires, created_at, last_login, last_failed_login, failed_login_attempts,
            language, webauthn_user_id, user_expires, auth_provider_id, federation_uid)
            values
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)"#,
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
        .bind(b.language)
        .bind(b.webauthn_user_id)
        .bind(b.user_expires)
        .bind(b.auth_provider_id)
        .bind(b.federation_uid)
        .execute(db_to)
        .await?;
    }

    // PASSKEYS
    debug!("Migrating table: passkeys");
    let before = sqlx::query_as::<_, PasskeyEntity>("SELECT * FROM rauthy.passkeys")
        .fetch_all(&db_from)
        .await?;
    sqlx::query("DELETE FROM passkeys").execute(db_to).await?;
    for b in before {
        sqlx::query!(
            r#"INSERT INTO passkeys
            (user_id, name, passkey_user_id, passkey, credential_id, registered, last_used, user_verified)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            b.user_id,
            b.name,
            b.passkey_user_id,
            b.passkey,
            b.credential_id,
            b.registered,
            b.last_used,
            b.user_verified,
        )
            .execute(db_to)
            .await?;
    }

    // Do not change the order - tables below have FKs to clients
    // CLIENTS
    debug!("Migrating table: clients");
    let before = sqlx::query_as::<_, Client>("select * from rauthy.clients")
        .fetch_all(&db_from)
        .await?;
    sqlx::query("delete from clients").execute(db_to).await?;
    for b in before {
        sqlx::query(
            r#"insert into clients (id, name, enabled, confidential, secret, secret_kid,
            redirect_uris, post_logout_redirect_uris, allowed_origins, flows_enabled, access_token_alg,
            id_token_alg, auth_code_lifetime, access_token_lifetime, scopes, default_scopes,
            challenge, force_mfa, client_uri, contacts)
            values
            ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20)"#)
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
            .bind(b.auth_code_lifetime)
            .bind(b.access_token_lifetime)
            .bind(&b.scopes)
            .bind(&b.default_scopes)
            .bind(&b.challenge)
            .bind(b.force_mfa)
            .bind(b.client_uri)
            .bind(b.contacts)
            .execute(db_to)
            .await?;
    }

    // CLIENTS DYN
    debug!("Migrating table: clients_dyn");
    let before = sqlx::query_as::<_, ClientDyn>("select * from rauthy.clients_dyn")
        .fetch_all(&db_from)
        .await?;
    sqlx::query("delete from clients_dyn")
        .execute(db_to)
        .await?;
    for b in before {
        sqlx::query(
            r#"INSERT INTO
            clients_dyn (id, created, registration_token, token_endpoint_auth_method)
            VALUES ($1, $2, $3, $4)"#,
        )
        .bind(&b.id)
        .bind(b.created)
        .bind(&b.registration_token)
        .bind(&b.token_endpoint_auth_method)
        .execute(db_to)
        .await?;
    }

    // CLIENT LOGOS
    let before = sqlx::query("select * from rauthy.client_logos")
        .fetch_all(&db_from)
        .await?;
    sqlx::query("delete from client_logos")
        .execute(db_to)
        .await?;
    for b in before {
        let id: String = b.get("client_id");
        let res: String = b.get("res");
        let content_type: String = b.get("content_type");
        let data: Vec<u8> = b.get("data");

        sqlx::query(
            r#"INSERT INTO client_logos (client_id, res, content_type, data)
                VALUES ($1, $2, $3, $4)"#,
        )
        .bind(id)
        .bind(res)
        .bind(content_type)
        .bind(data)
        .execute(db_to)
        .await?;
    }

    // COLORS
    let before = sqlx::query_as::<_, ColorEntity>("select * from rauthy.colors")
        .fetch_all(&db_from)
        .await?;
    sqlx::query("delete from colors").execute(db_to).await?;
    for b in before {
        sqlx::query("insert into colors (client_id, data) values ($1, $2)")
            .bind(b.client_id)
            .bind(b.data)
            .execute(db_to)
            .await?;
    }

    // GROUPS
    let before = sqlx::query_as::<_, Group>("select * from rauthy.groups")
        .fetch_all(&db_from)
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
    let before = sqlx::query_as::<_, Jwk>("select * from rauthy.jwks")
        .fetch_all(&db_from)
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
    let before = sqlx::query_as::<_, MagicLink>("select * from rauthy.magic_links")
        .fetch_all(&db_from)
        .await?;
    sqlx::query("delete from magic_links")
        .execute(db_to)
        .await?;
    for b in before {
        sqlx::query(
            r#"insert into magic_links
            (id, user_id, csrf_token, cookie, exp, used, usage)
            values ($1, $2, $3, $4, $5, $6, $7)"#,
        )
        .bind(&b.id)
        .bind(&b.user_id)
        .bind(&b.csrf_token)
        .bind(&b.cookie)
        .bind(b.exp)
        .bind(b.used)
        .bind(b.usage)
        .execute(db_to)
        .await?;
    }

    // PASSWORD POLICY
    let res = sqlx::query("select data from rauthy.config where id = 'password_policy'")
        .fetch_one(&db_from)
        .await?;
    let bytes: Vec<u8> = res.get("data");
    sqlx::query("update config set data = $1 where id = 'password_policy'")
        .bind(bytes)
        .execute(db_to)
        .await?;

    // REFRESH TOKENS
    let before = sqlx::query_as::<_, RefreshToken>("select * from rauthy.refresh_tokens")
        .fetch_all(&db_from)
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
    let before = sqlx::query_as::<_, Role>("select * from rauthy.roles")
        .fetch_all(&db_from)
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
    let before = sqlx::query_as::<_, Scope>("select * from rauthy.scopes")
        .fetch_all(&db_from)
        .await?;
    sqlx::query("delete from scopes").execute(db_to).await?;
    for b in before {
        sqlx::query(
            r#"insert into scopes (id, name, attr_include_access, attr_include_id)
            values ($1, $2, $3, $4)"#,
        )
        .bind(b.id)
        .bind(b.name)
        .bind(b.attr_include_access)
        .bind(b.attr_include_id)
        .execute(db_to)
        .await?;
    }

    // EVENTS
    let before = sqlx::query("select * from rauthy.events")
        .fetch_all(&db_from)
        .await?;
    sqlx::query("delete from events").execute(db_to).await?;
    for b in before {
        let id: String = b.get("id");
        let timestamp: i64 = b.get("timestamp");
        let level: i16 = b.get("level");
        let typ: i16 = b.get("typ");
        let ip: Option<String> = b.get("ip");
        let data: Option<i64> = b.get("data");
        let text: Option<String> = b.get("text");

        sqlx::query(
            r#"INSERT INTO events (id, timestamp, level, typ, ip, data, text)
            VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        )
        .bind(id)
        .bind(timestamp)
        .bind(level)
        .bind(typ)
        .bind(ip)
        .bind(data)
        .bind(text)
        .execute(db_to)
        .await?;
    }

    // USER ATTR CONFIG
    let before = sqlx::query_as::<_, UserAttrConfigEntity>("select * from rauthy.user_attr_config")
        .fetch_all(&db_from)
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
            DbType::Hiqlite => {
                todo!("Migration from Hiqlite has not been implemented yet");
            }
        }
        .bind(b.name)
        .bind(b.desc)
        .execute(db_to)
        .await?;
    }

    // USER ATTR VALUES
    let before = sqlx::query_as::<_, UserAttrValueEntity>("select * from rauthy.user_attr_values")
        .fetch_all(&db_from)
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

    // USERS VALUES
    debug!("Migrating table: users_values");
    let before = sqlx::query_as::<_, UserValues>("select * from rauthy.users_values")
        .fetch_all(&db_from)
        .await?;
    sqlx::query("delete from users_values")
        .execute(db_to)
        .await?;
    for b in before {
        sqlx::query(
            r#"INSERT OR REPLACE INTO
            users_values (id, birthdate, phone, street, zip, city, country)
            VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
        )
        .bind(b.id)
        .bind(b.birthdate)
        .bind(b.phone)
        .bind(b.street)
        .bind(b.zip)
        .bind(b.city)
        .bind(b.country)
        .execute(db_to)
        .await?;
    }

    // DEVICES
    debug!("Migrating table: devices");
    let before = sqlx::query_as::<_, DeviceEntity>("select * from rauthy.devices")
        .fetch_all(&db_from)
        .await?;
    sqlx::query("delete from devices").execute(db_to).await?;
    for b in before {
        sqlx::query(
            r#"INSERT INTO devices
            (id, client_id, user_id, created, access_exp, refresh_exp, peer_ip, name)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
        )
        .bind(b.id)
        .bind(b.client_id)
        .bind(b.user_id)
        .bind(b.created)
        .bind(b.access_exp)
        .bind(b.refresh_exp)
        .bind(b.peer_ip)
        .bind(b.name)
        .execute(db_to)
        .await?;
    }

    // REFRESH TOKENS DEVICES
    debug!("Migrating table: devices");
    let before =
        sqlx::query_as::<_, RefreshTokenDevice>("select * from rauthy.refresh_tokens_devices")
            .fetch_all(&db_from)
            .await?;
    sqlx::query("delete from refresh_tokens_devices")
        .execute(db_to)
        .await?;
    for b in before {
        sqlx::query(
            r#"INSERT INTO refresh_tokens_devices
                (id, device_id, user_id, nbf, exp, scope)
                VALUES ($1, $2, $3, $4, $5, $6)"#,
        )
        .bind(b.id)
        .bind(b.device_id)
        .bind(b.user_id)
        .bind(b.nbf)
        .bind(b.exp)
        .bind(b.scope)
        .execute(db_to)
        .await?;
    }

    // SESSIONS
    let before = sqlx::query_as::<_, Session>("select * from rauthy.sessions")
        .fetch_all(&db_from)
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
    let before =
        sqlx::query_as::<_, RecentPasswordsEntity>("select * from rauthy.recent_passwords")
            .fetch_all(&db_from)
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

    // WEBIDS
    debug!("Migrating table: webids");
    let before = sqlx::query_as::<_, WebId>("select * from rauthy.webids")
        .fetch_all(&db_from)
        .await?;
    sqlx::query("delete from webids").execute(db_to).await?;
    for b in before {
        sqlx::query(
            "INSERT INTO webids (user_id, custom_triples, expose_email) VALUES ($1, $2, $3)",
        )
        .bind(b.user_id)
        .bind(b.custom_triples)
        .bind(b.expose_email)
        .execute(db_to)
        .await?;
    }

    Ok(())
}
