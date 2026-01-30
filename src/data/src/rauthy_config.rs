use crate::ListenScheme;
use crate::email::mailer::{EMail, SmtpConnMode};
use crate::events::event::{Event, EventLevel};
use crate::events::listener::EventRouterMsg;
use crate::vault_config::VaultConfig;
use cryptr::EncKeys;
use hiqlite::NodeConfig;
use rauthy_common::constants::CookieMode;
use rauthy_common::logging::LogLevelAccess;
use rauthy_common::regex::{RE_LINUX_USERNAME, RE_PREFERRED_USERNAME};
use regex::Regex;
use serde::Serialize;
use spow::pow::Pow;
use std::borrow::Cow;
use std::env;
use std::error::Error;
use std::str::FromStr;
use std::sync::OnceLock;
use tokio::fs;
use tokio::sync::mpsc;
use toml::Value;
use tracing::{debug, info, warn};
use utoipa::ToSchema;
use webauthn_rs::Webauthn;

static CONFIG: OnceLock<RauthyConfig> = OnceLock::new();

#[derive(Debug)]
pub struct RauthyConfig {
    pub argon2_params: argon2::Params,
    pub issuer: String,
    pub is_primary_node: bool,
    pub is_ha_cluster: bool,
    pub listen_scheme: ListenScheme,
    pub log_level_access: LogLevelAccess,
    pub provider_callback_uri: String,
    pub provider_callback_uri_encoded: String,
    pub pub_url_with_scheme: String,
    pub tx_email: mpsc::Sender<EMail>,
    pub tx_events: flume::Sender<Event>,
    pub tx_events_router: flume::Sender<EventRouterMsg>,
    pub webauthn: Webauthn,
    pub vars: Vars,
}

impl RauthyConfig {
    pub async fn build(
        config_file: &str,
        tx_email: mpsc::Sender<EMail>,
        tx_events: flume::Sender<Event>,
        tx_events_router: flume::Sender<EventRouterMsg>,
    ) -> Result<(Self, hiqlite::NodeConfig), Box<dyn Error>> {
        let (vars, node_config) = Vars::load(config_file).await;
        vars.validate();
        if let Err(err) = node_config.is_valid() {
            panic!("Invalid `[cluster]` config: {err}");
        }

        let listen_scheme = match vars.server.scheme.as_ref() {
            "http" => {
                info!(
                    "Listen URL: http://{}:{}",
                    vars.server.listen_address, vars.server.port_http
                );
                ListenScheme::Http
            }
            "https" => {
                info!(
                    "Listen URL: https://{}:{}",
                    vars.server.listen_address, vars.server.port_https
                );
                ListenScheme::Https
            }
            "http_https" => {
                let port = format!("{{{}|{}}}", vars.server.port_http, vars.server.port_https);
                info!(
                    "Listen URL: {{http|https}}://{}:{port}",
                    vars.server.listen_address
                );
                ListenScheme::HttpHttps
            }
            #[cfg(not(target_os = "windows"))]
            "unix_http" => {
                info!("Listen URL: unix+http:{}", vars.server.listen_address);
                ListenScheme::UnixHttp
            }
            #[cfg(not(target_os = "windows"))]
            "unix_https" => {
                info!("Listen URL: unix+https:{}", vars.server.listen_address);
                ListenScheme::UnixHttps
            }
            _ => panic!(
                "'LISTEN_SCHEME' environment variable not correctly set (http | https | http_https)"
            ),
        };

        let argon2_params = argon2::Params::new(
            vars.hashing.argon2_m_cost,
            vars.hashing.argon2_t_cost,
            vars.hashing.argon2_p_cost,
            None,
        )
        .expect("Unable to build Argon2id params, check the values in the [hashing] section");

        #[cfg(target_os = "windows")]
        let is_https = matches!(listen_scheme, ListenScheme::HttpHttps | ListenScheme::Https)
            || vars.server.proxy_mode;
        #[cfg(not(target_os = "windows"))]
        let is_https = matches!(
            listen_scheme,
            ListenScheme::HttpHttps | ListenScheme::Https | ListenScheme::UnixHttps
        ) || vars.server.proxy_mode;
        let issuer_scheme = if is_https { "https" } else { "http" };
        let issuer = format!("{issuer_scheme}://{}/auth/v1", vars.server.pub_url);

        let Ok(log_level_access) = LogLevelAccess::from_str(&vars.logging.level_access) else {
            panic!(
                "Invalid value for `logging.level_access`: {}",
                vars.logging.level_access
            );
        };

        let pub_url = &vars.server.pub_url;
        let provider_callback_uri = {
            #[cfg(target_os = "windows")]
            let scheme = if listen_scheme == ListenScheme::Http && !vars.server.proxy_mode {
                "http"
            } else {
                "https"
            };

            #[cfg(not(target_os = "windows"))]
            let scheme = if (listen_scheme == ListenScheme::Http && !vars.server.proxy_mode)
                || listen_scheme == ListenScheme::UnixHttp
            {
                "http"
            } else {
                "https"
            };

            let pub_url = if vars.dev.dev_mode {
                vars.dev.provider_callback_url.as_deref().unwrap_or(pub_url)
            } else {
                pub_url
            };
            format!("{scheme}://{pub_url}/auth/v1/providers/callback")
        };
        let provider_callback_uri_encoded = provider_callback_uri
            .replace(':', "%3A")
            .replace('/', "%2F");

        #[cfg(target_os = "windows")]
        let pub_url_with_scheme = {
            let scheme = if listen_scheme == ListenScheme::Http && !vars.server.proxy_mode {
                "http"
            } else {
                "https"
            };
            format!("{}://{}", scheme, pub_url)
        };

        #[cfg(not(target_os = "windows"))]
        let pub_url_with_scheme = {
            let scheme = if (listen_scheme == ListenScheme::Http && !vars.server.proxy_mode)
                || listen_scheme == ListenScheme::UnixHttp
            {
                "http"
            } else {
                "https"
            };
            format!("{scheme}://{pub_url}")
        };

        let rp_origin = webauthn_rs::prelude::Url::parse(&vars.webauthn.rp_origin)
            .expect("Cannot parse `webauthn.rp_origin` to URL");
        let builder = webauthn_rs::WebauthnBuilder::new(&vars.webauthn.rp_id, &rp_origin)
            .expect("Invalid `webauthn.rp_id`")
            // Set a "nice" relying party name. Has no security properties - may be changed in the future.
            .rp_name(&vars.webauthn.rp_name);
        let webauthn = builder.build().expect("Invalid `webauthn.rp_name`");

        let slf = Self {
            argon2_params,
            issuer,
            is_primary_node: node_config.node_id == 1 || node_config.nodes.len() == 1,
            is_ha_cluster: node_config.nodes.len() > 1,
            listen_scheme,
            log_level_access,
            provider_callback_uri,
            provider_callback_uri_encoded,
            pub_url_with_scheme,
            tx_email,
            tx_events,
            tx_events_router,
            webauthn,
            vars,
        };

        Ok((slf, node_config))
    }

    pub fn debug_logs() {
        let slf = Self::get();

        // we don't to log everything, just the important values
        info!("Public URL: {}", slf.vars.server.pub_url);
        debug!("Proxy Mode: {}", slf.vars.server.proxy_mode);
        debug!("Trusted Proxies: {:?}", slf.vars.server.trusted_proxies);
        info!("Issuer: {}", slf.issuer);
        debug!("HA Deployment: {}", slf.is_ha_cluster);
        debug!(
            "Argon2id Params: m_code: {}, t_cost: {}, p_cost: {}",
            slf.argon2_params.m_cost(),
            slf.argon2_params.t_cost(),
            slf.argon2_params.p_cost()
        );

        if slf.vars.dev.dev_mode {
            warn!("You started with DEV Mode enabled. Do NOT use this in production!");
        }
    }

    pub fn init_static(self) {
        let Ok(enc_keys) = EncKeys::try_parse(
            self.vars.encryption.key_active.clone(),
            self.vars.encryption.keys.clone(),
        ) else {
            panic!(
                r#"The `ENC_KEYS`are not correctly set up. Please take a look at the documentation:
    https://sebadob.github.io/rauthy/config/encryption.html"#
            );
        };

        Pow::init_bytes(enc_keys.get_key(&enc_keys.enc_key_active).unwrap());
        enc_keys.init().unwrap();

        CONFIG.set(self).unwrap();
    }

    #[inline(always)]
    pub fn get() -> &'static Self {
        CONFIG.get().unwrap()
    }
}

#[derive(Debug)]
pub struct Vars {
    pub dev: VarsDev,
    pub access: VarsAccess,
    pub auth_headers: VarsAuthHeaders,
    pub atproto: VarsAtproto,
    pub backchannel_logout: VarsBackchannelLogout,
    pub bootstrap: VarsBootstrap,
    pub database: VarsDatabase,
    pub device_grant: VarsDeviceGrant,
    pub dpop: VarsDpop,
    pub dynamic_clients: VarsDynamicClients,
    pub email: VarsEmail,
    pub encryption: VarsEncryption,
    pub ephemeral_clients: VarsEphemeralClients,
    pub events: VarsEvents,
    pub fedcm: ConfigVarsFedCM,
    pub geo: ConfigVarsGeo,
    pub hashing: VarsHashing,
    pub http_client: VarsHttpClient,
    pub i18n: VarsI18n,
    pub lifetimes: VarsLifetimes,
    pub logging: VarsLogging,
    pub mfa: VarsMfa,
    pub pam: VarsPam,
    pub pow: VarsPow,
    pub scim: VarsScim,
    pub server: VarsServer,
    pub suspicious_requests: VarsSuspiciousRequests,
    pub templates: VarsTemplates,
    pub tls: VarsTls,
    pub tos: VarsToS,
    pub user_delete: VarsUserDelete,
    pub user_pictures: VarsUserPictures,
    pub user_registration: VarsUserRegistration,
    pub user_values: VarsUserValuesConfig,
    pub webauthn: VarsWebauthn,
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
                client_credentials_map_sub: false,
                token_len_limit: 4096,
                token_revoke_on_logout: false,
                token_revoke_device_tokens: false,
                whoami_headers: false,
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
                danger_allow_http: false,
                danger_allow_insecure: false,
                token_lifetime: 30,
                allow_clock_skew: 5,
                allowed_token_lifetime: 120,
            },
            bootstrap: VarsBootstrap {
                admin_email: "admin@localhost".to_string(),
                password_plain: None,
                pasword_argon2id: None,
                api_key: None,
                api_key_secret: None,
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
                migrate_pg_port: 5432,
                migrate_pg_user: None,
                migrate_pg_password: None,
                migrate_pg_db_name: "rauthy".into(),
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
                rauthy_admin_email: None,
                sub_prefix: "Rauthy IAM".into(),
                smtp_url: None,
                smtp_port: None,
                smtp_username: None,
                smtp_password: None,
                smtp_from: "Rauthy <rauthy@localhost>".into(),
                connect_retries: 3,
                jobs: VarsEmailJobs {
                    orphaned_seconds: 300,
                    scheduler_interval_seconds: 300,
                    batch_size: 3,
                    batch_delay_ms: 2000,
                },
                smtp_conn_mode: SmtpConnMode::Default,
                xoauth_url: None,
                xoauth_client_id: None,
                xoauth_client_secret: None,
                xoauth_scope: None,
                microsoft_graph_uri: None,
                root_ca: None,
                starttls_only: false,
                danger_insecure: false,
                tz_fmt: VarsEmailTzFmt {
                    de: "%d.%m.%Y %T (%Z)".into(),
                    en: "%m/%d/%Y %T (%Z)".into(),
                    ko: "%Y-%m-%d %T (%Z)".into(),
                    no: "%d.%m.%Y %T (%Z)".into(),
                    uk: "%d.%m.%Y %T (%Z)".into(),
                    zhhans: "%d-%m-%Y %T (%Z)".into(),
                    tz_fallback: "UTC".into(),
                },
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
                matrix_server_url: "https://matrix.org".into(),
                matrix_root_ca_path: None,
                matrix_danger_disable_tls_validation: false,
                matrix_error_no_panic: false,
                slack_webhook: None,
                notify_level_email: EventLevel::Warning,
                notify_level_matrix: EventLevel::Notice,
                notify_level_slack: EventLevel::Notice,
                persist_level: EventLevel::Info,
                cleanup_days: 31,
                generate_token_issued: true,
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
                level_backchannel_logout_failed: EventLevel::Critical,
                level_force_logout: EventLevel::Notice,
                level_user_login_revoke: EventLevel::Warning,
                level_scim_task_failed: EventLevel::Critical,
                level_suspicious_request: EventLevel::Notice,
                level_new_login_location: EventLevel::Notice,
                level_token_issued: EventLevel::Info,
                level_failed_logins_25: EventLevel::Critical,
                level_failed_logins_20: EventLevel::Critical,
                level_failed_logins_15: EventLevel::Warning,
                level_failed_logins_10: EventLevel::Warning,
                level_failed_logins_7: EventLevel::Notice,
                level_failed_login: EventLevel::Info,
                disable_app_version_check: false,
            },
            fedcm: ConfigVarsFedCM {
                experimental_enable: false,
                session_lifetime: 2592000,
                session_timeout: 259200,
            },
            geo: ConfigVarsGeo {
                block_is_whitelist: None,
                block_unknown: false,
                country_header: None,
                country_list: Vec::default(),
                maxmind_account_id: None,
                maxmind_license_key: None,
                maxmind_db_dir: "data".into(),
                maxmind_db_type: "GeoLite2-Country".into(),
                maxmind_update_cron: "0 0 5 * * * *".into(),
            },
            hashing: VarsHashing {
                argon2_m_cost: 131072,
                argon2_t_cost: 4,
                argon2_p_cost: 8,
                max_hash_threads: 2,
                hash_await_warn_time: 500,
            },
            http_client: VarsHttpClient {
                connect_timeout: 10,
                request_timeout: 10,
                min_tls: "1.3".into(),
                idle_timeout: 900,
                danger_unencrypted: false,
                danger_insecure: false,
                root_ca_bundle: None,
            },
            i18n: VarsI18n {
                filter_lang_common: vec![
                    "en".into(),
                    "de".into(),
                    "ko".into(),
                    "nb".into(),
                    "uk".into(),
                    "zhhans".into(),
                ],
                filter_lang_admin: vec![
                    "en".into(),
                    "de".into(),
                    "ko".into(),
                    "nb".into(),
                    "uk".into(),
                ],
            },
            lifetimes: VarsLifetimes {
                refresh_token_grace_time: 5,
                refresh_token_lifetime: 48,
                session_lifetime: 14400,
                session_renew_mfa: false,
                session_timeout: 5400,
                magic_link_pwd_reset: 30,
                magic_link_pwd_first: 4320,
                jwk_autorotate_cron: "0 30 3 1 * * *".into(),
            },
            logging: VarsLogging {
                level: "info".into(),
                level_database: "info".into(),
                level_access: "modifying".into(),
                log_fmt: "text".into(),
            },
            mfa: VarsMfa {
                admin_force_mfa: true,
            },
            pam: VarsPam {
                remote_password_len: 24,
                remote_password_ttl: 120,
                authorized_keys: VarsPamAuthorizedKeys {
                    authorized_keys_enable: true,
                    auth_required: true,
                    blacklist_used_keys: true,
                    blacklist_cleanup_days: 730,
                    include_comments: true,
                    forced_key_expiry_days: 365,
                },
            },
            pow: VarsPow {
                difficulty: 19,
                exp: 30,
            },
            scim: VarsScim {
                sync_delete_groups: false,
                sync_delete_users: false,
                retry_count: 100,
            },
            server: VarsServer {
                listen_address: "0.0.0.0".into(),
                port_http: 8080,
                port_https: 8443,
                scheme: "http_https".into(),
                pub_url: String::default(),
                http_workers: 0,
                proxy_mode: false,
                trusted_proxies: Vec::default(),
                additional_allowed_origin_schemes: Vec::default(),
                metrics_enable: false,
                metrics_addr: "0.0.0.0".into(),
                metrics_port: 9090,
                swagger_ui_enable: false,
                swagger_ui_public: false,
                see_keep_alive: 30,
                ssp_threshold: 1000,
            },
            suspicious_requests: VarsSuspiciousRequests {
                blacklist: 1440,
                log: false,
            },
            templates: VarsTemplates {
                password_new_de: VarsTemplate {
                    subject: "Neues Passwort".into(),
                    header: "Neues Passwort für".into(),
                    text: None,
                    click_link:
                        "Klicken Sie auf den unten stehenden Link um ein neues Passwort zu setzen."
                            .into(),
                    validity: "Dieser Link ist aus Sicherheitsgründen nur für kurze Zeit gültig."
                        .into(),
                    expires: "Link gültig bis:".into(),
                    button: "Passwort Setzen".into(),
                    footer: None,
                    button_text_request_new: None,
                },
                password_new_en: VarsTemplate {
                    subject: "New Password".into(),
                    header: "New password for".into(),
                    text: None,
                    click_link: "Click the link below to get forwarded to the password form."
                        .into(),
                    validity:
                        "This link is only valid for a short period of time for security reasons."
                            .into(),
                    expires: "Link expires:".into(),
                    button: "Set Password".into(),
                    footer: None,
                    button_text_request_new: None,
                },
                password_new_ko: VarsTemplate {
                    subject: "新密码".into(),
                    header: "新密码".into(),
                    text: None,
                    click_link: "点击下方链接以打开密码设置表单。".into(),
                    validity: "出于安全考虑，此链接仅在短时间内有效。".into(),
                    expires: "链接过期时间：".into(),
                    button: "设置密码".into(),
                    footer: None,
                    button_text_request_new: None,
                },
                password_new_nb: VarsTemplate {
                    subject: "Ny passord".into(),
                    header: "Ny passord for".into(),
                    text: None,
                    click_link: "Klikk på lenken under for å sette et nytt passord.".into(),
                    validity: "Denne lenken er kun gyldig i en kort periode av sikkerhetsgrunner."
                        .into(),
                    expires: "Lenken gyldig til:".into(),
                    button: "Sett passord".into(),
                    footer: None,
                    button_text_request_new: None,
                },
                password_new_uk: VarsTemplate {
                    subject: "Новий пароль".into(),
                    header: "Новий пароль для".into(),
                    text: None,
                    click_link:
                        "Натисніть посилання нижче, щоб перейти до форми встановлення пароля."
                            .into(),
                    validity:
                        "З міркувань безпеки це посилання дійсне лише протягом короткого часу."
                            .into(),
                    expires: "Посилання дійсне до:".into(),
                    button: "Встановити пароль".into(),
                    footer: None,
                    button_text_request_new: None,
                },
                password_new_zhhans: VarsTemplate {
                    subject: "새 비밀번호".into(),
                    header: "새 비밀번호를 설정해 주세요:".into(),
                    text: None,
                    click_link: "비밀번호 입력창으로 이동하려면, 아래의 링크를 클릭해 주세요."
                        .into(),
                    validity: "이 링크는 보안상의 이유로 짧은 시간 동안에만 유효합니다.".into(),
                    expires: "링크 만료일:".into(),
                    button: "비밀번호 설정".into(),
                    footer: None,
                    button_text_request_new: None,
                },
                password_reset_de: VarsTemplate {
                    subject: "Passwort Reset angefordert".into(),
                    header: "Passwort Reset angefordert für".into(),
                    text: None,
                    click_link: "Klicken Sie auf den unten stehenden Link für den Passwort Reset."
                        .into(),
                    validity: "Dieser Link ist aus Sicherheitsgründen nur für kurze Zeit gültig."
                        .into(),
                    expires: "Link gültig bis:".into(),
                    button: "Passwort Zurücksetzen".into(),
                    footer: Some(
                        "Sollte der Link abgelaufen sein, kann ein neuer angefordert werden."
                            .into(),
                    ),
                    button_text_request_new: Some("Neuen Link Anfordern".into()),
                },
                password_reset_en: VarsTemplate {
                    subject: "Password Reset Request".into(),
                    header: "Password reset request for".into(),
                    text: None,
                    click_link:
                        "Click the link below to get forwarded to the password request form.".into(),
                    validity:
                        "This link is only valid for a short period of time for security reasons."
                            .into(),
                    expires: "Link expires:".into(),
                    button: "Reset Password".into(),
                    footer: Some("If this link has expired, you can request a new one.".into()),
                    button_text_request_new: Some("Request New Link".into()),
                },
                password_reset_ko: VarsTemplate {
                    subject: "密码重置请求".into(),
                    header: "密码重置请求：".into(),
                    text: None,
                    click_link: "点击下方链接以打开密码重置表单。".into(),
                    validity: "出于安全考虑，此链接仅在短时间内有效。".into(),
                    expires: "链接过期时间".into(),
                    button: "重置密码".into(),
                    footer: Some("If this link has expired, you can request a new one.".into()),
                    button_text_request_new: Some("Request New Link".into()),
                },
                password_reset_nb: VarsTemplate {
                    subject: "Passordtilbakestilling etterspurt".into(),
                    header: "Passordtilbakestilling etterspurt for".into(),
                    text: None,
                    click_link: "Klikk på lenken under for å tilbakestille passordet.".into(),
                    validity: "Denne lenken er kun gyldig i en kort periode av sikkerhetsgrunner."
                        .into(),
                    expires: "Lenken utløper:".into(),
                    button: "Tilbakestill passord".into(),
                    footer: Some("Hvis lenken har utløpt, kan du be om en ny.".into()),
                    button_text_request_new: Some("Be om ny lenke".into()),
                },
                password_reset_uk: VarsTemplate {
                    subject: "Запит на скидання пароля".into(),
                    header: "Запит на скидання пароля для".into(),
                    text: None,
                    click_link: "Натисніть посилання нижче, щоб перейти до форми скидання пароля."
                        .into(),
                    validity:
                        "З міркувань безпеки це посилання дійсне лише протягом короткого часу."
                            .into(),
                    expires: "Посилання дійсне до:".into(),
                    button: "Скинути пароль".into(),
                    footer: Some(
                        "Якщо термін дії посилання минув, ви можете запросити нове.".into(),
                    ),
                    button_text_request_new: Some("Запросити нове посилання".into()),
                },
                password_reset_zhhans: VarsTemplate {
                    subject: "비밀번호 초기화 요청".into(),
                    header: "비밀번호 초기화 요청:".into(),
                    text: None,
                    click_link:
                        "비밀번호 초기화 요청 창으로 이동하려면, 아래의 링크를 클릭해 주세요."
                            .into(),
                    validity: "이 링크는 보안상의 이유로 짧은 시간 동안에만 유효합니다.".into(),
                    expires: "링크 만료일:".into(),
                    button: "비밀번호 초기화".into(),
                    footer: Some("If this link has expired, you can request a new one.".into()),
                    button_text_request_new: Some("Request New Link".into()),
                },
            },
            tls: VarsTls {
                cert_path: None,
                key_path: None,
                generate_self_signed: false,
            },
            tos: VarsToS {
                accept_timeout: 900,
            },
            user_delete: VarsUserDelete {
                enable_self_delete: false,
            },
            user_pictures: VarsUserPictures {
                storage_type: "db".into(),
                path: "./pictures".into(),
                s3_url: None,
                bucket: None,
                region: None,
                s3_key: None,
                s3_secret: None,
                s3_path_style: true,
                upload_limit_mb: 10,
                public: false,
            },
            user_registration: VarsUserRegistration {
                enable: false,
                domain_restriction: None,
                domain_blacklist: Vec::default(),
                allow_open_redirect: false,
            },
            user_values: VarsUserValuesConfig {
                given_name: UserValueConfigValue::Required,
                family_name: UserValueConfigValue::Optional,
                birthdate: UserValueConfigValue::Optional,
                street: UserValueConfigValue::Optional,
                zip: UserValueConfigValue::Optional,
                city: UserValueConfigValue::Optional,
                country: UserValueConfigValue::Optional,
                phone: UserValueConfigValue::Optional,
                tz: UserValueConfigValue::Optional,
                revalidate_during_login: false,
                preferred_username: VarsUserPreferredUsername {
                    preferred_username: UserValueConfigValue::Optional,
                    immutable: true,
                    blacklist: vec!["admin".into(), "administrator".into(), "root".into()],
                    pattern_html: "^[a-z][a-z0-9_\\-]{1,61}$".into(),
                    email_fallback: true,
                },
            },
            webauthn: VarsWebauthn {
                rp_id: String::default(),
                rp_origin: String::default(),
                rp_name: "Rauthy IAM".into(),
                req_exp: 60,
                data_exp: 90,
                renew_exp: 2160,
                force_uv: false,
                no_password_exp: true,
            },
            atproto: VarsAtproto { enable: false },
        }
    }
}

impl Vars {
    async fn load(path_config: &str) -> (Self, hiqlite::NodeConfig) {
        let use_vault_config = env::var("USE_VAULT_CONFIG")
            .unwrap_or_else(|_| "false".to_string())
            .parse::<bool>()
            .expect("Cannot parse USE_VAULT_CONFIG as bool");
        let slf = Self::default();

        let config = match use_vault_config {
            true => match VaultConfig::load_config().await {
                Ok(config) => config,
                Err(err) => {
                    panic!("Cannot read config from Vault. {err}");
                }
            },
            _ => {
                let Ok(config) = fs::read_to_string(path_config).await else {
                    panic!("Cannot read config file from {path_config}");
                };
                config
            }
        };

        Self::parse(slf, config).await
    }

    async fn parse(mut slf: Vars, config: String) -> (Self, hiqlite::NodeConfig) {
        // Note: these inner parsers are very verbose, but they allow the upfront memory allocation
        // and memory fragmentation, after the quite big toml has been freed and the config stays
        // in static memory.
        // It will also be easier in the future to maybe add other config sources into the mix
        // and use something else as default very easily, like e.g. a config fetched from a Vault.

        let mut table = config
            .parse::<toml::Table>()
            .expect("Cannot parse TOML file");

        slf.parse_dev(&mut table);
        slf.parse_atproto(&mut table);
        slf.parse_access(&mut table);
        slf.parse_auth_headers(&mut table);
        slf.parse_backchannel_logout(&mut table);
        slf.parse_bootstrap(&mut table);
        slf.parse_database(&mut table);
        slf.parse_device_grant(&mut table);
        slf.parse_dpop(&mut table);
        slf.parse_dynamic_clients(&mut table);
        slf.parse_email(&mut table);
        slf.parse_encryption(&mut table);
        slf.parse_ephemeral_clients(&mut table);
        slf.parse_events(&mut table);
        slf.parse_fedcm(&mut table);
        slf.parse_geo(&mut table);
        slf.parse_hashing(&mut table);
        slf.parse_http_client(&mut table);
        slf.parse_i18n(&mut table);
        slf.parse_lifetimes(&mut table);
        slf.parse_logging(&mut table);
        slf.parse_mfa(&mut table);
        slf.parse_pam(&mut table);
        slf.parse_pow(&mut table);
        slf.parse_scim(&mut table);
        slf.parse_server(&mut table);
        slf.parse_suspicious_requests(&mut table);
        slf.parse_templates(&mut table);
        slf.parse_tls(&mut table);
        slf.parse_tos(&mut table);
        slf.parse_user_delete(&mut table);
        slf.parse_user_pictures(&mut table);
        slf.parse_user_registration(&mut table);
        slf.parse_user_values(&mut table);
        slf.parse_webauthn(&mut table);

        let node_config = slf.parse_hiqlite_config(&mut table).await;

        (slf, node_config)
    }

    fn parse_dev(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "dev");

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

    fn parse_atproto(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "atproto");

        if let Some(v) = t_bool(&mut table, "atproto", "enable", "ATPROTO_ENABLE") {
            self.atproto.enable = v;
        }
    }

    fn parse_access(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "access");

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
        if let Some(v) = t_bool(
            &mut table,
            "access",
            "client_credentials_map_sub",
            "CLIENT_CREDENTIALS_MAP_SUB",
        ) {
            self.access.client_credentials_map_sub = v;
        }
        if let Some(v) = t_u32(&mut table, "access", "token_len_limit", "TOKEN_LEN_LIMIT") {
            self.access.token_len_limit = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "access",
            "token_revoke_on_logout",
            "TOKEN_REVOKE_ON_LOGOUT",
        ) {
            self.access.token_revoke_on_logout = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "access",
            "token_revoke_device_tokens",
            "TOKEN_REVOKE_DEVICE_TOKENS",
        ) {
            self.access.token_revoke_device_tokens = v;
        }
        if let Some(v) = t_bool(&mut table, "access", "whoami_headers", "WHOAMI_HEADERS") {
            self.access.whoami_headers = v;
        }
    }

    fn parse_auth_headers(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "auth_headers");

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

    fn parse_backchannel_logout(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "backchannel_logout");

        if let Some(v) = t_u16(
            &mut table,
            "backchannel_logout",
            "retry_count",
            "BACKCHANNEL_LOGOUT_RETRY_COUNT",
        ) {
            self.backchannel_logout.retry_count = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "backchannel_logout",
            "danger_allow_http",
            "BACKCHANNEL_DANGER_ALLOW_HTTP",
        ) {
            self.backchannel_logout.danger_allow_http = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "backchannel_logout",
            "danger_allow_insecure",
            "BACKCHANNEL_DANGER_ALLOW_INSECURE",
        ) {
            self.backchannel_logout.danger_allow_insecure = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "backchannel_logout",
            "token_lifetime",
            "LOGOUT_TOKEN_LIFETIME",
        ) {
            self.backchannel_logout.token_lifetime = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "backchannel_logout",
            "allow_clock_skew",
            "LOGOUT_TOKEN_ALLOW_CLOCK_SKEW",
        ) {
            self.backchannel_logout.allow_clock_skew = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "backchannel_logout",
            "allowed_token_lifetime",
            "LOGOUT_TOKEN_ALLOWED_LIFETIME",
        ) {
            self.backchannel_logout.allowed_token_lifetime = v;
        }
    }

    fn parse_bootstrap(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "bootstrap");

        if let Some(v) = t_str(
            &mut table,
            "bootstrap",
            "admin_email",
            "BOOTSTRAP_ADMIN_EMAIL",
        ) {
            self.bootstrap.admin_email = v;
        }
        if let Some(v) = t_str(
            &mut table,
            "bootstrap",
            "password_plain",
            "BOOTSTRAP_ADMIN_PASSWORD_PLAIN",
        ) {
            self.bootstrap.password_plain = Some(v);
        }
        if let Some(v) = t_str(
            &mut table,
            "bootstrap",
            "pasword_argon2id",
            "BOOTSTRAP_ADMIN_PASSWORD_ARGON2ID",
        ) {
            self.bootstrap.pasword_argon2id = Some(v);
        }
        if let Some(v) = t_str(&mut table, "bootstrap", "api_key", "BOOTSTRAP_API_KEY") {
            self.bootstrap.api_key = Some(v);
        }
        if let Some(v) = t_str(
            &mut table,
            "bootstrap",
            "api_key_secret",
            "BOOTSTRAP_API_KEY_SECRET",
        ) {
            self.bootstrap.api_key_secret = Some(v);
        }
    }

    fn parse_database(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "database");

        if let Some(v) = t_bool(&mut table, "database", "hiqlite", "HIQLITE") {
            self.database.hiqlite = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "database",
            "health_check_delay_secs",
            "HEALTH_CHECK_DELAY_SECS",
        ) {
            self.database.health_check_delay_secs = v;
        }

        if let Some(v) = t_str(&mut table, "database", "pg_host", "PG_HOST") {
            self.database.pg_host = Some(v);
        }
        if let Some(v) = t_u16(&mut table, "database", "pg_port", "PG_PORT") {
            self.database.pg_port = v;
        }
        if let Some(v) = t_str(&mut table, "database", "pg_user", "PG_USER") {
            self.database.pg_user = Some(v);
        }
        if let Some(v) = t_str(&mut table, "database", "pg_password", "PG_PASSWORD") {
            self.database.pg_password = Some(v);
        }
        if let Some(v) = t_str(&mut table, "database", "pg_db_name", "PG_DB_NAME") {
            self.database.pg_db_name = v.into();
        }
        if let Some(v) = t_bool(
            &mut table,
            "database",
            "pg_tls_no_verify",
            "PG_TLS_NO_VERIFY",
        ) {
            self.database.pg_tls_no_verify = v;
        }
        if let Some(v) = t_u16(&mut table, "database", "pg_max_conn", "PG_MAX_CONN") {
            self.database.pg_max_conn = v;
        }

        if let Some(v) = t_str(&mut table, "database", "migrate_pg_host", "MIGRATE_PG_HOST") {
            self.database.migrate_pg_host = Some(v);
        }
        if let Some(v) = t_u16(&mut table, "database", "migrate_pg_port", "MIGRATE_PG_PORT") {
            self.database.migrate_pg_port = v;
        }
        if let Some(v) = t_str(&mut table, "database", "migrate_pg_user", "MIGRATE_PG_USER") {
            self.database.migrate_pg_user = Some(v);
        }
        if let Some(v) = t_str(
            &mut table,
            "database",
            "migrate_pg_password",
            "MIGRATE_PG_PASSWORD",
        ) {
            self.database.migrate_pg_password = Some(v);
        }
        if let Some(v) = t_str(
            &mut table,
            "database",
            "migrate_pg_db_name",
            "MIGRATE_PG_DB_NAME",
        ) {
            self.database.migrate_pg_db_name = v.into();
        }

        if let Some(v) = t_u32(
            &mut table,
            "database",
            "sched_user_exp_mins",
            "SCHED_USER_EXP_MINS",
        ) {
            self.database.sched_user_exp_mins = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "database",
            "sched_user_exp_delete_mins",
            "SCHED_USER_EXP_DELETE_MINS",
        ) {
            self.database.sched_user_exp_delete_mins = Some(v);
        }
    }

    fn parse_device_grant(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "device_grant");

        if let Some(v) = t_u32(
            &mut table,
            "device_grant",
            "code_lifetime",
            "DEVICE_GRANT_CODE_LIFETIME",
        ) {
            self.device_grant.code_lifetime = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "device_grant",
            "user_code_length",
            "DEVICE_GRANT_USER_CODE_LENGTH",
        ) {
            self.device_grant.user_code_length = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "device_grant",
            "rate_limit",
            "DEVICE_GRANT_RATE_LIMIT",
        ) {
            self.device_grant.rate_limit = Some(v);
        }
        if let Some(v) = t_u32(
            &mut table,
            "device_grant",
            "poll_interval",
            "DEVICE_GRANT_POLL_INTERVAL",
        ) {
            self.device_grant.poll_interval = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "device_grant",
            "refresh_token_lifetime",
            "DEVICE_GRANT_REFRESH_TOKEN_LIFETIME",
        ) {
            self.device_grant.refresh_token_lifetime = v;
        }
    }

    fn parse_dpop(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "dpop");

        if let Some(v) = t_bool(&mut table, "dpop", "force_nonce", "DPOP_FORCE_NONCE") {
            self.dpop.force_nonce = v;
        }
        if let Some(v) = t_u32(&mut table, "dpop", "nonce_exp", "DPOP_NONCE_EXP") {
            self.dpop.nonce_exp = v;
        }
    }

    fn parse_dynamic_clients(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "dynamic_clients");

        if let Some(v) = t_bool(
            &mut table,
            "dynamic_clients",
            "enable",
            "ENABLE_DYN_CLIENT_REG",
        ) {
            self.dynamic_clients.enable = v;
        }
        if let Some(v) = t_str(
            &mut table,
            "dynamic_clients",
            "reg_token",
            "DYN_CLIENT_REG_TOKEN",
        ) {
            self.dynamic_clients.reg_token = Some(v);
        }
        if let Some(v) = t_u32(
            &mut table,
            "dynamic_clients",
            "default_token_lifetime",
            "DYN_CLIENT_DEFAULT_TOKEN_LIFETIME",
        ) {
            self.dynamic_clients.default_token_lifetime = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "dynamic_clients",
            "secret_auto_rotate",
            "DYN_CLIENT_SECRET_AUTO_ROTATE",
        ) {
            self.dynamic_clients.secret_auto_rotate = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "dynamic_clients",
            "cleanup_interval",
            "DYN_CLIENT_CLEANUP_INTERVAL",
        ) {
            self.dynamic_clients.cleanup_interval = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "dynamic_clients",
            "cleanup_minutes",
            "DYN_CLIENT_CLEANUP_MINUTES",
        ) {
            self.dynamic_clients.cleanup_minutes = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "dynamic_clients",
            "rate_limit_sec",
            "DYN_CLIENT_RATE_LIMIT_SEC",
        ) {
            self.dynamic_clients.rate_limit_sec = v;
        }
    }

    fn parse_email(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "email");

        self.email.rauthy_admin_email = t_str(
            &mut table,
            "email",
            "rauthy_admin_email",
            "RAUTHY_ADMIN_EMAIL",
        );
        if let Some(v) = t_str(&mut table, "email", "sub_prefix", "EMAIL_SUB_PREFIX") {
            self.email.sub_prefix = v.into();
        }
        self.email.smtp_url = t_str(&mut table, "email", "smtp_url", "SMTP_URL");
        self.email.smtp_port = t_u16(&mut table, "email", "smtp_port", "SMTP_PORT");
        self.email.smtp_username = t_str(&mut table, "email", "smtp_username", "SMTP_USERNAME");
        self.email.smtp_password = t_str(&mut table, "email", "smtp_password", "SMTP_PASSWORD");
        if let Some(v) = t_str(&mut table, "email", "smtp_from", "SMTP_FROM") {
            self.email.smtp_from = v.into();
        }
        if let Some(v) = t_u16(
            &mut table,
            "email",
            "connect_retries",
            "SMTP_CONNECT_RETRIES",
        ) {
            self.email.connect_retries = v;
        }
        self.email.root_ca = t_str(&mut table, "email", "root_ca", "SMTP_ROOT_CA");
        if let Some(v) = t_str(&mut table, "email", "smtp_conn_mode", "SMTP_CONN_MODE") {
            self.email.smtp_conn_mode = SmtpConnMode::from(v.as_str());
        }

        self.email.xoauth_url = t_str(&mut table, "email", "xoauth_url", "SMTP_XOAUTH2_URL");
        self.email.xoauth_client_id = t_str(
            &mut table,
            "email",
            "xoauth_client_id",
            "SMTP_XOAUTH2_CLIENT_ID",
        );
        self.email.xoauth_client_secret = t_str(
            &mut table,
            "email",
            "xoauth_client_secret",
            "SMTP_XOAUTH2_CLIENT_SECRET",
        );
        self.email.xoauth_scope = t_str(&mut table, "email", "xoauth_scope", "SMTP_XOAUTH2_SCOPE");

        self.email.microsoft_graph_uri = t_str(
            &mut table,
            "email",
            "microsoft_graph_uri",
            "SMTP_MICROSOFT_GRAPH_URI",
        );

        if let Some(v) = t_bool(&mut table, "email", "starttls_only", "SMTP_STARTTLS_ONLY") {
            self.email.starttls_only = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "email",
            "danger_insecure",
            "SMTP_DANGER_INSECURE",
        ) {
            self.email.danger_insecure = v;
        }

        // [email.jobs]
        let mut jobs = t_table(&mut table, "jobs");

        if let Some(v) = t_u32(
            &mut jobs,
            "email.jobs",
            "orphaned_seconds",
            "EMAIL_JOBS_ORPHANED_SECONDS",
        ) {
            self.email.jobs.orphaned_seconds = v;
        }
        if let Some(v) = t_u32(
            &mut jobs,
            "email.jobs",
            "scheduler_interval_seconds",
            "EMAIL_JOBS_SCHED_SECONDS",
        ) {
            self.email.jobs.scheduler_interval_seconds = v;
        }
        if let Some(v) = t_u16(
            &mut jobs,
            "email.jobs",
            "batch_size",
            "EMAIL_JOBS_BATCH_SIZE",
        ) {
            if v == 0 {
                panic!("[email.jobs].batch_size must not be 0");
            }
            self.email.jobs.batch_size = v;
        }
        if let Some(v) = t_u32(
            &mut jobs,
            "email.jobs",
            "batch_delay_ms",
            "EMAIL_JOBS_BATCH_DELAY_MS",
        ) {
            self.email.jobs.batch_delay_ms = v;
        }

        // [email.tz_fmt]
        let mut tz_fmt = t_table(&mut table, "tz_fmt");

        if let Some(v) = t_str(&mut tz_fmt, "email.tz_fmt", "de", "TZ_FMT_DE") {
            self.email.tz_fmt.de = v.into();
        }
        if let Some(v) = t_str(&mut tz_fmt, "email.tz_fmt", "en", "TZ_FMT_EN") {
            self.email.tz_fmt.en = v.into();
        }
        if let Some(v) = t_str(&mut tz_fmt, "email.tz_fmt", "ko", "TZ_FMT_KO") {
            self.email.tz_fmt.ko = v.into();
        }
        if let Some(v) = t_str(&mut tz_fmt, "email.tz_fmt", "no", "TZ_FMT_NO") {
            self.email.tz_fmt.no = v.into();
        }
        if let Some(v) = t_str(&mut tz_fmt, "email.tz_fmt", "zhhans", "TZ_FMT_ZHHANS") {
            self.email.tz_fmt.zhhans = v.into();
        }

        if let Some(v) = t_str(&mut tz_fmt, "email.tz_fmt", "tz_fallback", "TZ_FALLBACK") {
            self.email.tz_fmt.tz_fallback = v.into();
        }
    }

    fn parse_encryption(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "encryption");

        if let Some(v) = t_str(&mut table, "encryption", "key_active", "ENC_KEY_ACTIVE") {
            self.encryption.key_active = v;
        }
        if let Some(v) = t_str_vec(&mut table, "encryption", "keys", "ENC_KEYS") {
            self.encryption.keys = v;
        }
    }

    fn parse_ephemeral_clients(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "ephemeral_clients");

        if let Some(v) = t_bool(
            &mut table,
            "ephemeral_clients",
            "enable",
            "ENABLE_EPHEMERAL_CLIENTS",
        ) {
            self.ephemeral_clients.enable = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "ephemeral_clients",
            "enable_web_id",
            "ENABLE_WEB_ID",
        ) {
            self.ephemeral_clients.enable_web_id = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "ephemeral_clients",
            "enable_solid_aud",
            "ENABLE_SOLID_AUD",
        ) {
            self.ephemeral_clients.enable_solid_aud = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "ephemeral_clients",
            "force_mfa",
            "EPHEMERAL_CLIENTS_FORCE_MFA",
        ) {
            self.ephemeral_clients.force_mfa = v;
        }

        if let Some(v) = t_str_vec(
            &mut table,
            "ephemeral_clients",
            "allowed_flows",
            "EPHEMERAL_CLIENTS_ALLOWED_FLOWS",
        ) {
            self.ephemeral_clients.allowed_flows = v.into_iter().map(Cow::from).collect::<Vec<_>>();
        }
        if let Some(v) = t_str_vec(
            &mut table,
            "ephemeral_clients",
            "allowed_scopes",
            "EPHEMERAL_CLIENTS_ALLOWED_SCOPES",
        ) {
            self.ephemeral_clients.allowed_scopes =
                v.into_iter().map(Cow::from).collect::<Vec<_>>();
        }

        if let Some(v) = t_u32(
            &mut table,
            "ephemeral_clients",
            "cache_lifetime",
            "EPHEMERAL_CLIENTS_CACHE_LIFETIME",
        ) {
            self.ephemeral_clients.cache_lifetime = v;
        }
    }

    fn parse_events(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "events");

        if let Some(v) = t_str(&mut table, "events", "email", "EVENT_EMAIL") {
            self.events.email = Some(v);
        }

        if let Some(v) = t_str(
            &mut table,
            "events",
            "matrix_user_id",
            "EVENT_MATRIX_USER_ID",
        ) {
            self.events.matrix_user_id = Some(v);
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "matrix_room_id",
            "EVENT_MATRIX_ROOM_ID",
        ) {
            self.events.matrix_room_id = Some(v);
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "matrix_access_token",
            "EVENT_MATRIX_ACCESS_TOKEN",
        ) {
            self.events.matrix_access_token = Some(v);
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "matrix_user_password",
            "EVENT_MATRIX_USER_PASSWORD",
        ) {
            self.events.matrix_user_password = Some(v);
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "matrix_server_url",
            "EVENT_MATRIX_SERVER_URL",
        ) {
            self.events.matrix_server_url = v.into();
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "matrix_root_ca_path",
            "EVENT_MATRIX_ROOT_CA_PATH",
        ) {
            self.events.matrix_root_ca_path = Some(v);
        }
        if let Some(v) = t_bool(
            &mut table,
            "events",
            "matrix_danger_disable_tls_validation",
            "EVENT_MATRIX_DANGER_DISABLE_TLS_VALIDATION",
        ) {
            self.events.matrix_danger_disable_tls_validation = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "events",
            "matrix_error_no_panic",
            "EVENT_MATRIX_ERROR_NO_PANIC",
        ) {
            self.events.matrix_error_no_panic = v;
        }

        if let Some(v) = t_str(&mut table, "events", "slack_webhook", "EVENT_SLACK_WEBHOOK") {
            self.events.slack_webhook = Some(v);
        }

        if let Some(v) = t_str(
            &mut table,
            "events",
            "notify_level_email",
            "EVENT_NOTIFY_LEVEL_EMAIL",
        ) {
            self.events.notify_level_email =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for notify_level_email");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "notify_level_matrix",
            "EVENT_NOTIFY_LEVEL_MATRIX",
        ) {
            self.events.notify_level_matrix =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for notify_level_matrix");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "notify_level_slack",
            "EVENT_NOTIFY_LEVEL_SLACK",
        ) {
            self.events.notify_level_slack =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for notify_level_slack");
        }
        if let Some(v) = t_str(&mut table, "events", "persist_level", "EVENT_PERSIST_LEVEL") {
            self.events.persist_level =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for persist_level");
        }

        if let Some(v) = t_u32(&mut table, "events", "cleanup_days", "EVENT_CLEANUP_DAYS") {
            self.events.cleanup_days = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "events",
            "generate_token_issued",
            "EVENT_GENERATE_TOKEN_ISSUED",
        ) {
            self.events.generate_token_issued = v;
        }

        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_new_user",
            "EVENT_LEVEL_NEW_USER",
        ) {
            self.events.level_new_user =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for level_new_user");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_user_email_change",
            "EVENT_LEVEL_USER_EMAIL_CHANGE",
        ) {
            self.events.level_user_email_change = EventLevel::from_str(&v)
                .expect("Cannot parse EventLevel for level_user_email_change");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_user_password_reset",
            "EVENT_LEVEL_USER_PASSWORD_RESET",
        ) {
            self.events.level_user_password_reset = EventLevel::from_str(&v)
                .expect("Cannot parse EventLevel for level_user_password_reset");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_rauthy_admin",
            "EVENT_LEVEL_RAUTHY_ADMIN",
        ) {
            self.events.level_rauthy_admin =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for level_rauthy_admin");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_rauthy_version",
            "EVENT_LEVEL_RAUTHY_VERSION",
        ) {
            self.events.level_rauthy_version =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for level_rauthy_version");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_jwks_rotate",
            "EVENT_LEVEL_JWKS_ROTATE",
        ) {
            self.events.level_jwks_rotate =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for level_jwks_rotate");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_new_login_location",
            "EVENT_LEVEL_NEW_LOGIN_LOCATION",
        ) {
            self.events.level_new_login_location = EventLevel::from_str(&v)
                .expect("Cannot parse EventLevel for level_new_login_location");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_secrets_migrated",
            "EVENT_LEVEL_SECRETS_MIGRATED",
        ) {
            self.events.level_secrets_migrated = EventLevel::from_str(&v)
                .expect("Cannot parse EventLevel for level_secrets_migrated");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_rauthy_start",
            "EVENT_LEVEL_RAUTHY_START",
        ) {
            self.events.level_rauthy_start =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for level_rauthy_start");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_rauthy_healthy",
            "EVENT_LEVEL_RAUTHY_HEALTHY",
        ) {
            self.events.level_rauthy_healthy =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for level_rauthy_healthy");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_rauthy_unhealthy",
            "EVENT_LEVEL_RAUTHY_UNHEALTHY",
        ) {
            self.events.level_rauthy_unhealthy = EventLevel::from_str(&v)
                .expect("Cannot parse EventLevel for level_rauthy_unhealthy");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_ip_blacklisted",
            "EVENT_LEVEL_IP_BLACKLISTED",
        ) {
            self.events.level_ip_blacklisted =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for level_ip_blacklisted");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_backchannel_logout_failed",
            "EVENT_LEVEL_BACKCHANNEL_LOGOUT_FAILED",
        ) {
            self.events.level_backchannel_logout_failed = EventLevel::from_str(&v)
                .expect("Cannot parse EventLevel for backchannel_logout_failed");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_force_logout",
            "EVENT_LEVEL_FORCE_LOGOUT",
        ) {
            self.events.level_force_logout =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for force_logout");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_user_login_revoke",
            "EVENT_LEVEL_USER_LOGIN_REVOKE",
        ) {
            self.events.level_user_login_revoke =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for user_login_revoke");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_scim_task_failed",
            "EVENT_LEVEL_SCIM_TASK_FAILED",
        ) {
            self.events.level_scim_task_failed =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for scim_task_failed");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_suspicious_request",
            "EVENT_LEVEL_SUSPICIOUS_REQUEST",
        ) {
            self.events.level_suspicious_request =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for suspicious_request");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_token_issued",
            "EVENT_LEVEL_TOKEN_ISSUED",
        ) {
            self.events.level_token_issued =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for level_token_issued");
        }

        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_failed_logins_25",
            "EVENT_LEVEL_FAILED_LOGINS_25",
        ) {
            self.events.level_failed_logins_25 = EventLevel::from_str(&v)
                .expect("Cannot parse EventLevel for level_failed_logins_25");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_failed_logins_20",
            "EVENT_LEVEL_FAILED_LOGINS_20",
        ) {
            self.events.level_failed_logins_20 = EventLevel::from_str(&v)
                .expect("Cannot parse EventLevel for level_failed_logins_20");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_failed_logins_15",
            "EVENT_LEVEL_FAILED_LOGINS_15",
        ) {
            self.events.level_failed_logins_15 = EventLevel::from_str(&v)
                .expect("Cannot parse EventLevel for level_failed_logins_15");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_failed_logins_10",
            "EVENT_LEVEL_FAILED_LOGINS_10",
        ) {
            self.events.level_failed_logins_10 = EventLevel::from_str(&v)
                .expect("Cannot parse EventLevel for level_failed_logins_10");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_failed_logins_7",
            "EVENT_LEVEL_FAILED_LOGINS_7",
        ) {
            self.events.level_failed_logins_7 = EventLevel::from_str(&v)
                .expect("Cannot parse EventLevel for level_failed_logins_7");
        }
        if let Some(v) = t_str(
            &mut table,
            "events",
            "level_failed_login",
            "EVENT_LEVEL_FAILED_LOGIN",
        ) {
            self.events.level_failed_login =
                EventLevel::from_str(&v).expect("Cannot parse EventLevel for level_failed_login");
        }

        if let Some(v) = t_bool(
            &mut table,
            "events",
            "disable_app_version_check",
            "DISABLE_APP_VERSION_CHECK",
        ) {
            self.events.disable_app_version_check = v;
        }
    }

    fn parse_fedcm(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "fedcm");

        if let Some(v) = t_bool(
            &mut table,
            "fedcm",
            "experimental_enable",
            "EXPERIMENTAL_FED_CM_ENABLE",
        ) {
            self.fedcm.experimental_enable = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "fedcm",
            "session_lifetime",
            "SESSION_LIFETIME_FED_CM",
        ) {
            self.fedcm.session_lifetime = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "fedcm",
            "session_timeout",
            "SESSION_TIMEOUT_FED_CM",
        ) {
            self.fedcm.session_timeout = v;
        }
    }

    fn parse_geo(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "geolocation");

        if let Some(v) = t_bool(
            &mut table,
            "geolocation",
            "block_unknown",
            "GEO_BLOCK_UNKONW",
        ) {
            self.geo.block_unknown = v;
        }
        self.geo.country_header = t_str(
            &mut table,
            "geolocation",
            "country_header",
            "GEO_COUNTRY_HEADER",
        );

        if let Some(v) = t_str(&mut table, "geolocation", "country_list_type", "") {
            match v.as_str() {
                "whitelist" => self.geo.block_is_whitelist = Some(true),
                "blacklist" => self.geo.block_is_whitelist = Some(false),
                _ => {
                    panic!("`geolocation.country_list_type` can only be `whitelist` or `blacklist`")
                }
            }

            if let Some(v) = t_str_vec(&mut table, "geolocation", "country_list", "") {
                self.geo.country_list = v;
            }
        }

        self.geo.maxmind_account_id = t_str(
            &mut table,
            "geolocation",
            "maxmind_account_id",
            "GEO_MAXMIND_ACC_ID",
        );
        self.geo.maxmind_license_key = t_str(
            &mut table,
            "geolocation",
            "maxmind_license_key",
            "GEO_MAXMIND_LICENSE",
        );
        if let Some(v) = t_str(
            &mut table,
            "geolocation",
            "maxmind_db_dir",
            "GEO_MAXMIND_DIR",
        ) {
            self.geo.maxmind_db_dir = v.into();
        }
        if let Some(v) = t_str(
            &mut table,
            "geolocation",
            "maxmind_db_type",
            "GEO_MAXMIND_DB_TYPE",
        ) {
            self.geo.maxmind_db_type = v.into();
        }
        if let Some(v) = t_str(
            &mut table,
            "geolocation",
            "maxmind_update_cron",
            "GEO_MAXMIND_UPDATE_CRON",
        ) {
            self.geo.maxmind_update_cron = v.into();
        }
    }

    fn parse_hashing(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "hashing");

        if let Some(v) = t_u32(&mut table, "hashing", "argon2_m_cost", "ARGON2_M_COST") {
            self.hashing.argon2_m_cost = v;
        }
        if let Some(v) = t_u32(&mut table, "hashing", "argon2_t_cost", "ARGON2_T_COST") {
            self.hashing.argon2_t_cost = v;
        }
        if let Some(v) = t_u32(&mut table, "hashing", "argon2_p_cost", "ARGON2_P_COST") {
            self.hashing.argon2_p_cost = v;
        }

        if let Some(v) = t_u32(
            &mut table,
            "hashing",
            "max_hash_threads",
            "MAX_HASH_THREADS",
        ) {
            self.hashing.max_hash_threads = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "hashing",
            "hash_await_warn_time",
            "HASH_AWAIT_WARN_TIME",
        ) {
            self.hashing.hash_await_warn_time = v;
        }
    }

    async fn parse_hiqlite_config(&mut self, table: &mut toml::Table) -> hiqlite::NodeConfig {
        let table = t_table(table, "cluster");

        if self.encryption.key_active.is_empty() || self.encryption.keys.is_empty() {
            panic!("Missing `encryption.keys` / `encryption.key_active`");
        }
        let Ok(enc_keys) = cryptr::EncKeys::try_parse(
            self.encryption.key_active.clone(),
            self.encryption.keys.clone(),
        ) else {
            panic!("Invalid ENC_KEYS / ENC_KEY_ACTIVE");
        };

        match NodeConfig::from_toml_table(table, "cluster", Some(enc_keys)).await {
            Ok(config) => config,
            Err(err) => {
                panic!("Error parsing `[cluster]` section: {err:?}");
            }
        }
    }

    fn parse_http_client(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "http_client");

        if let Some(v) = t_u32(
            &mut table,
            "http_client",
            "connect_timeout",
            "HTTP_CONNECT_TIMEOUT",
        ) {
            self.http_client.connect_timeout = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "http_client",
            "request_timeout",
            "HTTP_REQUEST_TIMEOUT",
        ) {
            self.http_client.request_timeout = v;
        }
        if let Some(v) = t_str(&mut table, "http_client", "min_tls", "HTTP_MIN_TLS") {
            self.http_client.min_tls = v.into();
        }
        if let Some(v) = t_u32(
            &mut table,
            "http_client",
            "idle_timeout",
            "HTTP_IDLE_TIMEOUT",
        ) {
            self.http_client.idle_timeout = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "http_client",
            "danger_unencrypted",
            "HTTP_DANGER_UNENCRYPTED",
        ) {
            self.http_client.danger_unencrypted = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "http_client",
            "danger_insecure",
            "HTTP_DANGER_INSECURE",
        ) {
            self.http_client.danger_insecure = v;
        }
        if let Some(v) = t_str(
            &mut table,
            "http_client",
            "root_ca_bundle",
            "HTTP_CUST_ROOT_CA_BUNDLE",
        ) {
            self.http_client.root_ca_bundle = Some(v);
        }
    }

    fn parse_i18n(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "i18n");

        if let Some(v) = t_str_vec(
            &mut table,
            "i18n",
            "filter_lang_common",
            "FILTER_LANG_COMMON",
        ) {
            self.i18n.filter_lang_common = v.into_iter().map(Cow::from).collect::<Vec<_>>();
        }
        if let Some(v) = t_str_vec(&mut table, "i18n", "filter_lang_admin", "FILTER_LANG_ADMIN") {
            self.i18n.filter_lang_admin = v.into_iter().map(Cow::from).collect::<Vec<_>>();
        }
    }

    fn parse_lifetimes(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "lifetimes");

        if let Some(v) = t_u16(
            &mut table,
            "lifetimes",
            "refresh_token_grace_time",
            "REFRESH_TOKEN_GRACE_TIME",
        ) {
            self.lifetimes.refresh_token_grace_time = v;
        }
        if let Some(v) = t_u16(
            &mut table,
            "lifetimes",
            "refresh_token_lifetime",
            "REFRESH_TOKEN_LIFETIME",
        ) {
            self.lifetimes.refresh_token_lifetime = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "lifetimes",
            "session_lifetime",
            "SESSION_LIFETIME",
        ) {
            self.lifetimes.session_lifetime = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "lifetimes",
            "session_renew_mfa",
            "SESSION_RENEW_MFA",
        ) {
            self.lifetimes.session_renew_mfa = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "lifetimes",
            "session_timeout",
            "SESSION_TIMEOUT",
        ) {
            self.lifetimes.session_timeout = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "lifetimes",
            "magic_link_pwd_reset",
            "ML_LT_PWD_RESET",
        ) {
            self.lifetimes.magic_link_pwd_reset = v;
        }
        if let Some(v) = t_u32(
            &mut table,
            "lifetimes",
            "magic_link_pwd_first",
            "ML_LT_PWD_FIRST",
        ) {
            self.lifetimes.magic_link_pwd_first = v;
        }
        if let Some(v) = t_str(
            &mut table,
            "lifetimes",
            "jwk_autorotate_cron",
            "JWK_AUTOROTATE_CRON",
        ) {
            self.lifetimes.jwk_autorotate_cron = v.into();
        }
    }

    fn parse_logging(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "logging");

        if let Some(v) = t_str(&mut table, "logging", "level", "LOG_LEVEL") {
            self.logging.level = v.into();
        }
        if let Some(v) = t_str(
            &mut table,
            "logging",
            "level_database",
            "LOG_LEVEL_DATABASE",
        ) {
            self.logging.level_database = v.into();
        }
        if let Some(v) = t_str(&mut table, "logging", "level_access", "LOG_LEVEL_ACCESS") {
            self.logging.level_access = v.into();
        }
        if let Some(v) = t_str(&mut table, "logging", "log_fmt", "LOG_FMT") {
            self.logging.log_fmt = v.into();
        }
    }

    fn parse_mfa(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "mfa");

        if let Some(v) = t_bool(&mut table, "mfa", "admin_force_mfa", "ADMIN_FORCE_MFA") {
            self.mfa.admin_force_mfa = v;
        }
    }

    fn parse_pam(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "pam");

        if let Some(v) = t_u8(
            &mut table,
            "pam",
            "remote_password_len",
            "PAM_REMOTE_PASSWORD_LEN",
        ) {
            self.pam.remote_password_len = v;
        }
        if let Some(v) = t_u16(
            &mut table,
            "pam",
            "remote_password_ttl",
            "PAM_REMOTE_PASSWORD_TTL",
        ) {
            self.pam.remote_password_ttl = v;
        }

        // [pam.authorized_keys]
        let mut auth_keys = t_table(&mut table, "authorized_keys");

        if let Some(v) = t_bool(
            &mut auth_keys,
            "pam.authorized_keys",
            "authorized_keys_enable",
            "PAM_SSH_AUTHORIZED_KEYS_ENABLE",
        ) {
            self.pam.authorized_keys.authorized_keys_enable = v;
        }
        if let Some(v) = t_bool(
            &mut auth_keys,
            "pam.authorized_keys",
            "auth_required",
            "PAM_SSH_AUTH_REQUIRED",
        ) {
            self.pam.authorized_keys.auth_required = v;
        }
        if let Some(v) = t_bool(
            &mut auth_keys,
            "pam.authorized_keys",
            "blacklist_used_keys",
            "PAM_SSH_BLACKLIST_SSH_KEYS",
        ) {
            self.pam.authorized_keys.blacklist_used_keys = v;
        }
        if let Some(v) = t_u16(
            &mut auth_keys,
            "pam.authorized_keys",
            "blacklist_cleanup_days",
            "PAM_SSH_BLACKLIST_CLEANUP_DAYS",
        ) {
            self.pam.authorized_keys.blacklist_cleanup_days = v;
        }
        if let Some(v) = t_bool(
            &mut auth_keys,
            "pam.authorized_keys",
            "include_comments",
            "PAM_SSH_INCLUDE_COMMENTS",
        ) {
            self.pam.authorized_keys.include_comments = v;
        }
        if let Some(v) = t_u16(
            &mut auth_keys,
            "pam.authorized_keys",
            "forced_key_expiry_days",
            "PAM_SSH_KEY_EXP_DAYS",
        ) {
            self.pam.authorized_keys.forced_key_expiry_days = v;
        }
    }

    fn parse_pow(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "pow");

        if let Some(v) = t_u8(&mut table, "pow", "difficulty", "POW_DIFFICULTY") {
            self.pow.difficulty = v;
        }
        if let Some(v) = t_u16(&mut table, "pow", "exp", "POW_EXP") {
            self.pow.exp = v;
        }
    }

    fn parse_scim(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "scim");

        if let Some(v) = t_bool(
            &mut table,
            "scim",
            "sync_delete_groups",
            "SCIM_SYNC_DELETE_GROUPS",
        ) {
            self.scim.sync_delete_groups = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "scim",
            "sync_delete_users",
            "SCIM_SYNC_DELETE_USERS",
        ) {
            self.scim.sync_delete_users = v;
        }
        if let Some(v) = t_u16(&mut table, "scim", "retry_count", "SCIM_RETRY_COUNT") {
            self.scim.retry_count = v;
        }
    }

    fn parse_server(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "server");

        if let Some(v) = t_str(&mut table, "server", "listen_address", "LISTEN_ADDRESS") {
            self.server.listen_address = v.into();
        }
        if let Some(v) = t_u16(&mut table, "server", "port_http", "LISTEN_PORT_HTTP") {
            self.server.port_http = v;
        }
        if let Some(v) = t_u16(&mut table, "server", "port_https", "LISTEN_PORT_HTTPS") {
            self.server.port_https = v;
        }
        if let Some(v) = t_str(&mut table, "server", "scheme", "LISTEN_SCHEME") {
            self.server.scheme = v.into();
        }
        if let Some(v) = t_str(&mut table, "server", "pub_url", "PUB_URL") {
            self.server.pub_url = v;
        }
        if let Some(v) = t_u16(&mut table, "server", "http_workers", "HTTP_WORKERS") {
            self.server.http_workers = v;
        }

        if let Some(v) = t_bool(&mut table, "server", "proxy_mode", "PROXY_MODE") {
            self.server.proxy_mode = v;
        }
        if let Some(v) = t_str_vec(&mut table, "server", "trusted_proxies", "TRUSTED_PROXIES") {
            self.server.trusted_proxies = v;
        }
        if let Some(v) = t_str_vec(
            &mut table,
            "server",
            "additional_allowed_origin_schemes",
            "ADDITIONAL_ALLOWED_ORIGIN_SCHEMES",
        ) {
            self.server.additional_allowed_origin_schemes = v;
        }

        if let Some(v) = t_bool(&mut table, "server", "metrics_enable", "METRICS_ENABLE") {
            self.server.metrics_enable = v;
        }
        if let Some(v) = t_str(&mut table, "server", "metrics_addr", "METRICS_ADDR") {
            self.server.metrics_addr = v.into();
        }
        if let Some(v) = t_u16(&mut table, "server", "metrics_port", "METRICS_PORT") {
            self.server.metrics_port = v;
        }

        if let Some(v) = t_bool(
            &mut table,
            "server",
            "swagger_ui_enable",
            "SWAGGER_UI_ENABLE",
        ) {
            self.server.swagger_ui_enable = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "server",
            "swagger_ui_public",
            "SWAGGER_UI_PUBLIC",
        ) {
            self.server.swagger_ui_public = v;
        }

        if let Some(v) = t_u16(&mut table, "server", "see_keep_alive", "SSE_KEEP_ALIVE") {
            self.server.see_keep_alive = v;
        }
        if let Some(v) = t_u16(&mut table, "server", "ssp_threshold", "SSP_THRESHOLD") {
            self.server.ssp_threshold = v;
        }
    }

    fn parse_suspicious_requests(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "suspicious_requests");

        if let Some(v) = t_u16(
            &mut table,
            "suspicious_requests",
            "blacklist",
            "SUSPICIOUS_REQUESTS_BLACKLIST",
        ) {
            self.suspicious_requests.blacklist = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "suspicious_requests",
            "log",
            "SUSPICIOUS_REQUESTS_LOG",
        ) {
            self.suspicious_requests.log = v;
        }
    }

    fn parse_templates(&mut self, table: &mut toml::Table) {
        let Some(Value::Array(arr)) = table.remove("templates") else {
            return;
        };

        for entry in arr {
            let Value::Table(mut table) = entry else {
                panic!("{}", err_t("<entry>", "templates", "Table"));
            };

            let lang = t_str(&mut table, "[templates]", "lang", "")
                .expect("`lang` is mandatory for `[[templates]]`");
            let typ = t_str(&mut table, "[templates]", "typ", "")
                .expect("`typ` is mandatory for `[[templates]]`");

            let is_password_new = match typ.as_str() {
                "password_new" => true,
                "password_reset" => false,
                _ => {
                    panic!(
                        "Invalid value for `templates.typ`, allowed are: password_new password_reset"
                    )
                }
            };
            let mut tpl = match lang.as_str() {
                "en" => {
                    if is_password_new {
                        self.templates.password_new_en.clone()
                    } else {
                        self.templates.password_reset_en.clone()
                    }
                }
                "de" => {
                    if is_password_new {
                        self.templates.password_new_de.clone()
                    } else {
                        self.templates.password_reset_de.clone()
                    }
                }
                "ko" => {
                    if is_password_new {
                        self.templates.password_new_ko.clone()
                    } else {
                        self.templates.password_reset_ko.clone()
                    }
                }
                "nb" => {
                    if is_password_new {
                        self.templates.password_new_nb.clone()
                    } else {
                        self.templates.password_reset_nb.clone()
                    }
                }
                "uk" => {
                    if is_password_new {
                        self.templates.password_new_uk.clone()
                    } else {
                        self.templates.password_reset_uk.clone()
                    }
                }
                "zh_hans" => {
                    if is_password_new {
                        self.templates.password_new_zhhans.clone()
                    } else {
                        self.templates.password_reset_zhhans.clone()
                    }
                }
                _ => {
                    panic!("Invalid value for `templates.lang`, allowed are: en de ko zh_hans")
                }
            };

            if let Some(v) = t_str(&mut table, "templates", "subject", "") {
                tpl.subject = v.into();
            }
            if let Some(v) = t_str(&mut table, "templates", "header", "") {
                tpl.header = v.into();
            }
            if let Some(v) = t_str(&mut table, "templates", "text", "") {
                tpl.text = Some(v);
            }
            if let Some(v) = t_str(&mut table, "templates", "click_link", "") {
                tpl.click_link = v.into();
            }
            if let Some(v) = t_str(&mut table, "templates", "validity", "") {
                tpl.validity = v.into();
            }
            if let Some(v) = t_str(&mut table, "templates", "expires", "") {
                tpl.expires = v.into();
            }
            if let Some(v) = t_str(&mut table, "templates", "footer", "") {
                tpl.footer = Some(v);
            }
            if let Some(v) = t_str(&mut table, "templates", "button_text_request_new", "") {
                tpl.button_text_request_new = Some(v.into());
            }

            match lang.as_str() {
                "en" => {
                    if is_password_new {
                        self.templates.password_new_en = tpl;
                    } else {
                        self.templates.password_reset_en = tpl;
                    }
                }
                "de" => {
                    if is_password_new {
                        self.templates.password_new_de = tpl;
                    } else {
                        self.templates.password_reset_de = tpl;
                    }
                }
                "ko" => {
                    if is_password_new {
                        self.templates.password_new_ko = tpl;
                    } else {
                        self.templates.password_reset_ko = tpl;
                    }
                }
                "uk" => {
                    if is_password_new {
                        self.templates.password_new_uk = tpl;
                    } else {
                        self.templates.password_reset_uk = tpl;
                    }
                }
                "zh_hans" => {
                    if is_password_new {
                        self.templates.password_new_zhhans = tpl;
                    } else {
                        self.templates.password_reset_zhhans = tpl;
                    }
                }
                _ => {
                    panic!(
                        "Invalid value for `templates.lang`, allowed are: en de ko nb uk zh_hans"
                    )
                }
            }
        }
    }

    fn parse_tls(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "tls");

        if let Some(v) = t_str(&mut table, "tls", "cert_path", "TLS_CERT") {
            self.tls.cert_path = Some(v);
        }
        if let Some(v) = t_str(&mut table, "tls", "key_path", "TLS_KEY") {
            self.tls.key_path = Some(v);
        }

        if let Some(v) = t_bool(
            &mut table,
            "tls",
            "generate_self_signed",
            "TLS_GENERATE_SELF_SIGNED",
        ) {
            self.tls.generate_self_signed = v;
        }
    }

    fn parse_tos(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "tos");

        if let Some(v) = t_u16(&mut table, "tos", "accept_timeout", "TOS_ACCEPT_TIMEOUT") {
            self.tos.accept_timeout = v;
        }
    }

    fn parse_user_delete(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "user_delete");

        if let Some(v) = t_bool(
            &mut table,
            "user_delete",
            "enable_self_delete",
            "USER_ENABLE_SELF_DELETE",
        ) {
            self.user_delete.enable_self_delete = v;
        }
    }

    fn parse_user_pictures(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "user_pictures");

        if let Some(v) = t_str(
            &mut table,
            "user_pictures",
            "storage_type",
            "PICTURE_STORAGE_TYPE",
        ) {
            if !["db", "file", "s3", "disabled"].contains(&v.as_str()) {
                panic!("`user_pictures.storage_type` must be one of: db file s3 disabled");
            }
            self.user_pictures.storage_type = v.into();
        }
        if let Some(v) = t_str(&mut table, "user_pictures", "path", "PICTURE_PATH") {
            self.user_pictures.path = v.into();
        }

        if let Some(v) = t_str(&mut table, "user_pictures", "s3_url", "PIC_S3_URL") {
            self.user_pictures.s3_url = v.into();
        }
        if let Some(v) = t_str(&mut table, "user_pictures", "bucket", "PIC_S3_BUCKET") {
            self.user_pictures.bucket = Some(v);
        }
        if let Some(v) = t_str(&mut table, "user_pictures", "region", "PIC_S3_REGION") {
            self.user_pictures.region = Some(v);
        }
        if let Some(v) = t_str(&mut table, "user_pictures", "s3_key", "PIC_S3_KEY") {
            self.user_pictures.s3_key = Some(v);
        }
        if let Some(v) = t_str(&mut table, "user_pictures", "s3_secret", "PIC_S3_SECRET") {
            self.user_pictures.s3_secret = Some(v);
        }
        if let Some(v) = t_bool(
            &mut table,
            "user_pictures",
            "s3_path_style",
            "PIC_S3_PATH_STYLE",
        ) {
            self.user_pictures.s3_path_style = v;
        }

        if let Some(v) = t_u16(
            &mut table,
            "user_pictures",
            "upload_limit_mb",
            "PICTURE_UPLOAD_LIMIT_MB",
        ) {
            self.user_pictures.upload_limit_mb = v;
        }
        if let Some(v) = t_bool(&mut table, "user_pictures", "public", "PICTURE_PUBLIC") {
            self.user_pictures.public = v;
        }
    }

    fn parse_user_registration(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "user_registration");

        if let Some(v) = t_bool(&mut table, "user_registration", "enable", "OPEN_USER_REG") {
            self.user_registration.enable = v;
        }
        if let Some(v) = t_str(
            &mut table,
            "user_registration",
            "domain_restriction",
            "USER_REG_DOMAIN_RESTRICTION",
        ) {
            self.user_registration.domain_restriction = Some(v);
        }
        if let Some(v) = t_str_vec(
            &mut table,
            "user_registration",
            "domain_blacklist",
            "USER_REG_DOMAIN_BLACKLIST",
        ) {
            self.user_registration.domain_blacklist = v;
        }
    }

    fn parse_user_values(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "user_values");

        if let Some(v) = t_str(&mut table, "user_values", "given_name", "") {
            self.user_values.given_name = UserValueConfigValue::from(v.as_str());
        }
        if let Some(v) = t_str(&mut table, "user_values", "family_name", "") {
            self.user_values.family_name = UserValueConfigValue::from(v.as_str());
        }
        if let Some(v) = t_str(&mut table, "user_values", "birthdate", "") {
            self.user_values.birthdate = UserValueConfigValue::from(v.as_str());
        }
        if let Some(v) = t_str(&mut table, "user_values", "street", "") {
            self.user_values.street = UserValueConfigValue::from(v.as_str());
        }
        if let Some(v) = t_str(&mut table, "user_values", "zip", "") {
            self.user_values.zip = UserValueConfigValue::from(v.as_str());
        }
        if let Some(v) = t_str(&mut table, "user_values", "city", "") {
            self.user_values.city = UserValueConfigValue::from(v.as_str());
        }
        if let Some(v) = t_str(&mut table, "user_values", "country", "") {
            self.user_values.country = UserValueConfigValue::from(v.as_str());
        }
        if let Some(v) = t_str(&mut table, "user_values", "phone", "") {
            self.user_values.phone = UserValueConfigValue::from(v.as_str());
        }
        if let Some(v) = t_str(&mut table, "user_values", "tz", "") {
            self.user_values.tz = UserValueConfigValue::from(v.as_str());
        }

        if let Some(v) = t_bool(
            &mut table,
            "user_values",
            "revalidate_during_login",
            "USER_VALUES_REVALIDATE_DURING_LOGIN",
        ) {
            self.user_values.revalidate_during_login = v;
        }

        let mut table = t_table(&mut table, "preferred_username");

        if let Some(v) = t_str(
            &mut table,
            "user_values.preferred_username",
            "preferred_username",
            "",
        ) {
            self.user_values.preferred_username.preferred_username =
                UserValueConfigValue::from(v.as_str());
        }
        if let Some(v) = t_bool(
            &mut table,
            "user_values.preferred_username",
            "immutable",
            "",
        ) {
            self.user_values.preferred_username.immutable = v;
        }
        if let Some(v) = t_str_vec(
            &mut table,
            "user_values.preferred_username",
            "blacklist",
            "",
        ) {
            self.user_values.preferred_username.blacklist =
                v.into_iter().map(Cow::from).collect::<Vec<_>>();
        }
        if let Some(v) = t_str(
            &mut table,
            "user_values.preferred_username",
            "regex_rust",
            "",
        ) {
            RE_PREFERRED_USERNAME
                .set(
                    Regex::new(&v)
                        .expect("Invalid value for user_values.preferred_username.regex_rust"),
                )
                .unwrap();
        }
        if let Some(v) = t_str(
            &mut table,
            "user_values.preferred_username",
            "pattern_html",
            "",
        ) {
            self.user_values.preferred_username.pattern_html = v.into();
        }
        if let Some(v) = t_bool(
            &mut table,
            "user_values.preferred_username",
            "email_fallback",
            "",
        ) {
            self.user_values.preferred_username.email_fallback = v;
        }

        // linux username regex as fallback
        let _ = RE_PREFERRED_USERNAME.set(RE_LINUX_USERNAME.clone());
    }

    fn parse_webauthn(&mut self, table: &mut toml::Table) {
        let mut table = t_table(table, "webauthn");

        if let Some(v) = t_str(&mut table, "webauthn", "rp_id", "RP_ID") {
            self.webauthn.rp_id = v;
        }
        if let Some(v) = t_str(&mut table, "webauthn", "rp_origin", "RP_ORIGIN") {
            self.webauthn.rp_origin = v;
        }
        if let Some(v) = t_str(&mut table, "webauthn", "rp_name", "RP_NAME") {
            self.webauthn.rp_name = v.into();
        }
        if let Some(v) = t_u16(&mut table, "webauthn", "req_exp", "WEBAUTHN_REQ_EXP") {
            self.webauthn.req_exp = v;
        }
        if let Some(v) = t_u16(&mut table, "webauthn", "data_exp", "WEBAUTHN_DATA_EXP") {
            self.webauthn.data_exp = v;
        }
        if let Some(v) = t_u16(&mut table, "webauthn", "renew_exp", "WEBAUTHN_RENEW_EXP") {
            self.webauthn.renew_exp = v;
        }
        if let Some(v) = t_bool(&mut table, "webauthn", "force_uv", "WEBAUTHN_FORCE_UV") {
            self.webauthn.force_uv = v;
        }
        if let Some(v) = t_bool(
            &mut table,
            "webauthn",
            "no_password_exp",
            "WEBAUTHN_NO_PASSWORD_EXPIRY",
        ) {
            self.webauthn.no_password_exp = v;
        }
    }

    fn validate(&self) {
        if !self.database.hiqlite
            && (self.database.pg_host.is_none()
                || self.database.pg_user.is_none()
                || self.database.pg_password.is_none())
        {
            panic!("Database set to Postgres but missing `database.pg_*` config");
        }

        if self.device_grant.user_code_length > 255 {
            panic!("device_grant.user_code_length must be <=255");
        }

        if self.dynamic_clients.enable && self.dynamic_clients.reg_token.is_none() {
            warn!(
                "Open dynamic client registration - consider setting a registration token, if possible."
            );
        }

        if self.email.smtp_conn_mode == SmtpConnMode::XOauth2
            || self.email.smtp_conn_mode == SmtpConnMode::MicrosoftGraph
        {
            if self.email.xoauth_url.is_none() {
                panic!("'xoauth_url' not set");
            }
            if self.email.xoauth_client_id.is_none() {
                panic!("'xoauth_client_id' not set");
            }
            if self.email.xoauth_client_secret.is_none() {
                panic!("'xoauth_client_secret' not set");
            }
            if self.email.xoauth_scope.is_none() {
                panic!("'xoauth_scope' not set");
            }
        }
        if self.email.smtp_conn_mode == SmtpConnMode::MicrosoftGraph
            && self.email.microsoft_graph_uri.is_none()
        {
            panic!("'microsoft_graph_uri' not set");
        }

        if self.encryption.keys.is_empty() || self.encryption.key_active.is_empty() {
            panic!("Missing `encryption.keys` / `encryption.key_active`");
        }

        if self.geo.block_is_whitelist.unwrap_or(false) && self.geo.country_list.is_empty() {
            panic!(
                "`geolocation.country_list_type` is set to `whitelist` but \
                `geolocation.country_list` is empty, this will not work, because it would block \
                all requests."
            );
        }

        if self.server.pub_url.is_empty() {
            panic!("Empty `server.pub_url`");
        }
        if self.server.pub_url.contains("://") {
            panic!(
                "The `server.pub_url` must not contain the Scheme. Rauthy builds it automatically \
                depending on a few other values."
            );
        }

        if self.server.proxy_mode && self.server.trusted_proxies.is_empty() {
            panic!("`server.proxy_mode` is set but `server.trusted_proxies` is empty");
        }

        if self.webauthn.rp_id.is_empty() {
            panic!("`webauthn.rp_id` is missing");
        }
        if self.webauthn.rp_origin.is_empty() {
            panic!("`webauthn.rp_origin` is missing");
        }
        let (_, webauthn_port) = self
            .webauthn
            .rp_origin
            .rsplit_once(':')
            .expect("Invalid format for `webauthn.rp_origin` - missing port");
        if webauthn_port.parse::<u16>().is_err() {
            panic!("Invalid value for `webauthn.rp_origin` port");
        }
    }
}

#[derive(Debug)]
pub struct VarsDev {
    pub dev_mode: bool,
    pub dpop_http: bool,
    pub insecure_cookie: bool,
    pub provider_callback_url: Option<String>,
}

#[derive(Debug)]
pub struct VarsAccess {
    pub userinfo_strict: bool,
    pub danger_disable_introspect_auth: bool,
    pub disable_refresh_token_nbf: bool,
    pub sec_header_block: bool,
    pub session_validate_ip: bool,
    pub password_reset_cookie_binding: bool,
    pub peer_ip_header_name: Option<String>,
    pub cookie_mode: CookieMode,
    pub cookie_set_path: bool,
    pub client_credentials_map_sub: bool,
    pub token_len_limit: u32,
    pub token_revoke_on_logout: bool,
    pub token_revoke_device_tokens: bool,
    pub whoami_headers: bool,
}

#[derive(Debug)]
pub struct VarsAuthHeaders {
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

#[derive(Debug)]
pub struct VarsBackchannelLogout {
    pub retry_count: u16,
    pub danger_allow_http: bool,
    pub danger_allow_insecure: bool,
    pub token_lifetime: u32,
    pub allow_clock_skew: u32,
    pub allowed_token_lifetime: u32,
}

#[derive(Debug)]
pub struct VarsBootstrap {
    pub admin_email: String,
    pub password_plain: Option<String>,
    pub pasword_argon2id: Option<String>,
    pub api_key: Option<String>,
    pub api_key_secret: Option<String>,
}

#[derive(Debug)]
pub struct VarsDatabase {
    pub hiqlite: bool,
    pub health_check_delay_secs: u32,

    pub pg_host: Option<String>,
    pub pg_user: Option<String>,
    pub pg_port: u16,
    pub pg_password: Option<String>,
    pub pg_db_name: Cow<'static, str>,
    pub pg_tls_no_verify: bool,
    pub pg_max_conn: u16,

    pub migrate_pg_host: Option<String>,
    pub migrate_pg_port: u16,
    pub migrate_pg_user: Option<String>,
    pub migrate_pg_password: Option<String>,
    pub migrate_pg_db_name: Cow<'static, str>,

    pub sched_user_exp_mins: u32,
    pub sched_user_exp_delete_mins: Option<u32>,
}

#[derive(Debug)]
pub struct VarsDeviceGrant {
    pub code_lifetime: u32,
    pub user_code_length: u32,
    pub rate_limit: Option<u32>,
    pub poll_interval: u32,
    pub refresh_token_lifetime: u32,
}

#[derive(Debug)]
pub struct VarsDpop {
    pub force_nonce: bool,
    pub nonce_exp: u32,
}

#[derive(Debug)]
pub struct VarsDynamicClients {
    pub enable: bool,
    pub reg_token: Option<String>,
    pub default_token_lifetime: u32,
    pub secret_auto_rotate: bool,
    pub cleanup_interval: u32,
    pub cleanup_minutes: u32,
    pub rate_limit_sec: u32,
}

#[derive(Debug)]
pub struct VarsEmail {
    pub rauthy_admin_email: Option<String>,
    pub sub_prefix: Cow<'static, str>,
    pub smtp_url: Option<String>,
    pub smtp_port: Option<u16>,
    pub smtp_username: Option<String>,
    pub smtp_password: Option<String>,
    pub smtp_from: Cow<'static, str>,
    pub connect_retries: u16,
    pub jobs: VarsEmailJobs,
    pub smtp_conn_mode: SmtpConnMode,
    pub xoauth_url: Option<String>,
    pub xoauth_client_id: Option<String>,
    pub xoauth_client_secret: Option<String>,
    pub xoauth_scope: Option<String>,
    pub microsoft_graph_uri: Option<String>,
    pub root_ca: Option<String>,
    pub starttls_only: bool,
    pub danger_insecure: bool,
    pub tz_fmt: VarsEmailTzFmt,
}

#[derive(Debug)]
pub struct VarsEmailJobs {
    pub orphaned_seconds: u32,
    pub scheduler_interval_seconds: u32,
    pub batch_size: u16,
    pub batch_delay_ms: u32,
}

#[derive(Debug)]
pub struct VarsEmailTzFmt {
    pub de: Cow<'static, str>,
    pub en: Cow<'static, str>,
    pub ko: Cow<'static, str>,
    pub no: Cow<'static, str>,
    pub uk: Cow<'static, str>,
    pub zhhans: Cow<'static, str>,
    pub tz_fallback: Cow<'static, str>,
}

#[derive(Debug)]
pub struct VarsEncryption {
    pub key_active: String,
    pub keys: Vec<String>,
}

#[derive(Debug)]
pub struct VarsEphemeralClients {
    pub enable: bool,
    pub enable_web_id: bool,
    pub enable_solid_aud: bool,
    pub force_mfa: bool,
    pub allowed_flows: Vec<Cow<'static, str>>,
    pub allowed_scopes: Vec<Cow<'static, str>>,
    pub cache_lifetime: u32,
}

#[derive(Debug)]
pub struct VarsEvents {
    pub email: Option<String>,

    pub matrix_user_id: Option<String>,
    pub matrix_room_id: Option<String>,
    pub matrix_access_token: Option<String>,
    pub matrix_user_password: Option<String>,
    pub matrix_server_url: Cow<'static, str>,
    pub matrix_root_ca_path: Option<String>,
    pub matrix_danger_disable_tls_validation: bool,
    pub matrix_error_no_panic: bool,

    pub slack_webhook: Option<String>,

    pub notify_level_email: EventLevel,
    pub notify_level_matrix: EventLevel,
    pub notify_level_slack: EventLevel,
    pub persist_level: EventLevel,
    pub cleanup_days: u32,
    pub generate_token_issued: bool,

    pub level_new_user: EventLevel,
    pub level_user_email_change: EventLevel,
    pub level_user_password_reset: EventLevel,
    pub level_rauthy_admin: EventLevel,
    pub level_rauthy_version: EventLevel,
    pub level_jwks_rotate: EventLevel,
    pub level_secrets_migrated: EventLevel,
    pub level_rauthy_start: EventLevel,
    pub level_rauthy_healthy: EventLevel,
    pub level_rauthy_unhealthy: EventLevel,
    pub level_ip_blacklisted: EventLevel,
    pub level_backchannel_logout_failed: EventLevel,
    pub level_force_logout: EventLevel,
    pub level_user_login_revoke: EventLevel,
    pub level_scim_task_failed: EventLevel,
    pub level_suspicious_request: EventLevel,
    pub level_new_login_location: EventLevel,
    pub level_token_issued: EventLevel,

    pub level_failed_logins_25: EventLevel,
    pub level_failed_logins_20: EventLevel,
    pub level_failed_logins_15: EventLevel,
    pub level_failed_logins_10: EventLevel,
    pub level_failed_logins_7: EventLevel,
    pub level_failed_login: EventLevel,

    pub disable_app_version_check: bool,
}

#[derive(Debug)]
pub struct ConfigVarsFedCM {
    pub experimental_enable: bool,
    pub session_lifetime: u32,
    pub session_timeout: u32,
}

#[derive(Debug)]
pub struct ConfigVarsGeo {
    pub block_is_whitelist: Option<bool>,
    pub block_unknown: bool,
    pub country_header: Option<String>,
    pub country_list: Vec<String>,
    pub maxmind_account_id: Option<String>,
    pub maxmind_license_key: Option<String>,
    pub maxmind_db_dir: Cow<'static, str>,
    pub maxmind_db_type: Cow<'static, str>,
    pub maxmind_update_cron: Cow<'static, str>,
}

#[derive(Debug)]
pub struct VarsHashing {
    pub argon2_m_cost: u32,
    pub argon2_t_cost: u32,
    pub argon2_p_cost: u32,
    pub max_hash_threads: u32,
    pub hash_await_warn_time: u32,
}

#[derive(Debug)]
pub struct VarsHttpClient {
    pub connect_timeout: u32,
    pub request_timeout: u32,
    pub min_tls: Cow<'static, str>,
    pub idle_timeout: u32,
    pub danger_unencrypted: bool,
    pub danger_insecure: bool,
    pub root_ca_bundle: Option<String>,
}

#[derive(Debug)]
pub struct VarsI18n {
    pub filter_lang_common: Vec<Cow<'static, str>>,
    pub filter_lang_admin: Vec<Cow<'static, str>>,
}

#[derive(Debug)]
pub struct VarsLifetimes {
    pub refresh_token_grace_time: u16,
    pub refresh_token_lifetime: u16,
    pub session_lifetime: u32,
    pub session_renew_mfa: bool,
    pub session_timeout: u32,
    pub magic_link_pwd_reset: u32,
    pub magic_link_pwd_first: u32,
    pub jwk_autorotate_cron: Cow<'static, str>,
}

#[derive(Debug)]
pub struct VarsLogging {
    pub level: Cow<'static, str>,
    pub level_database: Cow<'static, str>,
    pub level_access: Cow<'static, str>,
    pub log_fmt: Cow<'static, str>,
}

#[derive(Debug)]
pub struct VarsMfa {
    pub admin_force_mfa: bool,
}

#[derive(Debug)]
pub struct VarsPam {
    pub remote_password_len: u8,
    pub remote_password_ttl: u16,
    pub authorized_keys: VarsPamAuthorizedKeys,
}

#[derive(Debug)]
pub struct VarsPamAuthorizedKeys {
    pub authorized_keys_enable: bool,
    pub auth_required: bool,
    pub blacklist_used_keys: bool,
    pub blacklist_cleanup_days: u16,
    pub include_comments: bool,
    pub forced_key_expiry_days: u16,
}

#[derive(Debug)]
pub struct VarsPow {
    pub difficulty: u8,
    pub exp: u16,
}

#[derive(Debug)]
pub struct VarsScim {
    pub sync_delete_groups: bool,
    pub sync_delete_users: bool,
    pub retry_count: u16,
}

#[derive(Debug)]
pub struct VarsServer {
    pub listen_address: Cow<'static, str>,
    pub port_http: u16,
    pub port_https: u16,
    pub scheme: Cow<'static, str>,
    pub pub_url: String,
    pub http_workers: u16,
    pub proxy_mode: bool,
    pub trusted_proxies: Vec<String>,
    pub additional_allowed_origin_schemes: Vec<String>,
    pub metrics_enable: bool,
    pub metrics_addr: Cow<'static, str>,
    pub metrics_port: u16,
    pub swagger_ui_enable: bool,
    pub swagger_ui_public: bool,
    pub see_keep_alive: u16,
    pub ssp_threshold: u16,
}

#[derive(Debug)]
pub struct VarsSuspiciousRequests {
    pub blacklist: u16,
    pub log: bool,
}

#[derive(Debug)]
pub struct VarsTemplates {
    pub password_new_de: VarsTemplate,
    pub password_new_en: VarsTemplate,
    pub password_new_ko: VarsTemplate,
    pub password_new_nb: VarsTemplate,
    pub password_new_uk: VarsTemplate,
    pub password_new_zhhans: VarsTemplate,

    pub password_reset_de: VarsTemplate,
    pub password_reset_en: VarsTemplate,
    pub password_reset_ko: VarsTemplate,
    pub password_reset_nb: VarsTemplate,
    pub password_reset_uk: VarsTemplate,
    pub password_reset_zhhans: VarsTemplate,
}

#[derive(Debug, Clone)]
pub struct VarsTemplate {
    pub subject: Cow<'static, str>,
    pub header: Cow<'static, str>,
    pub text: Option<String>,
    pub click_link: Cow<'static, str>,
    pub validity: Cow<'static, str>,
    pub expires: Cow<'static, str>,
    pub button: Cow<'static, str>,
    pub footer: Option<String>,
    pub button_text_request_new: Option<Cow<'static, str>>,
}

#[derive(Debug)]
pub struct VarsTls {
    pub cert_path: Option<String>,
    pub key_path: Option<String>,
    pub generate_self_signed: bool,
}

#[derive(Debug)]
pub struct VarsToS {
    pub accept_timeout: u16,
}

#[derive(Debug)]
pub struct VarsUserDelete {
    pub enable_self_delete: bool,
}

#[derive(Debug)]
pub struct VarsUserPictures {
    pub storage_type: Cow<'static, str>,
    pub path: Cow<'static, str>,
    pub s3_url: Option<String>,
    pub bucket: Option<String>,
    pub region: Option<String>,
    pub s3_key: Option<String>,
    pub s3_secret: Option<String>,
    pub s3_path_style: bool,
    pub upload_limit_mb: u16,
    pub public: bool,
}

#[derive(Debug)]
pub struct VarsUserRegistration {
    pub enable: bool,
    pub domain_restriction: Option<String>,
    pub domain_blacklist: Vec<String>,
    pub allow_open_redirect: bool,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VarsUserValuesConfig {
    pub given_name: UserValueConfigValue,
    pub family_name: UserValueConfigValue,
    pub birthdate: UserValueConfigValue,
    pub street: UserValueConfigValue,
    pub zip: UserValueConfigValue,
    pub city: UserValueConfigValue,
    pub country: UserValueConfigValue,
    pub phone: UserValueConfigValue,
    pub tz: UserValueConfigValue,
    pub revalidate_during_login: bool,
    pub preferred_username: VarsUserPreferredUsername,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VarsUserPreferredUsername {
    pub preferred_username: UserValueConfigValue,
    pub immutable: bool,
    pub blacklist: Vec<Cow<'static, str>>,
    // The Rust regex will not be saved here, but as a dedicated static
    // value instead, so it can easily be used with the `validator` crate
    // in macros.
    // pub regex_rust: String,
    pub pattern_html: Cow<'static, str>,
    pub email_fallback: bool,
}

#[derive(Debug, PartialEq, Serialize, ToSchema)]
#[serde(rename_all = "lowercase")]
pub enum UserValueConfigValue {
    Required,
    Optional,
    Hidden,
}

impl From<&str> for UserValueConfigValue {
    fn from(s: &str) -> Self {
        match s {
            "required" => Self::Required,
            "optional" => Self::Optional,
            "hidden" => Self::Hidden,
            _ => panic!("Invalid value for `UserValueConfig` in `user_values`: {s}"),
        }
    }
}

#[derive(Debug)]
pub struct VarsWebauthn {
    pub rp_id: String,
    pub rp_origin: String,
    pub rp_name: Cow<'static, str>,
    pub req_exp: u16,
    pub data_exp: u16,
    pub renew_exp: u16,
    pub force_uv: bool,
    pub no_password_exp: bool,
}

#[derive(Debug)]
pub struct VarsAtproto {
    pub enable: bool,
}

fn t_bool(map: &mut toml::Table, parent: &str, key: &str, env_var: &str) -> Option<bool> {
    if !env_var.is_empty()
        && let Ok(v) = env::var(env_var)
            .as_deref()
            .map(|v| match v.parse::<bool>() {
                Ok(b) => b,
                Err(_) => {
                    panic!("{}", err_env(env_var, "bool"));
                }
            })
    {
        return Some(v);
    }

    let Value::Boolean(b) = map.remove(key)? else {
        panic!("{}", err_t(key, parent, "bool"));
    };
    Some(b)
}

fn t_i64(map: &mut toml::Table, parent: &str, key: &str, env_var: &str) -> Option<i64> {
    if !env_var.is_empty()
        && let Ok(v) = env::var(env_var)
            .as_deref()
            .map(|v| match v.parse::<i64>() {
                Ok(b) => b,
                Err(_) => {
                    panic!("{}", err_env(env_var, "Integer"));
                }
            })
    {
        return Some(v);
    }

    let Value::Integer(i) = map.remove(key)? else {
        panic!("{}", err_t(key, parent, "bool"));
    };
    Some(i)
}

fn t_u32(map: &mut toml::Table, parent: &str, key: &str, env_overwrite: &str) -> Option<u32> {
    if let Some(v) = t_i64(map, parent, key, env_overwrite) {
        if v < 0 || v > u32::MAX as i64 {
            panic!("{}", err_t(key, parent, "u32"));
        }
        Some(v as u32)
    } else {
        None
    }
}
fn t_u16(map: &mut toml::Table, parent: &str, key: &str, env_overwrite: &str) -> Option<u16> {
    if let Some(v) = t_i64(map, parent, key, env_overwrite) {
        if v < 0 || v > u16::MAX as i64 {
            panic!("{}", err_t(key, parent, "u16"));
        }
        Some(v as u16)
    } else {
        None
    }
}
fn t_u8(map: &mut toml::Table, parent: &str, key: &str, env_overwrite: &str) -> Option<u8> {
    if let Some(v) = t_i64(map, parent, key, env_overwrite) {
        if v < 0 || v > u8::MAX as i64 {
            panic!("{}", err_t(key, parent, "u16"));
        }
        Some(v as u8)
    } else {
        None
    }
}

pub fn t_str(map: &mut toml::Table, parent: &str, key: &str, env_var: &str) -> Option<String> {
    if !env_var.is_empty()
        && let Ok(v) = env::var(env_var)
    {
        return Some(v);
    }
    let Value::String(s) = map.remove(key)? else {
        panic!("{}", err_t(key, parent, "String"));
    };
    Some(s)
}

fn t_str_vec(map: &mut toml::Table, parent: &str, key: &str, env_var: &str) -> Option<Vec<String>> {
    if !env_var.is_empty()
        && let Ok(arr) = env::var(env_var)
    {
        return Some(
            arr.lines()
                .filter_map(|l| {
                    let trimmed = l.trim().to_string();
                    if trimmed.is_empty() {
                        None
                    } else {
                        Some(trimmed)
                    }
                })
                .collect(),
        );
    }

    let Value::Array(arr) = map.remove(key)? else {
        return None;
    };
    let mut res = Vec::with_capacity(arr.len());
    for value in arr {
        let Value::String(s) = value else {
            panic!("{}", err_t(key, parent, "String"));
        };
        res.push(s);
    }
    Some(res)
}

pub fn t_table(map: &mut toml::Table, key: &str) -> toml::Table {
    match map.remove(key) {
        None => toml::Table::default(),
        Some(Value::Table(t)) => t,
        Some(_) => {
            panic!("Expected type `Table` for {key}")
        }
    }
}

#[inline]
fn err_env(var_name: &str, typ: &str) -> String {
    format!("Cannot parse {var_name} as `{typ}`")
}

#[inline]
pub fn err_t(key: &str, parent: &str, typ: &str) -> String {
    let sep = if parent.is_empty() { "" } else { "." };
    format!("Expected type `{typ}` for {parent}{sep}{key}")
}
