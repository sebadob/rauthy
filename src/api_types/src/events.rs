use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum EventLevel {
    Info,
    Notice,
    Warning,
    Critical,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, ToSchema)]
pub enum EventType {
    InvalidLogins,
    IpBlacklisted,
    IpBlacklistRemoved,
    JwksRotated,
    NewUserRegistered,
    NewRauthyAdmin,
    NewRauthyVersion,
    PossibleBruteForce,
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
}

#[derive(Deserialize, Validate, ToSchema, IntoParams)]
pub struct EventsListenParams {
    /// Validation: `0 <= latest <= 1000`
    #[validate(range(min = 0, max = 1000))]
    pub latest: Option<u16>,
    pub level: Option<EventLevel>,
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct EventsRequest {
    /// Unix timestamp in seconds
    #[validate(range(min = 1719784800))]
    pub from: i64,
    /// Unix timestamp in seconds
    #[validate(range(min = 1719784800))]
    pub until: Option<i64>,
    pub level: EventLevel,
    pub typ: Option<EventType>,
}

#[derive(Serialize, ToSchema)]
pub struct EventResponse {
    pub id: String,
    pub timestamp: i64,
    pub level: EventLevel,
    pub typ: EventType,
    pub ip: Option<String>,
    pub data: Option<i64>,
    pub text: Option<String>,
}
