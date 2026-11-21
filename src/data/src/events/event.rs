use crate::database::DB;
use crate::email::mailer;
use crate::entity::failed_scim_tasks::ScimAction;
use crate::entity::login_locations::LoginLocation;
use crate::entity::users::User;
use crate::rauthy_config::RauthyConfig;
use chrono::{DateTime, Timelike, Utc};
use hiqlite::macros::{FromRow, params};
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

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventLevel {
    #[default]
    Info,
    Notice,
    Warning,
    Critical,
}

impl From<&EventLevel> for NotificationLevel {
    fn from(value: &EventLevel) -> Self {
        match value {
            EventLevel::Info => Self::Info,
            EventLevel::Notice => Self::Notice,
            EventLevel::Warning => Self::Warning,
            EventLevel::Critical => Self::Critical,
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

impl Display for EventLevel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Self::Info => "INFO    ",
            Self::Notice => "NOTICE  ",
            Self::Warning => "WARNING ",
            Self::Critical => "CRITICAL",
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
            0 => Self::Info,
            1 => Self::Notice,
            2 => Self::Warning,
            3 => Self::Critical,
            _ => unreachable!(),
        }
    }
}

impl From<i64> for EventLevel {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::Info,
            1 => Self::Notice,
            2 => Self::Warning,
            3 => Self::Critical,
            _ => unreachable!(),
        }
    }
}

impl EventLevel {
    #[must_use]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Info => "INFO",
            Self::Notice => "NOTICE",
            Self::Warning => "WARNING",
            Self::Critical => "CRITICAL",
        }
    }

    #[must_use]
    pub const fn value(&self) -> i16 {
        match self {
            Self::Info => 0,
            Self::Notice => 1,
            Self::Warning => 2,
            Self::Critical => 3,
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventType {
    // InvalidLogins is always the lowest int value -> needed for DB queries
    #[default]
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
    LoginNewLocation,
    TokenIssued,
    CredentialStuffing,
    EmailSendError,
    ConsentRevoked,
    LdapLogin,
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
            Self::SuspiciousApiScan => write!(f, "Suspicious API scan"),
            Self::LoginNewLocation => write!(f, "Login from new location"),
            Self::TokenIssued => write!(f, "JWT Token issued"),
            Self::CredentialStuffing => write!(f, "Possible credential stuffing"),
            Self::EmailSendError => write!(f, "E-Mail send error"),
            Self::ConsentRevoked => write!(f, "User consent revoked"),
            Self::LdapLogin => write!(f, "LDAP login"),
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
            rauthy_api_types::events::EventType::LoginNewLocation => Self::LoginNewLocation,
            rauthy_api_types::events::EventType::TokenIssued => Self::TokenIssued,
            rauthy_api_types::events::EventType::CredentialStuffing => Self::CredentialStuffing,
            rauthy_api_types::events::EventType::EmailSendError => Self::EmailSendError,
            rauthy_api_types::events::EventType::ConsentRevoked => Self::ConsentRevoked,
            rauthy_api_types::events::EventType::LdapLogin => Self::LdapLogin,
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
            EventType::LoginNewLocation => Self::LoginNewLocation,
            EventType::TokenIssued => Self::TokenIssued,
            EventType::CredentialStuffing => Self::CredentialStuffing,
            EventType::EmailSendError => Self::EmailSendError,
            EventType::ConsentRevoked => Self::ConsentRevoked,
            EventType::LdapLogin => Self::LdapLogin,
        }
    }
}

impl EventType {
    #[must_use]
    pub const fn as_str(&self) -> &str {
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
            Self::LoginNewLocation => "LoginNewLocation",
            Self::TokenIssued => "TokenIssued",
            Self::CredentialStuffing => "CredentialStuffing",
            Self::EmailSendError => "EmailSendError",
            Self::ConsentRevoked => "ConsentRevoked",
            Self::LdapLogin => "LdapLogin",
        }
    }

    // CAUTION: DO NOT change the order of these values! If new types should be added, always
    // add them to the end! To save space in the DB, these are not saved as String's but
    // as integers instead and deserialization of older events will fail, if the order is mixed up!
    #[must_use]
    pub const fn value(&self) -> i16 {
        match self {
            Self::InvalidLogins => 0,
            Self::IpBlacklisted => 1,
            Self::IpBlacklistRemoved => 2,
            Self::JwksRotated => 3,
            Self::NewUserRegistered => 4,
            Self::NewRauthyAdmin => 5,
            Self::NewRauthyVersion => 6,
            Self::PossibleBruteForce => 7,
            Self::RauthyStarted => 8,
            Self::RauthyHealthy => 9,
            Self::RauthyUnhealthy => 10,
            Self::SecretsMigrated => 11,
            Self::UserEmailChange => 12,
            Self::UserPasswordReset => 13,
            Self::Test => 14,
            Self::BackchannelLogoutFailed => 15,
            Self::ScimTaskFailed => 16,
            Self::ForcedLogout => 17,
            Self::UserLoginRevoke => 18,
            Self::SuspiciousApiScan => 19,
            Self::LoginNewLocation => 20,
            Self::TokenIssued => 21,
            Self::CredentialStuffing => 22,
            Self::EmailSendError => 23,
            Self::ConsentRevoked => 24,
            Self::LdapLogin => 25,
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
            "LoginNewLocation" => Self::LoginNewLocation,
            "TokenIssued" => Self::TokenIssued,
            "CredentialStuffing" => Self::CredentialStuffing,
            "EmailSendError" => Self::EmailSendError,
            "ConsentRevoked" => Self::ConsentRevoked,
            "LdapLogin" => Self::LdapLogin,
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
        Self::from(i64::from(value))
    }
}

impl From<i64> for EventType {
    fn from(value: i64) -> Self {
        match value {
            0 => Self::InvalidLogins,
            1 => Self::IpBlacklisted,
            2 => Self::IpBlacklistRemoved,
            3 => Self::JwksRotated,
            4 => Self::NewUserRegistered,
            5 => Self::NewRauthyAdmin,
            6 => Self::NewRauthyVersion,
            7 => Self::PossibleBruteForce,
            8 => Self::RauthyStarted,
            9 => Self::RauthyHealthy,
            10 => Self::RauthyUnhealthy,
            11 => Self::SecretsMigrated,
            12 => Self::UserEmailChange,
            13 => Self::UserPasswordReset,
            #[allow(clippy::match_same_arms)]
            14 => Self::Test,
            15 => Self::BackchannelLogoutFailed,
            16 => Self::ScimTaskFailed,
            17 => Self::ForcedLogout,
            18 => Self::UserLoginRevoke,
            19 => Self::SuspiciousApiScan,
            20 => Self::LoginNewLocation,
            21 => Self::TokenIssued,
            22 => Self::CredentialStuffing,
            23 => Self::EmailSendError,
            24 => Self::ConsentRevoked,
            25 => Self::LdapLogin,
            _ => Self::Test,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, FromRow)]
pub struct Event {
    pub id: String,
    pub timestamp: i64,
    #[column(from_i64)]
    pub level: EventLevel,
    #[column(from_i64)]
    pub typ: EventType,
    pub ip: Option<String>,
    pub data: Option<i64>,
    pub text: Option<String>,
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
            EventType::NewRauthyVersion
            | EventType::RauthyStarted
            | EventType::RauthyHealthy
            | EventType::RauthyUnhealthy
            | EventType::UserEmailChange
            | EventType::UserPasswordReset
            | EventType::Test
            | EventType::UserLoginRevoke
            | EventType::SuspiciousApiScan
            | EventType::LoginNewLocation
            | EventType::TokenIssued
            | EventType::EmailSendError
            | EventType::ConsentRevoked
            | EventType::LdapLogin => value.text.clone(),
            EventType::PossibleBruteForce => Some(format!(
                "From IP: `{}`",
                value.ip.as_deref().unwrap_or_default()
            )),
            EventType::SecretsMigrated => value.ip.clone(),
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
            EventType::CredentialStuffing => Some(format!(
                "Potential credential stuffing detected from IP: '{}'",
                value.ip.as_deref().unwrap_or_default()
            )),
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

        let sql = r"
INSERT INTO events (id, timestamp, level, typ, ip, data, text)
VALUES ($1, $2, $3, $4, $5, $6, $7)";

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
            let sql = r"
SELECT * FROM events
WHERE timestamp >= $1 AND timestamp <= $2 AND level >= $3 AND typ = $4
ORDER BY timestamp DESC";

            if is_hiqlite() {
                DB::hql()
                    .query_map(sql, params!(from, until, level, typ))
                    .await?
            } else {
                DB::pg_query(sql, &[&from, &until, &level, &typ], 32).await?
            }
        } else {
            let sql = r"
SELECT * FROM events
WHERE timestamp >= $1 AND timestamp <= $2 AND level >= $3
ORDER BY timestamp DESC";

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

    #[must_use]
    pub fn as_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

impl Event {
    #[must_use]
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

    /// The `EventLevel` will change depending on the amount of invalid logins
    #[must_use]
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
            level,
            EventType::InvalidLogins,
            Some(ip),
            Some(i64::from(failed_logins)),
            None,
        )
    }

    #[must_use]
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

    #[must_use]
    pub fn brute_force(ip: String) -> Self {
        Self::new(
            EventLevel::Critical,
            EventType::PossibleBruteForce,
            Some(ip),
            None,
            None,
        )
    }

    /// `revoked_by` names who revoked the consent: the user themselves, an admin by email, or
    /// an API key by name (see [`ConsentRevokedBy`]).
    #[must_use]
    pub fn consent_revoked(
        user_email: &str,
        client_id: &str,
        resource: &str,
        revoked_by: ConsentRevokedBy<'_>,
        ip: IpAddr,
    ) -> Self {
        let text = consent_revoked_text(user_email, client_id, resource, revoked_by);
        Self::new(
            RauthyConfig::get()
                .vars
                .events
                .level_consent_revoked
                .clone(),
            EventType::ConsentRevoked,
            Some(ip.to_string()),
            None,
            Some(text),
        )
    }

    #[must_use]
    pub fn cred_stuff(ip: String) -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_cred_stuff.clone(),
            EventType::CredentialStuffing,
            Some(ip),
            None,
            None,
        )
    }

    #[must_use]
    pub fn email_send_err(typ: mailer::EmailType, address: &str) -> Self {
        Self::new(
            RauthyConfig::get()
                .vars
                .events
                .level_email_send_error
                .clone(),
            EventType::EmailSendError,
            None,
            None,
            Some(format!("{typ} -> {address}")),
        )
    }

    #[must_use]
    pub fn force_logout(user_email: String) -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_force_logout.clone(),
            EventType::ForcedLogout,
            None,
            None,
            Some(user_email),
        )
    }

    #[must_use]
    pub fn ip_blacklisted(exp: DateTime<Utc>, ip: String) -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_ip_blacklisted.clone(),
            EventType::IpBlacklisted,
            Some(ip),
            Some(exp.timestamp()),
            None,
        )
    }

    #[must_use]
    pub fn ip_blacklist_removed(ip: String) -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_ip_blacklisted.clone(),
            EventType::IpBlacklistRemoved,
            Some(ip),
            None,
            None,
        )
    }

    #[must_use]
    pub fn new_login_location(user: &User, loc: &LoginLocation) -> Self {
        let text = loc.location.as_ref().map_or_else(
            || format!("{} / {}", user.email, loc.user_agent),
            |location| format!("{} / {} / {}", user.email, loc.user_agent, location),
        );

        Self::new(
            RauthyConfig::get()
                .vars
                .events
                .level_new_login_location
                .clone(),
            EventType::LoginNewLocation,
            Some(loc.ip.clone()),
            None,
            Some(text),
        )
    }

    #[must_use]
    pub fn new_user(email: String, ip: String) -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_new_user.clone(),
            EventType::NewUserRegistered,
            Some(ip),
            None,
            Some(email),
        )
    }

    #[must_use]
    pub fn new_rauthy_admin(email: String, ip: String) -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_rauthy_admin.clone(),
            EventType::NewRauthyAdmin,
            Some(ip),
            None,
            Some(email),
        )
    }

    #[must_use]
    pub fn new_rauthy_version(version_url: String) -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_rauthy_version.clone(),
            EventType::NewRauthyVersion,
            None,
            None,
            Some(version_url),
        )
    }

    #[must_use]
    pub fn jwks_rotated() -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_jwks_rotate.clone(),
            EventType::JwksRotated,
            None,
            None,
            None,
        )
    }

    #[must_use]
    pub fn user_login_revoke(user_email: &str, bad_ip: IpAddr, bad_location: Option<&str>) -> Self {
        let loc = bad_location.unwrap_or("Unknown Location");
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

    #[must_use]
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

    #[must_use]
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

    #[must_use]
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

    #[must_use]
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

    #[must_use]
    pub fn scim_task_failed(client_id: &str, action: &ScimAction, retries: i32) -> Self {
        Self::new(
            RauthyConfig::get()
                .vars
                .events
                .level_scim_task_failed
                .clone(),
            EventType::BackchannelLogoutFailed,
            None,
            Some(i64::from(retries)),
            Some(format!("{client_id} / {action:?}")),
        )
    }

    #[must_use]
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

    #[must_use]
    pub fn suspicious_request(path: &str, ip: IpAddr, location: Option<&str>) -> Self {
        let loc = location.unwrap_or("Unknown Location");
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

    #[must_use]
    pub fn test(ip: IpAddr) -> Self {
        Self::new(
            EventLevel::Info,
            EventType::Test,
            Some(ip.to_string()),
            None,
            Some("This is a Test-Event".to_string()),
        )
    }

    #[must_use]
    pub fn ldap_login(client_id: &str, email: &str, ip: IpAddr) -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_ldap_login.clone(),
            EventType::LdapLogin,
            Some(ip.to_string()),
            None,
            Some(format!("{client_id} {email}")),
        )
    }

    #[must_use]
    pub fn token_issued(flow: &str, client_id: &str, email: Option<&str>) -> Self {
        Self::new(
            RauthyConfig::get().vars.events.level_token_issued.clone(),
            EventType::TokenIssued,
            None,
            None,
            Some(format!(
                "{} ({}) {}",
                client_id,
                flow,
                email.unwrap_or_default()
            )),
        )
    }

    #[must_use]
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

    #[must_use]
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

    #[must_use]
    pub fn fmt_data(&self) -> String {
        match self.typ {
            EventType::InvalidLogins => format!("Counter: {}", self.data.unwrap_or_default()),
            EventType::IpBlacklisted => {
                let d =
                    DateTime::from_timestamp(self.data.unwrap_or_default(), 0).unwrap_or_default();
                format!("IP blacklisted until {}", d.format("%Y/%m/%d %H:%M:%S"))
            }
            EventType::IpBlacklistRemoved => "IP removed from blacklist".to_string(),
            EventType::CredentialStuffing => format!(
                "Potential CredentialStuffing from IP: '{}'",
                self.ip.as_deref().unwrap_or_default()
            ),
            EventType::JwksRotated | EventType::PossibleBruteForce | EventType::SecretsMigrated => {
                String::default()
            }
            EventType::NewUserRegistered
            | EventType::NewRauthyAdmin
            | EventType::UserEmailChange => {
                format!("User E-Mail: {}", self.text.as_deref().unwrap_or_default())
            }
            EventType::NewRauthyVersion => {
                format!(
                    "New Rauthy Version available: {}",
                    self.text.as_deref().unwrap_or_default()
                )
            }
            // PossibleBruteForce is currently not in use
            EventType::RauthyStarted | EventType::RauthyHealthy | EventType::RauthyUnhealthy => {
                self.text.clone().unwrap()
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
            EventType::UserLoginRevoke
            | EventType::SuspiciousApiScan
            | EventType::LoginNewLocation
            | EventType::TokenIssued
            | EventType::EmailSendError
            | EventType::ConsentRevoked
            | EventType::LdapLogin => self.text.clone().unwrap_or_default(),
        }
    }

    #[inline(always)]
    pub async fn send(self) -> Result<(), ErrorResponse> {
        match RauthyConfig::get().tx_events.send_async(self).await {
            Ok(()) => Ok(()),
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

/// Who revoked a consent, for [`Event::consent_revoked`]
#[derive(Debug, Clone, Copy)]
pub enum ConsentRevokedBy<'a> {
    User,
    /// The email of the admin
    Admin(&'a str),
    /// The name of the API key
    ApiKey(&'a str),
}

/// The text of [`Event::consent_revoked`].
fn consent_revoked_text(
    user_email: &str,
    client_id: &str,
    resource: &str,
    revoked_by: ConsentRevokedBy<'_>,
) -> String {
    let target = if resource.is_empty() {
        format!("`{client_id}`")
    } else {
        format!("`{client_id}` / `{resource}`")
    };
    let by = match revoked_by {
        ConsentRevokedBy::User => "the user".to_string(),
        ConsentRevokedBy::Admin(admin) => format!("admin `{admin}`"),
        ConsentRevokedBy::ApiKey(name) => format!("API key `{name}`"),
    };
    format!("Consent of `{user_email}` for client {target} revoked by {by}")
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn consent_revoked_round_trip() {
        let typ = EventType::ConsentRevoked;
        assert_eq!(typ.value(), 24);
        assert_eq!(EventType::from(typ.value()), typ);
        assert_eq!(EventType::from(i64::from(typ.value())), typ);
        assert_eq!(typ.as_str(), "ConsentRevoked");
        assert_eq!(EventType::from(typ.as_str().to_string()), typ);
        assert_eq!(typ.to_string(), "User consent revoked");
        assert_eq!(
            EventType::from(rauthy_api_types::events::EventType::from(typ.clone())),
            typ
        );
    }

    #[test]
    fn consent_revoked_text_by_target() {
        assert_eq!(
            consent_revoked_text("u@example.com", "c1", "", ConsentRevokedBy::User),
            "Consent of `u@example.com` for client `c1` revoked by the user"
        );
        assert_eq!(
            consent_revoked_text("u@example.com", "c1", "", ConsentRevokedBy::ApiKey("k1")),
            "Consent of `u@example.com` for client `c1` revoked by API key `k1`"
        );
        let text = consent_revoked_text(
            "u@example.com",
            "c1",
            "https://api.example.com",
            ConsentRevokedBy::Admin("admin@example.com"),
        );
        assert_eq!(
            text,
            "Consent of `u@example.com` for client `c1` / `https://api.example.com` revoked \
            by admin `admin@example.com`"
        );

        // the text is what the event shows
        let event = Event::new(
            EventLevel::Notice,
            EventType::ConsentRevoked,
            Some("127.0.0.1".to_string()),
            None,
            Some(text.clone()),
        );
        assert_eq!(event.fmt_data(), text);
    }
}
