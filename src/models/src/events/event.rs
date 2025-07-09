use crate::database::DB;
use crate::entity::failed_scim_tasks::ScimAction;
use crate::rauthy_config::RauthyConfig;
use chrono::{DateTime, Timelike, Utc};
use hiqlite::Row;
use hiqlite_macros::params;
use rauthy_api_types::events::EventResponse;
use rauthy_common::is_hiqlite;
use rauthy_common::utils::{get_local_hostname, get_rand};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_notify::{Notification, NotificationLevel};
use serde::{Deserialize, Serialize};
use std::cmp::max;
use std::fmt::{Display, Formatter};
use std::net::IpAddr;
use std::str::FromStr;
use tracing::error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventLevel {
    Info,
    Notice,
    Warning,
    Critical,
}

impl From<&EventLevel> for NotificationLevel {
    fn from(value: &EventLevel) -> Self {
        match value {
            EventLevel::Info => NotificationLevel::Info,
            EventLevel::Notice => NotificationLevel::Notice,
            EventLevel::Warning => NotificationLevel::Warning,
            EventLevel::Critical => NotificationLevel::Critical,
        }
    }
}

impl From<rauthy_api_types::events::EventLevel> for EventLevel {
    fn from(value: rauthy_api_types::events::EventLevel) -> Self {
        match value {
            rauthy_api_types::events::EventLevel::Info => Self::Info,
            rauthy_api_types::events::EventLevel::Notice => Self::Notice,
            rauthy_api_types::events::EventLevel::Warning => Self::Warning,
            rauthy_api_types::events::EventLevel::Critical => Self::Critical,
        }
    }
}

impl From<EventLevel> for rauthy_api_types::events::EventLevel {
    fn from(value: EventLevel) -> Self {
        match value {
            EventLevel::Info => Self::Info,
            EventLevel::Notice => Self::Notice,
            EventLevel::Warning => Self::Warning,
            EventLevel::Critical => Self::Critical,
        }
    }
}

impl Default for EventLevel {
    fn default() -> Self {
        Self::Info
    }
}

impl Display for EventLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            EventLevel::Info => "INFO    ",
            EventLevel::Notice => "NOTICE  ",
            EventLevel::Warning => "WARNING ",
            EventLevel::Critical => "CRITICAL",
        };
        write!(f, "{s}")
    }
}

impl FromStr for EventLevel {
    type Err = ErrorResponse;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let slf = match s {
            "info" => Self::Info,
            "notice" => Self::Notice,
            "warning" => Self::Warning,
            "critical" => Self::Critical,
            _ => {
                return Err(ErrorResponse::new(
                    ErrorResponseType::Internal,
                    "Cannot parse EventLevel",
                ));
            }
        };
        Ok(slf)
    }
}

impl From<i16> for EventLevel {
    fn from(value: i16) -> Self {
        match value {
            0 => EventLevel::Info,
            1 => EventLevel::Notice,
            2 => EventLevel::Warning,
            3 => EventLevel::Critical,
            _ => unreachable!(),
        }
    }
}

impl From<i64> for EventLevel {
    fn from(value: i64) -> Self {
        match value {
            0 => EventLevel::Info,
            1 => EventLevel::Notice,
            2 => EventLevel::Warning,
            3 => EventLevel::Critical,
            _ => unreachable!(),
        }
    }
}

impl EventLevel {
    pub fn as_str(&self) -> &str {
        match self {
            EventLevel::Info => "INFO",
            EventLevel::Notice => "NOTICE",
            EventLevel::Warning => "WARNING",
            EventLevel::Critical => "CRITICAL",
        }
    }

    pub fn value(&self) -> i16 {
        match self {
            EventLevel::Info => 0,
            EventLevel::Notice => 1,
            EventLevel::Warning => 2,
            EventLevel::Critical => 3,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventType {
    InvalidLogins,
    IpBlacklisted,
    IpBlacklistRemoved,
    JwksRotated,
    NewUserRegistered,
    NewRauthyAdmin,
    NewRauthyVersion,
    PossibleBruteForce, // currently unused - kind of covered by IpBlacklisted
    RauthyStarted,
    RauthyHealthy,
    RauthyUnhealthy,
    SecretsMigrated,
    UserEmailChange,
    UserPasswordReset,
    Test,
    BackchannelLogoutFailed,
    ScimTaskFailed,
    ForcedLogout,
    UserLoginRevoke,
    SuspiciousApiScan,
}

impl Default for EventType {
    fn default() -> Self {
        // InvalidLogins is always the lowest int value -> needed for DB queries
        Self::InvalidLogins
    }
}

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLogins => write!(f, "Invalid logins"),
            Self::IpBlacklisted => write!(f, "IP blacklisted"),
            Self::IpBlacklistRemoved => write!(f, "IP blacklist removed"),
            Self::JwksRotated => write!(f, "JWKS has been rotated"),
            Self::NewUserRegistered => write!(f, "New user registered"),
            Self::NewRauthyAdmin => write!(f, "New Rauthy_admin member"),
            Self::NewRauthyVersion => write!(f, "New Rauthy App Version available"),
            Self::PossibleBruteForce => write!(f, "Possible brute force"),
            Self::RauthyStarted => write!(f, "Rauthy has been restarted"),
            Self::RauthyHealthy => write!(f, "Rauthy is healthy"),
            Self::RauthyUnhealthy => write!(f, "Rauthy is unhealthy"),
            Self::SecretsMigrated => write!(f, "Secrets have been migrated"),
            Self::UserEmailChange => write!(f, "User's E-Mail has been changed"),
            Self::UserPasswordReset => write!(f, "User has reset its password"),
            Self::Test => write!(f, "TEST"),
            Self::BackchannelLogoutFailed => write!(f, "Backchannel logout failed"),
            Self::ScimTaskFailed => write!(f, "SCIM task failed"),
            Self::ForcedLogout => write!(f, "Forced user logout"),
            Self::UserLoginRevoke => write!(f, "User revoked illegal login"),
            Self::SuspiciousApiScan => write!(f, "Suspicous API scan"),
        }
    }
}

impl From<rauthy_api_types::events::EventType> for EventType {
    fn from(value: rauthy_api_types::events::EventType) -> Self {
        match value {
            rauthy_api_types::events::EventType::InvalidLogins => Self::InvalidLogins,
            rauthy_api_types::events::EventType::IpBlacklisted => Self::IpBlacklisted,
            rauthy_api_types::events::EventType::IpBlacklistRemoved => Self::IpBlacklistRemoved,
            rauthy_api_types::events::EventType::JwksRotated => Self::JwksRotated,
            rauthy_api_types::events::EventType::NewUserRegistered => Self::NewUserRegistered,
            rauthy_api_types::events::EventType::NewRauthyAdmin => Self::NewRauthyAdmin,
            rauthy_api_types::events::EventType::NewRauthyVersion => Self::NewRauthyVersion,
            rauthy_api_types::events::EventType::PossibleBruteForce => Self::PossibleBruteForce,
            rauthy_api_types::events::EventType::RauthyStarted => Self::RauthyStarted,
            rauthy_api_types::events::EventType::RauthyHealthy => Self::RauthyHealthy,
            rauthy_api_types::events::EventType::RauthyUnhealthy => Self::RauthyUnhealthy,
            rauthy_api_types::events::EventType::SecretsMigrated => Self::SecretsMigrated,
            rauthy_api_types::events::EventType::UserEmailChange => Self::UserEmailChange,
            rauthy_api_types::events::EventType::UserPasswordReset => Self::UserPasswordReset,
            rauthy_api_types::events::EventType::Test => Self::Test,
            rauthy_api_types::events::EventType::BackchannelLogoutFailed => {
                Self::BackchannelLogoutFailed
            }
            rauthy_api_types::events::EventType::ScimTaskFailed => Self::ScimTaskFailed,
            rauthy_api_types::events::EventType::ForcedLogout => Self::ForcedLogout,
            rauthy_api_types::events::EventType::UserLoginRevoke => Self::UserLoginRevoke,
            rauthy_api_types::events::EventType::SuspiciousApiScan => Self::SuspiciousApiScan,
        }
    }
}

impl From<EventType> for rauthy_api_types::events::EventType {
    fn from(value: EventType) -> Self {
        match value {
            EventType::InvalidLogins => Self::InvalidLogins,
            EventType::IpBlacklisted => Self::IpBlacklisted,
            EventType::IpBlacklistRemoved => Self::IpBlacklistRemoved,
            EventType::JwksRotated => Self::JwksRotated,
            EventType::NewUserRegistered => Self::NewUserRegistered,
            EventType::NewRauthyAdmin => Self::NewRauthyAdmin,
            EventType::NewRauthyVersion => Self::NewRauthyVersion,
            EventType::PossibleBruteForce => Self::PossibleBruteForce,
            EventType::RauthyStarted => Self::RauthyStarted,
            EventType::RauthyHealthy => Self::RauthyHealthy,
            EventType::RauthyUnhealthy => Self::RauthyUnhealthy,
            EventType::SecretsMigrated => Self::SecretsMigrated,
            EventType::UserEmailChange => Self::UserEmailChange,
            EventType::UserPasswordReset => Self::UserPasswordReset,
            EventType::Test => Self::Test,
            EventType::BackchannelLogoutFailed => Self::BackchannelLogoutFailed,
            EventType::ScimTaskFailed => Self::ScimTaskFailed,
            EventType::ForcedLogout => Self::ForcedLogout,
            EventType::UserLoginRevoke => Self::UserLoginRevoke,
            EventType::SuspiciousApiScan => Self::SuspiciousApiScan,
        }
    }
}

impl EventType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::InvalidLogins => "InvalidLogins",
            Self::IpBlacklisted => "IpBlacklisted",
            Self::IpBlacklistRemoved => "IpBlacklistRemoved",
            Self::JwksRotated => "JwksRotated",
            Self::NewUserRegistered => "NewUserRegistered",
            Self::NewRauthyAdmin => "NewRauthyAdmin",
            Self::NewRauthyVersion => "NewRauthyVersion",
            Self::PossibleBruteForce => "PossibleBruteForce",
            Self::RauthyStarted => "RauthyRestarted",
            Self::RauthyHealthy => "RauthyHealthy",
            Self::RauthyUnhealthy => "RauthyUnhealthy",
            Self::SecretsMigrated => "SecretsMigrated",
            Self::UserEmailChange => "UserEmailChange",
            Self::UserPasswordReset => "UserPasswordReset",
            Self::Test => "TEST",
            Self::BackchannelLogoutFailed => "BackchannelLogoutFailed",
            Self::ScimTaskFailed => "ScimTaskFailed",
            Self::ForcedLogout => "ForcedLogout",
            Self::UserLoginRevoke => "UserLoginRevoke",
            Self::SuspiciousApiScan => "SuspiciousApiScan",
        }
    }

    // CAUTION: DO NOT change the order of these values! If new types should be added, always
    // add them to the end! To save space in the DB, these are not saved as String's but
    // as integers instead and deserialization of older events will fail, if the order is mixed up!
    pub fn value(&self) -> i16 {
        match self {
            EventType::InvalidLogins => 0,
            EventType::IpBlacklisted => 1,
            EventType::IpBlacklistRemoved => 2,
            EventType::JwksRotated => 3,
            EventType::NewUserRegistered => 4,
            EventType::NewRauthyAdmin => 5,
            EventType::NewRauthyVersion => 6,
            EventType::PossibleBruteForce => 7,
            EventType::RauthyStarted => 8,
            EventType::RauthyHealthy => 9,
            EventType::RauthyUnhealthy => 10,
            EventType::SecretsMigrated => 11,
            EventType::UserEmailChange => 12,
            EventType::UserPasswordReset => 13,
            EventType::Test => 14,
            EventType::BackchannelLogoutFailed => 15,
            EventType::ScimTaskFailed => 16,
            EventType::ForcedLogout => 17,
            EventType::UserLoginRevoke => 18,
            EventType::SuspiciousApiScan => 19,
        }
    }
}

impl From<String> for EventType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "InvalidLogins" => Self::InvalidLogins,
            "IpBlacklisted" => Self::IpBlacklisted,
            "IpBlacklistRemoved" => Self::IpBlacklistRemoved,
            "JwksRotated" => Self::JwksRotated,
            "NewUserRegistered" => Self::NewUserRegistered,
            "NewRauthyAdmin" => Self::NewRauthyAdmin,
            "NewRauthyVersion" => Self::NewRauthyVersion,
            "PossibleBruteForce" => Self::PossibleBruteForce,
            "RauthyRestarted" => Self::RauthyStarted,
            "RauthyHealthy" => Self::RauthyHealthy,
            "RauthyUnhealthy" => Self::RauthyUnhealthy,
            "SecretsMigrated" => Self::SecretsMigrated,
            "UserEmailChange" => Self::UserEmailChange,
            "UserPasswordReset" => Self::UserPasswordReset,
            "TEST" => Self::Test,
            "BackchannelLogoutFailed" => Self::BackchannelLogoutFailed,
            "ScimTaskFailed" => Self::ScimTaskFailed,
            "ForcedLogout" => Self::ForcedLogout,
            "UserLoginRevoke" => Self::UserLoginRevoke,
            "SuspiciousApiScan" => Self::SuspiciousApiScan,
            // just return test to never panic
            s => {
                error!("EventType::from() for invalid String: {s}");
                Self::Test
            }
        }
    }
}

impl From<i16> for EventType {
    fn from(value: i16) -> Self {
        Self::from(value as i64)
    }
}

impl From<i64> for EventType {
    fn from(value: i64) -> Self {
        match value {
            0 => EventType::InvalidLogins,
            1 => EventType::IpBlacklisted,
            2 => EventType::IpBlacklistRemoved,
            3 => EventType::JwksRotated,
            4 => EventType::NewUserRegistered,
            5 => EventType::NewRauthyAdmin,
            6 => EventType::NewRauthyVersion,
            7 => EventType::PossibleBruteForce,
            8 => EventType::RauthyStarted,
            9 => EventType::RauthyHealthy,
            10 => EventType::RauthyUnhealthy,
            11 => EventType::SecretsMigrated,
            12 => EventType::UserEmailChange,
            13 => EventType::UserPasswordReset,
            14 => EventType::Test,
            15 => EventType::BackchannelLogoutFailed,
            16 => EventType::ScimTaskFailed,
            17 => EventType::ForcedLogout,
            18 => EventType::UserLoginRevoke,
            19 => EventType::SuspiciousApiScan,
            _ => EventType::Test,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub timestamp: i64,
    pub level: EventLevel,
    pub typ: EventType,
    pub ip: Option<String>,
    pub data: Option<i64>,
    pub text: Option<String>,
}

impl<'r> From<hiqlite::Row<'r>> for Event {
    fn from(mut row: Row<'r>) -> Self {
        Self {
            id: row.get("id"),
            timestamp: row.get("timestamp"),
            level: EventLevel::from(row.get::<i64>("level")),
            typ: EventType::from(row.get::<i64>("typ")),
            ip: row.get("ip"),
            data: row.get("data"),
            text: row.get("text"),
        }
    }
}

impl From<tokio_postgres::Row> for Event {
    fn from(row: tokio_postgres::Row) -> Self {
        Self {
            id: row.get("id"),
            timestamp: row.get("timestamp"),
            level: EventLevel::from(row.get::<_, i16>("level")),
            typ: EventType::from(row.get::<_, i16>("typ")),
            ip: row.get("ip"),
            data: row.get("data"),
            text: row.get("text"),
        }
    }
}

impl From<&Event> for Notification {
    fn from(value: &Event) -> Self {
        let icon = match value.level {
            EventLevel::Info => "ℹ️",
            EventLevel::Notice => "📢",
            EventLevel::Warning => "⚠️",
            EventLevel::Critical => "🆘",
        };
        let prefix = RauthyConfig::get().vars.email.sub_prefix.as_ref();
        let head = format!("{icon} {prefix} - {}", value.level.as_str());

        let d = DateTime::from_timestamp(value.timestamp / 1000, 0).unwrap_or_default();
        let row_1 = format!("{} {}", d.format("%Y/%m/%d %H:%M:%S"), value.typ);

        let row_2 = match value.typ {
            EventType::InvalidLogins => Some(format!(
                "{} invalid logins from IP: `{}`",
                value.data.unwrap_or_default(),
                value.ip.as_deref().unwrap_or_default()
            )),
            EventType::IpBlacklisted => {
                let d =
                    DateTime::from_timestamp(value.data.unwrap_or_default(), 0).unwrap_or_default();
                Some(format!(
                    "IP `{}` blacklisted until {}",
                    value.ip.as_deref().unwrap_or_default(),
                    d.format("%Y/%m/%d %H:%M:%S"),
                ))
            }
            EventType::IpBlacklistRemoved => Some(format!(
                "IP `{}` was removed from the blacklist",
                value.ip.as_deref().unwrap_or_default()
            )),
            EventType::JwksRotated => None,
            EventType::NewUserRegistered => Some(format!(
                "E-Mail `{}` registered from IP: `{}`",
                value.text.as_deref().unwrap_or_default(),
                value.ip.as_deref().unwrap_or_default()
            )),
            EventType::NewRauthyAdmin => Some(format!(
                "User `{}` added to `rauthy_admin` role from IP: `{}`",
                value.text.as_deref().unwrap_or_default(),
                value.ip.as_deref().unwrap_or_default()
            )),
            EventType::NewRauthyVersion => value.text.clone(),
            EventType::PossibleBruteForce => Some(format!(
                "From IP: `{}`",
                value.ip.as_deref().unwrap_or_default()
            )),
            EventType::RauthyStarted => value.text.clone(),
            EventType::RauthyHealthy => value.text.clone(),
            EventType::RauthyUnhealthy => value.text.clone(),
            EventType::SecretsMigrated => value.ip.clone(),
            EventType::UserEmailChange => value.text.clone(),
            EventType::UserPasswordReset => value.text.clone(),
            EventType::Test => value.text.clone(),
            EventType::BackchannelLogoutFailed => Some(format!(
                "Backchannel logout retries ({}) exceeded for: {}",
                value.data.unwrap_or_default(),
                value.text.as_deref().unwrap_or_default()
            )),
            EventType::ScimTaskFailed => Some(format!(
                "SCIM task retries ({}) exceeded for: {}",
                value.data.unwrap_or_default(),
                value.text.as_deref().unwrap_or_default()
            )),
            EventType::ForcedLogout => Some(format!(
                "User `{}` was force-logged-out",
                value.text.as_deref().unwrap_or_default()
            )),
            EventType::UserLoginRevoke => value.text.clone(),
            EventType::SuspiciousApiScan => value.text.clone(),
        };

        Self {
            level: NotificationLevel::from(&value.level),
            head,
            row_1,
            row_2,
        }
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ts = DateTime::from_timestamp_millis(self.timestamp).unwrap_or_default();
        write!(
            f,
            "EVENT {} {}:{}:{} {} {} {} {}",
            ts.date_naive(),
            ts.hour(),
            ts.minute(),
            ts.second(),
            self.level,
            self.typ,
            self.fmt_data(),
            self.ip.as_deref().unwrap_or_default(),
        )
    }
}

impl Event {
    pub async fn insert(&self) -> Result<(), ErrorResponse> {
        let level = self.level.value();
        let typ = self.typ.value();

        let sql = r#"
INSERT INTO events (id, timestamp, level, typ, ip, data, text)
VALUES ($1, $2, $3, $4, $5, $6, $7)"#;

        if is_hiqlite() {
            DB::hql()
                .execute(
                    sql,
                    params!(
                        &self.id,
                        self.timestamp,
                        level,
                        typ,
                        &self.ip,
                        self.data,
                        &self.text
                    ),
                )
                .await?;
        } else {
            DB::pg_execute(
                sql,
                &[
                    &self.id,
                    &self.timestamp,
                    &level,
                    &typ,
                    &self.ip,
                    &self.data,
                    &self.text,
                ],
            )
            .await?;
        }

        Ok(())
    }

    pub async fn find_all(
        mut from: i64,
        mut until: i64,
        level: EventLevel,
        typ: Option<EventType>,
    ) -> Result<Vec<Self>, ErrorResponse> {
        let level = level.value();

        // Events are special inside Rauthy -> they use ms precision.
        // To keep the API internally the same, we expect timestamps in seconds though.
        from *= 1000;
        until *= 1000;

        let res = if let Some(typ) = typ {
            let typ = typ.value();
            let sql = r#"
SELECT * FROM events
WHERE timestamp >= $1 AND timestamp <= $2 AND level >= $3 AND typ = $4
ORDER BY timestamp DESC"#;

            if is_hiqlite() {
                DB::hql()
                    .query_map(sql, params!(from, until, level, typ))
                    .await?
            } else {
                DB::pg_query(sql, &[&from, &until, &level, &typ], 32).await?
            }
        } else {
            let sql = r#"
SELECT * FROM events
WHERE timestamp >= $1 AND timestamp <= $2 AND level >= $3
ORDER BY timestamp DESC"#;

            if is_hiqlite() {
                DB::hql()
                    .query_map(sql, params!(from, until, level))
                    .await?
            } else {
                DB::pg_query(sql, &[&from, &until, &level], 32).await?
            }
        };

        Ok(res)
    }

    pub async fn find_latest(limit: i64) -> Result<Vec<Self>, ErrorResponse> {
        let sql = "SELECT * FROM events ORDER BY timestamp DESC LIMIT $1";
        let res = if is_hiqlite() {
            DB::hql().query_map(sql, params!(limit)).await?
        } else {
            let size_hint = max(limit, 1) as usize;
            DB::pg_query(sql, &[&limit], size_hint).await?
        };

        Ok(res)
    }

    pub fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl Event {
    pub fn new(
        level: EventLevel,
        typ: EventType,
        ip: Option<String>,
        data: Option<i64>,
        text: Option<String>,
    ) -> Self {
        // These short random strings are enough "id" because the PK in the DB is 'id + timestamp_millis'
        let id = get_rand(8);

        Self {
            id,
            timestamp: Utc::now().timestamp_millis(),
            level,
            typ,
            ip,
            data,
            text,
        }
    }

    /// The EventLevel will change depending on the amount of invalid logins
    pub fn invalid_login(failed_logins: u32, ip: String) -> Self {
        let events = &RauthyConfig::get().vars.events;
        let level = match failed_logins {
            l if l >= 25 => events.level_failed_logins_25.clone(),
            l if l >= 20 => events.level_failed_logins_20.clone(),
            l if l >= 15 => events.level_failed_logins_15.clone(),
            l if l >= 10 => events.level_failed_logins_10.clone(),
            l if l >= 7 => events.level_failed_logins_7.clone(),
            _ => events.level_failed_login.clone(),
        };
        Self::new(
            level.clone(),
            EventType::InvalidLogins,
            Some(ip),
            Some(failed_logins as i64),
            None,
        )
    }

    pub fn backchannel_logout_failed(client_id: &str, user_id: &str, retries: i64) -> Self {
        Self::new(
            RauthyConfig::get()
                .vars
                .events
                .level_backchannel_logout_failed
                .clone(),
            EventType::BackchannelLogoutFailed,
            None,
            Some(retries),
            Some(format!("{client_id} / {user_id}")),
        )
    }

    pub fn brute_force(ip: String) -> Self {
        Self::new(
            EventLevel::Critical,
            EventType::PossibleBruteForce,
            Some(ip),
            None,
            None,
        )
    }

    pub fn force_logout(user_email: String) -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_force_logout.clone(),
            EventType::ForcedLogout,
            None,
            None,
            Some(user_email),
        )
    }

    pub fn ip_blacklisted(exp: DateTime<Utc>, ip: String) -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_ip_blacklisted.clone(),
            EventType::IpBlacklisted,
            Some(ip),
            Some(exp.timestamp()),
            None,
        )
    }

    pub fn ip_blacklist_removed(ip: String) -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_ip_blacklisted.clone(),
            EventType::IpBlacklistRemoved,
            Some(ip),
            None,
            None,
        )
    }

    pub fn new_user(email: String, ip: String) -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_new_user.clone(),
            EventType::NewUserRegistered,
            Some(ip),
            None,
            Some(email),
        )
    }

    pub fn new_rauthy_admin(email: String, ip: String) -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_rauthy_admin.clone(),
            EventType::NewRauthyAdmin,
            Some(ip),
            None,
            Some(email),
        )
    }

    pub fn new_rauthy_version(version_url: String) -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_rauthy_version.clone(),
            EventType::NewRauthyVersion,
            None,
            None,
            Some(version_url),
        )
    }

    pub fn jwks_rotated() -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_jwks_rotate.clone(),
            EventType::JwksRotated,
            None,
            None,
            None,
        )
    }

    pub fn user_login_revoke(
        user_email: &str,
        bad_ip: IpAddr,
        bad_location: Option<String>,
    ) -> Self {
        let loc = bad_location.as_deref().unwrap_or("Unknown Location");
        let text = format!("User `{user_email}` revoked illegal login from {bad_ip} ({loc})");

        Self::new(
            RauthyConfig::get()
                .vars
                .events
                .level_user_login_revoke
                .clone(),
            EventType::UserLoginRevoke,
            Some(bad_ip.to_string()),
            None,
            Some(text),
        )
    }

    pub fn rauthy_started() -> Self {
        let text = format!("Rauthy has been started on host {}", get_local_hostname());
        Self::new(
            RauthyConfig::get().vars.events.level_rauthy_start.clone(),
            EventType::RauthyStarted,
            None,
            None,
            Some(text),
        )
    }

    pub fn rauthy_healthy() -> Self {
        let text = format!("Rauthy is healthy now on host {}", get_local_hostname());
        Self::new(
            RauthyConfig::get().vars.events.level_rauthy_healthy.clone(),
            EventType::RauthyHealthy,
            None,
            None,
            Some(text),
        )
    }

    pub fn rauthy_unhealthy_cache() -> Self {
        let text = format!(
            "The HA Cache layer is unhealthy on host {}",
            get_local_hostname()
        );
        Self::new(
            RauthyConfig::get()
                .vars
                .events
                .level_rauthy_unhealthy
                .clone(),
            EventType::RauthyUnhealthy,
            None,
            None,
            Some(text),
        )
    }

    pub fn rauthy_unhealthy_db() -> Self {
        Self::new(
            RauthyConfig::get()
                .vars
                .events
                .level_rauthy_unhealthy
                .clone(),
            EventType::RauthyUnhealthy,
            None,
            None,
            Some("The database connection is unhealthy".to_string()),
        )
    }

    pub fn scim_task_failed(client_id: &str, action: &ScimAction, retries: i64) -> Self {
        Self::new(
            RauthyConfig::get()
                .vars
                .events
                .level_scim_task_failed
                .clone(),
            EventType::BackchannelLogoutFailed,
            None,
            Some(retries),
            Some(format!("{client_id} / {action:?}")),
        )
    }

    pub fn secrets_migrated(ip: IpAddr) -> Self {
        Self::new(
            RauthyConfig::get()
                .vars
                .events
                .level_secrets_migrated
                .clone(),
            EventType::SecretsMigrated,
            Some(ip.to_string()),
            None,
            None,
        )
    }

    pub fn suspicious_request(path: &str, ip: IpAddr, location: Option<String>) -> Self {
        let loc = location.as_deref().unwrap_or("Unknown Location");
        let text = format!("Suspicious request to '{path}' from {ip} ({loc})");

        Self::new(
            RauthyConfig::get()
                .vars
                .events
                .level_suspicious_request
                .clone(),
            EventType::SuspiciousApiScan,
            Some(ip.to_string()),
            None,
            Some(text),
        )
    }

    pub fn test(ip: IpAddr) -> Self {
        Self::new(
            EventLevel::Info,
            EventType::Test,
            Some(ip.to_string()),
            None,
            Some("This is a Test-Event".to_string()),
        )
    }

    pub fn user_email_change(text: String, ip: Option<IpAddr>) -> Self {
        Self::new(
            RauthyConfig::get()
                .vars
                .events
                .level_user_email_change
                .clone(),
            EventType::UserEmailChange,
            ip.map(|ip| ip.to_string()),
            None,
            Some(text),
        )
    }

    pub fn user_password_reset(text: String, ip: Option<String>) -> Self {
        Self::new(
            RauthyConfig::get()
                .vars
                .events
                .level_user_password_reset
                .clone(),
            EventType::UserPasswordReset,
            ip,
            None,
            Some(text),
        )
    }

    pub fn fmt_data(&self) -> String {
        match self.typ {
            EventType::InvalidLogins => format!("Counter: {}", self.data.unwrap_or_default()),
            EventType::IpBlacklisted => {
                let d =
                    DateTime::from_timestamp(self.data.unwrap_or_default(), 0).unwrap_or_default();
                format!("IP blacklisted until {}", d.format("%Y/%m/%d %H:%M:%S"))
            }
            EventType::IpBlacklistRemoved => "IP removed from blacklist".to_string(),
            EventType::JwksRotated => String::default(),
            EventType::NewUserRegistered => {
                format!("User E-Mail: {}", self.text.as_deref().unwrap_or_default())
            }
            EventType::NewRauthyAdmin => {
                format!("User E-Mail: {}", self.text.as_deref().unwrap_or_default())
            }
            EventType::NewRauthyVersion => {
                format!(
                    "New Rauthy Version available: {}",
                    self.text.as_deref().unwrap_or_default()
                )
            }
            // PossibleBruteForce is currently not in use
            EventType::PossibleBruteForce => String::default(),
            EventType::RauthyStarted => self.text.clone().unwrap(),
            EventType::RauthyHealthy => self.text.clone().unwrap(),
            EventType::RauthyUnhealthy => self.text.clone().unwrap(),
            EventType::SecretsMigrated => String::default(),
            EventType::UserEmailChange => {
                format!("User E-Mail: {}", self.text.as_deref().unwrap_or_default())
            }
            EventType::UserPasswordReset => {
                format!(
                    "User {} has reset its password",
                    self.text.as_deref().unwrap_or_default()
                )
            }
            EventType::Test => {
                format!("Test Message: {}", self.text.as_deref().unwrap_or_default())
            }
            EventType::BackchannelLogoutFailed => {
                format!(
                    "Backchannel logout retries ({}) exceeded for: {}",
                    self.data.unwrap_or_default(),
                    self.text.as_deref().unwrap_or_default()
                )
            }
            EventType::ScimTaskFailed => {
                format!(
                    "SCIM task retries ({}) exceeded for: {}",
                    self.data.unwrap_or_default(),
                    self.text.as_deref().unwrap_or_default()
                )
            }
            EventType::ForcedLogout => {
                format!(
                    "User `{}` was force-logged-out",
                    self.text.as_deref().unwrap_or_default()
                )
            }
            EventType::UserLoginRevoke => self.text.clone().unwrap_or_default(),
            EventType::SuspiciousApiScan => self.text.clone().unwrap_or_default(),
        }
    }

    #[inline(always)]
    pub async fn send(self) -> Result<(), ErrorResponse> {
        match RauthyConfig::get().tx_events.send_async(self).await {
            Ok(_) => Ok(()),
            Err(err) => {
                error!(?err, "Event::send()");
                Err(ErrorResponse::new(
                    ErrorResponseType::Internal,
                    format!("Error sending event internally: {err:?}"),
                ))
            }
        }
    }
}

impl From<Event> for EventResponse {
    fn from(e: Event) -> Self {
        Self {
            id: e.id,
            timestamp: e.timestamp,
            level: e.level.into(),
            typ: e.typ.into(),
            ip: e.ip,
            data: e.data,
            text: e.text,
        }
    }
}
