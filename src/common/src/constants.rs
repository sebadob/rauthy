use chrono::{DateTime, Utc};
use serde::Deserialize;
use std::env;
use std::sync::{LazyLock, OnceLock};

#[derive(Debug, Default, PartialEq, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CookieMode {
    #[default]
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
                panic!("Cannot parse {v} as CookieMode");
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

pub static BUILD_TIME: LazyLock<DateTime<Utc>> = LazyLock::new(|| {
    DateTime::from_timestamp(env!("BUILD_TIME").parse::<i64>().unwrap(), 0).unwrap()
});

pub const RAUTHY_VERSION: &str = env!("CARGO_PKG_VERSION");
pub static CONTENT_TYPE_WEBP: &str = "image/webp";
pub static HEADER_DPOP_NONCE: &str = "DPoP-Nonce";
pub static HEADER_ALLOW_ALL_ORIGINS: (&str, &str) = ("access-control-allow-origin", "*");
pub static HEADER_HTML: (&str, &str) = ("content-type", "text/html;charset=utf-8");
pub static HEADER_JSON: (&str, &str) = ("content-type", "application/json");
pub static HEADER_RETRY_NOT_BEFORE: &str = "x-retry-not-before";
pub static APPLICATION_JSON: &str = "application/json";
pub static APPLICATION_JSON_SCIM: &str = "application/scim+json";
pub static TEXT_TURTLE: &str = "text/turtle";

pub static TOKEN_API_KEY: &str = "API-Key";
pub static TOKEN_BEARER: &str = "Bearer";
pub static TOKEN_DPOP: &str = "DPoP";
pub static TOKEN_DPOP_NONCE: &str = "DPoP-nonce";
pub static COOKIE_SESSION: &str = "RauthySession";
pub static COOKIE_SESSION_FED_CM: &str = "RauthySessionFedCM";
pub static COOKIE_MFA: &str = "RauthyMfa";
pub static COOKIE_LOCALE: &str = "locale";
pub static COOKIE_UPSTREAM_CALLBACK: &str = "UpstreamAuthCallback";
pub static PROVIDER_ATPROTO: &str = "atproto";
pub static PROVIDER_LINK_COOKIE: &str = "rauthy-provider-link";
pub static PWD_RESET_COOKIE: &str = "rauthy-pwd-reset";
pub static APP_ID_HEADER: &str = "mfa-app-id";
pub static CSRF_HEADER: &str = "x-csrf-token";
pub static PWD_CSRF_HEADER: &str = "x-pwd-csrf-token";

pub static PAM_WHEEL_ID: u32 = 100_000;
pub static PAM_WHEEL_NAME: &str = "wheel-rauthy";

pub static ARGON2ID_M_COST_MIN: u32 = 32768;
pub static ARGON2ID_T_COST_MIN: u32 = 1;
pub static API_KEY_LENGTH: usize = 64;
pub static DEVICE_KEY_LENGTH: u8 = 64;
pub static EVENTS_LATEST_LIMIT: u16 = 100;
pub static GRANT_TYPE_DEVICE_CODE: &str = "urn:ietf:params:oauth:grant-type:device_code";
pub const UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS: u16 = 300;
pub const CACHE_TTL_APP: Option<i64> = Some(43200);
pub const CACHE_TTL_AUTH_PROVIDER_CALLBACK: Option<i64> =
    Some(UPSTREAM_AUTH_CALLBACK_TIMEOUT_SECS as i64);
pub const CACHE_TTL_SESSION: Option<i64> = Some(14400);
pub const CACHE_TTL_USER: Option<i64> = Some(600);

pub static IDX_APP_VERSION: &str = "rauthy_app_version";
pub static IDX_AUTH_PROVIDER: &str = "auth_provider_";
pub static IDX_AUTH_PROVIDER_LOGO: &str = "auth_provider_logo_";
pub static IDX_AUTH_PROVIDER_TEMPLATE: &str = "provider_json_tpl";
pub static IDX_CLIENTS: &str = "clients_";
pub static IDX_CLIENT_LOGO: &str = "client_logo_";
pub static IDX_GROUPS: &str = "groups_";
pub static IDX_JWK_KID: &str = "jwk_kid_";
pub static IDX_JWK_LATEST: &str = "jwk_latest_";
pub static IDX_JWKS: &str = "jkws_";
pub static IDX_LOGIN_TIME: &str = "login_time_";
pub static IDX_MFA_MOD: &str = "mfa_mod_";
pub static IDX_PASSWORD_RULES: &str = "password_rules_";
pub static IDX_ROLES: &str = "roles_";
pub static IDX_SCOPES: &str = "scopes_";
pub static IDX_SESSIONS: &str = "sessions";
pub static IDX_SMTP_OAUTH_TOKEN: &str = "smtp_oauth_token";
pub static IDX_USERS: &str = "users_";
pub static IDX_USER_COUNT: &str = "users_count_total";
pub static IDX_USERS_VALUES: &str = "users_values_";
pub static IDX_USER_ATTR_CONFIG: &str = "user_attrs_";
pub static IDX_WEBAUTHN: &str = "webauthn_";

pub static APP_START: LazyLock<DateTime<Utc>> = LazyLock::new(Utc::now);
pub static RAUTHY_ADMIN_ROLE: &str = "rauthy_admin";

pub static DEV_MODE: OnceLock<bool> = OnceLock::new();

pub static DPOP_TOKEN_ENDPOINT: OnceLock<String> = OnceLock::new();
pub static PEER_IP_HEADER_NAME: OnceLock<Option<String>> = OnceLock::new();
pub static PROXY_MODE: OnceLock<bool> = OnceLock::new();

pub static TRUSTED_PROXIES: OnceLock<Vec<cidr::IpCidr>> = OnceLock::new();

pub static SECRET_LEN_CLIENTS: usize = 64;
