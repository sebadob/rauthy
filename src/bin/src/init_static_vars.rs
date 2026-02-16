use rauthy_common::compression::*;
use rauthy_common::constants::*;
use rauthy_common::password_hasher::*;
use rauthy_common::regex::*;
use rauthy_common::utils::build_trusted_proxies;
use rauthy_common::{DB_TYPE, DbType, HTTP_CLIENT};
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_handlers::generic::{I18N_CONFIG, TIMEZONES_BR};
use regex::Regex;
use reqwest::tls;
use std::time::Duration;
use tracing::{debug, warn};

/// The only job of this function is to trigger the `LazyLock` init for some values that will be
/// used all the time anyway. When this is triggered at the very start of the application, the
/// beginning of the Heap will be much more compact and we will have a bit less fragmentation down
/// the road.
///
/// The other advantage is, that we want to make Rauthy panic at startup immediately if any
/// configuration values are invalid, instead of some time later, when a value is lazily
/// initialized.
///
/// Excludes some values that are probably not used in most standard scenarios.
pub fn trigger() {
    let vars = &RauthyConfig::get().vars;
    // special handling for some to avoid circular dependencies
    {
        let additional_schemes = vars.server.additional_allowed_origin_schemes.join("|");
        let pattern = if additional_schemes.is_empty() {
            r"^(http|https)://[a-z0-9.:-]+$".to_string()
        } else {
            format!("^(http|https|{additional_schemes})://[a-z0-9.:-]+$")
        };
        RE_ORIGIN.set(Regex::new(&pattern).unwrap()).unwrap()
    }

    {
        let scheme = if vars.dev.dev_mode && vars.dev.dpop_http {
            "http"
        } else {
            "https"
        };
        let uri = format!("{scheme}://{}/auth/v1/oidc/token", vars.server.pub_url);
        DPOP_TOKEN_ENDPOINT.set(uri).unwrap()
    }

    DEV_MODE.set(vars.dev.dev_mode).unwrap();

    let db_type = {
        if vars.database.hiqlite {
            DbType::Hiqlite
        } else {
            DbType::Postgres
        }
    };
    DB_TYPE.set(db_type).unwrap();

    PEER_IP_HEADER_NAME
        .set(vars.access.peer_ip_header_name.clone())
        .unwrap();
    PROXY_MODE.set(vars.server.proxy_mode).unwrap();
    TRUSTED_PROXIES
        .set(build_trusted_proxies(&vars.server.trusted_proxies))
        .unwrap();

    ARGON2_PARAMS
        .set(RauthyConfig::get().argon2_params.clone())
        .unwrap();
    HASH_CHANNELS
        .set(flume::bounded(vars.hashing.max_hash_threads as usize))
        .unwrap();
    HASH_AWAIT_WARN_TIME
        .set(vars.hashing.hash_await_warn_time)
        .unwrap();

    let http_client = {
        let tls_version = match vars.http_client.min_tls.as_ref() {
            "1.3" => tls::Version::TLS_1_3,
            "1.2" => tls::Version::TLS_1_2,
            "1.1" => {
                warn!(
                    r#"
    You are allowing TLS 1.1 for the global HTTP client.
    Only do this, if you know what you are doing!
    "#
                );
                tls::Version::TLS_1_1
            }
            "1.0" => {
                warn!(
                    r#"
    You are allowing TLS 1.0 for the global HTTP client.
    Only do this, if you know what you are doing!
    "#
                );
                tls::Version::TLS_1_0
            }
            _ => panic!("Invalid value for HTTP_MIN_TLS, allowed: '1.3', '1.2', '1.1', '1.0'"),
        };

        let mut builder = reqwest::Client::builder()
            .connect_timeout(Duration::from_secs(vars.http_client.connect_timeout as u64))
            .timeout(Duration::from_secs(vars.http_client.request_timeout as u64))
            .pool_idle_timeout(Duration::from_secs(vars.http_client.idle_timeout as u64))
            .min_tls_version(tls_version)
            .user_agent(format!("Rauthy Client v{RAUTHY_VERSION}"))
            .https_only(!vars.http_client.danger_unencrypted || !vars.dev.dev_mode)
            .danger_accept_invalid_certs(vars.http_client.danger_insecure || vars.dev.dev_mode)
            .use_rustls_tls();

        if let Some(bundle) = vars.http_client.root_ca_bundle.as_ref() {
            let certs = reqwest::Certificate::from_pem_bundle(bundle.trim().as_bytes())
                .expect("Cannot parse given HTTP_CUST_ROOT_CA_BUNDLE");
            debug!(
                "Adding {} custom Root CA certificates to HTTP Client",
                certs.len()
            );

            for cert in certs {
                builder = builder.add_root_certificate(cert);
            }
        }

        builder.build().unwrap()
    };
    HTTP_CLIENT.set(http_client).unwrap();

    // constants
    let _ = *APP_START;
    let _ = *BUILD_TIME;

    // regexes
    let _ = *RE_ALNUM;
    let _ = *RE_ALNUM_48;
    let _ = *RE_ALNUM_64;
    let _ = *RE_API_KEY;
    let _ = *RE_APP_ID;
    let _ = *RE_ATTR;
    let _ = *RE_ATTR_DESC;
    let _ = *RE_BASE64;
    let _ = *RE_CODE_CHALLENGE_METHOD;
    let _ = *RE_CITY;
    if vars.ephemeral_clients.enable {
        let _ = *RE_CLIENT_ID;
    }
    let _ = *RE_CLIENT_NAME;
    let _ = *RE_CODE_CHALLENGE;
    let _ = *RE_CODE_VERIFIER;
    let _ = *RE_CONTACT;
    let _ = *RE_CSS_VALUE_LOOSE;
    let _ = *RE_DATE_STR;
    let _ = *RE_GRANT_TYPES;
    let _ = *RE_GRANT_TYPES_EPHEMERAL;
    let _ = *RE_GROUPS;
    let _ = *RE_ROLES_SCOPES;
    let _ = *RE_LOWERCASE;
    let _ = *RE_LOWERCASE_SPACE;
    let _ = *RE_MFA_CODE;
    let _ = *RE_PHONE;
    let _ = *RE_SCOPE_SPACE;
    let _ = *RE_SEARCH;
    let _ = *RE_STREET;
    let _ = *RE_URI;
    let _ = *RE_USER_NAME;
    let _ = *RE_TOKEN_68;
    let _ = *RE_TOKEN_ENDPOINT_AUTH_METHOD;

    // lazy values in other places
    let _ = *BROTLI_PARAMS;
    let _ = *BROTLI_PARAMS_9;
    let _ = *BROTLI_PARAMS_DYN;

    let _ = *I18N_CONFIG;
    let _ = *TIMEZONES_BR;
}
