use regex::Regex;
use std::sync::{LazyLock, OnceLock};

pub static RE_ALNUM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9]+$").unwrap());
pub static RE_ALNUM_48: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9]{48}$").unwrap());
pub static RE_ALNUM_64: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9]{64}$").unwrap());
pub static RE_API_KEY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9_/-]{2,24}$").unwrap());
pub static RE_APP_ID: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9]{12}$").unwrap());
pub static RE_ATTR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-_/]{2,32}$").unwrap());
pub static RE_ATTR_DESC: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-_/\s]{0,128}$").unwrap());
pub static RE_BASE64: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9+/=]{4}$").unwrap());
pub static RE_CODE_CHALLENGE_METHOD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(plain|S256)$").unwrap());
pub static RE_CITY: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9À-ÿ-]{0,48}$").unwrap());
pub static RE_CLIENT_ID_EPHEMERAL: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,256}$").unwrap());
pub static RE_CLIENT_NAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9À-ɏ-\s\x{3041}-\x{3096}\x{30A0}-\x{30FF}\x{3400}-\x{4DB5}\x{4E00}-\x{9FCB}\x{F900}-\x{FA6A}\x{2E80}-\x{2FD5}\x{FF66}-\x{FF9F}\x{FFA1}-\x{FFDC}\x{31F0}-\x{31FF}]{2,128}$").unwrap()
});
pub static RE_CODE_CHALLENGE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-._~]{43,128}$").unwrap());
pub static RE_CODE_VERIFIER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-._~+/=]+$").unwrap());
pub static RE_CONTACT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9+.@/:]{0,48}$").unwrap());
pub static RE_CSS_VALUE_LOOSE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9-,.#()%/\s]+$").unwrap());
pub static RE_DATE_STR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$").unwrap());
pub static RE_GRANT_TYPES: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(authorization_code|client_credentials|urn:ietf:params:oauth:grant-type:device_code|password|refresh_token)$").unwrap()
});
pub static RE_GRANT_TYPES_EPHEMERAL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(authorization_code|client_credentials|password|refresh_token)$").unwrap()
});
pub static RE_LINUX_HOSTNAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9-.]*[a-zA-Z0-9]$").unwrap());
// slightly modified from the original: at least 2 characters and max 62 (we will apply a prefix)
pub static RE_LINUX_USERNAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z][a-z0-9_-]{1,61}$").unwrap());
pub static RE_ROLES_SCOPES: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9-_/,:*.]{2,64}$").unwrap());
pub static RE_GROUPS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-_/,:*\s]{2,64}$").unwrap());
pub static RE_LOWERCASE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9-_/]{2,128}$").unwrap());
pub static RE_LOWERCASE_SPACE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9-_/\s]{2,128}$").unwrap());
pub static RE_MFA_CODE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9]{48}$").unwrap());
pub static RE_ORIGIN: OnceLock<Regex> = OnceLock::new();
pub static RE_PHONE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\+[0-9]{0,32}$").unwrap());
pub static RE_PREFERRED_USERNAME: OnceLock<Regex> = OnceLock::new();
pub static RE_SCOPE_SPACE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9-_/:\s*.]{0,512}$").unwrap());
pub static RE_SEARCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%@]+$").unwrap());
pub static RE_STREET: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9À-ÿ-.\s]{0,48}$").unwrap());
pub static RE_URI: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]+$").unwrap());
pub static RE_USER_NAME: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^[a-zA-Z0-9À-ɏ-'\s\x{3041}-\x{3096}\x{30A0}-\x{30FF}\x{3400}-\x{4DB5}\x{4E00}-\x{9FCB}\x{F900}-\x{FA6A}\x{2E80}-\x{2FD5}\x{FF66}-\x{FF9F}\x{FFA1}-\x{FFDC}\x{31F0}-\x{31FF}]{1,32}$").unwrap()
});
pub static RE_TOKEN_68: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-._~+/]+=*$").unwrap());
pub static RE_TOKEN_ENDPOINT_AUTH_METHOD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(client_secret_post|client_secret_basic|none)$").unwrap());

pub static RE_ATPROTO_HANDLE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(did:[a-z]+:[a-zA-Z0-9._:%-]*[a-zA-Z0-9._-]|([a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)$").unwrap()
});
