use crate::DbType;
use lazy_static::lazy_static;
use regex::Regex;
use std::env;
use std::str::FromStr;

pub const RAUTHY_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const HEADER_HTML: (&str, &str) = ("content-type", "text/html;charset=utf-8");
pub const APPLICATION_JSON: &str = "application/json";
pub const TOKEN_BEARER: &str = "Bearer";
pub const COOKIE_SESSION: &str = "rauthy-session";
pub const COOKIE_MFA: &str = "rauthy-mfa";
pub const COOKIE_LOCALE: &str = "locale";
pub const PWD_RESET_COOKIE: &str = "rauthy-pwd-reset";
pub const APP_ID_HEADER: &str = "mfa-app-id";
pub const CSRF_HEADER: &str = "csrf-token";
pub const PWD_CSRF_HEADER: &str = "pwd-csrf-token";

pub const ARGON2ID_M_COST_MIN: u32 = 32768;
pub const ARGON2ID_T_COST_MIN: u32 = 1;

pub const CACHE_NAME_12HR: &str = "12hr";
pub const CACHE_NAME_AUTH_CODES: &str = "auth-codes";
pub const CACHE_NAME_LOGIN_DELAY: &str = "login-dly";
pub const CACHE_NAME_SESSIONS: &str = "sessions";
pub const CACHE_NAME_POW: &str = "pow";
pub const CACHE_NAME_WEBAUTHN: &str = "webauthn";
pub const CACHE_NAME_WEBAUTHN_DATA: &str = "webauthn-data";

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
pub const IDX_USER_ATTR_CONFIG: &str = "user_attrs_";
pub const IDX_WEBAUTHN: &str = "webauthn_";

lazy_static! {
    pub static ref DATABASE_URL: String = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    pub static ref DB_TYPE: DbType = DbType::from_str(&DATABASE_URL).unwrap();
    pub static ref ROLE_ADMIN: String = "ROLE_rauthy_admin".to_string();
    pub static ref DEV_MODE: bool = env::var("DEV_MODE")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .expect("DEV_MODE cannot be parsed to bool - bad format");

    pub static ref RE_ALG: Regex = Regex::new(r"^(RS256|RS384|RS512|EdDSA)$").unwrap();
    pub static ref RE_ATTR: Regex = Regex::new(r"^[a-zA-Z0-9-_/]{2,32}$").unwrap();
    pub static ref RE_ATTR_DESC: Regex = Regex::new(r"^[a-zA-Z0-9-_/\s]{0,128}$").unwrap();
    pub static ref RE_ALNUM: Regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
    pub static ref RE_ALNUM_24: Regex = Regex::new(r"^[a-zA-Z0-9]{24}$").unwrap();
    pub static ref RE_ALNUM_48: Regex = Regex::new(r"^[a-zA-Z0-9]{48}$").unwrap();
    pub static ref RE_ALNUM_64: Regex = Regex::new(r"^[a-zA-Z0-9]{64}$").unwrap();
    pub static ref RE_ALNUM_SPACE: Regex = Regex::new(r"^[a-zA-Z0-9\s]+$").unwrap();
    pub static ref RE_APP_ID: Regex = Regex::new(r"^[a-zA-Z0-9]{12}$").unwrap();
    pub static ref RE_BASE64: Regex = Regex::new(r"^[a-zA-Z0-9+/=]{4}$").unwrap();
    pub static ref RE_CHALLENGE: Regex = Regex::new(r"^(plain|S256)$").unwrap();
    pub static ref RE_CLIENT_NAME: Regex = Regex::new(r"^[a-zA-Z0-9À-ÿ-\s]{2,128}$").unwrap();
    pub static ref RE_CODE_CHALLENGE: Regex = Regex::new(r"^[a-zA-Z0-9-\._~]{43,128}$").unwrap();
    pub static ref RE_CODE_VERIFIER: Regex = Regex::new(r"^[a-zA-Z0-9-\._~+/=]+$").unwrap();
    pub static ref RE_FLOWS: Regex = Regex::new(r"^(authorization_code|client_credentials|password|refresh_token)$").unwrap();
    pub static ref RE_LOWERCASE: Regex = Regex::new(r"^[a-z0-9-_/]{2,128}$").unwrap();
    pub static ref RE_GROUPS: Regex = Regex::new(r"^[a-z0-9-_/,]{2,32}$").unwrap();
    pub static ref RE_MFA_CODE: Regex = Regex::new(r"^[a-zA-Z0-9]{48}$").unwrap();
    pub static ref RE_URI: Regex = Regex::new(r"^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]+$").unwrap();
    pub static ref RE_USER_NAME: Regex = Regex::new(r"^[a-zA-Z0-9À-ÿ-\s]{2,32}$").unwrap();

    pub static ref PROXY_MODE: bool = env::var("PROXY_MODE")
        .unwrap_or_else(|_| String::from("false"))
        .parse::<bool>()
        .unwrap_or(true);

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

    pub static ref POW_IT: u64 = env::var("POW_IT")
        .unwrap_or_else(|_| String::from("1000000"))
        .parse::<u64>()
        .expect("POW_IT cannot be parsed to u64 - bad format");
    pub static ref POW_EXP: u64 = env::var("POW_EXP")
        .unwrap_or_else(|_| String::from("300"))
        .parse::<u64>()
        .expect("POW_EXP cannot be parsed to u64 - bad format");
    pub static ref POW_EXP_DUR: time::Duration = if *POW_EXP > i64::MAX as u64 {
        time::Duration::seconds(i64::MAX)
    } else {
        time::Duration::seconds(*POW_EXP as i64)
    };

    // Offline Token lifetime in seconds
    pub static ref OFFLINE_TOKEN_LT: i64 = {
        let lt = env::var("OFFLINE_TOKEN_LIFETIME")
            .unwrap_or_else(|_| String::from("720"))
            .parse::<i64>()
            .expect("OFFLINE_TOKEN_LIFETIME cannot be parsed to i64 - bad format");
        lt * 3600
    };

    pub static ref ADMIN_FORCE_MFA: bool = env::var("ADMIN_FORCE_MFA")
        .unwrap_or_else(|_| String::from("true"))
        .parse::<bool>()
        .expect("ADMIN_FORCE_MFA cannot be parsed to bool - bad format");
    pub static ref ADMIN_ACCESS_SESSION_ONLY: bool = env::var("ADMIN_ACCESS_SESSION_ONLY")
        .unwrap_or_else(|_| String::from("true"))
        .parse::<bool>()
        .expect("ADMIN_ACCESS_SESSION_ONLY cannot be parsed to bool - bad format");

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

    pub static ref SMTP_USERNAME: String = env::var("SMTP_USERNAME")
        .expect("SMTP_USERNAME is not set")
        .trim()
        .to_string();
    pub static ref SMTP_PASSWORD: String = env::var("SMTP_PASSWORD")
        .expect("SMTP_USERNAME is not set")
        .trim()
        .to_string();
    pub static ref SMTP_URL: String = env::var("SMTP_URL")
        .expect("SMTP_URL is not set")
        .trim()
        .to_string();
    pub static ref SMTP_FROM: String = env::var("SMTP_FROM")
        .expect("SMTP_FROM is not set")
        .trim()
        .to_string();

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
