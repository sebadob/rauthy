use chrono::{DateTime, Timelike, Utc};
use rauthy_common::error_response::{ErrorResponse, ErrorResponseType};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use tracing::error;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl EventLevel {
    pub fn as_str(&self) -> &str {
        match self {
            EventLevel::Info => "INFO",
            EventLevel::Notice => "NOTICE",
            EventLevel::Warning => "WARNING",
            EventLevel::Critical => "CRITICAL",
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

impl ToString for EventType {
    fn to_string(&self) -> String {
        match self {
            EventType::InvalidLogins(count) => format!("invalid logins: {}", count),
            EventType::IpBlacklisted => "IP blacklisted".to_string(),
            EventType::NewRauthyAdmin(email) => format!("new rauthy_admin member: {}", email),
            EventType::PossibleBruteForce => "possible brute force".to_string(),
            EventType::PossibleDoS => "possible DoS".to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub level: EventLevel,
    pub typ: EventType,
    pub ip: Option<String>,
}

impl Display for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "EVENT {} {}:{}:{} {} {} {}",
            self.timestamp.date_naive(),
            self.timestamp.hour(),
            self.timestamp.minute(),
            self.timestamp.second(),
            self.level,
            self.typ.to_string(),
            self.ip.as_deref().unwrap_or_default(),
        )
    }
}

impl Event {
    pub fn new(level: EventLevel, typ: EventType, ip: Option<String>) -> Self {
        Self {
            id: Uuid::now_v7(),
            timestamp: Utc::now(),
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
