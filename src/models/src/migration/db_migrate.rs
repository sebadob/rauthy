use crate::database::DB;
use crate::entity::api_keys::ApiKeyEntity;
use crate::entity::auth_providers::AuthProvider;
use crate::entity::clients::Client;
use crate::entity::clients_dyn::ClientDyn;
use crate::entity::config::ConfigEntity;
use crate::entity::db_version::DbVersion;
use crate::entity::devices::DeviceEntity;
use crate::entity::failed_backchannel_logout::FailedBackchannelLogout;
use crate::entity::groups::Group;
use crate::entity::jwk::Jwk;
use crate::entity::logos::Logo;
use crate::entity::magic_links::MagicLink;
use crate::entity::password::RecentPasswordsEntity;
use crate::entity::pictures::UserPicture;
use crate::entity::refresh_tokens::RefreshToken;
use crate::entity::refresh_tokens_devices::RefreshTokenDevice;
use crate::entity::roles::Role;
use crate::entity::scopes::Scope;
use crate::entity::sessions::Session;
use crate::entity::theme::ThemeCssFull;
use crate::entity::user_attr::{UserAttrConfigEntity, UserAttrValueEntity};
use crate::entity::user_login_states::UserLoginState;
use crate::entity::users::User;
use crate::entity::users_values::UserValues;
use crate::entity::webauthn::PasskeyEntity;
use crate::entity::webids::WebId;
use crate::events::event::{Event, EventLevel, EventType};
use crate::migration::inserts;
use hiqlite::params;
use itertools::Itertools;
use rauthy_common::constants::RAUTHY_VERSION;
use rauthy_common::utils::deserialize;
use rauthy_error::ErrorResponse;
use semver::Version;
use serde::Deserialize;
use std::env;
use std::fmt::Debug;
use std::str::FromStr;
use tracing::{debug, info};

async fn query_sqlite<T>(conn: &rusqlite::Connection, query: &str) -> Result<Vec<T>, ErrorResponse>
where
    T: Debug + for<'a> Deserialize<'a>,
{
    let mut stmt = conn.prepare(query)?;
    let rows = stmt.query(())?;
    Ok(serde_rusqlite::from_rows(rows)
        .map(|r| match r {
            Ok(t) => t,
            Err(err) => {
                panic!("Error deserializing row from query\n{}\n{:?}", query, err);
            }
        })
        .collect::<Vec<_>>())
}

/// Migrates `MIGRATE_DB_FROM` to current database
///
/// Note: This uses `rusqlite` blocking calls, but we don't really care, since this function
/// will be executed at startup only once, before the API is even up and running. Blocking calls
/// here will make the code cleaner.
pub async fn migrate_from_sqlite(db_from: &str) -> Result<(), ErrorResponse> {
    info!("Starting migration to another DB");

    let url = db_from.strip_prefix("sqlite:").unwrap_or(db_from);
    let conn = rusqlite::Connection::open(url)?;
    conn.pragma_update(None, "journal_mode", "WAL")?;
    conn.pragma_update(None, "foreign_keys", "ON")?;

    // before doing anything, make sure that we are on the same feature version
    let mut res = DB::hql()
        .query_raw_one("SELECT data FROM config WHERE id = 'db_version'", params!())
        .await?;
    let bytes: Vec<u8> = res.get("data");
    let version = deserialize::<DbVersion>(&bytes)?.version;
    check_feature_version_migrate(version);

    // CONFIG
    debug!("Migrating table: config");
    let before =
        query_sqlite::<ConfigEntity>(&conn, "SELECT * FROM config WHERE id = 'password_policy'")
            .await?;
    inserts::config(before).await?;

    // API KEYS
    debug!("Migrating table: api_keys");
    let before = query_sqlite::<ApiKeyEntity>(&conn, "SELECT * FROM api_keys").await?;
    inserts::api_keys(before).await?;

    // The users table has a FK to auth_providers - the order is important here!
    // AUTH PROVIDERS
    debug!("Migrating table: auth_providers");
    let before = query_sqlite::<AuthProvider>(&conn, "SELECT * FROM auth_providers").await?;
    inserts::auth_providers(before).await?;

    // AUTH PROVIDER LOGOS
    debug!("Migrating table: auth_provider_logos");
    let before = query_sqlite::<Logo>(
        &conn,
        "SELECT auth_provider_id AS id, res, content_type, data FROM auth_provider_logos",
    )
    .await?;
    inserts::auth_provider_logos(before).await?;

    // users has an FK to pictures
    // PICTURES
    debug!("Migrating table: pictures");
    let before = query_sqlite::<UserPicture>(&conn, "SELECT * FROM pictures").await?;
    inserts::pictures(before).await?;

    // USERS
    debug!("Migrating table: users");
    let before = query_sqlite::<User>(&conn, "SELECT * FROM users").await?;
    inserts::users(before).await?;

    // PASSKEYS
    debug!("Migrating table: passkeys");
    let before = query_sqlite::<PasskeyEntity>(&conn, "SELECT * FROM passkeys").await?;
    inserts::passkeys(before).await?;

    // Do not change the order - tables below have FKs to clients
    // CLIENTS
    debug!("Migrating table: clients");
    let before = query_sqlite::<Client>(&conn, "SELECT * FROM clients").await?;
    inserts::clients(before).await?;

    // CLIENTS DYN
    debug!("Migrating table: clients_dyn");
    let before = query_sqlite::<ClientDyn>(&conn, "SELECT * FROM clients_dyn").await?;
    inserts::clients_dyn(before).await?;

    // CLIENT LOGOS
    debug!("Migrating table: client_logos");
    let before = query_sqlite::<Logo>(
        &conn,
        "SELECT client_id AS id, res, content_type, data FROM client_logos",
    )
    .await?;
    inserts::client_logos(before).await?;

    // GROUPS
    debug!("Migrating table: groups");
    let before = query_sqlite::<Group>(&conn, "SELECT * FROM groups").await?;
    inserts::groups(before).await?;

    // JWKS
    debug!("Migrating table: jwks");
    let before = query_sqlite::<Jwk>(&conn, "SELECT * FROM jwks").await?;
    inserts::jwks(before).await?;

    // MAGIC LINKS
    debug!("Migrating table: magic_links");
    let before = query_sqlite::<MagicLink>(&conn, "SELECT * FROM magic_links").await?;
    inserts::magic_links(before).await?;

    // PASSWORD POLICY
    debug!("Migrating table: password_policy");
    let mut stmt = conn.prepare("SELECT data FROM config WHERE id = 'password_policy'")?;
    let mut rows = stmt.query(())?;
    let row = rows.next()?.unwrap();
    let bytes: Vec<u8> = row.get("data")?;
    inserts::password_policy(bytes).await?;

    // REFRESH TOKENS
    debug!("Migrating table: refresh_tokens");
    let before = query_sqlite::<RefreshToken>(&conn, "SELECT * FROM refresh_tokens").await?;
    inserts::refresh_tokens(before).await?;

    // ROLES
    debug!("Migrating table: roles");
    let before = query_sqlite::<Role>(&conn, "SELECT * FROM roles").await?;
    inserts::roles(before).await?;

    // SCOPES
    debug!("Migrating table: scopes");
    let before = query_sqlite::<Scope>(&conn, "SELECT * FROM scopes").await?;
    inserts::scopes(before).await?;

    // EVENTS
    debug!("Migrating table: events");
    let mut stmt = conn.prepare("SELECT * FROM events")?;
    let before = stmt
        .query_map([], |row| {
            Ok(Event {
                id: row.get("id")?,
                timestamp: row.get("timestamp")?,
                level: EventLevel::from(row.get::<_, i64>("level")?),
                typ: EventType::from(row.get::<_, i64>("typ")?),
                ip: row.get("ip")?,
                data: row.get("data")?,
                text: row.get("text")?,
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::events(before).await?;

    // USER ATTR CONFIG
    debug!("Migrating table: user_attr_config");
    let before =
        query_sqlite::<UserAttrConfigEntity>(&conn, "SELECT * FROM user_attr_config").await?;
    inserts::user_attr_config(before).await?;

    // USER ATTR VALUES
    debug!("Migrating table: user_attr_values");
    let before =
        query_sqlite::<UserAttrValueEntity>(&conn, "SELECT * FROM user_attr_values").await?;
    inserts::user_attr_values(before).await?;

    // USERS VALUES
    debug!("Migrating table: users_values");
    let before = query_sqlite::<UserValues>(&conn, "SELECT * FROM users_values").await?;
    inserts::users_values(before).await?;

    // DEVICES
    debug!("Migrating table: devices");
    let before = query_sqlite::<DeviceEntity>(&conn, "SELECT * FROM devices").await?;
    inserts::devices(before).await?;

    // REFRESH TOKENS DEVICES
    debug!("Migrating table: refresh_tokens_devices");
    let before =
        query_sqlite::<RefreshTokenDevice>(&conn, "SELECT * FROM refresh_tokens_devices").await?;
    inserts::refresh_tokens_devices(before).await?;

    // SESSIONS
    debug!("Migrating table: sessions");
    let before = query_sqlite::<Session>(&conn, "SELECT * FROM sessions").await?;
    inserts::sessions(before).await?;

    // USER LOGIN STATES
    debug!("Migrating table: user_login_states");
    let before = query_sqlite::<UserLoginState>(&conn, "SELECT * FROM user_login_states").await?;
    inserts::user_login_states(before).await?;

    // FAILED BACKCHANNEL LOGOUTS
    debug!("Migrating table: failed_backchannel_logouts");
    let before =
        query_sqlite::<FailedBackchannelLogout>(&conn, "SELECT * FROM failed_backchannel_logouts")
            .await?;
    inserts::failed_backchannel_logouts(before).await?;

    // RECENT PASSWORDS
    debug!("Migrating table: recent_passwords");
    let before =
        query_sqlite::<RecentPasswordsEntity>(&conn, "SELECT * FROM recent_passwords").await?;
    inserts::recent_passwords(before).await?;

    // THEMES
    debug!("Migrating table: themes");
    let before = query_sqlite::<ThemeCssFull>(&conn, "SELECT * FROM themes").await?;
    inserts::themes(before).await?;

    // WEBIDS
    debug!("Migrating table: webids");
    let before = query_sqlite::<WebId>(&conn, "SELECT * FROM webids").await?;
    inserts::webids(before).await?;

    Ok(())
}

pub async fn migrate_from_postgres() -> Result<(), ErrorResponse> {
    info!("Starting migration from Postgres to another DB");

    let host = env::var("MIGRATE_PG_HOST").expect("MIGRATE_PG_HOST is not set");
    let port = env::var("MIGRATE_PG_PORT")
        .unwrap_or_else(|_| "5432".to_string())
        .parse::<u16>()
        .expect("Cannot parse MIGRATE_PG_PORT to u16");
    let user = env::var("MIGRATE_PG_USER").expect("MIGRATE_PG_USER is not set");
    let password = env::var("MIGRATE_PG_PASSWORD").expect("MIGRATE_PG_PASSWORD is not set");
    let db_name = env::var("MIGRATE_PG_DB_NAME").unwrap_or_else(|_| "rauthy".to_string());
    let pool = DB::connect_postgres(&host, port, &user, &password, &db_name, 1).await?;
    let cl = pool.get().await?;

    // before doing anything, make sure that we are on the same feature version
    let mut rows: Vec<ConfigEntity> =
        DB::pg_query_map_with(&cl, "SELECT * FROM config WHERE id = 'db_version'", &[], 1).await?;
    if rows.is_empty() {
        panic!("The MIGRATE_DB_FROM database is empty or too old");
    }
    let bytes = rows.swap_remove(0).data;
    let version = deserialize::<DbVersion>(&bytes)?.version;
    check_feature_version_migrate(version);

    // CONFIG
    debug!("Migrating table: config");
    let before = DB::pg_query_map_with(
        &cl,
        "SELECT * FROM config WHERE id = 'password_policy'",
        &[],
        1,
    )
    .await?;
    inserts::config(before).await?;

    // API KEYS
    debug!("Migrating table: api_keys");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM api_keys", &[], 0).await?;
    inserts::api_keys(before).await?;

    // The users table has a FK to auth_providers - the order is important here!
    // AUTH PROVIDERS
    debug!("Migrating table: auth_providers");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM auth_providers", &[], 0).await?;
    inserts::auth_providers(before).await?;

    // AUTH PROVIDER LOGOS
    debug!("Migrating table: auth_provider_logos");
    let before = DB::pg_query_map_with(
        &cl,
        "SELECT auth_provider_id AS id, res, content_type, data FROM auth_provider_logos",
        &[],
        0,
    )
    .await?;
    inserts::auth_provider_logos(before).await?;

    // PICTURES
    debug!("Migrating table: pictures");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM pictures", &[], 0).await?;
    inserts::pictures(before).await?;

    // USERS
    debug!("Migrating table: users");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM users", &[], 2).await?;
    debug!("before users insert");
    inserts::users(before).await?;
    debug!("after users insert");

    // PASSKEYS
    debug!("Migrating table: passkeys");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM passkeys", &[], 0).await?;
    inserts::passkeys(before).await?;

    // Do not change the order - tables below have FKs to clients
    // CLIENTS
    debug!("Migrating table: clients");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM clients", &[], 2).await?;
    inserts::clients(before).await?;

    // CLIENTS DYN
    debug!("Migrating table: clients_dyn");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM clients_dyn", &[], 0).await?;
    inserts::clients_dyn(before).await?;

    // CLIENT LOGOS
    debug!("Migrating table: client_logos");
    let before = DB::pg_query_map_with(
        &cl,
        "SELECT client_id AS id, res, content_type, data FROM client_logos",
        &[],
        0,
    )
    .await?;
    inserts::client_logos(before).await?;

    // GROUPS
    debug!("Migrating table: groups");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM groups", &[], 4).await?;
    inserts::groups(before).await?;

    // JWKS
    debug!("Migrating table: jwks");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM jwks", &[], 8).await?;
    inserts::jwks(before).await?;

    // MAGIC LINKS
    debug!("Migrating table: magic_links");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM magic_links", &[], 0).await?;
    inserts::magic_links(before).await?;

    // PASSWORD POLICY
    debug!("Migrating table: password_policy");
    let mut rows: Vec<ConfigEntity> = DB::pg_query_map_with(
        &cl,
        "SELECT data FROM config WHERE id = 'password_policy'",
        &[],
        1,
    )
    .await?;
    let bytes = rows.swap_remove(0).data;
    inserts::password_policy(bytes).await?;

    // REFRESH TOKENS
    debug!("Migrating table: refresh_tokens");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM refresh_tokens", &[], 0).await?;
    inserts::refresh_tokens(before).await?;

    // ROLES
    debug!("Migrating table: roles");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM roles", &[], 6).await?;
    inserts::roles(before).await?;

    // SCOPES
    debug!("Migrating table: scopes");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM scopes", &[], 0).await?;
    inserts::scopes(before).await?;

    // EVENTS
    debug!("Migrating table: events");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM events", &[], 32).await?;
    inserts::events(before).await?;

    // USER ATTR CONFIG
    debug!("Migrating table: user_attr_config");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM user_attr_config", &[], 0).await?;
    inserts::user_attr_config(before).await?;

    // USER ATTR VALUES
    debug!("Migrating table: user_attr_values");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM user_attr_values", &[], 0).await?;
    inserts::user_attr_values(before).await?;

    // USERS VALUES
    debug!("Migrating table: users_values");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM users_values", &[], 0).await?;
    inserts::users_values(before).await?;

    // DEVICES
    debug!("Migrating table: devices");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM devices", &[], 0).await?;
    inserts::devices(before).await?;

    // REFRESH TOKENS DEVICES
    debug!("Migrating table: refresh_tokens_devices");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM refresh_tokens_devices", &[], 0).await?;
    inserts::refresh_tokens_devices(before).await?;

    // SESSIONS
    debug!("Migrating table: sessions");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM sessions", &[], 16).await?;
    inserts::sessions(before).await?;

    // USER LOGIN STATES
    debug!("Migrating table: user_login_states");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM user_login_states", &[], 0).await?;
    inserts::user_login_states(before).await?;

    // FAILED BACKCHANNEL LOGOUTS
    debug!("Migrating table: failed_backchannel_logouts");
    let before =
        DB::pg_query_map_with(&cl, "SELECT * FROM failed_backchannel_logouts", &[], 0).await?;
    inserts::failed_backchannel_logouts(before).await?;

    // RECENT PASSWORDS
    debug!("Migrating table: recent_passwords");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM recent_passwords", &[], 0).await?;
    inserts::recent_passwords(before).await?;

    // THEMES
    debug!("Migrating table: themes");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM themes", &[], 0).await?;
    inserts::themes(before).await?;

    // WEBIDS
    debug!("Migrating table: webids");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM webids", &[], 0).await?;
    inserts::webids(before).await?;

    Ok(())
}

/// Makes sure that the given version matches the current app version in major and feature level.
/// Will panic if they don't match.
fn check_feature_version_migrate(version: semver::Version) {
    let rauthy_version = Version::from_str(RAUTHY_VERSION).unwrap();

    if version.major != rauthy_version.major || version.minor != rauthy_version.minor {
        panic!(
            "MIGRATE_DB_FROM can only be used within the same major and minor version. \
            Only a difference in patch level is allowed"
        );
    }
}
