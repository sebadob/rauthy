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
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9À-ÿ-\p{Zs}]{0,48}$").unwrap());
pub static RE_CLIENT_ID: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%]{2,256}$").unwrap());
// Stricter pattern for manually managed (non-ephemeral) clients. Mirrors the admin UI's
// `PATTERN_CLIENT_ID_NEW`: alphanumeric plus `.`, `_`, `-`, so a client id can never be a URI.
// Ephemeral clients keep using `RE_CLIENT_ID`, which still allows full URI ids.
pub static RE_CLIENT_ID_STRICT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9._\-]{2,256}$").unwrap());
pub static RE_CLIENT_NAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[\p{L}\p{M}\p{N}\p{Zs}()._-]{2,128}$").unwrap());
pub static RE_CODE_CHALLENGE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-._~]{43,128}$").unwrap());
pub static RE_CODE_VERIFIER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-._~+/=]+$").unwrap());
pub static RE_CONTACT: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9+.@/:-]{0,48}$").unwrap());
pub static RE_CSS_VALUE_LOOSE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9-,.#()%/\s]+$").unwrap());
pub static RE_DATE_STR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[0-9]{4}-[0-9]{2}-[0-9]{2}$").unwrap());
pub static RE_LINUX_HOSTNAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9][a-zA-Z0-9-.]*[a-zA-Z0-9]$").unwrap());
// slightly modified from the original: at least 2 characters and max 62 (we will apply a prefix)
pub static RE_LINUX_USERNAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z][a-z0-9_-]{1,61}$").unwrap());
pub static RE_ROLES_SCOPES: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-_/,:*.]{2,64}$").unwrap());
pub static RE_GROUPS: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-_/,:*\s]{2,64}$").unwrap());
pub static RE_LOWERCASE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9-_/]{2,128}$").unwrap());
pub static RE_LOWERCASE_SPACE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-z0-9-_/\p{Zs}]{2,128}$").unwrap());
pub static RE_MFA_CODE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9]{48}$").unwrap());
pub static RE_ORIGIN: OnceLock<Regex> = OnceLock::new();
pub static RE_PHONE: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^\+[0-9]{0,32}$").unwrap());
pub static RE_PREFERRED_USERNAME: OnceLock<Regex> = OnceLock::new();
pub static RE_SCOPE_SPACE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-_/:\p{Zs}*.]{0,512}$").unwrap());
pub static RE_SEARCH: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%@]+$").unwrap());
pub static RE_STREET: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9À-ÿ-.\p{Zs}]{0,48}$").unwrap());
pub static RE_URI: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9,.:/_\-&?=~#!$'()*+%@]+$").unwrap());
// Like `RE_URI` but WITHOUT `#`, so a value cannot contain a fragment. Used for RFC 8707
// `resource` indicators, which MUST be an absolute URI without a fragment (RFC 8707 §2).
pub static RE_RESOURCE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9,.:/_\-&?=~!$'()*+%@]+$").unwrap());
pub static RE_USER_NAME: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[\p{L}\p{M}\p{N}\p{Zs}'.-]{1,32}$").unwrap());
pub static RE_TOKEN_68: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^[a-zA-Z0-9-._~+/]+=*$").unwrap());
pub static RE_TOKEN_ENDPOINT_AUTH_METHOD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"^(client_secret_post|client_secret_basic|none)$").unwrap());

pub static RE_ATPROTO_HANDLE: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"^(did:[a-z]+:[a-zA-Z0-9._:%-]*[a-zA-Z0-9._-]|([a-zA-Z0-9]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?\.)+[a-zA-Z]([a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)$").unwrap()
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_re_client_name() {
        assert!(RE_CLIENT_NAME.is_match("My Client 123"));
        assert!(RE_CLIENT_NAME.is_match("()_-"));
        assert!(RE_CLIENT_NAME.is_match("クライアント"));

        assert!(!RE_CLIENT_NAME.is_match("\r"));
        assert!(!RE_CLIENT_NAME.is_match("\n"));
        assert!(!RE_CLIENT_NAME.is_match("<script>"));
        assert!(!RE_CLIENT_NAME.is_match("😉"));
    }

    #[test]
    fn test_re_user_name() {
        assert!(RE_USER_NAME.is_match("My Name 123"));
        assert!(RE_USER_NAME.is_match("äöü"));
        assert!(RE_USER_NAME.is_match("user-name"));
        assert!(RE_USER_NAME.is_match("クライアント"));
        assert!(RE_USER_NAME.is_match("Виктория Ефанова Дарья Перминова"));

        assert!(!RE_USER_NAME.is_match("\r"));
        assert!(!RE_USER_NAME.is_match("\n"));
        assert!(!RE_USER_NAME.is_match("<script>"));
        assert!(!RE_USER_NAME.is_match("😉"));
    }
}
