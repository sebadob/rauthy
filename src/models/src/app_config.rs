use crate::ListenScheme;
use crate::email::EMail;
use crate::events::event::{Event, EventLevel};
use crate::events::listener::EventRouterMsg;
use rauthy_common::constants::CookieMode;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::env;
use std::error::Error;
use std::sync::Arc;
use tokio::fs;
use tokio::sync::mpsc;
use toml::Value;
use webauthn_rs::Webauthn;

#[derive(Debug)]
pub struct AppConfig {
    pub public_url: String,
    pub argon2_params: argon2::Params,
    pub issuer: String,
    pub listen_addr: String,
    pub listen_scheme: ListenScheme,
    pub refresh_grace_time: u32,
    pub ml_lt_pwd_first: u32,
    pub ml_lt_pwd_reset: u32,
    pub tx_email: mpsc::Sender<EMail>,
    pub tx_events: flume::Sender<Event>,
    pub tx_events_router: flume::Sender<EventRouterMsg>,
    pub webauthn: Arc<Webauthn>,
}

impl AppConfig {
    pub async fn build(
        tx_email: mpsc::Sender<EMail>,
        tx_events: flume::Sender<Event>,
        tx_events_router: flume::Sender<EventRouterMsg>,
    ) -> Result<Self, Box<dyn Error>> {
        todo!()
    }
}

#[derive(Debug, Deserialize)]
struct Vars {
    pub dev: VarsDev,
    pub access: VarsAccess,
    pub auth_headers: VarsAuthHeaders,
    pub backchannel_logout: VarsBackchannelLogout,
    pub database: VarsDatabase,
    pub device_grant: VarsDeviceGrant,
    pub dpop: VarsDpop,
    pub dynamic_clients: VarsDynamicClients,
    pub email: VarsEmail,
    pub encryption: VarsEncryption,
    pub ephemeral_clients: VarsEphemeralClients,
    pub events: VarsEvents,
    pub fedcm: Option<ConfigVarsFedCM>,
    pub hashing: VarsHashing,
}

impl Default for Vars {
    fn default() -> Self {
        Self {
            dev: VarsDev {
                dev_mode: false,
                dpop_http: false,
                insecure_cookie: false,
                provider_callback_url: None,
            },
            access: VarsAccess {
                userinfo_strict: true,
                danger_disable_introspect_auth: false,
                disable_refresh_token_nbf: false,
                sec_header_block: true,
                session_validate_ip: true,
                password_reset_cookie_binding: false,
                peer_ip_header_name: None,
                cookie_mode: CookieMode::Host,
                cookie_set_path: true,
                token_len_limit: 4096,
            },
            auth_headers: VarsAuthHeaders {
                enable: false,
                user: "x-forwarded-user".into(),
                roles: "x-forwarded-user-roles".into(),
                groups: "x-forwarded-user-groups".into(),
                email: "x-forwarded-user-email".into(),
                email_verified: "x-forwarded-user-email-verified".into(),
                family_name: "x-forwarded-user-family-name".into(),
                given_name: "x-forwarded-user-given-name".into(),
                mfa: "x-forwarded-user-mfa".into(),
            },
            backchannel_logout: VarsBackchannelLogout {
                retry_count: 100,
                damger_allow_http: false,
                danger_allow_insecure: false,
                token_lifetime: 30,
                allow_clock_skew: 5,
                allowed_token_lifetime: 120,
            },
            database: VarsDatabase {
                hiqlite: true,
                health_check_delay_secs: 30,
                pg_host: None,
                pg_port: 5432,
                pg_user: None,
                pg_password: None,
                pg_db_name: "rauthy".into(),
                pg_tls_no_verify: false,
                pg_max_conn: 20,
                migrate_pg_host: None,
                migrate_pg_port: None,
                migrate_pg_user: None,
                migrate_pg_password: None,
                migrate_pg_db_name: None,
                sched_user_exp_mins: 60,
                sched_user_exp_delete_mins: None,
            },
            device_grant: VarsDeviceGrant {
                code_lifetime: 300,
                user_code_length: 8,
                rate_limit: None,
                poll_interval: 5,
                refresh_token_lifetime: 72,
            },
            dpop: VarsDpop {
                force_nonce: true,
                nonce_exp: 900,
            },
            dynamic_clients: VarsDynamicClients {
                enable: false,
                reg_token: None,
                default_token_lifetime: 1800,
                secret_auto_rotate: true,
                cleanup_interval: 60,
                cleanup_minutes: 60,
                rate_limit_sec: 60,
            },
            email: VarsEmail {
                rauthy_admin_email: Cow::default(),
                sub_prefix: "Rauthy IAM".into(),
                smtp_url: None,
                smtp_from: None,
                connect_retries: 3,
                danger_insecure: false,
            },
            encryption: VarsEncryption {
                key_active: String::default(),
                keys: Vec::default(),
            },
            ephemeral_clients: VarsEphemeralClients {
                enable: false,
                enable_web_id: false,
                enable_solid_aud: false,
                force_mfa: false,
                allowed_flows: vec!["authorization_code".into()],
                allowed_scopes: vec![
                    "openid".into(),
                    "profile".into(),
                    "email".into(),
                    "webid".into(),
                ],
                cache_lifetime: 3600,
            },
            events: VarsEvents {
                email: None,
                matrix_user_id: None,
                matrix_room_id: None,
                matrix_access_token: None,
                matrix_user_password: None,
                matrix_server_url: None,
                matrix_root_ca_path: None,
                matrix_danger_disable_tls_validation: false,
                matrix_error_no_panic: false,
                slack_webhook: None,
                notify_level_email: EventLevel::Warning,
                notify_level_matrix: EventLevel::Notice,
                notify_level_slack: EventLevel::Notice,
                persist_level: EventLevel::Info,
                cleanup_days: 30,
                level_new_user: EventLevel::Info,
                level_user_email_change: EventLevel::Notice,
                level_user_password_reset: EventLevel::Notice,
                level_rauthy_admin: EventLevel::Notice,
                level_rauthy_version: EventLevel::Notice,
                level_jwks_rotate: EventLevel::Notice,
                level_secrets_migrated: EventLevel::Notice,
                level_rauthy_start: EventLevel::Info,
                level_rauthy_healthy: EventLevel::Notice,
                level_rauthy_unhealthy: EventLevel::Critical,
                level_ip_blacklisted: EventLevel::Warning,
                level_failed_logins_25: EventLevel::Critical,
                level_failed_logins_20: EventLevel::Critical,
                level_failed_logins_15: EventLevel::Warning,
                level_failed_logins_10: EventLevel::Warning,
                level_failed_logins_7: EventLevel::Notice,
                level_failed_login: EventLevel::Info,
                disable_app_version_check: false,
            },
            fedcm: None,
            hashing: VarsHashing {
                argon2_m_cost: 131072,
                argon2_t_cost: 4,
                argon2_p_cost: 8,
                max_hash_threads: 2,
                hash_await_warn_time: 500,
                jwk_autorotate_cron: "0 30 3 1 * * *".into(),
            },
        }
    }
}

impl Vars {
    async fn load(path_config: &str) -> Self {
        // we want to make sure we grab all the necessary memory BEFORE
        // loading the toml to avoid bigger memory fragmentation chunks.
        let mut slf = Self::default();

        let Ok(config) = fs::read_to_string(path_config).await else {
            panic!("Cannot read config file from {}", path_config);
        };

        // Note: these inner parsers are very verbose, but they allow the upfront memory allocation
        // and memory fragmentation, after the quite big toml has been freed and the config stays
        // in static memory.

        let mut table = config.parse::<toml::Table>().unwrap();
        slf.parse_dev(&mut table);
        slf.parse_access(&mut table);
        // TODO

        slf
    }

    fn parse_dev(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "dev").unwrap();

        if let Some(v) = t_bool(&mut table, "dev", "dev_mode", "DEV_MODE") {
            self.dev.dev_mode = v;
        }
        if let Some(v) = t_bool(&mut table, "dev", "dpop_http", "DEV_DPOP_HTTP") {
            self.dev.dpop_http = v;
        }
        if let Some(v) = t_bool(&mut table, "dev", "insecure_cookie", "HQL_INSECURE_COOKIE") {
            self.dev.insecure_cookie = v;
        }
        if let Some(v) = t_str(
            &mut table,
            "dev",
            "provider_callback_url",
            "DEV_MODE_PROVIDER_CALLBACK_URL",
        ) {
            self.dev.provider_callback_url = Some(v);
        }
    }

    fn parse_access(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "access").unwrap();

        if let Some(v) = t_bool(&mut table, "access", "userinfo_strict", "USERINFO_STRICT") {
            self.access.userinfo_strict = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "access",
            "danger_disable_introspect_auth",
            "DANGER_DISABLE_INTROSPECT_AUTH",
        ) {
            self.access.danger_disable_introspect_auth = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "access",
            "disable_refresh_token_nbf",
            "DISABLE_REFRESH_TOKEN_NBF",
        ) {
            self.access.disable_refresh_token_nbf = v;
        }
        if let Some(v) = t_bool(&mut table, "access", "sec_header_block", "SEC_HEADER_BLOCK") {
            self.access.sec_header_block = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "access",
            "session_validate_ip",
            "SESSION_VALIDATE_IP",
        ) {
            self.access.session_validate_ip = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "access",
            "password_reset_cookie_binding",
            "PASSWORD_RESET_COOKIE_BINDING",
        ) {
            self.access.password_reset_cookie_binding = v;
        }
        if let Some(v) = t_str(
            &mut table,
            "access",
            "peer_ip_header_name",
            "PEER_IP_HEADER_NAME",
        ) {
            self.access.peer_ip_header_name = Some(v);
        }
        if let Some(v) = t_str(&mut table, "access", "cookie_mode", "COOKIE_MODE") {
            self.access.cookie_mode = CookieMode::from(v.as_str());
        }
        if let Some(v) = t_bool(&mut table, "access", "cookie_set_path", "COOKIE_SET_PATH") {
            self.access.cookie_set_path = v;
        }
        if let Some(v) = t_i64(&mut table, "access", "token_len_limit", "TOKEN_LEN_LIMIT") {
            self.access.token_len_limit = v;
        }
    }

    fn parse_auth_headers(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "auth_headers").unwrap();

        if let Some(v) = t_bool(&mut table, "auth_headers", "enable", "AUTH_HEADERS_ENABLE") {
            self.auth_headers.enable = v;
        }

        if let Some(v) = t_str(&mut table, "auth_headers", "user", "AUTH_HEADER_USER") {
            self.auth_headers.user = v.into();
        }
        if let Some(v) = t_str(&mut table, "auth_headers", "roles", "AUTH_HEADER_ROLES") {
            self.auth_headers.roles = v.into();
        }
        if let Some(v) = t_str(&mut table, "auth_headers", "groups", "AUTH_HEADER_GROUPS") {
            self.auth_headers.groups = v.into();
        }
        if let Some(v) = t_str(&mut table, "auth_headers", "email", "AUTH_HEADER_EMAIL") {
            self.auth_headers.email = v.into();
        }
        if let Some(v) = t_str(
            &mut table,
            "auth_headers",
            "email_verified",
            "AUTH_HEADER_EMAIL_VERIFIED",
        ) {
            self.auth_headers.email_verified = v.into();
        }
        if let Some(v) = t_str(
            &mut table,
            "auth_headers",
            "family_name",
            "AUTH_HEADER_FAMILY_NAME",
        ) {
            self.auth_headers.family_name = v.into();
        }
        if let Some(v) = t_str(
            &mut table,
            "auth_headers",
            "given_name",
            "AUTH_HEADER_GIVEN_NAME",
        ) {
            self.auth_headers.given_name = v.into();
        }
        if let Some(v) = t_str(&mut table, "auth_headers", "mfa", "AUTH_HEADER_MFA") {
            self.auth_headers.mfa = v.into();
        }
    }
}

#[derive(Debug, Deserialize)]
struct VarsDev {
    #[serde(default)]
    pub dev_mode: bool,
    #[serde(default)]
    pub dpop_http: bool,
    #[serde(default)]
    pub insecure_cookie: bool,
    pub provider_callback_url: Option<String>,
}

// impl ConfigVarsDev {
//     fn parse(tbl: toml::Table) -> Self {
//         Self {
//             dev_mode: tbl.get("dev_mode").unwrap_or(false),
//             dpop_http: false,
//             insecure_cookie: false,
//             provider_callback_url: None,
//         }
//     }
// }
//

// macro_rules! value {
//     ($t:ty, $tbl:expr, $key:expr) => {
//         let v = $tbl.get($key).unwrap();
//         let toml::value::Value::$ty(v) = v else {
//             panic!("Invalid type for key {}, expected {:?}", $key, &t);
//         }
//         v
//     };
// }

fn value<T>(tbl: &toml::Table, key: &str) -> T {
    let v = tbl.get(key).unwrap();
    match v {
        Value::String(_) => {}
        Value::Integer(_) => {}
        Value::Float(_) => {}
        Value::Boolean(_) => {}
        Value::Datetime(_) => {}
        Value::Array(_) => {}
        Value::Table(_) => {}
    }
    todo!()
}

#[derive(Debug, Deserialize)]
struct VarsAccess {
    #[serde(default = "bool_true")]
    pub userinfo_strict: bool,
    #[serde(default)]
    pub danger_disable_introspect_auth: bool,
    #[serde(default)]
    pub disable_refresh_token_nbf: bool,
    #[serde(default = "bool_true")]
    pub sec_header_block: bool,
    #[serde(default = "bool_true")]
    pub session_validate_ip: bool,
    #[serde(default)]
    pub password_reset_cookie_binding: bool,
    pub peer_ip_header_name: Option<String>,
    #[serde(default)]
    pub cookie_mode: CookieMode,
    #[serde(default = "bool_true")]
    pub cookie_set_path: bool,
    #[serde(default = "u32_4096")]
    pub token_len_limit: i64,
}

impl Default for VarsAccess {
    fn default() -> Self {
        Self {
            userinfo_strict: true,
            danger_disable_introspect_auth: false,
            disable_refresh_token_nbf: false,
            sec_header_block: true,
            session_validate_ip: true,
            password_reset_cookie_binding: false,
            peer_ip_header_name: None,
            cookie_mode: Default::default(),
            cookie_set_path: true,
            token_len_limit: 4096,
        }
    }
}

#[derive(Debug, Deserialize)]
struct VarsAuthHeaders {
    #[serde(default)]
    pub enable: bool,
    pub user: Cow<'static, str>,
    pub roles: Cow<'static, str>,
    pub groups: Cow<'static, str>,
    pub email: Cow<'static, str>,
    pub email_verified: Cow<'static, str>,
    pub family_name: Cow<'static, str>,
    pub given_name: Cow<'static, str>,
    pub mfa: Cow<'static, str>,
}

#[derive(Debug, Deserialize)]
struct VarsBackchannelLogout {
    #[serde(default = "u32_100")]
    pub retry_count: u32,
    #[serde(default)]
    pub damger_allow_http: bool,
    #[serde(default)]
    pub danger_allow_insecure: bool,
    #[serde(default = "u32_30")]
    pub token_lifetime: u32,
    #[serde(default = "u32_5")]
    pub allow_clock_skew: u32,
    #[serde(default = "u32_120")]
    pub allowed_token_lifetime: u32,
}

impl Default for VarsBackchannelLogout {
    fn default() -> Self {
        Self {
            retry_count: 100,
            damger_allow_http: false,
            danger_allow_insecure: false,
            token_lifetime: 30,
            allow_clock_skew: 5,
            allowed_token_lifetime: 120,
        }
    }
}

#[derive(Debug, Deserialize)]
struct VarsDatabase {
    #[serde(default = "bool_true")]
    pub hiqlite: bool,
    #[serde(default = "u32_30")]
    pub health_check_delay_secs: u32,

    pub pg_host: Option<String>,
    pub pg_user: Option<String>,
    pub pg_port: u32,
    pub pg_password: Option<String>,
    pub pg_db_name: Cow<'static, str>,
    #[serde(default)]
    pub pg_tls_no_verify: bool,
    #[serde(default = "u32_30")]
    pub pg_max_conn: u32,

    pub migrate_pg_host: Option<String>,
    pub migrate_pg_port: Option<u32>,
    pub migrate_pg_user: Option<String>,
    pub migrate_pg_password: Option<String>,
    pub migrate_pg_db_name: Option<String>,

    #[serde(default = "u32_60")]
    pub sched_user_exp_mins: u32,
    pub sched_user_exp_delete_mins: Option<u32>,
}

#[derive(Debug, Deserialize)]
struct VarsDeviceGrant {
    #[serde(default = "u32_300")]
    pub code_lifetime: u32,
    #[serde(default = "u32_8")]
    pub user_code_length: u32,
    pub rate_limit: Option<u32>,
    #[serde(default = "u32_5")]
    pub poll_interval: u32,
    #[serde(default = "u32_72")]
    pub refresh_token_lifetime: u32,
}

impl Default for VarsDeviceGrant {
    fn default() -> Self {
        Self {
            code_lifetime: 300,
            user_code_length: 8,
            rate_limit: None,
            poll_interval: 5,
            refresh_token_lifetime: 72,
        }
    }
}

#[derive(Debug, Deserialize)]
struct VarsDpop {
    #[serde(default = "bool_true")]
    pub force_nonce: bool,
    #[serde(default = "u32_900")]
    pub nonce_exp: u32,
}

impl Default for VarsDpop {
    fn default() -> Self {
        Self {
            force_nonce: true,
            nonce_exp: 900,
        }
    }
}

#[derive(Debug, Deserialize)]
struct VarsDynamicClients {
    #[serde(default)]
    pub enable: bool,
    pub reg_token: Option<String>,
    #[serde(default = "u32_1800")]
    pub default_token_lifetime: u32,
    #[serde(default = "bool_true")]
    pub secret_auto_rotate: bool,
    #[serde(default = "u32_60")]
    pub cleanup_interval: u32,
    #[serde(default = "u32_60")]
    pub cleanup_minutes: u32,
    #[serde(default = "u32_60")]
    pub rate_limit_sec: u32,
}

impl Default for VarsDynamicClients {
    fn default() -> Self {
        Self {
            enable: false,
            reg_token: None,
            default_token_lifetime: 1800,
            secret_auto_rotate: true,
            cleanup_interval: 60,
            cleanup_minutes: 60,
            rate_limit_sec: 60,
        }
    }
}

#[derive(Debug, Deserialize)]
struct VarsEmail {
    pub rauthy_admin_email: Cow<'static, str>,
    pub sub_prefix: Cow<'static, str>,
    pub smtp_url: Option<String>,
    pub smtp_from: Option<String>,
    pub connect_retries: u32,
    pub danger_insecure: bool,
}

#[derive(Debug, Deserialize)]
struct VarsEncryption {
    pub key_active: String,
    pub keys: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct VarsEphemeralClients {
    #[serde(default)]
    pub enable: bool,
    #[serde(default)]
    pub enable_web_id: bool,
    #[serde(default)]
    pub enable_solid_aud: bool,
    #[serde(default)]
    pub force_mfa: bool,
    pub allowed_flows: Vec<Cow<'static, str>>,
    pub allowed_scopes: Vec<Cow<'static, str>>,
    #[serde(default = "u32_3600")]
    pub cache_lifetime: u32,
}

#[derive(Debug, Deserialize)]
struct VarsEvents {
    pub email: Option<String>,

    pub matrix_user_id: Option<String>,
    pub matrix_room_id: Option<String>,
    pub matrix_access_token: Option<String>,
    pub matrix_user_password: Option<String>,
    pub matrix_server_url: Option<String>,
    pub matrix_root_ca_path: Option<String>,
    #[serde(default)]
    pub matrix_danger_disable_tls_validation: bool,
    #[serde(default)]
    pub matrix_error_no_panic: bool,

    pub slack_webhook: Option<String>,

    #[serde(default = "ev_lv_warn")]
    pub notify_level_email: EventLevel,
    #[serde(default = "ev_lv_notice")]
    pub notify_level_matrix: EventLevel,
    #[serde(default = "ev_lv_notice")]
    pub notify_level_slack: EventLevel,
    #[serde(default = "ev_lv_info")]
    pub persist_level: EventLevel,
    #[serde(default = "u32_30")]
    pub cleanup_days: u32,

    #[serde(default = "ev_lv_info")]
    pub level_new_user: EventLevel,
    #[serde(default = "ev_lv_notice")]
    pub level_user_email_change: EventLevel,
    #[serde(default = "ev_lv_notice")]
    pub level_user_password_reset: EventLevel,
    #[serde(default = "ev_lv_notice")]
    pub level_rauthy_admin: EventLevel,
    #[serde(default = "ev_lv_notice")]
    pub level_rauthy_version: EventLevel,
    #[serde(default = "ev_lv_notice")]
    pub level_jwks_rotate: EventLevel,
    #[serde(default = "ev_lv_notice")]
    pub level_secrets_migrated: EventLevel,
    #[serde(default = "ev_lv_info")]
    pub level_rauthy_start: EventLevel,
    #[serde(default = "ev_lv_notice")]
    pub level_rauthy_healthy: EventLevel,
    #[serde(default = "ev_lv_crit")]
    pub level_rauthy_unhealthy: EventLevel,
    #[serde(default = "ev_lv_warn")]
    pub level_ip_blacklisted: EventLevel,
    #[serde(default = "ev_lv_crit")]
    pub level_failed_logins_25: EventLevel,
    #[serde(default = "ev_lv_crit")]
    pub level_failed_logins_20: EventLevel,
    #[serde(default = "ev_lv_warn")]
    pub level_failed_logins_15: EventLevel,
    #[serde(default = "ev_lv_warn")]
    pub level_failed_logins_10: EventLevel,
    #[serde(default = "ev_lv_notice")]
    pub level_failed_logins_7: EventLevel,
    #[serde(default = "ev_lv_info")]
    pub level_failed_login: EventLevel,

    #[serde(default)]
    pub disable_app_version_check: bool,
}

impl Default for VarsEvents {
    fn default() -> Self {
        Self {
            email: None,
            matrix_user_id: None,
            matrix_room_id: None,
            matrix_access_token: None,
            matrix_user_password: None,
            matrix_server_url: None,
            matrix_root_ca_path: None,
            matrix_danger_disable_tls_validation: false,
            matrix_error_no_panic: false,
            slack_webhook: None,
            notify_level_email: EventLevel::Warning,
            notify_level_matrix: EventLevel::Notice,
            notify_level_slack: EventLevel::Notice,
            persist_level: EventLevel::Info,
            cleanup_days: 30,
            level_new_user: EventLevel::Info,
            level_user_email_change: EventLevel::Notice,
            level_user_password_reset: EventLevel::Notice,
            level_rauthy_admin: EventLevel::Notice,
            level_rauthy_version: EventLevel::Notice,
            level_jwks_rotate: EventLevel::Notice,
            level_secrets_migrated: EventLevel::Notice,
            level_rauthy_start: EventLevel::Info,
            level_rauthy_healthy: EventLevel::Notice,
            level_rauthy_unhealthy: EventLevel::Critical,
            level_ip_blacklisted: EventLevel::Warning,
            level_failed_logins_25: EventLevel::Critical,
            level_failed_logins_20: EventLevel::Critical,
            level_failed_logins_15: EventLevel::Warning,
            level_failed_logins_10: EventLevel::Warning,
            level_failed_logins_7: EventLevel::Notice,
            level_failed_login: EventLevel::Info,

            disable_app_version_check: false,
        }
    }
}

#[derive(Debug, Deserialize)]
struct ConfigVarsFedCM {
    #[serde(default)]
    pub experimental_enable: bool,
    #[serde(default = "u32_2592000")]
    pub session_lifetime: u32,
    #[serde(default = "u32_259200")]
    pub session_timeout: u32,
}

#[derive(Debug, Deserialize)]
struct VarsHashing {
    #[serde(default = "u32_131072")]
    pub argon2_m_cost: u32,
    #[serde(default = "u32_4")]
    pub argon2_t_cost: u32,
    #[serde(default = "u32_8")]
    pub argon2_p_cost: u32,
    #[serde(default = "u32_2")]
    pub max_hash_threads: u32,
    #[serde(default = "u32_500")]
    pub hash_await_warn_time: u32,
    #[serde(default = "jwk_cron")]
    pub jwk_autorotate_cron: Cow<'static, str>,
}

impl Default for VarsHashing {
    fn default() -> Self {
        Self {
            argon2_m_cost: 131072,
            argon2_t_cost: 4,
            argon2_p_cost: 8,
            max_hash_threads: 2,
            hash_await_warn_time: 500,
            jwk_autorotate_cron: "0 30 3 1 * * *".into(),
        }
    }
}

fn bool_true() -> bool {
    true
}
fn u32_2() -> u32 {
    2
}
fn u32_4() -> u32 {
    4
}
fn u32_5() -> u32 {
    5
}
fn u32_8() -> u32 {
    8
}
fn u32_30() -> u32 {
    30
}
fn u32_60() -> u32 {
    60
}
fn u32_72() -> u32 {
    72
}
fn u32_100() -> u32 {
    100
}
fn u32_120() -> u32 {
    100
}
fn u32_300() -> u32 {
    300
}
fn u32_500() -> u32 {
    500
}
fn u32_900() -> u32 {
    900
}
fn u32_1800() -> u32 {
    1800
}
fn u32_3600() -> u32 {
    3600
}
fn u32_4096() -> u32 {
    4096
}
fn u32_131072() -> u32 {
    131072
}
fn u32_259200() -> u32 {
    259200
}
fn u32_2592000() -> u32 {
    2592000
}
fn ev_lv_info() -> EventLevel {
    EventLevel::Info
}
fn ev_lv_notice() -> EventLevel {
    EventLevel::Notice
}
fn ev_lv_warn() -> EventLevel {
    EventLevel::Warning
}
fn ev_lv_crit() -> EventLevel {
    EventLevel::Critical
}
fn jwk_cron() -> Cow<'static, str> {
    "0 30 3 1 * * *".into()
}

fn t_bool(map: &mut toml::Table, parent: &str, key: &str, env_overwrite: &str) -> Option<bool> {
    let Value::Boolean(b) = map.remove(key)? else {
        panic!("{}", err_t(key, parent, "bool"));
    };
    if env_overwrite.is_empty() {
        Some(b)
    } else {
        Some(
            env::var(env_overwrite)
                .as_deref()
                .map(|v| match v.parse::<bool>() {
                    Ok(b) => b,
                    Err(_) => {
                        panic!("{}", err_env(env_overwrite, "bool"));
                    }
                })
                .unwrap_or(b),
        )
    }
}

fn t_i64(map: &mut toml::Table, parent: &str, key: &str, env_overwrite: &str) -> Option<i64> {
    let Value::Integer(i) = map.remove(key)? else {
        panic!("{}", err_t(key, parent, "bool"));
    };
    if env_overwrite.is_empty() {
        Some(i)
    } else {
        Some(
            env::var(env_overwrite)
                .as_deref()
                .map(|v| match v.parse::<i64>() {
                    Ok(b) => b,
                    Err(_) => {
                        panic!("{}", err_env(env_overwrite, "Integer"));
                    }
                })
                .unwrap_or(i),
        )
    }
}

fn t_str(map: &mut toml::Table, parent: &str, key: &str, env_overwrite: &str) -> Option<String> {
    let Value::String(s) = map.remove(key)? else {
        panic!("{}", err_t(key, parent, "String"));
    };
    if env_overwrite.is_empty() {
        Some(s)
    } else {
        Some(env::var(env_overwrite).unwrap_or(s))
    }
}

fn t_table(map: &mut toml::Table, key: &str) -> Option<toml::Table> {
    let Value::Table(t) = map.remove(key)? else {
        panic!("Expected type `Table` for {}", key)
    };
    Some(t)
}

#[inline]
fn err_env(var_name: &str, typ: &str) -> String {
    format!("Cannot parse {} as `{}`", var_name, typ)
}

#[inline]
fn err_t(key: &str, parent: &str, typ: &str) -> String {
    let sep = if parent.is_empty() { "" } else { "." };
    format!("Expected type `{}` for {}{}{}", typ, parent, sep, key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs;

    #[tokio::test]
    async fn test_config_vars_build() {
        let vars = Vars::load("../../config.toml").await;
        //
        // let s = fs::read_to_string("../../config.toml").unwrap();
        //
        // let mut table_root = s.parse::<toml::Table>().unwrap();
        //
        // let mut table = t_table(&mut table_root, "dev").unwrap();
        // let mut vars = VarsDev {
        //     dev_mode: false,
        //     dpop_http: false,
        //     insecure_cookie: false,
        //     provider_callback_url: None,
        // };
        //
        // let vars_dev = VarsDev {
        //     dev_mode: t_bool(&mut table, "dev", "dev_mode", "DEV_MODE").unwrap(),
        //     dpop_http: t_bool(&mut table, "dev", "dpop_http", "").unwrap(),
        //     insecure_cookie: t_bool(&mut table, "dev", "insecure_cookie", "INSECURE_COOKIE")
        //         .unwrap(),
        //     provider_callback_url: t_str(&mut table, "dev", "provider_callback_url", ""),
        // };
        println!("{:?}", vars);

        let dev_mode = assert_eq!(1, 2);
    }
}
