use crate::DbType;
use crate::utils::build_trusted_proxies;
use actix_web::http::Uri;
use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::env;
use std::ops::Not;
use std::string::ToString;
use std::sync::{LazyLock, OnceLock};

#[derive(Debug, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CookieMode {
    Host,
    Secure,
    DangerInsecure,
}

impl From<&str> for CookieMode {
    fn from(s: &str) -> Self {
        match s {
            "host" => Self::Host,
            "secure" => Self::Secure,
            "danger-insecure" => Self::DangerInsecure,
            v => {
                panic!("Cannot parse {} as CookieMode", v);
            }
        }
    }
}

impl CookieMode {
    pub fn as_str(&self) -> &str {
        match self {
            CookieMode::Host => "host",
            CookieMode::Secure => "secure",
            CookieMode::DangerInsecure => "danger-insecure",
        }
    }
}

impl Default for CookieMode {
    fn default() -> Self {
        Self::Host
    }
}

pub static BUILD_TIME: LazyLock<DateTime<Utc>> = LazyLock::new(|| {
    DateTime::from_timestamp(env!("BUILD_TIME").parse::<i64>().unwrap(), 0).unwrap()
});

pub const RAUTHY_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CONTENT_TYPE_WEBP: &str = "image/webp";
pub const HEADER_DPOP_NONCE: &str = "DPoP-Nonce";
pub const HEADER_ALLOW_ALL_ORIGINS: (&str, &str) = ("access-control-allow-origin", "*");
pub const HEADER_HTML: (&str, &str) = ("content-type", "text/html;charset=utf-8");
pub const HEADER_JSON: (&str, &str) = ("content-type", "application/json");
pub const HEADER_RETRY_NOT_BEFORE: &str = "x-retry-not-before";
pub const APPLICATION_JSON: &str = "application/json";
pub const APPLICATION_JSON_SCIM: &str = "application/scim+json";
pub const TEXT_TURTLE: &str = "text/turtle";

pub const TOKEN_API_KEY: &str = "API-Key";
pub const TOKEN_BEARER: &str = "Bearer";
pub const TOKEN_DPOP: &str = "DPoP";
pub const TOKEN_DPOP_NONCE: &str = "DPoP-nonce";
pub const COOKIE_SESSION: &str = "RauthySession";
pub const COOKIE_SESSION_FED_CM: &str = "RauthySessionFedCM";
pub const COOKIE_MFA: &str = "RauthyMfa";
pub const COOKIE_LOCALE: &str = "locale";
pub const COOKIE_UPSTREAM_CALLBACK: &str = "UpstreamAuthCallback";
pub const PROVIDER_LINK_COOKIE: &str = "rauthy-provider-link";
pub const PWD_RESET_COOKIE: &str = "rauthy-pwd-reset";
pub const APP_ID_HEADER: &str = "mfa-app-id";
pub const CSRF_HEADER: &str = "x-csrf-token";
pub const PWD_CSRF_HEADER: &str = "x-pwd-csrf-token";

pub const ARGON2ID_M_COST_MIN: u32 = 32768;
pub const ARGON2ID_T_COST_MIN: u32 = 1;
pub const API_KEY_LENGTH: usize = 64;
pub const DEVICE_KEY_LENGTH: u8 = 64;
pub const EVENTS_LATEST_LIMIT: u16 = 100;
pub const GRANT_TYPE_DEVICE_CODE: &str = "urn:ietf:params:oauth:grant-type:device_code";
pub const UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS: u16 = 300;

pub const CACHE_TTL_APP: Option<i64> = Some(43200);
pub const CACHE_TTL_AUTH_PROVIDER_CALLBACK: Option<i64> =
    Some(UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS as i64);
pub const CACHE_TTL_SESSION: Option<i64> = Some(14400);
// TODO maybe add a size limit to hiqlite to fix users cache, or simply always append?
// No size limit means the cache could grow infinitely in theory, at least until every user from
// the DB is inside the cache. However, this will not exceed a too high value even for 1_000_000
// users.
//
// CAUTION: With WebID enabled there can actually exist 4 entries per user with different indexes
// in the current layout!
pub const CACHE_TTL_USER: Option<i64> = Some(600);

pub const IDX_APP_VERSION: &str = "rauthy_app_version";
pub const IDX_AUTH_PROVIDER: &str = "auth_provider_";
pub const IDX_AUTH_PROVIDER_LOGO: &str = "auth_provider_logo_";
pub const IDX_AUTH_PROVIDER_TEMPLATE: &str = "provider_json_tpl";
pub const IDX_CLIENTS: &str = "clients_";
pub const IDX_CLIENT_LOGO: &str = "client_logo_";
pub const IDX_GROUPS: &str = "groups_";
pub const IDX_JWK_KID: &str = "jwk_kid_";
pub const IDX_JWK_LATEST: &str = "jwk_latest_";
pub const IDX_JWKS: &str = "jkws_";
pub const IDX_LOGIN_TIME: &str = "login_time_";
pub const IDX_MFA_APP: &str = "mfa_app_";
pub const IDX_MFA_LOGIN_REQ: &str = "mfa_login_req_";
pub const IDX_PASSWORD_RULES: &str = "password_rules_";
pub const IDX_ROLES: &str = "roles_";
pub const IDX_SCOPES: &str = "scopes_";
pub const IDX_SESSIONS: &str = "sessions";
pub const IDX_USERS: &str = "users_";
pub const IDX_USER_COUNT: &str = "users_count_total";
pub const IDX_USERS_VALUES: &str = "users_values_";
pub const IDX_USER_ATTR_CONFIG: &str = "user_attrs_";
pub const IDX_WEBAUTHN: &str = "webauthn_";

pub static APP_START: LazyLock<DateTime<Utc>> = LazyLock::new(Utc::now);
pub static RAUTHY_ADMIN_ROLE: &str = "rauthy_admin";

// Cache TTLS
pub static CACHE_TTL_AUTH_CODE: LazyLock<Option<i64>> =
    LazyLock::new(|| Some(300 + *WEBAUTHN_REQ_EXP as i64));
pub static CACHE_TTL_WEBAUTHN: LazyLock<Option<i64>> =
    LazyLock::new(|| Some(*WEBAUTHN_REQ_EXP as i64));
pub static CACHE_TTL_WEBAUTHN_DATA: LazyLock<Option<i64>> =
    LazyLock::new(|| Some(*WEBAUTHN_DATA_EXP as i64));

pub static DB_TYPE: LazyLock<DbType> = LazyLock::new(DbType::build);

pub static DEV_MODE: LazyLock<bool> = LazyLock::new(|| {
    env::var("DEV_MODE")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("DEV_MODE cannot be parsed as bool")
});

pub static DPOP_TOKEN_ENDPOINT: OnceLock<Uri> = OnceLock::new();
pub static PEER_IP_HEADER_NAME: OnceLock<Option<String>> = OnceLock::new();
pub static PROXY_MODE: OnceLock<bool> = OnceLock::new();

pub static SUSPICIOUS_REQUESTS_BLACKLIST: LazyLock<u16> = LazyLock::new(|| {
    env::var("SUSPICIOUS_REQUESTS_BLACKLIST")
        .as_deref()
        .unwrap_or("1440")
        .parse::<u16>()
        .expect("SUSPICIOUS_REQUESTS_BLACKLIST cannot be parsed as u16")
});
pub static SUSPICIOUS_REQUESTS_LOG: LazyLock<bool> = LazyLock::new(|| {
    env::var("SUSPICIOUS_REQUESTS_LOG")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("SUSPICIOUS_REQUESTS_LOG cannot be parsed as bool")
});
pub static SSE_KEEP_ALIVE: LazyLock<u16> = LazyLock::new(|| {
    env::var("SSE_KEEP_ALIVE")
        .as_deref()
        .unwrap_or("30")
        .parse::<u16>()
        .expect("SSE_KEEP_ALIVE cannot be parsed as u16")
});
pub static SSP_THRESHOLD: LazyLock<u16> = LazyLock::new(|| {
    env::var("SSP_THRESHOLD")
        .as_deref()
        .unwrap_or("1000")
        .parse::<u16>()
        .expect("SSP_THRESHOLD cannot be parsed as u16")
});

pub static TRUSTED_PROXIES: LazyLock<Vec<cidr::IpCidr>> = LazyLock::new(build_trusted_proxies);
pub static USER_REG_DOMAIN_RESTRICTION: LazyLock<Option<String>> =
    LazyLock::new(|| env::var("USER_REG_DOMAIN_RESTRICTION").ok());
pub static USER_REG_DOMAIN_BLACKLIST: LazyLock<Option<Vec<String>>> = LazyLock::new(|| {
    env::var("USER_REG_DOMAIN_BLACKLIST").ok().map(|blacklist| {
        blacklist
            .lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                trimmed.is_empty().not().then_some(trimmed.to_string())
            })
            .collect()
    })
});
pub static USER_REG_OPEN_REDIRECT: LazyLock<bool> = LazyLock::new(|| {
    env::var("USER_REG_DOMAIN_BLACKLIST")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("Cannot parse USER_REG_OPEN_REDIRECT to bool")
});

pub static WEBAUTHN_REQ_EXP: LazyLock<u32> = LazyLock::new(|| {
    env::var("WEBAUTHN_REQ_EXP")
        .as_deref()
        .unwrap_or("60")
        .parse::<u32>()
        .expect("WEBAUTHN_REQ_EXP cannot be parsed as u32")
});
pub static WEBAUTHN_DATA_EXP: LazyLock<u32> = LazyLock::new(|| {
    env::var("WEBAUTHN_DATA_EXP")
        .as_deref()
        .unwrap_or("90")
        .parse::<u32>()
        .expect("WEBAUTHN_DATA_EXP cannot be parsed as u32")
});
pub static WEBAUTHN_RENEW_EXP: LazyLock<i64> = LazyLock::new(|| {
    env::var("WEBAUTHN_RENEW_EXP")
        .as_deref()
        .unwrap_or("2160")
        .parse::<i64>()
        .expect("WEBAUTHN_RENEW_EXP cannot be parsed as u32")
});
pub static WEBAUTHN_FORCE_UV: LazyLock<bool> = LazyLock::new(|| {
    env::var("WEBAUTHN_FORCE_UV")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("WEBAUTHN_FORCE_UV cannot be parsed as bool")
});
pub static WEBAUTHN_NO_PASSWORD_EXPIRY: LazyLock<bool> = LazyLock::new(|| {
    env::var("WEBAUTHN_NO_PASSWORD_EXPIRY")
        .as_deref()
        .unwrap_or("true")
        .parse::<bool>()
        .expect("WEBAUTHN_NO_PASSWORD_EXPIRY cannot be parsed as bool")
});
