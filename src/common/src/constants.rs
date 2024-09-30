use crate::utils::build_trusted_proxies;
use crate::DbType;
use actix_web::http::Uri;
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::ops::Not;
use std::str::FromStr;
use std::string::ToString;

#[derive(Debug, PartialEq)]
pub enum CookieMode {
    Host,
    Secure,
    DangerInsecure,
}

pub const RAUTHY_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const CONTENT_TYPE_WEBP: &str = "image/webp";
pub const HEADER_DPOP_NONCE: &str = "DPoP-Nonce";
pub const HEADER_ALLOW_ALL_ORIGINS: (&str, &str) = ("access-control-allow-origin", "*");
pub const HEADER_HTML: (&str, &str) = ("content-type", "text/html;charset=utf-8");
pub const HEADER_JSON: (&str, &str) = ("content-type", "application/json");
pub const HEADER_RETRY_NOT_BEFORE: &str = "x-retry-not-before";
pub const APPLICATION_JSON: &str = "application/json";
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
pub const CSRF_HEADER: &str = "csrf-token";
pub const PWD_CSRF_HEADER: &str = "pwd-csrf-token";

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
pub const IDX_SESSION: &str = "session_";
pub const IDX_SESSIONS: &str = "sessions";
pub const IDX_USERS: &str = "users_";
pub const IDX_USER_COUNT: &str = "users_count_total";
pub const IDX_USERS_VALUES: &str = "users_values_";
pub const IDX_USER_ATTR_CONFIG: &str = "user_attrs_";
pub const IDX_WEBAUTHN: &str = "webauthn_";

// TODO drop `lazy_static` and use rust 1.80 built-in features
lazy_static! {
    pub static ref APP_START: DateTime<Utc> = Utc::now();

    pub static ref CACHE_TTL_AUTH_CODE: Option<i64> = Some(300 + *WEBAUTHN_REQ_EXP as i64);
    pub static ref CACHE_TTL_DEVICE_CODE: Option<i64> = Some(*DEVICE_GRANT_CODE_LIFETIME as i64);
    pub static ref CACHE_TTL_DYN_CLIENT: Option<i64> = Some(*DYN_CLIENT_RATE_LIMIT_SEC as i64);
    pub static ref CACHE_TTL_DPOP_NONCE: Option<i64> = Some(*DPOP_NONCE_EXP as i64);
    pub static ref CACHE_TTL_EPHEMERAL_CLIENT: Option<i64> = Some(env::var("EPHEMERAL_CLIENTS_CACHE_LIFETIME")
            .unwrap_or_else(|_| String::from("3600"))
            .parse::<i64>()
            .expect("EPHEMERAL_CLIENTS_CACHE_LIFETIME cannot be parsed to i64 - bad format"));
    pub static ref CACHE_TTL_IP_RATE_LIMIT: Option<i64> = (*DEVICE_GRANT_RATE_LIMIT).map(|i| i as i64);
    pub static ref CACHE_TTL_POW: Option<i64> = Some(*POW_EXP as i64);
    pub static ref CACHE_TTL_WEBAUTHN: Option<i64> = Some(*WEBAUTHN_REQ_EXP as i64);
    pub static ref CACHE_TTL_WEBAUTHN_DATA: Option<i64> = Some(*WEBAUTHN_DATA_EXP as i64);

    pub static ref RAUTHY_ADMIN_ROLE: String = "rauthy_admin".to_string();

    pub static ref DATABASE_URL: String = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    pub static ref DB_TYPE: DbType = DbType::from_str(&DATABASE_URL).unwrap();

    pub static ref DEV_MODE: bool = env::var("DEV_MODE")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("DEV_MODE cannot be parsed to bool - bad format");
    pub static ref DEV_DPOP_HTTP: bool = env::var("DEV_DPOP_HTTP")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("DEV_DPOP_HTTP cannot be parsed to bool - bad format");

    pub static ref RE_ATTR: Regex = Regex::new(r"^[a-zA-Z0-9-_/]{2,32}$").unwrap();
    pub static ref RE_ATTR_DESC: Regex = Regex::new(r"^[a-zA-Z0-9-_/\s]{0,128}$").unwrap();
    pub static ref RE_ALNUM: Regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
    pub static ref RE_ALNUM_24: Regex = Regex::new(r"^[a-zA-Z0-9]{24}$").unwrap();
    pub static ref RE_ALNUM_48: Regex = Regex::new(r"^[a-zA-Z0-9]{48}$").unwrap();
    pub static ref RE_ALNUM_64: Regex = Regex::new(r"^[a-zA-Z0-9]{64}$").unwrap();
    pub static ref RE_ALNUM_SPACE: Regex = Regex::new(r"^[a-zA-Z0-9\s]+$").unwrap();
    pub static ref RE_API_KEY: Regex = Regex::new(r"^[a-zA-Z0-9_/-]{2,24}$").unwrap();
    pub static ref RE_APP_ID: Regex = Regex::new(r"^[a-zA-Z0-9]{12}$").unwrap();
    pub static ref RE_BASE64: Regex = Regex::new(r"^[a-zA-Z0-9+/=]{4}$").unwrap();
    pub static ref RE_CHALLENGE: Regex = Regex::new(r"^(plain|S256)$").unwrap();
    pub static ref RE_CITY: Regex = Regex::new(r"^[a-zA-Z0-9À-ÿ-]{0,48}$").unwrap();
    pub static ref RE_CLIENT_ID_EPHEMERAL: Regex = Regex::new(r"^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,256}$").unwrap();
    pub static ref RE_CLIENT_NAME: Regex = Regex::new(r"^[a-zA-Z0-9À-ÿ-\s\x{3041}-\x{3096}\x{30A0}-\x{30FF}\x{3400}-\x{4DB5}\x{4E00}-\x{9FCB}\x{F900}-\x{FA6A}\x{2E80}-\x{2FD5}\x{FF66}-\x{FF9F}\x{FFA1}-\x{FFDC}\x{31F0}-\x{31FF}]{2,128}$").unwrap();
    pub static ref RE_CODE_CHALLENGE: Regex = Regex::new(r"^[a-zA-Z0-9-\._~]{43,128}$").unwrap();
    pub static ref RE_CODE_VERIFIER: Regex = Regex::new(r"^[a-zA-Z0-9-\._~+/=]+$").unwrap();
    pub static ref RE_CONTACT: Regex = Regex::new(r"^[a-zA-Z0-9\+.@/:]{0,48}$").unwrap();
    pub static ref RE_DATE_STR: Regex = Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$").unwrap();
    pub static ref RE_GRANT_TYPES: Regex = Regex::new(r"^(authorization_code|client_credentials|urn:ietf:params:oauth:grant-type:device_code|password|refresh_token)$").unwrap();
    pub static ref RE_GRANT_TYPES_EPHEMERAL: Regex = Regex::new(r"^(authorization_code|client_credentials|password|refresh_token)$").unwrap();
    pub static ref RE_GROUPS: Regex = Regex::new(r"^[a-z0-9-_/,:*]{2,64}$").unwrap();
    pub static ref RE_LOWERCASE: Regex = Regex::new(r"^[a-z0-9-_/]{2,128}$").unwrap();
    pub static ref RE_LOWERCASE_SPACE: Regex = Regex::new(r"^[a-z0-9-_/\s]{2,128}$").unwrap();
    pub static ref RE_MFA_CODE: Regex = Regex::new(r"^[a-zA-Z0-9]{48}$").unwrap();
    pub static ref RE_ORIGIN: Regex = {
        let additional_schemes = ADDITIONAL_ALLOWED_ORIGIN_SCHEMES.join("|");
        let pattern = if additional_schemes.is_empty() {
            r"^(http|https)://[a-z0-9.:-]+$".to_string()
        } else {
            format!("^(http|https|{})://[a-z0-9.:-]+$", additional_schemes)
        };
        Regex::new(&pattern).unwrap()
    };
    pub static ref RE_PEM: Regex = Regex::new(r"^(-----BEGIN CERTIFICATE-----)[a-zA-Z0-9+/=\n]+(-----END CERTIFICATE-----)$").unwrap();
    pub static ref RE_PHONE: Regex = Regex::new(r"^\+[0-9]{0,32}$").unwrap();
    // we have a pretty high upper limit for characters here just to be sure that even if
    // multiple values like 'urn:ietf:params:oauth:grant-type:device_code' would not fail
    pub static ref RE_SCOPE_SPACE: Regex = Regex::new(r"^[a-z0-9-_/:\s*]{0,512}$").unwrap();
    pub static ref RE_SEARCH: Regex = Regex::new(r"^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%@]+$").unwrap();
    pub static ref RE_STREET: Regex = Regex::new(r"^[a-zA-Z0-9À-ÿ-.\s]{0,48}$").unwrap();
    pub static ref RE_URI: Regex = Regex::new(r"^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]+$").unwrap();
    pub static ref RE_USER_NAME: Regex = Regex::new(r"^[a-zA-Z0-9À-ÿ-\s\x{3041}-\x{3096}\x{30A0}-\x{30FF}\x{3400}-\x{4DB5}\x{4E00}-\x{9FCB}\x{F900}-\x{FA6A}\x{2E80}-\x{2FD5}\x{FF66}-\x{FF9F}\x{FFA1}-\x{FFDC}\x{31F0}-\x{31FF}]{1,32}$").unwrap();
    pub static ref RE_TOKEN_68: Regex = Regex::new(r"^[a-zA-Z0-9-._~+/]+=*$").unwrap();
    pub static ref RE_TOKEN_ENDPOINT_AUTH_METHOD: Regex = Regex::new(r"^(client_secret_post|client_secret_basic|none)$").unwrap();

    pub static ref USERINFO_STRICT: bool = env::var("USERINFO_STRICT")
        .unwrap_or_else(|_| String::from("true"))
        .parse::<bool>()
        .expect("USERINFO_STRICT cannot be parsed to bool - bad format");

    pub static ref DANGER_DISABLE_INTROSPECT_AUTH: bool = env::var("DANGER_DISABLE_INTROSPECT_AUTH")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("DANGER_DISABLE_INTROSPECT_AUTH cannot be parsed to bool - bad format");

    pub static ref SEC_HEADER_BLOCK: bool = env::var("SEC_HEADER_BLOCK")
        .unwrap_or_else(|_| String::from("true"))
        .parse::<bool>()
        .expect("SEC_HEADER_BLOCK cannot be parsed to bool - bad format");

    pub static ref AUTH_HEADERS_ENABLE: bool = env::var("AUTH_HEADERS_ENABLE")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("Cannot parse AUTH_HEADERS_ENABLE to bool");
    pub static ref AUTH_HEADER_USER: String = env::var("AUTH_HEADER_USER")
        .unwrap_or_else(|_| String::from("x-forwarded-user"));
    pub static ref AUTH_HEADER_ROLES: String = env::var("AUTH_HEADER_ROLES")
        .unwrap_or_else(|_| String::from("x-forwarded-user-roles"));
    pub static ref AUTH_HEADER_GROUPS: String = env::var("AUTH_HEADER_GROUPS")
        .unwrap_or_else(|_| String::from("x-forwarded-user-groups"));
    pub static ref AUTH_HEADER_EMAIL: String = env::var("AUTH_HEADER_EMAIL")
        .unwrap_or_else(|_| String::from("x-forwarded-user-email"));
    pub static ref AUTH_HEADER_EMAIL_VERIFIED: String = env::var("AUTH_HEADER_EMAIL_VERIFIED")
        .unwrap_or_else(|_| String::from("x-forwarded-user-email-verified"));
    pub static ref AUTH_HEADER_FAMILY_NAME: String = env::var("AUTH_HEADER_FAMILY_NAME")
        .unwrap_or_else(|_| String::from("x-forwarded-user-family-name"));
    pub static ref AUTH_HEADER_GIVEN_NAME: String = env::var("AUTH_HEADER_GIVEN_NAME")
        .unwrap_or_else(|_| String::from("x-forwarded-user-given-name"));
    pub static ref AUTH_HEADER_MFA: String = env::var("AUTH_HEADER_MFA")
        .unwrap_or_else(|_| String::from("x-forwarded-user-mfa"));

     pub static ref HEALTH_CHECK_DELAY_SECS: u16 = env::var("HEALTH_CHECK_DELAY_SECS")
        .unwrap_or_else(|_| "30".to_string())
        .parse::<u16>()
        .expect("HEALTH_CHECK_DELAY_SECS cannot be parsed to u16 - bad format");

    pub static ref COOKIE_MODE: CookieMode = {
        let var = env::var("COOKIE_MODE").unwrap_or_else(|_| "host".to_string());
        match var.as_str() {
            "host" => CookieMode::Host,
            "secure" => CookieMode::Secure,
            "danger-insecure" => CookieMode::DangerInsecure,
            _ => panic!("COOKIE_MODE must be one of: host, secure, danger-insecure")
        }
    };
    pub static ref COOKIE_SET_PATH: bool = env::var("COOKIE_SET_PATH")
        .unwrap_or_else(|_| String::from("true"))
        .parse::<bool>()
        .expect("COOKIE_SET_PATH cannot be parsed to bool - bad format");

    pub static ref SUSPICIOUS_REQUESTS_BLACKLIST: u16 = env::var("SUSPICIOUS_REQUESTS_BLACKLIST")
        .unwrap_or_else(|_| String::from("1440"))
        .parse::<u16>()
        .expect("SUSPICIOUS_REQUESTS_BLACKLIST cannot be parsed to u16 - bad format");
    pub static ref SUSPICIOUS_REQUESTS_LOG: bool = env::var("SUSPICIOUS_REQUESTS_LOG")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("SUSPICIOUS_REQUESTS_LOG cannot be parsed to bool - bad format");

    pub static ref PUB_URL: String = env::var("PUB_URL").expect("PUB_URL env var is not set");
    pub static ref PUB_URL_WITH_SCHEME: String = {
        let listen_scheme = env::var("LISTEN_SCHEME");
        let scheme = if (listen_scheme.as_deref() == Ok("http") && !*PROXY_MODE)
            || listen_scheme.as_deref() == Ok("unix_http") {
            "http"
        } else {
            "https"
        };
        format!("{}://{}", scheme, *PUB_URL)
    };
    pub static ref ADDITIONAL_ALLOWED_ORIGIN_SCHEMES: Vec<String> = env::var("ADDITIONAL_ALLOWED_ORIGIN_SCHEMES")
        .unwrap_or_else(|_| String::from(""))
        .split(' ')
        .filter_map(|scheme| scheme.is_empty().not().then_some(scheme.to_string()))
        .collect();

    pub static ref PROVIDER_CALLBACK_URI: String = {
        let listen_scheme = env::var("LISTEN_SCHEME");
        let scheme = if (listen_scheme.as_deref() == Ok("http") && !*PROXY_MODE)
            || listen_scheme.as_deref() == Ok("unix_http") {
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
    };
    pub static ref PROVIDER_CALLBACK_URI_ENCODED: String = {
        PROVIDER_CALLBACK_URI.replace(':', "%3A").replace('/', "%2F")
    };

    pub static ref DEVICE_GRANT_CODE_LIFETIME: u16 = env::var("DEVICE_GRANT_CODE_LIFETIME")
        .unwrap_or_else(|_| String::from("300"))
        .parse::<u16>()
        .expect("DEVICE_GRANT_CODE_LIFETIME cannot be parsed to u16 - bad format");
    pub static ref DEVICE_GRANT_USER_CODE_LENGTH: u8 = env::var("DEVICE_GRANT_USER_CODE_LENGTH")
        .unwrap_or_else(|_| String::from("8"))
        .parse::<u8>()
        .expect("DEVICE_GRANT_USER_CODE_LENGTH cannot be parsed to u8 - bad format");
    pub static ref DEVICE_GRANT_RATE_LIMIT: Option<u32> = env::var("DEVICE_GRANT_RATE_LIMIT")
        .map(|rl| rl.parse::<u32>()
        .expect("DEVICE_GRANT_RATE_LIMIT cannot be parsed to u32 - bad format"))
        .ok();
    pub static ref DEVICE_GRANT_POLL_INTERVAL: u8 = env::var("DEVICE_GRANT_POLL_INTERVAL")
        .unwrap_or_else(|_| String::from("5"))
        .parse::<u8>()
        .expect("DEVICE_GRANT_POLL_INTERVAL cannot be parsed to u8 - bad format");
    pub static ref DEVICE_GRANT_REFRESH_TOKEN_LIFETIME: u16 = env::var("DEVICE_GRANT_REFRESH_TOKEN_LIFETIME")
       .unwrap_or_else(|_| String::from("72"))
       .parse::<u16>()
       .expect("DEVICE_GRANT_REFRESH_TOKEN_LIFETIME cannot be parsed to u16 - bad format");

    pub static ref DPOP_TOKEN_ENDPOINT: Uri = {
        let scheme = if *DEV_MODE && *DEV_DPOP_HTTP { "http" } else { "https" };
        let uri = format!("{}://{}/auth/v1/oidc/token", scheme, *PUB_URL);
        Uri::from_str(&uri).unwrap()
    };
    pub static ref DPOP_FORCE_NONCE: bool = env::var("DPOP_NONCE_FORCE")
        .unwrap_or_else(|_| String::from("true"))
        .parse::<bool>()
        .expect("Cannot parse DPOP_FORCE_NONCE to bool");

    pub static ref ENABLE_DYN_CLIENT_REG: bool = env::var("ENABLE_DYN_CLIENT_REG")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("ENABLE_DYN_CLIENT_REG cannot be parsed to bool - bad format");
    pub static ref DYN_CLIENT_REG_TOKEN: Option<String> = env::var("DYN_CLIENT_REG_TOKEN").ok();
    pub static ref DYN_CLIENT_DEFAULT_TOKEN_LIFETIME: i32 = env::var("DYN_CLIENT_DEFAULT_TOKEN_LIFETIME")
        .unwrap_or_else(|_| String::from("1800"))
        .parse::<i32>()
        .expect("DYN_CLIENT_DEFAULT_TOKEN_LIFETIME cannot be parsed to i32 - bad format");
    pub static ref DYN_CLIENT_SECRET_AUTO_ROTATE: bool = env::var("DYN_CLIENT_SECRET_AUTO_ROTATE")
        .unwrap_or_else(|_| String::from("true"))
        .parse::<bool>()
        .expect("DYN_CLIENT_SECRET_AUTO_ROTATE cannot be parsed to bool - bad format");
    pub static ref DYN_CLIENT_CLEANUP_INTERVAL: u64 = env::var("DYN_CLIENT_CLEANUP_INTERVAL")
        .unwrap_or_else(|_| String::from("60"))
        .parse::<u64>()
        .expect("DYN_CLIENT_CLEANUP_INTERVAL cannot be parsed to u64 - bad format");
    pub static ref DYN_CLIENT_CLEANUP_MINUTES: i64 = env::var("DYN_CLIENT_CLEANUP_MINUTES")
        .unwrap_or_else(|_| String::from("60"))
        .parse::<i64>()
        .expect("DYN_CLIENT_CLEANUP_MINUTES cannot be parsed to i64 - bad format");
    pub static ref DYN_CLIENT_RATE_LIMIT_SEC: u64 = env::var("DYN_CLIENT_RATE_LIMIT_SEC")
        .unwrap_or_else(|_| String::from("60"))
        .parse::<u64>()
        .expect("DYN_CLIENT_RATE_LIMIT_SEC cannot be parsed to u64 - bad format");

    pub static ref ENABLE_EPHEMERAL_CLIENTS: bool = env::var("ENABLE_EPHEMERAL_CLIENTS")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("ENABLE_EPHEMERAL_CLIENTS cannot be parsed to bool - bad format");
    pub static ref ENABLE_WEB_ID: bool = env::var("ENABLE_WEB_ID")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("ENABLE_WEB_ID cannot be parsed to bool - bad format");
    pub static ref ENABLE_SOLID_AUD: bool = env::var("ENABLE_SOLID_AUD")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("ENABLE_SOLID_AUD cannot be parsed to bool - bad format");
    pub static ref EPHEMERAL_CLIENTS_FORCE_MFA: bool = env::var("EPHEMERAL_CLIENTS_FORCE_MFA")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("EPHEMERAL_CLIENTS_FORCE_MFA cannot be parsed to bool - bad format");
    pub static ref EPHEMERAL_CLIENTS_ALLOWED_FLOWS: String = env::var("EPHEMERAL_CLIENTS_ALLOWED_FLOWS")
            .unwrap_or_else(|_| String::from("authorization_code"))
            .split(' ')
            .map(|flow| {
                let flow = flow.trim();
                if !RE_GRANT_TYPES_EPHEMERAL.is_match(flow) {
                    panic!("unknown EPHEMERAL_CLIENTS_ALLOWED_FLOWS: {}", flow)
                }
                flow.to_string()
            })
            .collect::<Vec<String>>()
            .join(",");
    pub static ref EPHEMERAL_CLIENTS_ALLOWED_SCOPES: String = env::var("EPHEMERAL_CLIENTS_ALLOWED_SCOPES")
            .unwrap_or_else(|_| String::from("openid profile email webid"))
            .split(' ')
            .filter(|scope| !scope.is_empty())
            .map(|scope| scope.to_string())
            .collect::<Vec<String>>()
            .join(",");

    pub static ref EXPERIMENTAL_FED_CM_ENABLE: bool = env::var("EXPERIMENTAL_FED_CM_ENABLE")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("EXPERIMENTAL_FED_CM_ENABLE cannot be parsed to bool - bad format");

    pub static ref REFRESH_TOKEN_LIFETIME: u16 = env::var("REFRESH_TOKEN_LIFETIME")
       .unwrap_or_else(|_| String::from("48"))
       .parse::<u16>()
       .expect("REFRESH_TOKEN_LIFETIME cannot be parsed to u16 - bad format");

    pub static ref PROXY_MODE: bool = env::var("PROXY_MODE")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .unwrap_or(true);
    pub static ref TRUSTED_PROXIES: Vec<cidr::IpCidr> = build_trusted_proxies();

    pub static ref OPEN_USER_REG: bool = env::var("OPEN_USER_REG")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("OPEN_USER_REG cannot be parsed to bool - bad format");
    pub static ref USER_REG_DOMAIN_RESTRICTION: Option<String> = {
        match env::var("USER_REG_DOMAIN_RESTRICTION") {
            Err(_) => None,
            Ok(domain) => Some(domain)
        }
    };
    pub static ref USER_REG_DOMAIN_BLACKLIST: Option<Vec<String>> = env::var("USER_REG_DOMAIN_BLACKLIST")
        .ok()
        .map(|blacklist| blacklist.lines()
            .filter_map(|line| {
                let trimmed = line.trim();
                trimmed.is_empty().not().then_some(trimmed.to_string())
            })
            .collect()
        );

    pub static ref PEER_IP_HEADER_NAME: Option<String> = env::var("PEER_IP_HEADER_NAME").ok();

    pub static ref POW_IT: u64 = env::var("POW_IT")
        .unwrap_or_else(|_| String::from("1000000"))
        .parse::<u64>()
        .expect("POW_IT cannot be parsed to u64 - bad format");
    pub static ref POW_EXP: u32 = env::var("POW_EXP")
        .unwrap_or_else(|_| String::from("30"))
        .parse::<u32>()
        .expect("POW_EXP cannot be parsed to u32 - bad format");
    pub static ref POW_DIFFICULTY: u8 = env::var("POW_DIFFICULTY")
            .unwrap_or_else(|_| String::from("20"))
            .parse::<u8>()
            .expect("POW_DIFFICULTY cannot be parsed to u8 - bad format");

    pub static ref ADMIN_FORCE_MFA: bool = env::var("ADMIN_FORCE_MFA")
        .unwrap_or_else(|_| String::from("true"))
        .parse::<bool>()
        .expect("ADMIN_FORCE_MFA cannot be parsed to bool - bad format");

    pub static ref DPOP_NONCE_EXP: u32 = env::var("DPOP_NONCE_EXP")
        .unwrap_or_else(|_| String::from("900"))
        .parse::<u32>()
        .expect("DPOP_NONCE_EXP cannot be parsed to u32 - bad format");

    pub static ref SESSION_LIFETIME_FED_CM: i64 = env::var("SESSION_LIFETIME_FED_CM")
        .unwrap_or_else(|_| String::from("2592000"))
        .parse::<i64>()
        .expect("SESSION_LIFETIME_FED_CM cannot be parsed to i64 - bad format");
    pub static ref SESSION_TIMEOUT_FED_CM: u32 = env::var("SESSION_TIMEOUT_FED_CM")
        .unwrap_or_else(|_| String::from("259200"))
        .parse::<u32>()
        .expect("SESSION_TIMEOUT_FED_CM cannot be parsed to u32 - bad format");

    pub static ref SESSION_LIFETIME: u32 = env::var("SESSION_LIFETIME")
        .unwrap_or_else(|_| String::from("14400"))
        .parse::<u32>()
        .expect("SESSION_LIFETIME cannot be parsed to u32 - bad format");
    pub static ref SESSION_RENEW_MFA: bool = env::var("SESSION_RENEW_MFA")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("SESSION_RENEW_MFA cannot be parsed to bool - bad format");
    pub static ref SESSION_TIMEOUT: u32 = env::var("SESSION_TIMEOUT")
        .unwrap_or_else(|_| String::from("5400"))
        .parse::<u32>()
        .expect("SESSION_TIMEOUT cannot be parsed to u32 - bad format");
    pub static ref SESSION_VALIDATE_IP: bool = env::var("SESSION_VALIDATE_IP")
        .unwrap_or_else(|_| String::from("true"))
        .parse::<bool>()
        .expect("SESSION_VALIDATE_IP cannot be parsed to bool - bad format");

    pub static ref SSE_KEEP_ALIVE: u16 = env::var("SSE_KEEP_ALIVE")
        .unwrap_or_else(|_| String::from("30"))
        .parse::<u16>()
        .expect("SSE_KEEP_ALIVE cannot be parsed to u16 - bad format");

    pub static ref RAUTHY_ADMIN_EMAIL: Option<String> = env::var("RAUTHY_ADMIN_EMAIL").ok();
    pub static ref EMAIL_SUB_PREFIX: String = env::var("EMAIL_SUB_PREFIX")
        .unwrap_or_else(|_| String::from("Rauthy IAM"));
    pub static ref SMTP_USERNAME: String = env::var("SMTP_USERNAME")
        .expect("SMTP_USERNAME is not set")
        .trim()
        .to_string();
    pub static ref SMTP_PASSWORD: String = env::var("SMTP_PASSWORD")
        .expect("SMTP_USERNAME is not set")
        .trim()
        .to_string();
    pub static ref SMTP_URL: Option<String> = env::var("SMTP_URL")
        .ok()
        .map(|url| url.trim().to_string());
    pub static ref SMTP_FROM: String = env::var("SMTP_FROM")
        .unwrap_or_else(|_| "Rauthy <rauthy@localhost.de>".to_string())
        .trim()
        .to_string();

    pub static ref SWAGGER_UI_INTERNAL: bool = env::var("SESSION_VALIDATE_IP")
        .unwrap_or_else(|_| String::from("true"))
        .parse::<bool>()
        .expect("SWAGGER_UI_INTERNAL cannot be parsed to bool - bad format");
    pub static ref SWAGGER_UI_EXTERNAL: bool = env::var("SWAGGER_UI_EXTERNAL")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("SWAGGER_UI_EXTERNAL cannot be parsed to bool - bad format");

     pub static ref SSP_THRESHOLD: u16 = env::var("SSP_THRESHOLD")
        .unwrap_or_else(|_| String::from("1000"))
        .parse::<u16>()
        .expect("SSP_THRESHOLD cannot be parsed to u16 - bad format");

    pub static ref PASSWORD_RESET_COOKIE_BINDING: bool = env::var("PASSWORD_RESET_COOKIE_BINDING")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("PASSWORD_RESET_COOKIE_BINDING cannot be parsed to bool - bad format");

    pub static ref WEBAUTHN_REQ_EXP: u64 = env::var("WEBAUTHN_REQ_EXP")
        .unwrap_or_else(|_| String::from("60"))
        .parse::<u64>()
        .expect("WEBAUTHN_REQ_EXP cannot be parsed to u64 - bad format");
    pub static ref WEBAUTHN_DATA_EXP: u64 = env::var("WEBAUTHN_DATA_EXP")
        .unwrap_or_else(|_| String::from("90"))
        .parse::<u64>()
        .expect("WEBAUTHN_DATA_EXP cannot be parsed to u64 - bad format");
    pub static ref WEBAUTHN_RENEW_EXP: i64 = env::var("WEBAUTHN_RENEW_EXP")
        .unwrap_or_else(|_| String::from("2160"))
        .parse::<i64>()
        .expect("WEBAUTHN_RENEW_EXP cannot be parsed to u64 - bad format");
    pub static ref WEBAUTHN_FORCE_UV: bool = env::var("WEBAUTHN_FORCE_UV")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("WEBAUTHN_FORCE_UV cannot be parsed to bool - bad format");
    pub static ref WEBAUTHN_NO_PASSWORD_EXPIRY: bool = env::var("WEBAUTHN_FORCE_UV")
        .unwrap_or_else(|_| String::from("true"))
        .parse::<bool>()
        .expect("WEBAUTHN_NO_PASSWORD_EXPIRY cannot be parsed to bool - bad format");
}
