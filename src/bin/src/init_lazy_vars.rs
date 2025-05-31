use rauthy_common::HTTP_CLIENT;
use rauthy_common::compression::*;
use rauthy_common::constants::*;
use rauthy_common::password_hasher::*;
use rauthy_common::regex::*;
use rauthy_handlers::generic::I18N_CONFIG;

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
    // constants
    let _ = *APP_START;
    let _ = *BUILD_TIME;
    let _ = *CACHE_TTL_AUTH_CODE;
    let _ = *CACHE_TTL_DEVICE_CODE;
    let _ = *CACHE_TTL_DYN_CLIENT;
    if *ENABLE_EPHEMERAL_CLIENTS {
        let _ = *CACHE_TTL_EPHEMERAL_CLIENT;
        let _ = *EPHEMERAL_CLIENTS_FORCE_MFA;
        let _ = *EPHEMERAL_CLIENTS_ALLOWED_FLOWS;
        let _ = *EPHEMERAL_CLIENTS_ALLOWED_SCOPES;
    }
    let _ = *CACHE_TTL_WEBAUTHN;
    let _ = *CACHE_TTL_WEBAUTHN_DATA;
    let _ = *ADDITIONAL_ALLOWED_ORIGIN_SCHEMES;
    let _ = *ADMIN_FORCE_MFA;
    if *AUTH_HEADERS_ENABLE {
        let _ = *AUTH_HEADER_USER;
        let _ = *AUTH_HEADER_ROLES;
        let _ = *AUTH_HEADER_GROUPS;
        let _ = *AUTH_HEADER_EMAIL;
        let _ = *AUTH_HEADER_EMAIL_VERIFIED;
        let _ = *AUTH_HEADER_FAMILY_NAME;
        let _ = *AUTH_HEADER_GIVEN_NAME;
        let _ = *AUTH_HEADER_MFA;
    }
    let _ = *COOKIE_MODE;
    let _ = *COOKIE_SET_PATH;
    let _ = *DANGER_DISABLE_INTROSPECT_AUTH;
    let _ = *DB_TYPE;
    if *ENABLE_DYN_CLIENT_REG {
        let _ = *DYN_CLIENT_REG_TOKEN;
        let _ = *DYN_CLIENT_DEFAULT_TOKEN_LIFETIME;
        let _ = *DYN_CLIENT_SECRET_AUTO_ROTATE;
        let _ = *DYN_CLIENT_CLEANUP_INTERVAL;
        let _ = *DYN_CLIENT_CLEANUP_MINUTES;
        let _ = *DYN_CLIENT_RATE_LIMIT_SEC;
    }
    let _ = *EMAIL_SUB_PREFIX;
    let _ = *ENABLE_SOLID_AUD;
    let _ = *ENABLE_WEB_ID;
    let _ = *HEALTH_CHECK_DELAY_SECS;
    let _ = *OPEN_USER_REG;
    let _ = *PASSWORD_RESET_COOKIE_BINDING;
    let _ = *PEER_IP_HEADER_NAME;
    let _ = *POW_EXP;
    let _ = *POW_DIFFICULTY;
    let _ = *POW_IT;
    if *PROXY_MODE {
        let _ = *TRUSTED_PROXIES;
    }
    let _ = *PUB_URL;
    let _ = *PUB_URL_WITH_SCHEME;
    let _ = *RAUTHY_ADMIN_EMAIL;
    let _ = *REFRESH_TOKEN_LIFETIME;
    let _ = *SEC_HEADER_BLOCK;
    let _ = *SESSION_LIFETIME;
    let _ = *SESSION_TIMEOUT;
    let _ = *SESSION_RENEW_MFA;
    let _ = *SESSION_VALIDATE_IP;
    let _ = *SMTP_FROM;
    if (*SMTP_URL).is_some() {
        let _ = *SMTP_FROM;
    }
    let _ = *SUSPICIOUS_REQUESTS_BLACKLIST;
    let _ = *SUSPICIOUS_REQUESTS_LOG;
    let _ = *SSE_KEEP_ALIVE;
    let _ = *SSP_THRESHOLD;
    let _ = *USER_REG_DOMAIN_RESTRICTION;
    let _ = *USER_REG_DOMAIN_BLACKLIST;
    let _ = *USER_REG_OPEN_REDIRECT;
    let _ = *USERINFO_STRICT;
    let _ = *WEBAUTHN_REQ_EXP;
    let _ = *WEBAUTHN_DATA_EXP;
    let _ = *WEBAUTHN_RENEW_EXP;
    let _ = *WEBAUTHN_FORCE_UV;
    let _ = *WEBAUTHN_NO_PASSWORD_EXPIRY;

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
    if *ENABLE_EPHEMERAL_CLIENTS {
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
    let _ = *RE_ORIGIN;
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
}
