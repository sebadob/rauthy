use crate::app_state::DbPool;
use chrono::{NaiveDateTime, Timelike, Utc};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use rauthy_common::utils::get_rand;
use serde::{Deserialize, Serialize};
use sqlx::{query, query_as};
use std::fmt::{Display, Formatter};
use tracing::error;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EventLevel {
    Info,
    Notice,
    Warning,
    Critical,
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
    /// number of invalid logins
    InvalidLogins(u32),
    IpBlacklisted,
    /// E-Mail of the new admin role member
    NewRauthyAdmin(String),
    PossibleBruteForce,
    PossibleDoS,
}

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::InvalidLogins(count) => write!(f, "invalid logins: {}", count),
            EventType::IpBlacklisted => write!(f, "IP blacklisted"),
            EventType::NewRauthyAdmin(email) => write!(f, "new rauthy_admin member: {}", email),
            EventType::PossibleBruteForce => write!(f, "possible brute force"),
            EventType::PossibleDoS => write!(f, "possible DoS"),
        }
    }
}

impl EventType {
    fn db_value(&self) -> String {
        match self {
            EventType::InvalidLogins(count) => format!("InvalidLogins${}", count),
            EventType::IpBlacklisted => "IpBlacklisted".to_string(),
            EventType::NewRauthyAdmin(email) => format!("NewRauthyAdmin${}", email),
            EventType::PossibleBruteForce => "PossibleBruteForce".to_string(),
            EventType::PossibleDoS => "PossibleDoS".to_string(),
        }
    }
}

impl From<String> for EventType {
    fn from(value: String) -> Self {
        let (typ, value) = value.split_once('$').unwrap_or((value.as_str(), ""));
        match typ {
            "InvalidLogins" => Self::InvalidLogins(value.parse::<u32>().unwrap_or_default()),
            "IpBlacklisted" => Self::IpBlacklisted,
            "NewRauthyAdmin" => Self::NewRauthyAdmin(value.to_string()),
            "PossibleBruteForce" => Self::PossibleBruteForce,
            "PossibleDoS" => Self::PossibleDoS,
            _ => unreachable!(),
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
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let ts = NaiveDateTime::from_timestamp_millis(self.timestamp).unwrap_or_default();
        write!(
            f,
            "EVENT {} {}:{}:{} {} {} {}",
            ts.date(),
            ts.hour(),
            ts.minute(),
            ts.second(),
            self.level,
            self.typ,
            self.ip.as_deref().unwrap_or_default(),
        )
    }
}

impl Event {
    pub async fn insert(&self, db: &DbPool) -> Result<(), ErrorResponse> {
        let level = self.level.value();
        let typ = self.typ.db_value();

        query!(
            "INSERT INTO events (id, timestamp, level, typ, ip) VALUES ($1, $2, $3, $4, $5)",
            self.id,
            self.timestamp,
            level,
            typ,
            self.ip,
        )
        .execute(db)
        .await?;
        Ok(())
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
    pub fn new(level: EventLevel, typ: EventType, ip: Option<String>) -> Self {
        // These short random strings are enough "id" because the PK in the DB is 'id + timestamp_millis'
        let id = get_rand(8);

        Self {
            id,
            timestamp: Utc::now().timestamp_millis(),
            level,
            typ,
            ip,
        }
    }

    /// The EventLevel will change depending on the amount of invalid logins
    pub fn invalid_login(count: u32, ip: String) -> Self {
        let level = if count > 6 {
            EventLevel::Warning
        } else if count > 3 {
            EventLevel::Notice
        } else {
            EventLevel::Info
        };
        Self::new(level, EventType::InvalidLogins(count), Some(ip))
    }

    pub fn ip_blacklisted(ip: String) -> Self {
        Self::new(EventLevel::Warning, EventType::IpBlacklisted, Some(ip))
    }

    pub fn rauthy_admin(email: String) -> Self {
        Self::new(EventLevel::Notice, EventType::NewRauthyAdmin(email), None)
    }

    pub fn brute_force(ip: String) -> Self {
        Self::new(
            EventLevel::Critical,
            EventType::PossibleBruteForce,
            Some(ip),
        )
    }

    pub fn dos(ip: String) -> Self {
        Self::new(EventLevel::Critical, EventType::PossibleDoS, Some(ip))
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
