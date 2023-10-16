use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Event {
    pub id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub level: EventLevel,
    pub typ: EventType,
    pub message: String,
    pub ip: Option<String>,
    // we probably won't need this additional data field
    // pub data: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EventLevel {
    Info,
    Notice,
    Warning,
    Critical,
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
