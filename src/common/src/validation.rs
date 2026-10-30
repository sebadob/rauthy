use crate::constants::CONSENT_SOURCE_MAX_LEN;
use crate::regex::RE_RESOURCE;
use ammonia::Url;
use ammonia::url::form_urlencoded;
use rauthy_error::{ErrorResponse, ErrorResponseType};
use std::borrow::Cow;

/// Query keys an authorization response sets itself (RFC 6749 §4.1.2 / §4.1.2.1, RFC 9207).
pub const RESERVED_REDIRECT_QUERY_KEYS: [&str; 6] = [
    "code",
    "state",
    "error",
    "error_description",
    "error_uri",
    "iss",
];

/// Validates a redirect URI. Makes sure it is absolute and can be parsed successfully. Also
/// rejects any `#` or `,` in the URL, as well as reserved auth query params if
/// `validate_reserved_params`.
pub fn validate_redirect_uri(
    uri: &str,
    allow_wildcard: bool,
    validate_reserved_params: bool,
) -> Result<Url, ErrorResponse> {
    if !allow_wildcard && uri.ends_with('*') {
        return bad_request("wildcard `redirect_uri` not allowed");
    }
    if uri.contains(['#', ',']) {
        return bad_request("`redirect_uri` must not contain any of: # ,");
    }
    let Ok(url) = Url::parse(uri) else {
        return bad_request("cannot parse `redirect_uri`");
    };
    if url.host().is_none() {
        return bad_request("invalid `redirect_uri` - missing origin");
    }

    for pair in url.query().unwrap_or_default().split(['&', ';']) {
        let Some((key, _)) = form_urlencoded::parse(pair.as_bytes()).next() else {
            continue;
        };
        if key.chars().any(char::is_control) {
            return bad_request("`redirect_uri` must not contain control characters");
        }
        if validate_reserved_params
            && let Some(reserved) = reserved_query_key(&key, RESERVED_REDIRECT_QUERY_KEYS.as_ref())
        {
            return bad_request(format!(
                "`redirect_uri` must not contain reserved query param {reserved}"
            ));
        }
    }

    Ok(url)
}

/// Returns the key of `reserved_keys` a decoded query `key` would be folded into by common
/// parsers.
#[inline]
fn reserved_query_key(key: &str, reserved_keys: &[&'static str]) -> Option<&'static str> {
    let key = key.trim().to_ascii_lowercase();
    // PHP turns `.` and ` ` into `_`, and `code[]` / `iss[0]` become arrays under the plain key
    let normalized = key.replace(['.', ' '], "_");
    let base = normalized
        .split_once('[')
        .map_or(normalized.as_str(), |(base, _)| base);

    reserved_keys
        .iter()
        .copied()
        .find(|reserved| *reserved == base)
        .or_else(|| {
            // `qs` with `allowDots` nests `iss.x` under `iss`
            reserved_keys.iter().copied().find(|reserved| {
                key.strip_prefix(reserved)
                    .is_some_and(|rest| rest.starts_with(['[', '.']))
            })
        })
}

#[inline]
/// `value` must be an absolute `https://` URL with a host, without userinfo and without a
/// fragment.
pub fn validate_https_url(value: &str) -> Result<Url, String> {
    let url = Url::parse(value).map_err(|err| format!("invalid URL '{value}': {err}"))?;
    if url.scheme() != "https" {
        return Err(format!("'{value}' must be an https URL"));
    }
    if url.host_str().is_none_or(|h| h.is_empty()) {
        return Err(format!("'{value}' has no host"));
    }
    if !url.username().is_empty() || url.password().is_some() {
        return Err(format!("'{value}' must not contain userinfo"));
    }
    if url.fragment().is_some() {
        return Err(format!("'{value}' must not contain a fragment"));
    }
    Ok(url)
}

/// The grammar of a remote consent source URL, shared by the API validation and the runtime
/// parsing: at most [`CONSENT_SOURCE_MAX_LEN`] bytes of [`RE_RESOURCE`] characters, an URL
/// accepted by [`validate_https_url`], and no `lang` query key, which is appended on each fetch.
pub fn validate_consent_source_url(value: &str) -> Result<Url, String> {
    if value.len() > CONSENT_SOURCE_MAX_LEN {
        return Err(format!(
            "consent source URL must not exceed {CONSENT_SOURCE_MAX_LEN} bytes"
        ));
    }
    if !RE_RESOURCE.is_match(value) {
        return Err(format!("'{value}' must match `{}`", RE_RESOURCE.as_str()));
    }
    let url = validate_https_url(value)?;
    if url.query_pairs().any(|(k, _)| k == "lang") {
        return Err(format!(
            "'{value}' must not contain a `lang` query parameter, it is added by Rauthy"
        ));
    }
    Ok(url)
}

fn bad_request<T: Into<Cow<'static, str>>>(msg: T) -> Result<Url, ErrorResponse> {
    Err(ErrorResponse::new(ErrorResponseType::BadRequest, msg))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consent_source_url() {
        for v in [
            "https://consent.example.com",
            "https://consent.example.com/api?v=1&language=x",
            "https://127.0.0.1:8443/consent",
        ] {
            assert!(validate_consent_source_url(v).is_ok(), "{v}");
        }
        for v in [
            "",
            "local",
            "https://",
            "http://consent.example.com",
            "https://consent.example.com/#x",
            "https://consent.example.com/a b",
            "https://consent.example.com/a\nb",
            "https://u:p@consent.example.com/api",
            "https://u@consent.example.com/api",
            "https://:p@consent.example.com/api",
            "https://consent.example.com/api?lang=en",
            "https://consent.example.com/api?v=1&lang=",
        ] {
            assert!(validate_consent_source_url(v).is_err(), "{v}");
        }

        let prefix = "https://consent.example.com/";
        let max = format!(
            "{prefix}{}",
            "a".repeat(CONSENT_SOURCE_MAX_LEN - prefix.len())
        );
        assert!(validate_consent_source_url(&max).is_ok());
        assert!(validate_consent_source_url(&format!("{max}a")).is_err());
    }

    #[test]
    fn test_https_url_rejects_userinfo() {
        assert!(validate_https_url("https://example.com/terms").is_ok());
        assert!(validate_https_url("https://u:p@example.com/terms").is_err());
        assert!(validate_https_url("https://u@example.com/terms").is_err());
    }

    #[test]
    fn test_redirect_uri() {
        let uris = vec![
            "http://localhost:1234/callback",
            "https://localhost:33333/callback/secure",
            "http://localhost/callback",
            "https://localhost/callback/secure",
            "http://localhost/callback?query=param",
            "http://localhost/callback?query=param&another=one",
            "https://example.com",
            "https://example.com/",
            "https://example.com/cb",
            "tauri://my.app",
            "https://app.example.com/cb",
            "https://app.example.com/cb?foo=bar&x=y",
            "http://localhost:1234/cb?foo=bar",
            "https://app.example.com/cb?foo=iss&issuer=x&code_x=1",
        ];
        for uri in uris {
            validate_redirect_uri(uri, false, false).unwrap();
        }

        let uris = vec![
            "http://localhost/callback",
            "http://localhost:1234/*",
            "http://localhost/*",
            "https://localhost:33333/callback/*",
            "https://example.com/*",
            "https://example.com:8000/*",
            "tauri://my.app/*",
        ];
        for uri in uris {
            validate_redirect_uri(uri, true, false).unwrap();
        }

        let uris = vec![
            "http://localhost:1234/*",
            "http://localhost/*",
            "https://localhost:33333/callback/*",
            "https://example.com/*",
            "https://example.com:8000/*",
            "tauri://my.app/*",
        ];
        for uri in &uris {
            assert!(validate_redirect_uri(uri, false, false).is_err());
        }

        let good = vec![
            "https://example.com/?code=123",
            "https://example.com/?state=456",
            "https://example.com/?error=nonono",
            "https://example.com/?iss=evil.org",
            "https://app.example.com/cb?%69ss=x",
            "https://app.example.com/cb?x=1;iss=https://evil.example",
            "https://app.example.com/cb?x=1;%69ss=x",
            "https://app.example.com/*?x=1;state=x",
            "http://localhost/cb?%69ss=x",
            "http://localhost/cb?x=1;iss=x",
            "https://app.example.com/cb?+iss=x",
            "https://app.example.com/cb?%20iss=x",
        ];

        let bad = vec![
            // control characters in a decoded key
            "https://app.example.com/cb?%09iss=x",
            "https://app.example.com/cb?iss%00=x",
            "https://app.example.com/cb?x%0Ay=1",
            "https://app.example.com/cb?%7Fcode=x",
            // fragments are rejected, wildcard registrations included
            "https://app.example.com/#/callback",
            "https://app.example.com/cb#",
            "https://app.example.com/cb?foo=bar#/route",
            "https://app.example.com/cb#/callback?iss=https%3A%2F%2Fattacker.example%2F",
            "https://app.example.com/#/*",
            "http://localhost/cb#x",
        ];
        for uri in &good {
            validate_redirect_uri(uri, false, false).unwrap();
        }
        for uri in bad {
            assert!(validate_redirect_uri(uri, false, false).is_err());
        }

        for uri in good {
            assert!(validate_redirect_uri(uri, false, true).is_err());
        }
    }
}
