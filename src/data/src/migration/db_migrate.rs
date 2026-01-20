use crate::database::DB;
use crate::entity::api_keys::ApiKeyEntity;
use crate::entity::auth_providers::AuthProvider;
use crate::entity::clients::Client;
use crate::entity::clients_dyn::ClientDyn;
use crate::entity::clients_scim::ClientScim;
use crate::entity::config::ConfigEntity;
use crate::entity::db_version::DbVersion;
use crate::entity::devices::DeviceEntity;
use crate::entity::email_jobs::{EmailContentType, EmailJob, EmailJobFilter, EmailJobStatus};
use crate::entity::failed_backchannel_logout::FailedBackchannelLogout;
use crate::entity::failed_scim_tasks::{FailedScimTask, ScimAction};
use crate::entity::groups::Group;
use crate::entity::issued_tokens::IssuedToken;
use crate::entity::jwk::Jwk;
use crate::entity::login_locations::LoginLocation;
use crate::entity::logos::Logo;
use crate::entity::magic_links::MagicLink;
use crate::entity::pam::authorized_keys::AuthorizedKey;
use crate::entity::pam::groups::{PamGroup, PamGroupType};
use crate::entity::pam::hosts::PamHost;
use crate::entity::pam::users::PamUser;
use crate::entity::password::RecentPasswordsEntity;
use crate::entity::pictures::UserPicture;
use crate::entity::refresh_tokens::RefreshToken;
use crate::entity::refresh_tokens_devices::RefreshTokenDevice;
use crate::entity::roles::Role;
use crate::entity::scopes::Scope;
use crate::entity::sessions::{Session, SessionState};
use crate::entity::theme::{ThemeCss, ThemeCssFull};
use crate::entity::tos::ToS;
use crate::entity::tos_user_accept::ToSUserAccept;
use crate::entity::user_attr::{UserAttrConfigEntity, UserAttrValueEntity};
use crate::entity::user_login_states::UserLoginState;
use crate::entity::user_revoke::UserRevoke;
use crate::entity::users::User;
use crate::entity::users_values::UserValues;
use crate::entity::webauthn::PasskeyEntity;
use crate::entity::webids::WebId;
use crate::events::event::{Event, EventLevel, EventType};
use crate::migration::inserts;
use crate::rauthy_config::RauthyConfig;
use hiqlite_macros::params;
use itertools::Itertools;
use rauthy_common::constants::RAUTHY_VERSION;
use rauthy_common::utils::deserialize;
use rauthy_error::ErrorResponse;
use semver::Version;
use serde::Deserialize;
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
                panic!("Error deserializing row from query\n{query}\n{err:?}");
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
    let before = query_sqlite::<ConfigEntity>(&conn, "SELECT * FROM config").await?;
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
        "SELECT auth_provider_id AS id, res, content_type, data, updated FROM auth_provider_logos",
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
        "SELECT client_id AS id, res, content_type, data, updated FROM client_logos",
    )
    .await?;
    inserts::client_logos(before).await?;

    // CLIENTS SCIM
    debug!("Migrating table: clients_scim");
    let mut stmt = conn.prepare("SELECT * FROM clients_scim")?;
    let before = stmt
        .query_map([], |row| {
            let bearer_token = ClientScim::decrypt_bearer_token(row.get("bearer_token")?)
                .expect("Column clients_scim.bearer_token corrupted");
            Ok(ClientScim {
                client_id: row.get("client_id")?,
                bearer_token,
                base_uri: row.get("base_endpoint")?,
                sync_groups: row.get("sync_groups")?,
                group_sync_prefix: row.get("group_sync_prefix")?,
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::clients_scim(before).await?;

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

    // USER REVOKE
    debug!("Migrating table: user_revoke");
    let mut stmt = conn.prepare("SELECT * FROM user_revoke")?;
    let before = stmt
        .query_map([], |row| {
            Ok(UserRevoke {
                user_id: row.get("user_id")?,
                code: row.get("code")?,
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::user_revoke(before).await?;

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
    let mut stmt = conn.prepare("SELECT * FROM sessions")?;
    let before = stmt
        .query_map([], |row| {
            let state = SessionState::from_str(&row.get::<_, String>("state")?)
                .unwrap_or(SessionState::Unknown);
            Ok(Session {
                id: row.get("id")?,
                csrf_token: row.get("csrf_token")?,
                user_id: row.get("user_id")?,
                roles: row.get("roles")?,
                groups: row.get("groups")?,
                is_mfa: row.get("is_mfa")?,
                state,
                exp: row.get("exp")?,
                last_seen: row.get("last_seen")?,
                remote_ip: row.get("remote_ip")?,
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::sessions(before).await?;

    // USER LOGIN STATES
    debug!("Migrating table: user_login_states");
    let before = query_sqlite::<UserLoginState>(&conn, "SELECT * FROM user_login_states").await?;
    inserts::user_login_states(before).await?;

    // LOGIN LOCATIONS
    debug!("Migrating table: login_locations");
    let mut stmt = conn.prepare("SELECT * FROM login_locations")?;
    let before = stmt
        .query_map([], |row| {
            Ok(LoginLocation {
                user_id: row.get("user_id")?,
                browser_id: row.get("browser_id")?,
                ip: row.get("ip")?,
                last_seen: row.get("last_seen")?,
                user_agent: row.get("user_agent")?,
                location: row.get("location")?,
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::login_locations(before).await?;

    // FAILED BACKCHANNEL LOGOUTS
    debug!("Migrating table: failed_backchannel_logouts");
    let before =
        query_sqlite::<FailedBackchannelLogout>(&conn, "SELECT * FROM failed_backchannel_logouts")
            .await?;
    inserts::failed_backchannel_logouts(before).await?;

    // FAILED SCIM TASKS
    debug!("Migrating table: failed_scim_tasks");
    let mut stmt = conn.prepare("SELECT * FROM failed_scim_tasks")?;
    let before = stmt
        .query_map([], |row| {
            Ok(FailedScimTask {
                action: ScimAction::from(row.get::<_, String>("action")?.as_str()),
                client_id: row.get("client_id")?,
                retry_count: row.get::<_, i64>("retry_count")?,
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::failed_scim_tasks(before).await?;

    // RECENT PASSWORDS
    debug!("Migrating table: recent_passwords");
    let before =
        query_sqlite::<RecentPasswordsEntity>(&conn, "SELECT * FROM recent_passwords").await?;
    inserts::recent_passwords(before).await?;

    // THEMES
    debug!("Migrating table: themes");
    let mut stmt = conn.prepare("SELECT * FROM themes")?;
    let before = stmt
        .query_map([], |row| {
            let version: i64 = row.get("version")?;
            let (light, dark) = if version == 1 {
                let light = ThemeCss::from(row.get::<_, Vec<u8>>("light")?.as_slice());
                let dark = ThemeCss::from(row.get::<_, Vec<u8>>("dark")?.as_slice());
                (light, dark)
            } else {
                (ThemeCss::default_light(), ThemeCss::default_dark())
            };

            Ok(ThemeCssFull {
                client_id: row.get("client_id")?,
                last_update: row.get("last_update")?,
                version,
                light,
                dark,
                border_radius: row.get("border_radius")?,
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::themes(before).await?;

    // WEBIDS
    debug!("Migrating table: webids");
    let before = query_sqlite::<WebId>(&conn, "SELECT * FROM webids").await?;
    inserts::webids(before).await?;

    // PAM GROUPS
    debug!("Migrating table: pam_groups");
    let mut stmt = conn.prepare("SELECT * FROM pam_groups")?;
    let before = stmt
        .query_map([], |row| {
            Ok(PamGroup {
                id: row.get::<_, i64>("id")? as u32,
                name: row.get("name")?,
                typ: PamGroupType::from(row.get::<_, String>("typ")?.as_str()),
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::pam_groups(before).await?;

    // PAM HOSTS
    debug!("Migrating table: pam_hosts");
    let mut stmt = conn.prepare("SELECT * FROM pam_hosts")?;
    let before = stmt
        .query_map([], |row| {
            Ok(PamHost {
                id: row.get("id")?,
                hostname: row.get("hostname")?,
                gid: row.get::<_, i64>("gid")? as u32,
                secret: row.get("secret")?,
                force_mfa: row.get("force_mfa")?,
                local_password_only: row.get("local_password_only")?,
                notes: row.get("notes")?,
                ips: Vec::default(),
                aliases: Vec::default(),
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::pam_hosts(before).await?;

    // PAM HOSTS ALIASES
    debug!("Migrating table: pam_hosts_aliases");
    let mut stmt = conn.prepare("SELECT * FROM pam_hosts_aliases")?;
    let before = stmt
        .query_map([], |row| {
            let id: String = row.get("host_id")?;
            let alias: String = row.get("alias")?;
            Ok((id, alias))
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::pam_hosts_aliases(before).await?;

    // PAM HOSTS IPS
    debug!("Migrating table: pam_hosts_ips");
    let mut stmt = conn.prepare("SELECT * FROM pam_hosts_ips")?;
    let before = stmt
        .query_map([], |row| {
            let id: String = row.get("host_id")?;
            let ip: String = row.get("ip")?;
            Ok((id, ip))
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::pam_hosts_ips(before).await?;

    // PAM USERS
    debug!("Migrating table: pam_users");
    let mut stmt = conn.prepare("SELECT * FROM pam_users")?;
    let before = stmt
        .query_map([], |row| {
            Ok(PamUser {
                id: row.get::<_, i64>("id")? as u32,
                name: row.get("name")?,
                gid: row.get::<_, i64>("gid")? as u32,
                email: row.get("email")?,
                shell: row.get("shell")?,
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::pam_users(before).await?;

    // PAM REL GROUPS USERS
    debug!("Migrating table: pam_rel_groups_users");
    let mut stmt = conn.prepare("SELECT * FROM pam_rel_groups_users")?;
    let before = stmt
        .query_map([], |row| {
            let gid: i64 = row.get("gid")?;
            let uid: i64 = row.get("uid")?;
            let wheel: bool = row.get("wheel")?;
            Ok((gid, uid, wheel))
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::pam_rel_groups_users(before).await?;

    // TOS
    debug!("Migrating table: tos");
    let before = query_sqlite::<ToS>(&conn, "SELECT * FROM tos").await?;
    inserts::tos(before).await?;

    // TOS USER ACCEPT
    debug!("Migrating table: tos_user_accept");
    let before = query_sqlite::<ToSUserAccept>(&conn, "SELECT * FROM tos_user_accept").await?;
    inserts::tos_user_accept(before).await?;

    // EMAIL JOBS
    debug!("Migrating table: email_jobs");
    let mut stmt = conn.prepare("SELECT * FROM email_jobs")?;
    let before = stmt
        .query_map([], |row| {
            let status = EmailJobStatus::from(row.get::<_, i64>("status")? as i16);
            let filter = EmailJobFilter::from(row.get::<_, String>("filter")?.as_str());
            let content_type =
                EmailContentType::from(row.get::<_, String>("content_type")?.as_str());

            Ok(EmailJob {
                id: row.get("id")?,
                scheduled: row.get("scheduled")?,
                status,
                updated: row.get("updated")?,
                last_user_ts: row.get("last_user_ts")?,
                filter,
                content_type,
                subject: row.get("subject")?,
                body: row.get("body")?,
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::email_jobs(before).await?;

    // SSH AUTH KEYS
    debug!("Migrating table: ssh_auth_keys");
    let mut stmt = conn.prepare("SELECT * FROM ssh_auth_keys")?;
    let before = stmt
        .query_map([], |row| {
            Ok(AuthorizedKey {
                // type cast is safe because Rauthy controls the input
                pam_uid: row.get::<_, i64>("pam_uid")? as u32,
                ts_added: row.get("ts_added")?,
                expires: row.get("expires")?,
                typ: row.get("typ")?,
                data: row.get("data")?,
                comment: row.get("comment")?,
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::ssh_auth_keys(before).await?;

    // SSH AUTH KEYS USED
    debug!("Migrating table: ssh_auth_keys_used");
    let mut stmt = conn.prepare("SELECT * FROM ssh_auth_keys_used")?;
    let before = stmt
        .query_map([], |row| {
            let used_key_hash = row.get::<_, String>("used_key_hash")?;
            let ts_added = row.get::<_, i64>("ts_added")?;
            Ok((used_key_hash, ts_added))
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::ssh_auth_keys_used(before).await?;

    // ISSUED TOKENS
    debug!("Migrating table: issued_tokens");
    let mut stmt = conn.prepare("SELECT * FROM issued_tokens")?;
    let before = stmt
        .query_map([], |row| {
            Ok(IssuedToken {
                jti: row.get("jti")?,
                user_id: row.get("user_id")?,
                did: row.get("did")?,
                sid: row.get("sid")?,
                exp: row.get("exp")?,
                revoked: row.get("revoked")?,
            })
        })?
        .map(|r| r.unwrap())
        .collect_vec();
    inserts::issued_tokens(before).await?;

    Ok(())
}

pub async fn migrate_from_postgres() -> Result<(), ErrorResponse> {
    info!("Starting migration from Postgres to another DB");

    let vars = &RauthyConfig::get().vars.database;
    let host = vars
        .migrate_pg_host
        .as_ref()
        .expect("MIGRATE_PG_HOST is not set");
    let user = vars
        .migrate_pg_user
        .as_ref()
        .expect("MIGRATE_PG_USER is not set");
    let password = vars
        .migrate_pg_password
        .as_ref()
        .expect("MIGRATE_PG_PASSWORD is not set");
    let db_name = vars.migrate_pg_db_name.as_ref();
    let pool = DB::connect_postgres(host, vars.migrate_pg_port, user, password, db_name, 1).await?;
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
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM config", &[], 1).await?;
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
        "SELECT auth_provider_id AS id, res, content_type, data, updated FROM auth_provider_logos",
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
    inserts::users(before).await?;

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
        "SELECT client_id AS id, res, content_type, data, updated FROM client_logos",
        &[],
        0,
    )
    .await?;
    inserts::client_logos(before).await?;

    // CLIENTS SCIM
    debug!("Migrating table: clients_scim");
    let before = DB::pg_query_rows_with(&cl, "SELECT * FROM clients_scim", &[], 0)
        .await?
        .into_iter()
        .map(|row| {
            let bearer_token =
                ClientScim::decrypt_bearer_token(row.get::<_, Vec<u8>>("bearer_token"))
                    .expect("Column clients_scim.bearer_token corrupted");
            ClientScim {
                client_id: row.get("client_id"),
                bearer_token,
                base_uri: row.get("base_endpoint"),
                sync_groups: row.get("sync_groups"),
                group_sync_prefix: row.get("group_sync_prefix"),
            }
        })
        .collect::<Vec<_>>();
    inserts::clients_scim(before).await?;

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

    // USER REVOKE
    debug!("Migrating table: user_revoke");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM user_revoke", &[], 0).await?;
    inserts::user_revoke(before).await?;

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

    // LOGIN LOCATIONS
    debug!("Migrating table: login_locations");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM login_locations", &[], 0).await?;
    inserts::login_locations(before).await?;

    // FAILED BACKCHANNEL LOGOUTS
    debug!("Migrating table: failed_backchannel_logouts");
    let before =
        DB::pg_query_map_with(&cl, "SELECT * FROM failed_backchannel_logouts", &[], 0).await?;
    inserts::failed_backchannel_logouts(before).await?;

    // FAILED SCIM TASKS
    debug!("Migrating table: failed_scim_tasks");
    let before = DB::pg_query_rows_with(&cl, "SELECT * FROM failed_scim_tasks", &[], 0)
        .await?
        .into_iter()
        .map(|row| FailedScimTask {
            action: ScimAction::from(row.get::<_, String>("action").as_str()),
            client_id: row.get("client_id"),
            retry_count: row.get::<_, i32>("retry_count") as i64,
        })
        .collect::<Vec<_>>();
    inserts::failed_scim_tasks(before).await?;

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

    // PAM GROUPS
    debug!("Migrating table: pam_groups");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM pam_groups", &[], 0).await?;
    inserts::pam_groups(before).await?;

    // PAM HOSTS
    debug!("Migrating table: pam_hosts");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM pam_hosts", &[], 0).await?;
    inserts::pam_hosts(before).await?;

    // PAM HOSTS ALIASES
    debug!("Migrating table: pam_hosts_aliases");
    let before = DB::pg_query_rows_with(&cl, "SELECT * FROM pam_hosts_aliases", &[], 0)
        .await?
        .into_iter()
        .map(|row| {
            let id: String = row.get("host_id");
            let alias: String = row.get("alias");
            (id, alias)
        })
        .collect::<Vec<_>>();
    inserts::pam_hosts_aliases(before).await?;

    // PAM HOSTS IPS
    debug!("Migrating table: pam_hosts_ips");
    let before = DB::pg_query_rows_with(&cl, "SELECT * FROM pam_hosts_ips", &[], 0)
        .await?
        .into_iter()
        .map(|row| {
            let id: String = row.get("host_id");
            let ip: String = row.get("ip");
            (id, ip)
        })
        .collect::<Vec<_>>();
    inserts::pam_hosts_ips(before).await?;

    // PAM USERS
    debug!("Migrating table: pam_users");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM pam_users", &[], 0).await?;
    inserts::pam_users(before).await?;

    // PAM REL GROUPS USERS
    debug!("Migrating table: pam_rel_groups_users");
    let before = DB::pg_query_rows_with(&cl, "SELECT * FROM pam_rel_groups_users", &[], 0)
        .await?
        .into_iter()
        .map(|row| {
            let gid: i64 = row.get("gid");
            let uid: i64 = row.get("uid");
            let wheel: bool = row.get("wheel");
            (gid, uid, wheel)
        })
        .collect::<Vec<_>>();
    inserts::pam_rel_groups_users(before).await?;

    // TOS
    debug!("Migrating table: tos");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM tos", &[], 0).await?;
    inserts::tos(before).await?;

    // TOS USER ACCEPT
    debug!("Migrating table: tos_user_accept");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM tos_user_accept", &[], 0).await?;
    inserts::tos_user_accept(before).await?;

    // EMAIL JOBS
    debug!("Migrating table: email_jobs");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM email_jobs", &[], 0).await?;
    inserts::email_jobs(before).await?;

    // SSH AUTH KEYS
    debug!("Migrating table: ssh_auth_keys");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM ssh_auth_keys", &[], 0).await?;
    inserts::ssh_auth_keys(before).await?;

    // SSH AUTH KEYS USED
    debug!("Migrating table: ssh_auth_keys_used");
    let before = DB::pg_query_rows_with(&cl, "SELECT * FROM ssh_auth_keys_used", &[], 0)
        .await?
        .into_iter()
        .map(|row| {
            let used_key_hash: String = row.get("used_key_hash");
            let ts_added: i64 = row.get("ts_added");
            (used_key_hash, ts_added)
        })
        .collect::<Vec<_>>();
    inserts::ssh_auth_keys_used(before).await?;

    // ISSUED TOKENS
    debug!("Migrating table: issued_tokens");
    let before = DB::pg_query_map_with(&cl, "SELECT * FROM issued_tokens", &[], 0).await?;
    inserts::issued_tokens(before).await?;

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
