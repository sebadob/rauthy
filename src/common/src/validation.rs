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
fn bad_request<T: Into<Cow<'static, str>>>(msg: T) -> Result<Url, ErrorResponse> {
    Err(ErrorResponse::new(ErrorResponseType::BadRequest, msg))
}

#[cfg(test)]
mod tests {
    use super::*;

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
