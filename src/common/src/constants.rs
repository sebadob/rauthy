use crate::DbType;
use crate::regex::RE_GRANT_TYPES_EPHEMERAL;
use crate::utils::build_trusted_proxies;
use actix_web::http::Uri;
use chrono::{DateTime, Utc};
use std::env;
use std::ops::Not;
use std::str::FromStr;
use std::string::ToString;
use std::sync::LazyLock;

#[derive(Debug, PartialEq)]
pub enum CookieMode {
    Host,
    Secure,
    DangerInsecure,
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
pub static CACHE_TTL_DEVICE_CODE: LazyLock<Option<i64>> =
    LazyLock::new(|| Some(*DEVICE_GRANT_CODE_LIFETIME as i64));
pub static CACHE_TTL_DYN_CLIENT: LazyLock<Option<i64>> =
    LazyLock::new(|| Some(*DYN_CLIENT_RATE_LIMIT_SEC as i64));
pub static CACHE_TTL_EPHEMERAL_CLIENT: LazyLock<Option<i64>> = LazyLock::new(|| {
    Some(
        env::var("EPHEMERAL_CLIENTS_CACHE_LIFETIME")
            .as_deref()
            .unwrap_or("3600")
            .parse::<i64>()
            .expect("EPHEMERAL_CLIENTS_CACHE_LIFETIME cannot be parsed to i64 - bad format"),
    )
});
pub static CACHE_TTL_WEBAUTHN: LazyLock<Option<i64>> =
    LazyLock::new(|| Some(*WEBAUTHN_REQ_EXP as i64));
pub static CACHE_TTL_WEBAUTHN_DATA: LazyLock<Option<i64>> =
    LazyLock::new(|| Some(*WEBAUTHN_DATA_EXP as i64));

pub static ADDITIONAL_ALLOWED_ORIGIN_SCHEMES: LazyLock<Vec<String>> = LazyLock::new(|| {
    env::var("ADDITIONAL_ALLOWED_ORIGIN_SCHEMES")
        .as_deref()
        .unwrap_or("")
        .split(' ')
        .filter_map(|scheme| scheme.is_empty().not().then_some(scheme.to_string()))
        .collect()
});
pub static ADMIN_FORCE_MFA: LazyLock<bool> = LazyLock::new(|| {
    env::var("ADMIN_FORCE_MFA")
        .as_deref()
        .unwrap_or("true")
        .parse::<bool>()
        .expect("ADMIN_FORCE_MFA cannot be parsed as bool")
});

pub static AUTH_HEADERS_ENABLE: LazyLock<bool> = LazyLock::new(|| {
    env::var("AUTH_HEADERS_ENABLE")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("AUTH_HEADERS_ENABLE cannot be parsed as bool")
});
pub static AUTH_HEADER_USER: LazyLock<String> = LazyLock::new(|| {
    env::var("AUTH_HEADER_USER").unwrap_or_else(|_| String::from("x-forwarded-user"))
});
pub static AUTH_HEADER_ROLES: LazyLock<String> = LazyLock::new(|| {
    env::var("AUTH_HEADER_ROLES").unwrap_or_else(|_| String::from("x-forwarded-user-roles"))
});
pub static AUTH_HEADER_GROUPS: LazyLock<String> = LazyLock::new(|| {
    env::var("AUTH_HEADER_GROUPS").unwrap_or_else(|_| String::from("x-forwarded-user-groups"))
});
pub static AUTH_HEADER_EMAIL: LazyLock<String> = LazyLock::new(|| {
    env::var("AUTH_HEADER_EMAIL").unwrap_or_else(|_| String::from("x-forwarded-user-email"))
});
pub static AUTH_HEADER_EMAIL_VERIFIED: LazyLock<String> = LazyLock::new(|| {
    env::var("AUTH_HEADER_EMAIL_VERIFIED")
        .unwrap_or_else(|_| String::from("x-forwarded-user-email-verified"))
});
pub static AUTH_HEADER_FAMILY_NAME: LazyLock<String> = LazyLock::new(|| {
    env::var("AUTH_HEADER_FAMILY_NAME")
        .unwrap_or_else(|_| String::from("x-forwarded-user-family-name"))
});
pub static AUTH_HEADER_GIVEN_NAME: LazyLock<String> = LazyLock::new(|| {
    env::var("AUTH_HEADER_GIVEN_NAME")
        .unwrap_or_else(|_| String::from("x-forwarded-user-given-name"))
});
pub static AUTH_HEADER_MFA: LazyLock<String> = LazyLock::new(|| {
    env::var("AUTH_HEADER_MFA").unwrap_or_else(|_| String::from("x-forwarded-user-mfa"))
});

pub static COOKIE_MODE: LazyLock<CookieMode> =
    LazyLock::new(
        || match env::var("COOKIE_MODE").as_deref().unwrap_or("host") {
            "host" => CookieMode::Host,
            "secure" => CookieMode::Secure,
            "danger-insecure" => CookieMode::DangerInsecure,
            _ => panic!("COOKIE_MODE must be one of: host, secure, danger-insecure"),
        },
    );
pub static COOKIE_SET_PATH: LazyLock<bool> = LazyLock::new(|| {
    env::var("COOKIE_SET_PATH")
        .as_deref()
        .unwrap_or("true")
        .parse::<bool>()
        .expect("COOKIE_SET_PATH cannot be parsed as bool")
});
pub static DANGER_DISABLE_INTROSPECT_AUTH: LazyLock<bool> = LazyLock::new(|| {
    env::var("DANGER_DISABLE_INTROSPECT_AUTH")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("DANGER_DISABLE_INTROSPECT_AUTH cannot be parsed as bool")
});
pub static DB_TYPE: LazyLock<DbType> = LazyLock::new(DbType::build);

pub static DEV_MODE: LazyLock<bool> = LazyLock::new(|| {
    env::var("DEV_MODE")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("DEV_MODE cannot be parsed as bool")
});
pub static DEV_DPOP_HTTP: LazyLock<bool> = LazyLock::new(|| {
    env::var("DEV_DPOP_HTTP")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("DEV_DPOP_HTTP cannot be parsed as bool")
});

pub static DEVICE_GRANT_CODE_LIFETIME: LazyLock<u16> = LazyLock::new(|| {
    env::var("DEVICE_GRANT_CODE_LIFETIME")
        .as_deref()
        .unwrap_or("300")
        .parse::<u16>()
        .expect("DEVICE_GRANT_CODE_LIFETIME cannot be parsed as u16")
});
pub static DEVICE_GRANT_USER_CODE_LENGTH: LazyLock<u8> = LazyLock::new(|| {
    env::var("DEVICE_GRANT_USER_CODE_LENGTH")
        .as_deref()
        .unwrap_or("8")
        .parse::<u8>()
        .expect("DEVICE_GRANT_USER_CODE_LENGTH cannot be parsed as u8")
});
pub static DEVICE_GRANT_RATE_LIMIT: LazyLock<Option<u32>> = LazyLock::new(|| {
    env::var("DEVICE_GRANT_RATE_LIMIT")
        .map(|rl| {
            rl.parse::<u32>()
                .expect("DEVICE_GRANT_RATE_LIMIT cannot be parsed as u32")
        })
        .ok()
});
pub static DEVICE_GRANT_POLL_INTERVAL: LazyLock<u8> = LazyLock::new(|| {
    env::var("DEVICE_GRANT_POLL_INTERVAL")
        .as_deref()
        .unwrap_or("5")
        .parse::<u8>()
        .expect("DEVICE_GRANT_POLL_INTERVAL cannot be parsed as u8")
});
pub static DEVICE_GRANT_REFRESH_TOKEN_LIFETIME: LazyLock<u16> = LazyLock::new(|| {
    env::var("DEVICE_GRANT_REFRESH_TOKEN_LIFETIME")
        .as_deref()
        .unwrap_or("72")
        .parse::<u16>()
        .expect("DEVICE_GRANT_REFRESH_TOKEN_LIFETIME cannot be parsed as u16")
});

pub static DISABLE_REFRESH_TOKEN_NBF: LazyLock<bool> = LazyLock::new(|| {
    env::var("DISABLE_REFRESH_TOKEN_NBF")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("DISABLE_REFRESH_TOKEN_NBF cannot be parsed as bool")
});

pub static DPOP_FORCE_NONCE: LazyLock<bool> = LazyLock::new(|| {
    env::var("DPOP_NONCE_FORCE")
        .as_deref()
        .unwrap_or("true")
        .parse::<bool>()
        .expect("Cannot parse DPOP_FORCE_NONCE to bool")
});
pub static DPOP_NONCE_EXP: LazyLock<u32> = LazyLock::new(|| {
    env::var("DPOP_NONCE_EXP")
        .as_deref()
        .unwrap_or("900")
        .parse::<u32>()
        .expect("DPOP_NONCE_EXP cannot be parsed as u32")
});
pub static DPOP_TOKEN_ENDPOINT: LazyLock<Uri> = LazyLock::new(|| {
    let scheme = if *DEV_MODE && *DEV_DPOP_HTTP {
        "http"
    } else {
        "https"
    };
    let uri = format!("{}://{}/auth/v1/oidc/token", scheme, *PUB_URL);
    Uri::from_str(&uri).unwrap()
});

pub static DYN_CLIENT_REG_TOKEN: LazyLock<Option<String>> =
    LazyLock::new(|| env::var("DYN_CLIENT_REG_TOKEN").ok());
pub static DYN_CLIENT_DEFAULT_TOKEN_LIFETIME: LazyLock<i32> = LazyLock::new(|| {
    env::var("DYN_CLIENT_DEFAULT_TOKEN_LIFETIME")
        .as_deref()
        .unwrap_or("1800")
        .parse::<u16>()
        .expect("DYN_CLIENT_DEFAULT_TOKEN_LIFETIME cannot be parsed as u16") as i32
});
pub static DYN_CLIENT_SECRET_AUTO_ROTATE: LazyLock<bool> = LazyLock::new(|| {
    env::var("DYN_CLIENT_SECRET_AUTO_ROTATE")
        .as_deref()
        .unwrap_or("true")
        .parse::<bool>()
        .expect("DYN_CLIENT_SECRET_AUTO_ROTATE cannot be parsed as bool")
});
pub static DYN_CLIENT_CLEANUP_INTERVAL: LazyLock<u64> = LazyLock::new(|| {
    env::var("DYN_CLIENT_CLEANUP_INTERVAL")
        .as_deref()
        .unwrap_or("60")
        .parse::<u64>()
        .expect("DYN_CLIENT_CLEANUP_INTERVAL cannot be parsed as u64")
});
pub static DYN_CLIENT_CLEANUP_MINUTES: LazyLock<u16> = LazyLock::new(|| {
    env::var("DYN_CLIENT_CLEANUP_MINUTES")
        .as_deref()
        .unwrap_or("60")
        .parse::<u16>()
        .expect("DYN_CLIENT_CLEANUP_MINUTES cannot be parsed as u16")
});
pub static DYN_CLIENT_RATE_LIMIT_SEC: LazyLock<u16> = LazyLock::new(|| {
    env::var("DYN_CLIENT_RATE_LIMIT_SEC")
        .as_deref()
        .unwrap_or("60")
        .parse::<u16>()
        .expect("DYN_CLIENT_RATE_LIMIT_SEC cannot be parsed as u16")
});
pub static EMAIL_SUB_PREFIX: LazyLock<String> =
    LazyLock::new(|| env::var("EMAIL_SUB_PREFIX").unwrap_or_else(|_| String::from("Rauthy IAM")));

pub static ENABLE_DYN_CLIENT_REG: LazyLock<bool> = LazyLock::new(|| {
    env::var("ENABLE_DYN_CLIENT_REG")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("ENABLE_DYN_CLIENT_REG cannot be parsed as bool")
});
pub static ENABLE_EPHEMERAL_CLIENTS: LazyLock<bool> = LazyLock::new(|| {
    env::var("ENABLE_EPHEMERAL_CLIENTS")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("ENABLE_EPHEMERAL_CLIENTS cannot be parsed as bool")
});
pub static ENABLE_SOLID_AUD: LazyLock<bool> = LazyLock::new(|| {
    env::var("ENABLE_SOLID_AUD")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("ENABLE_SOLID_AUD cannot be parsed as bool")
});
pub static ENABLE_WEB_ID: LazyLock<bool> = LazyLock::new(|| {
    env::var("ENABLE_WEB_ID")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("ENABLE_WEB_ID cannot be parsed as bool")
});

pub static EPHEMERAL_CLIENTS_FORCE_MFA: LazyLock<bool> = LazyLock::new(|| {
    env::var("EPHEMERAL_CLIENTS_FORCE_MFA")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("EPHEMERAL_CLIENTS_FORCE_MFA cannot be parsed as bool")
});
pub static EPHEMERAL_CLIENTS_ALLOWED_FLOWS: LazyLock<String> = LazyLock::new(|| {
    env::var("EPHEMERAL_CLIENTS_ALLOWED_FLOWS")
        .as_deref()
        .unwrap_or("authorization_code")
        .split(' ')
        .map(|flow| {
            let flow = flow.trim();
            if !RE_GRANT_TYPES_EPHEMERAL.is_match(flow) {
                panic!("unknown EPHEMERAL_CLIENTS_ALLOWED_FLOWS: {}", flow)
            }
            flow.to_string()
        })
        .collect::<Vec<String>>()
        .join(",")
});
pub static EPHEMERAL_CLIENTS_ALLOWED_SCOPES: LazyLock<String> = LazyLock::new(|| {
    env::var("EPHEMERAL_CLIENTS_ALLOWED_SCOPES")
        .as_deref()
        .unwrap_or("openid profile email webid")
        .split(' ')
        .filter(|scope| !scope.is_empty())
        .map(|scope| scope.to_string())
        .collect::<Vec<String>>()
        .join(",")
});

pub static EXPERIMENTAL_FED_CM_ENABLE: LazyLock<bool> = LazyLock::new(|| {
    env::var("EXPERIMENTAL_FED_CM_ENABLE")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("EXPERIMENTAL_FED_CM_ENABLE cannot be parsed as bool")
});
pub static HEALTH_CHECK_DELAY_SECS: LazyLock<u16> = LazyLock::new(|| {
    env::var("HEALTH_CHECK_DELAY_SECS")
        .as_deref()
        .unwrap_or("30")
        .parse::<u16>()
        .expect("HEALTH_CHECK_DELAY_SECS cannot be parsed as u16")
});
pub static OPEN_USER_REG: LazyLock<bool> = LazyLock::new(|| {
    env::var("OPEN_USER_REG")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("OPEN_USER_REG cannot be parsed as bool")
});
pub static PASSWORD_RESET_COOKIE_BINDING: LazyLock<bool> = LazyLock::new(|| {
    env::var("PASSWORD_RESET_COOKIE_BINDING")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("PASSWORD_RESET_COOKIE_BINDING cannot be parsed as bool")
});
pub static PEER_IP_HEADER_NAME: LazyLock<Option<String>> =
    LazyLock::new(|| env::var("PEER_IP_HEADER_NAME").ok());

pub static POW_EXP: LazyLock<u32> = LazyLock::new(|| {
    env::var("POW_EXP")
        .as_deref()
        .unwrap_or("30")
        .parse::<u32>()
        .expect("POW_EXP cannot be parsed as u32")
});
pub static POW_DIFFICULTY: LazyLock<u8> = LazyLock::new(|| {
    if *DEV_MODE {
        10
    } else {
        env::var("POW_DIFFICULTY")
            .as_deref()
            .unwrap_or("20")
            .parse::<u8>()
            .expect("POW_DIFFICULTY cannot be parsed as u8")
    }
});
pub static POW_IT: LazyLock<u64> = LazyLock::new(|| {
    env::var("POW_IT")
        .as_deref()
        .unwrap_or("1000000")
        .parse::<u64>()
        .expect("POW_IT cannot be parsed as u64")
});

pub static PROVIDER_CALLBACK_URI: LazyLock<String> = LazyLock::new(|| {
    let listen_scheme = env::var("LISTEN_SCHEME");
    let scheme = if (listen_scheme.as_deref() == Ok("http") && !*PROXY_MODE)
        || listen_scheme.as_deref() == Ok("unix_http")
    {
        "http"
    } else {
        "https"
    };
    let pub_url = if *DEV_MODE {
        env::var("DEV_MODE_PROVIDER_CALLBACK_URL").unwrap_or_else(|_| PUB_URL.to_string())
    } else {
        PUB_URL.to_string()
    };
    format!("{}://{}/auth/v1/providers/callback", scheme, pub_url)
});
pub static PROVIDER_CALLBACK_URI_ENCODED: LazyLock<String> = LazyLock::new(|| {
    PROVIDER_CALLBACK_URI
        .replace(':', "%3A")
        .replace('/', "%2F")
});
pub static PROXY_MODE: LazyLock<bool> = LazyLock::new(|| {
    env::var("PROXY_MODE")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("PROXY_MODE cannot be parsed as bool")
});
pub static PUB_URL: LazyLock<String> =
    LazyLock::new(|| env::var("PUB_URL").expect("PUB_URL env var is not set"));
pub static PUB_URL_WITH_SCHEME: LazyLock<String> = LazyLock::new(|| {
    let listen_scheme = env::var("LISTEN_SCHEME");
    let scheme = if (listen_scheme.as_deref() == Ok("http") && !*PROXY_MODE)
        || listen_scheme.as_deref() == Ok("unix_http")
    {
        "http"
    } else {
        "https"
    };
    format!("{}://{}", scheme, *PUB_URL)
});
pub static RAUTHY_ADMIN_EMAIL: LazyLock<Option<String>> =
    LazyLock::new(|| env::var("RAUTHY_ADMIN_EMAIL").ok());
pub static REFRESH_TOKEN_LIFETIME: LazyLock<u16> = LazyLock::new(|| {
    env::var("REFRESH_TOKEN_LIFETIME")
        .as_deref()
        .unwrap_or("48")
        .parse::<u16>()
        .expect("REFRESH_TOKEN_LIFETIME cannot be parsed as u16")
});
pub static SEC_HEADER_BLOCK: LazyLock<bool> = LazyLock::new(|| {
    env::var("SEC_HEADER_BLOCK")
        .as_deref()
        .unwrap_or("true")
        .parse::<bool>()
        .expect("SEC_HEADER_BLOCK cannot be parsed as bool")
});

pub static SESSION_LIFETIME: LazyLock<u32> = LazyLock::new(|| {
    env::var("SESSION_LIFETIME")
        .as_deref()
        .unwrap_or("2592000")
        .parse::<u32>()
        .expect("SESSION_LIFETIME cannot be parsed as u32")
});
pub static SESSION_LIFETIME_FED_CM: LazyLock<i64> = LazyLock::new(|| {
    env::var("SESSION_LIFETIME_FED_CM")
        .as_deref()
        .unwrap_or("2592000")
        .parse::<u32>()
        .expect("SESSION_LIFETIME_FED_CM cannot be parsed as u32") as i64
});
pub static SESSION_TIMEOUT: LazyLock<u32> = LazyLock::new(|| {
    env::var("SESSION_TIMEOUT")
        .as_deref()
        .unwrap_or("5400")
        .parse::<u32>()
        .expect("SESSION_TIMEOUT cannot be parsed as u32")
});
pub static SESSION_TIMEOUT_FED_CM: LazyLock<u32> = LazyLock::new(|| {
    env::var("SESSION_TIMEOUT_FED_CM")
        .as_deref()
        .unwrap_or("259200")
        .parse::<u32>()
        .expect("SESSION_TIMEOUT_FED_CM cannot be parsed as u32")
});
pub static SESSION_RENEW_MFA: LazyLock<bool> = LazyLock::new(|| {
    env::var("SESSION_RENEW_MFA")
        .as_deref()
        .unwrap_or("false")
        .parse::<bool>()
        .expect("SESSION_RENEW_MFA cannot be parsed as bool")
});
pub static SESSION_VALIDATE_IP: LazyLock<bool> = LazyLock::new(|| {
    env::var("SESSION_VALIDATE_IP")
        .as_deref()
        .unwrap_or("true")
        .parse::<bool>()
        .expect("SESSION_VALIDATE_IP cannot be parsed as bool")
});

pub static SMTP_FROM: LazyLock<String> = LazyLock::new(|| {
    env::var("SMTP_FROM")
        .unwrap_or_else(|_| "Rauthy <rauthy@localhost>".to_string())
        .trim()
        .to_string()
});
pub static SMTP_PASSWORD: LazyLock<String> = LazyLock::new(|| {
    env::var("SMTP_PASSWORD")
        .expect("SMTP_PASSWORD is not set")
        .trim()
        .to_string()
});
pub static SMTP_URL: LazyLock<Option<String>> =
    LazyLock::new(|| env::var("SMTP_URL").ok().map(|url| url.trim().to_string()));
pub static SMTP_USERNAME: LazyLock<String> = LazyLock::new(|| {
    env::var("SMTP_USERNAME")
        .expect("SMTP_USERNAME is not set")
        .trim()
        .to_string()
});

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

pub static SWAGGER_UI_EXTERNAL: LazyLock<bool> = LazyLock::new(|| {
    env::var("SWAGGER_UI_EXTERNAL")
        .as_deref()
        .unwrap_or("true")
        .parse::<bool>()
        .expect("SWAGGER_UI_EXTERNAL cannot be parsed as bool")
});
pub static SWAGGER_UI_INTERNAL: LazyLock<bool> = LazyLock::new(|| {
    env::var("SWAGGER_UI_INTERNAL")
        .as_deref()
        .unwrap_or("true")
        .parse::<bool>()
        .expect("SWAGGER_UI_INTERNAL cannot be parsed as bool")
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
pub static USERINFO_STRICT: LazyLock<bool> = LazyLock::new(|| {
    env::var("USERINFO_STRICT")
        .as_deref()
        .unwrap_or("true")
        .parse::<bool>()
        .expect("USERINFO_STRICT cannot be parsed as bool")
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
