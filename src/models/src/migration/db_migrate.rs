use crate::entity::api_keys::ApiKeyEntity;
use crate::entity::auth_providers::AuthProvider;
use crate::entity::clients::Client;
use crate::entity::clients_dyn::ClientDyn;
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
use crate::entity::theme::ThemeCssFull;
use crate::entity::user_attr::{UserAttrConfigEntity, UserAttrValueEntity};
use crate::entity::users::User;
use crate::entity::users_values::UserValues;
use crate::entity::webauthn::PasskeyEntity;
use crate::entity::webids::WebId;
use crate::events::event::Event;
use crate::migration::inserts;
use rauthy_error::ErrorResponse;
use sqlx::{FromRow, Row};
use tracing::{debug, info};

/// Migrates `MIGRATE_DB_FROM` to current database
pub async fn migrate_from_sqlite(db_from: sqlx::SqlitePool) -> Result<(), ErrorResponse> {
    info!("Starting migration to another DB");

    // CONFIG
    debug!("Migrating table: config");
    let before =
        sqlx::query_as::<_, ConfigEntity>("SELECT * FROM config WHERE id = 'password_policy'")
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
    let before = sqlx::query_as::<_, AuthProvider>("SELECT * FROM auth_providers")
        .fetch_all(&db_from)
        .await?;
    inserts::auth_providers(before).await?;

    // AUTH PROVIDER LOGOS
    debug!("Migrating table: auth_provider_logos");
    let before = sqlx::query_as::<_, Logo>(
        "SELECT auth_provider_id AS id, res, content_type, data FROM auth_provider_logos",
    )
    .fetch_all(&db_from)
    .await?;
    inserts::auth_provider_logos(before).await?;

    // USERS
    debug!("Migrating table: users");
    let before = sqlx::query_as::<_, User>("SELECT * FROM users")
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
    let before = sqlx::query_as::<_, Client>("SELECT * FROM clients")
        .fetch_all(&db_from)
        .await?;
    inserts::clients(before).await?;

    // CLIENTS DYN
    debug!("Migrating table: clients_dyn");
    let before = sqlx::query_as::<_, ClientDyn>("SELECT * FROM clients_dyn")
        .fetch_all(&db_from)
        .await?;
    inserts::clients_dyn(before).await?;

    // CLIENT LOGOS
    debug!("Migrating table: client_logos");
    let before = sqlx::query_as::<_, Logo>(
        "SELECT client_id AS id, res, content_type, data FROM client_logos",
    )
    .fetch_all(&db_from)
    .await?;
    inserts::client_logos(before).await?;

    // GROUPS
    debug!("Migrating table: groups");
    let before = sqlx::query_as::<_, Group>("SELECT * FROM groups")
        .fetch_all(&db_from)
        .await?;
    inserts::groups(before).await?;

    // JWKS
    debug!("Migrating table: jwks");
    let before = sqlx::query_as::<_, Jwk>("SELECT * FROM jwks")
        .fetch_all(&db_from)
        .await?;
    inserts::jwks(before).await?;

    // MAGIC LINKS
    debug!("Migrating table: magic_links");
    let before = sqlx::query_as::<_, MagicLink>("SELECT * FROM magic_links")
        .fetch_all(&db_from)
        .await?;
    inserts::magic_links(before).await?;

    // PASSWORD POLICY
    debug!("Migrating table: password_policy");
    let res = sqlx::query("SELECT data FROM config WHERE id = 'password_policy'")
        .fetch_one(&db_from)
        .await?;
    let bytes: Vec<u8> = res.get("data");
    inserts::password_policy(bytes).await?;

    // REFRESH TOKENS
    debug!("Migrating table: refresh_tokens");
    let before = sqlx::query_as::<_, RefreshToken>("SELECT * FROM refresh_tokens")
        .fetch_all(&db_from)
        .await?;
    inserts::refresh_tokens(before).await?;

    // ROLES
    debug!("Migrating table: roles");
    let before = sqlx::query_as::<_, Role>("SELECT * FROM roles")
        .fetch_all(&db_from)
        .await?;
    inserts::roles(before).await?;

    // SCOPES
    debug!("Migrating table: scopes");
    let before = sqlx::query_as::<_, Scope>("SELECT * FROM scopes")
        .fetch_all(&db_from)
        .await?;
    inserts::scopes(before).await?;

    // EVENTS
    debug!("Migrating table: events");
    let before = sqlx::query_as::<_, Event>("SELECT * FROM events")
        .fetch_all(&db_from)
        .await?;
    inserts::events(before).await?;

    // USER ATTR CONFIG
    debug!("Migrating table: user_attr_config");
    let before = sqlx::query_as::<_, UserAttrConfigEntity>("SELECT * FROM user_attr_config")
        .fetch_all(&db_from)
        .await?;
    inserts::user_attr_config(before).await?;

    // USER ATTR VALUES
    debug!("Migrating table: user_attr_values");
    let before = sqlx::query_as::<_, UserAttrValueEntity>("SELECT * FROM user_attr_values")
        .fetch_all(&db_from)
        .await?;
    inserts::user_attr_values(before).await?;

    // USERS VALUES
    debug!("Migrating table: users_values");
    let before = sqlx::query_as::<_, UserValues>("SELECT * FROM users_values")
        .fetch_all(&db_from)
        .await?;
    inserts::users_values(before).await?;

    // DEVICES
    debug!("Migrating table: devices");
    let before = sqlx::query_as::<_, DeviceEntity>("SELECT * FROM devices")
        .fetch_all(&db_from)
        .await?;
    inserts::devices(before).await?;

    // REFRESH TOKENS DEVICES
    debug!("Migrating table: devices");
    let before = sqlx::query_as::<_, RefreshTokenDevice>("SELECT * FROM refresh_tokens_devices")
        .fetch_all(&db_from)
        .await?;
    inserts::refresh_tokens_devices(before).await?;

    // SESSIONS
    debug!("Migrating table: sessions");
    let before = sqlx::query_as::<_, Session>("SELECT * FROM sessions")
        .fetch_all(&db_from)
        .await?;
    inserts::sessions(before).await?;

    // RECENT PASSWORDS
    debug!("Migrating table: recent_passwords");
    let before = sqlx::query_as::<_, RecentPasswordsEntity>("SELECT * FROM recent_passwords")
        .fetch_all(&db_from)
        .await?;
    inserts::recent_passwords(before).await?;

    // THEMES
    debug!("Migrating table: themes");
    let before = sqlx::query_as::<_, ThemeCssFull>("SELECT * FROM themes")
        .fetch_all(&db_from)
        .await?;
    inserts::themes(before).await?;

    // WEBIDS
    debug!("Migrating table: webids");
    let before = sqlx::query_as::<_, WebId>("SELECT * FROM webids")
        .fetch_all(&db_from)
        .await?;
    inserts::webids(before).await?;

    Ok(())
}

pub async fn migrate_from_postgres(db_from: sqlx::PgPool) -> Result<(), ErrorResponse> {
    info!("Starting migration to another DB");

    // CONFIG
    debug!("Migrating table: config");
    let before =
        sqlx::query_as::<_, ConfigEntity>("SELECT * FROM config WHERE id = 'password_policy'")
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
    let before = sqlx::query_as::<_, AuthProvider>("SELECT * FROM auth_providers")
        .fetch_all(&db_from)
        .await?;
    inserts::auth_providers(before).await?;

    // AUTH PROVIDER LOGOS
    debug!("Migrating table: auth_provider_logos");
    let before = sqlx::query_as::<_, Logo>(
        "SELECT auth_provider_id AS id, res, content_type, data FROM auth_provider_logos",
    )
    .fetch_all(&db_from)
    .await?;
    inserts::auth_provider_logos(before).await?;

    // USERS
    debug!("Migrating table: users");
    let before = sqlx::query_as::<_, User>("SELECT * FROM users")
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
    let before = sqlx::query_as::<_, Client>("SELECT * FROM clients")
        .fetch_all(&db_from)
        .await?;
    inserts::clients(before).await?;

    // CLIENTS DYN
    debug!("Migrating table: clients_dyn");
    let before = sqlx::query_as::<_, ClientDyn>("SELECT * FROM clients_dyn")
        .fetch_all(&db_from)
        .await?;
    inserts::clients_dyn(before).await?;

    // CLIENT LOGOS
    debug!("Migrating table: client_logos");
    let before = sqlx::query_as::<_, Logo>(
        "SELECT client_id AS id, res, content_type, data FROM client_logos",
    )
    .fetch_all(&db_from)
    .await?;
    inserts::client_logos(before).await?;

    // GROUPS
    debug!("Migrating table: groups");
    let before = sqlx::query_as::<_, Group>("SELECT * FROM groups")
        .fetch_all(&db_from)
        .await?;
    inserts::groups(before).await?;

    // JWKS
    debug!("Migrating table: jwks");
    let before = sqlx::query_as::<_, Jwk>("SELECT * FROM jwks")
        .fetch_all(&db_from)
        .await?;
    inserts::jwks(before).await?;

    // MAGIC LINKS
    debug!("Migrating table: magic_links");
    let before = sqlx::query_as::<_, MagicLink>("SELECT * FROM magic_links")
        .fetch_all(&db_from)
        .await?;
    inserts::magic_links(before).await?;

    // PASSWORD POLICY
    debug!("Migrating table: password_policy");
    let res = sqlx::query("SELECT data FROM config WHERE id = 'password_policy'")
        .fetch_one(&db_from)
        .await?;
    let bytes: Vec<u8> = res.get("data");
    inserts::password_policy(bytes).await?;

    // REFRESH TOKENS
    debug!("Migrating table: refresh_tokens");
    let before = sqlx::query_as::<_, RefreshToken>("SELECT * FROM refresh_tokens")
        .fetch_all(&db_from)
        .await?;
    inserts::refresh_tokens(before).await?;

    // ROLES
    debug!("Migrating table: roles");
    let before = sqlx::query_as::<_, Role>("SELECT * FROM roles")
        .fetch_all(&db_from)
        .await?;
    inserts::roles(before).await?;

    // SCOPES
    debug!("Migrating table: scopes");
    let before = sqlx::query_as::<_, Scope>("SELECT * FROM scopes")
        .fetch_all(&db_from)
        .await?;
    inserts::scopes(before).await?;

    // EVENTS
    debug!("Migrating table: events");
    let before = sqlx::query("SELECT * FROM events")
        .fetch_all(&db_from)
        .await?
        .into_iter()
        .map(|row| Event::from_row(&row).unwrap())
        .collect::<Vec<_>>();
    inserts::events(before).await?;

    // USER ATTR CONFIG
    debug!("Migrating table: user_attr_config");
    let before = sqlx::query_as::<_, UserAttrConfigEntity>("SELECT * FROM user_attr_config")
        .fetch_all(&db_from)
        .await?;
    inserts::user_attr_config(before).await?;

    // USER ATTR VALUES
    debug!("Migrating table: user_attr_values");
    let before = sqlx::query_as::<_, UserAttrValueEntity>("SELECT * FROM user_attr_values")
        .fetch_all(&db_from)
        .await?;
    inserts::user_attr_values(before).await?;

    // USERS VALUES
    debug!("Migrating table: users_values");
    let before = sqlx::query_as::<_, UserValues>("SELECT * FROM users_values")
        .fetch_all(&db_from)
        .await?;
    inserts::users_values(before).await?;

    // DEVICES
    debug!("Migrating table: devices");
    let before = sqlx::query_as::<_, DeviceEntity>("SELECT * FROM devices")
        .fetch_all(&db_from)
        .await?;
    inserts::devices(before).await?;

    // REFRESH TOKENS DEVICES
    debug!("Migrating table: devices");
    let before = sqlx::query_as::<_, RefreshTokenDevice>("SELECT * FROM refresh_tokens_devices")
        .fetch_all(&db_from)
        .await?;
    inserts::refresh_tokens_devices(before).await?;

    // SESSIONS
    debug!("Migrating table: sessions");
    let before = sqlx::query_as::<_, Session>("SELECT * FROM sessions")
        .fetch_all(&db_from)
        .await?;
    inserts::sessions(before).await?;

    // RECENT PASSWORDS
    debug!("Migrating table: recent_passwords");
    let before = sqlx::query_as::<_, RecentPasswordsEntity>("SELECT * FROM recent_passwords")
        .fetch_all(&db_from)
        .await?;
    inserts::recent_passwords(before).await?;

    // THEMES
    debug!("Migrating table: themes");
    let before = sqlx::query_as::<_, ThemeCssFull>("SELECT * FROM themes")
        .fetch_all(&db_from)
        .await?;
    inserts::themes(before).await?;

    // WEBIDS
    debug!("Migrating table: webids");
    let before = sqlx::query_as::<_, WebId>("SELECT * FROM webids")
        .fetch_all(&db_from)
        .await?;
    inserts::webids(before).await?;

    Ok(())
}
