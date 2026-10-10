use rauthy_common::compression::*;
use rauthy_common::constants::*;
use rauthy_common::password_hasher::*;
use rauthy_common::regex::*;
use rauthy_common::utils::build_trusted_proxies;
use rauthy_common::{DB_TYPE, DbType, HTTP_CLIENT};
use rauthy_data::rauthy_config::RauthyConfig;
use rauthy_handlers::generic::{I18N_CONFIG, TIMEZONES_BR};
use regex::Regex;

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
pub async fn trigger() {
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
    HASH_AWAIT_WARN_SECS
        .set(vars.hashing.hash_await_warn_time.as_secs())
        .unwrap();

    let http_client = rauthy_data::http_client::http_client_builder()
        .build()
        .expect("Cannot build global HTTP client");
    HTTP_CLIENT.set(http_client).unwrap();
    // fail the boot on a bad TLS / resolver config instead of the first CIMD request
    if vars.ephemeral_clients.enable {
        rauthy_data::http_client::init_ephemeral_fetcher();
    }

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
    let _ = *RE_BASE64_NO_PAD;
    let _ = *RE_CODE_CHALLENGE_METHOD;
    let _ = *RE_CITY;
    if vars.ephemeral_clients.enable {
        let _ = *RE_CLIENT_ID;
    }
    let _ = *RE_CLIENT_ID_STRICT;
    let _ = *RE_CLIENT_NAME;
    let _ = *RE_CLIENT_URI;
    let _ = *RE_CODE_CHALLENGE;
    let _ = *RE_CODE_VERIFIER;
    let _ = *RE_CONTACT;
    let _ = *RE_CSS_VALUE_LOOSE;
    let _ = *RE_DATE_STR;
    let _ = *RE_GROUPS;
    let _ = *RE_KV_KEY;
    let _ = *RE_ROLES;
    let _ = *RE_SCOPES;
    let _ = *RE_NAME_ASCII;
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

    let _ = *I18N_CONFIG;

    let zones = chrono_tz::TZ_VARIANTS
        .iter()
        .map(|tz| tz.name())
        .collect::<Vec<_>>();

    let json = serde_json::to_string(&zones).unwrap();
    let data = compress_br(json.as_bytes()).await.unwrap();
    TIMEZONES_BR.set(data.clone()).unwrap();
}
