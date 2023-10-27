use crate::events::event::EventLevel;
use rauthy_common::error_response::ErrorResponse;
use std::env;
use std::sync::OnceLock;
use tracing::info;

pub mod event;
pub mod health_watch;
pub mod ip_blacklist_handler;
pub mod listener;
pub mod notifier;

pub static EVENT_PERSIST_LEVEL: OnceLock<i16> = OnceLock::new();
pub static EVENT_LEVEL_NEW_USER: OnceLock<EventLevel> = OnceLock::new();
pub static EVENT_LEVEL_USER_EMAIL_CHANGE: OnceLock<EventLevel> = OnceLock::new();
pub static EVENT_LEVEL_NEW_RAUTHY_ADMIN: OnceLock<EventLevel> = OnceLock::new();
pub static EVENT_LEVEL_NEW_RAUTHY_VERSION: OnceLock<EventLevel> = OnceLock::new();
pub static EVENT_LEVEL_JWKS_ROTATE: OnceLock<EventLevel> = OnceLock::new();
pub static EVENT_LEVEL_SECRETS_MIGRATED: OnceLock<EventLevel> = OnceLock::new();
pub static EVENT_LEVEL_RAUTHY_START: OnceLock<EventLevel> = OnceLock::new();
pub static EVENT_LEVEL_RAUTHY_HEALTHY: OnceLock<EventLevel> = OnceLock::new();
pub static EVENT_LEVEL_RAUTHY_UNHEALTHY: OnceLock<EventLevel> = OnceLock::new();
pub static EVENT_LEVEL_IP_BLACKLISTED: OnceLock<EventLevel> = OnceLock::new();
pub static EVENT_LEVEL_FAILED_LOGINS_25: OnceLock<EventLevel> = OnceLock::new();
pub static EVENT_LEVEL_FAILED_LOGINS_20: OnceLock<EventLevel> = OnceLock::new();
pub static EVENT_LEVEL_FAILED_LOGINS_15: OnceLock<EventLevel> = OnceLock::new();
pub static EVENT_LEVEL_FAILED_LOGINS_10: OnceLock<EventLevel> = OnceLock::new();
pub static EVENT_LEVEL_FAILED_LOGINS_7: OnceLock<EventLevel> = OnceLock::new();
pub static EVENT_LEVEL_FAILED_LOGIN: OnceLock<EventLevel> = OnceLock::new();

pub fn init_event_vars() -> Result<(), ErrorResponse> {
    let level = map_env_var_level("EVENT_PERSIST_LEVEL", EventLevel::Info);
    info!("Event Persistence Level: {:?}", level);
    EVENT_PERSIST_LEVEL.set(level.value()).unwrap();

    EVENT_LEVEL_NEW_USER
        .set(map_env_var_level("EVENT_LEVEL_NEW_USER", EventLevel::Info))
        .unwrap();
    EVENT_LEVEL_USER_EMAIL_CHANGE
        .set(map_env_var_level(
            "EVENT_LEVEL_USER_EMAIL_CHANGE",
            EventLevel::Notice,
        ))
        .unwrap();
    EVENT_LEVEL_NEW_RAUTHY_ADMIN
        .set(map_env_var_level(
            "EVENT_LEVEL_RAUTHY_ADMIN",
            EventLevel::Notice,
        ))
        .unwrap();
    EVENT_LEVEL_NEW_RAUTHY_VERSION
        .set(map_env_var_level(
            "EVENT_LEVEL_RAUTHY_VERSION",
            EventLevel::Notice,
        ))
        .unwrap();
    EVENT_LEVEL_JWKS_ROTATE
        .set(map_env_var_level(
            "EVENT_LEVEL_JWKS_ROTATE",
            EventLevel::Notice,
        ))
        .unwrap();
    EVENT_LEVEL_SECRETS_MIGRATED
        .set(map_env_var_level(
            "EVENT_LEVEL_SECRETS_MIGRATED",
            EventLevel::Notice,
        ))
        .unwrap();
    EVENT_LEVEL_RAUTHY_START
        .set(map_env_var_level(
            "EVENT_LEVEL_RAUTHY_START",
            EventLevel::Info,
        ))
        .unwrap();
    EVENT_LEVEL_RAUTHY_HEALTHY
        .set(map_env_var_level(
            "EVENT_LEVEL_RAUTHY_HEALTHY",
            EventLevel::Notice,
        ))
        .unwrap();
    EVENT_LEVEL_RAUTHY_UNHEALTHY
        .set(map_env_var_level(
            "EVENT_LEVEL_RAUTHY_UNHEALTHY",
            EventLevel::Critical,
        ))
        .unwrap();
    EVENT_LEVEL_IP_BLACKLISTED
        .set(map_env_var_level(
            "EVENT_LEVEL_IP_BLACKLISTED",
            EventLevel::Warning,
        ))
        .unwrap();
    EVENT_LEVEL_FAILED_LOGINS_25
        .set(map_env_var_level(
            "EVENT_LEVEL_FAILED_LOGINS_25",
            EventLevel::Critical,
        ))
        .unwrap();
    EVENT_LEVEL_FAILED_LOGINS_20
        .set(map_env_var_level(
            "EVENT_LEVEL_FAILED_LOGINS_20",
            EventLevel::Critical,
        ))
        .unwrap();
    EVENT_LEVEL_FAILED_LOGINS_15
        .set(map_env_var_level(
            "EVENT_LEVEL_FAILED_LOGINS_15",
            EventLevel::Warning,
        ))
        .unwrap();
    EVENT_LEVEL_FAILED_LOGINS_10
        .set(map_env_var_level(
            "EVENT_LEVEL_FAILED_LOGINS_10",
            EventLevel::Warning,
        ))
        .unwrap();
    EVENT_LEVEL_FAILED_LOGINS_7
        .set(map_env_var_level(
            "EVENT_LEVEL_FAILED_LOGINS_7",
            EventLevel::Notice,
        ))
        .unwrap();
    EVENT_LEVEL_FAILED_LOGIN
        .set(map_env_var_level(
            "EVENT_LEVEL_FAILED_LOGIN",
            EventLevel::Info,
        ))
        .unwrap();

    Ok(())
}

fn map_env_var_level(var: &str, default: EventLevel) -> EventLevel {
    let err_msg = format!(
        "Cannot parse {}. Possible values: info, notice, warning, critical",
        var
    );
    env::var(var)
        .map(|level| level.parse::<EventLevel>().expect(&err_msg))
        .unwrap_or(default)
}
