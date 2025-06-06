use actix_web::http::Uri;
use rauthy_common::HTTP_CLIENT;
use rauthy_common::compression::*;
use rauthy_common::constants::*;
use rauthy_common::password_hasher::*;
use rauthy_common::regex::*;
use rauthy_common::utils::build_trusted_proxies;
use rauthy_handlers::generic::I18N_CONFIG;
use rauthy_models::rauthy_config::RauthyConfig;
use regex::Regex;
use std::str::FromStr;

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

    // constants
    let _ = *APP_START;
    let _ = *BUILD_TIME;
    let _ = *DB_TYPE;

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
        let _ = *RE_CLIENT_ID_EPHEMERAL;
    }
    let _ = *RE_CLIENT_NAME;
    let _ = *RE_CODE_CHALLENGE;
    let _ = *RE_CODE_VERIFIER;
    let _ = *RE_CONTACT;
    let _ = *RE_CSS_VALUE_LOOSE;
    let _ = *RE_DATE_STR;
    let _ = *RE_GRANT_TYPES;
    let _ = *RE_GRANT_TYPES_EPHEMERAL;
    let _ = *RE_GROUPS_ROLES_SCOPES;
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
    let _ = *ARGON2_PARAMS;
    let _ = *HASH_AWAIT_WARN_TIME;
    let _ = *HASH_CHANNELS;

    let _ = *BROTLI_PARAMS;
    let _ = *BROTLI_PARAMS_9;
    let _ = *BROTLI_PARAMS_DYN;

    let _ = *I18N_CONFIG;
    let _ = *HTTP_CLIENT;

    // special handling for some to avoid circular dependencies
    {
        let additional_schemes = vars.server.additional_allowed_origin_schemes.join("|");
        let pattern = if additional_schemes.is_empty() {
            r"^(http|https)://[a-z0-9.:-]+$".to_string()
        } else {
            format!("^(http|https|{})://[a-z0-9.:-]+$", additional_schemes)
        };
        RE_ORIGIN.set(Regex::new(&pattern).unwrap()).unwrap()
    }
    {
        let scheme = if vars.dev.dev_mode && vars.dev.dpop_http {
            "http"
        } else {
            "https"
        };
        let uri = format!("{}://{}/auth/v1/oidc/token", scheme, vars.server.pub_url);
        DPOP_TOKEN_ENDPOINT
            .set(Uri::from_str(&uri).unwrap())
            .unwrap()
    }
    DEV_MODE.set(vars.dev.dev_mode).unwrap();
    PEER_IP_HEADER_NAME
        .set(vars.access.peer_ip_header_name.clone())
        .unwrap();
    PROXY_MODE.set(vars.server.proxy_mode).unwrap();
    TRUSTED_PROXIES
        .set(build_trusted_proxies(&vars.server.trusted_proxies))
        .unwrap();
}
