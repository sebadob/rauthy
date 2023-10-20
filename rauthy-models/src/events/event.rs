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
    InvalidLogins,
    IpBlacklisted,
    NewRauthyAdmin,
    PossibleBruteForce,
    PossibleDoS,
    Test,
}

impl Display for EventType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            EventType::InvalidLogins => write!(f, "invalid logins"),
            EventType::IpBlacklisted => write!(f, "IP blacklisted"),
            EventType::NewRauthyAdmin => write!(f, "new rauthy_admin member"),
            EventType::PossibleBruteForce => write!(f, "possible brute force"),
            EventType::PossibleDoS => write!(f, "possible DoS"),
            EventType::Test => write!(f, "TEST"),
        }
    }
}

impl EventType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::InvalidLogins => "InvalidLogins",
            Self::IpBlacklisted => "IpBlacklisted",
            Self::NewRauthyAdmin => "NewRauthyAdmin",
            Self::PossibleBruteForce => "PossibleBruteForce",
            Self::PossibleDoS => "PossibleDoS",
            Self::Test => "TEST",
        }
    }

    pub fn value(&self) -> i16 {
        match self {
            Self::Test => 0,
            Self::InvalidLogins => 1,
            Self::NewRauthyAdmin => 2,
            Self::IpBlacklisted => 3,
            Self::PossibleBruteForce => 4,
            Self::PossibleDoS => 5,
        }
    }
}

impl From<String> for EventType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "InvalidLogins" => Self::InvalidLogins,
            "IpBlacklisted" => Self::IpBlacklisted,
            "NewRauthyAdmin" => Self::NewRauthyAdmin,
            "PossibleBruteForce" => Self::PossibleBruteForce,
            "PossibleDoS" => Self::PossibleDoS,
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
            0 => Self::Test,
            1 => Self::InvalidLogins,
            2 => Self::NewRauthyAdmin,
            3 => Self::IpBlacklisted,
            4 => Self::PossibleBruteForce,
            5 => Self::PossibleDoS,
            // just return test to never panic
            _ => Self::Test,
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
    pub fn invalid_login(count: u32, ip: String) -> Self {
        let level = if count > 6 {
            EventLevel::Warning
        } else if count > 3 {
            EventLevel::Notice
        } else {
            EventLevel::Info
        };
        Self::new(
            level,
            EventType::InvalidLogins,
            Some(ip),
            Some(count as i64),
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

    pub fn ip_blacklisted(ip: String) -> Self {
        Self::new(
            EventLevel::Warning,
            EventType::IpBlacklisted,
            Some(ip),
            None,
            None,
        )
    }

    pub fn rauthy_admin(email: String) -> Self {
        Self::new(
            EventLevel::Notice,
            EventType::NewRauthyAdmin,
            None,
            None,
            Some(email),
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

    pub fn dos(ip: String) -> Self {
        Self::new(
            EventLevel::Critical,
            EventType::PossibleDoS,
            Some(ip),
            None,
            None,
        )
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
