use crate::app_state::DbPool;
use crate::events::{
    EVENT_LEVEL_FAILED_LOGIN, EVENT_LEVEL_FAILED_LOGINS_10, EVENT_LEVEL_FAILED_LOGINS_15,
    EVENT_LEVEL_FAILED_LOGINS_20, EVENT_LEVEL_FAILED_LOGINS_25, EVENT_LEVEL_FAILED_LOGINS_7,
    EVENT_LEVEL_IP_BLACKLISTED, EVENT_LEVEL_JWKS_ROTATE, EVENT_LEVEL_NEW_RAUTHY_ADMIN,
    EVENT_LEVEL_NEW_RAUTHY_VERSION, EVENT_LEVEL_NEW_USER, EVENT_LEVEL_RAUTHY_HEALTHY,
    EVENT_LEVEL_RAUTHY_START, EVENT_LEVEL_RAUTHY_UNHEALTHY, EVENT_LEVEL_SECRETS_MIGRATED,
    EVENT_LEVEL_USER_EMAIL_CHANGE, EVENT_LEVEL_USER_PASSWORD_RESET,
};
use chrono::{DateTime, Timelike, Utc};
use rauthy_common::constants::EMAIL_SUB_PREFIX;
use rauthy_common::utils::{get_local_hostname, get_rand};
use rauthy_error::{ErrorResponse, ErrorResponseType};
use rauthy_notify::{Notification, NotificationLevel};
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use std::fmt::{Display, Formatter};
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

impl From<rauthy_api_types::EventLevel> for EventLevel {
    fn from(value: rauthy_api_types::EventLevel) -> Self {
        match value {
            rauthy_api_types::EventLevel::Info => Self::Info,
            rauthy_api_types::EventLevel::Notice => Self::Notice,
            rauthy_api_types::EventLevel::Warning => Self::Warning,
            rauthy_api_types::EventLevel::Critical => Self::Critical,
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
        write!(f, "{}", s)
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
    PossibleBruteForce, // TODO
    RauthyStarted,
    RauthyHealthy,
    RauthyUnhealthy,
    SecretsMigrated,
    UserEmailChange,
    UserPasswordReset,
    Test,
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
            EventType::InvalidLogins => write!(f, "Invalid logins"),
            EventType::IpBlacklisted => write!(f, "IP blacklisted"),
            EventType::IpBlacklistRemoved => write!(f, "IP blacklist removed"),
            EventType::JwksRotated => write!(f, "JWKS has been rotated"),
            EventType::NewUserRegistered => write!(f, "New user registered"),
            EventType::NewRauthyAdmin => write!(f, "New rauthy_admin member"),
            EventType::NewRauthyVersion => write!(f, "New Rauthy App Version available"),
            EventType::PossibleBruteForce => write!(f, "Possible brute force"),
            EventType::RauthyStarted => write!(f, "Rauthy has been restarted"),
            EventType::RauthyHealthy => write!(f, "Rauthy is healthy"),
            EventType::RauthyUnhealthy => write!(f, "Rauthy is unhealthy"),
            EventType::SecretsMigrated => write!(f, "Secrets have been migrated"),
            EventType::UserEmailChange => write!(f, "User's E-Mail has been changed"),
            EventType::UserPasswordReset => write!(f, "User has reset its password"),
            EventType::Test => write!(f, "TEST"),
        }
    }
}

impl From<rauthy_api_types::EventType> for EventType {
    fn from(value: rauthy_api_types::EventType) -> Self {
        match value {
            rauthy_api_types::EventType::InvalidLogins => Self::InvalidLogins,
            rauthy_api_types::EventType::IpBlacklisted => Self::IpBlacklisted,
            rauthy_api_types::EventType::IpBlacklistRemoved => Self::IpBlacklistRemoved,
            rauthy_api_types::EventType::JwksRotated => Self::JwksRotated,
            rauthy_api_types::EventType::NewUserRegistered => Self::NewUserRegistered,
            rauthy_api_types::EventType::NewRauthyAdmin => Self::NewRauthyAdmin,
            rauthy_api_types::EventType::NewRauthyVersion => Self::NewRauthyVersion,
            rauthy_api_types::EventType::PossibleBruteForce => Self::PossibleBruteForce,
            rauthy_api_types::EventType::RauthyStarted => Self::RauthyStarted,
            rauthy_api_types::EventType::RauthyHealthy => Self::RauthyHealthy,
            rauthy_api_types::EventType::RauthyUnhealthy => Self::RauthyUnhealthy,
            rauthy_api_types::EventType::SecretsMigrated => Self::SecretsMigrated,
            rauthy_api_types::EventType::UserEmailChange => Self::UserEmailChange,
            rauthy_api_types::EventType::UserPasswordReset => Self::UserPasswordReset,
            rauthy_api_types::EventType::Test => Self::Test,
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
        }
    }

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
            // just return test to never panic
            _ => Self::Test,
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

impl From<&Event> for Notification {
    fn from(value: &Event) -> Self {
        let icon = match value.level {
            EventLevel::Info => "ℹ️",
            EventLevel::Notice => "📢",
            EventLevel::Warning => "⚠️",
            EventLevel::Critical => "🆘",
        };
        let head = format!("{} {} - {}", icon, *EMAIL_SUB_PREFIX, value.level.as_str());

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
    pub async fn insert(&self, db: &DbPool) -> Result<(), ErrorResponse> {
        let level = self.level.value();
        let typ = self.typ.value();

        query!(
            r#"INSERT INTO events (id, timestamp, level, typ, ip, data, text)
            VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
            self.id,
            self.timestamp,
            level,
            typ,
            self.ip,
            self.data,
            self.text,
        )
        .execute(db)
        .await?;
        Ok(())
    }

    pub async fn find_all(
        db: &DbPool,
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
            query_as!(
                Self,
                r#"SELECT * FROM events
                WHERE timestamp >= $1 AND timestamp <= $2 AND level >= $3 AND typ = $4
                ORDER BY timestamp DESC"#,
                from,
                until,
                level,
                typ,
            )
            .fetch_all(db)
            .await
        } else {
            query_as!(
                Self,
                r#"SELECT * FROM events
                WHERE timestamp >= $1 AND timestamp <= $2 AND level >= $3
                ORDER BY timestamp DESC"#,
                from,
                until,
                level,
            )
            .fetch_all(db)
            .await
        }?;

        Ok(res)
    }

    pub async fn find_latest(db: &DbPool, limit: i64) -> Result<Vec<Self>, ErrorResponse> {
        let res = query_as!(
            Self,
            "SELECT * FROM events ORDER BY timestamp DESC LIMIT $1",
            limit
        )
        .fetch_all(db)
        .await?;
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
        let level = match failed_logins {
            l if l >= 25 => EVENT_LEVEL_FAILED_LOGINS_25.get().unwrap(),
            l if l >= 20 => EVENT_LEVEL_FAILED_LOGINS_20.get().unwrap(),
            l if l >= 15 => EVENT_LEVEL_FAILED_LOGINS_15.get().unwrap(),
            l if l >= 10 => EVENT_LEVEL_FAILED_LOGINS_10.get().unwrap(),
            l if l >= 7 => EVENT_LEVEL_FAILED_LOGINS_7.get().unwrap(),
            _ => EVENT_LEVEL_FAILED_LOGIN.get().unwrap(),
        };
        Self::new(
            level.clone(),
            EventType::InvalidLogins,
            Some(ip),
            Some(failed_logins as i64),
            None,
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

    pub fn ip_blacklisted(exp: DateTime<Utc>, ip: String) -> Self {
        Self::new(
            EVENT_LEVEL_IP_BLACKLISTED.get().cloned().unwrap(),
            EventType::IpBlacklisted,
            Some(ip),
            Some(exp.timestamp()),
            None,
        )
    }

    pub fn ip_blacklist_removed(ip: String) -> Self {
        Self::new(
            EVENT_LEVEL_IP_BLACKLISTED.get().cloned().unwrap(),
            EventType::IpBlacklistRemoved,
            Some(ip),
            None,
            None,
        )
    }

    pub fn new_user(email: String, ip: Option<String>) -> Self {
        Self::new(
            EVENT_LEVEL_NEW_USER.get().cloned().unwrap(),
            EventType::NewUserRegistered,
            ip,
            None,
            Some(email),
        )
    }

    pub fn new_rauthy_admin(email: String, ip: Option<String>) -> Self {
        Self::new(
            EVENT_LEVEL_NEW_RAUTHY_ADMIN.get().cloned().unwrap(),
            EventType::NewRauthyAdmin,
            ip,
            None,
            Some(email),
        )
    }

    pub fn new_rauthy_version(version_url: String) -> Self {
        Self::new(
            EVENT_LEVEL_NEW_RAUTHY_VERSION.get().cloned().unwrap(),
            EventType::NewRauthyVersion,
            None,
            None,
            Some(version_url),
        )
    }

    pub fn jwks_rotated() -> Self {
        Self::new(
            EVENT_LEVEL_JWKS_ROTATE.get().cloned().unwrap(),
            EventType::JwksRotated,
            None,
            None,
            None,
        )
    }

    pub fn rauthy_started() -> Self {
        let text = format!("Rauthy has been started on host {}", get_local_hostname());
        Self::new(
            EVENT_LEVEL_RAUTHY_START.get().cloned().unwrap(),
            EventType::RauthyStarted,
            None,
            None,
            Some(text),
        )
    }

    pub fn rauthy_healthy() -> Self {
        let text = format!("Rauthy is healthy now on host {}", get_local_hostname());
        Self::new(
            EVENT_LEVEL_RAUTHY_HEALTHY.get().cloned().unwrap(),
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
            EVENT_LEVEL_RAUTHY_UNHEALTHY.get().cloned().unwrap(),
            EventType::RauthyUnhealthy,
            None,
            None,
            Some(text),
        )
    }

    pub fn rauthy_unhealthy_db() -> Self {
        Self::new(
            EVENT_LEVEL_RAUTHY_UNHEALTHY.get().cloned().unwrap(),
            EventType::RauthyUnhealthy,
            None,
            None,
            Some("The database connection is unhealthy".to_string()),
        )
    }

    pub fn secrets_migrated(ip: Option<String>) -> Self {
        Self::new(
            EVENT_LEVEL_SECRETS_MIGRATED.get().cloned().unwrap(),
            EventType::SecretsMigrated,
            ip,
            None,
            None,
        )
    }

    pub fn test(ip: Option<String>) -> Self {
        Self::new(
            EventLevel::Info,
            EventType::Test,
            ip,
            None,
            Some("This is a Test-Event".to_string()),
        )
    }

    pub fn user_email_change(text: String, ip: Option<String>) -> Self {
        Self::new(
            EVENT_LEVEL_USER_EMAIL_CHANGE.get().cloned().unwrap(),
            EventType::UserEmailChange,
            ip,
            None,
            Some(text),
        )
    }

    pub fn user_password_reset(text: String, ip: Option<String>) -> Self {
        Self::new(
            EVENT_LEVEL_USER_PASSWORD_RESET.get().cloned().unwrap(),
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
            // PossibleBruteForce is not yet implemented / recognized
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
        }
    }

    #[inline(always)]
    pub async fn send(self, tx: &flume::Sender<Self>) -> Result<(), ErrorResponse> {
        match tx.send_async(self).await {
            Ok(_) => Ok(()),
            Err(err) => {
                error!("Event::send: {:?}", err);
                Err(ErrorResponse::new(
                    ErrorResponseType::Internal,
                    format!("Error sending event internally: {:?}", err),
                ))
            }
        }
    }
}
