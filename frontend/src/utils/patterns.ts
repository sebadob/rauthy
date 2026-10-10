export const PATTERN_ALNUM = '^[a-zA-Z0-9]*$';
// export const PATTERN_ALNUM_64 = '^[a-zA-Z0-9]{64}$';
export const PATTERN_ATPROTO_ID =
    '^(?:@?([a-zA-Z0-9]([a-zA-Z0-9\\-]{0,61}[a-zA-Z0-9])?\\.)+[a-zA-Z]([a-zA-Z0-9\\-]{0,61}[a-zA-Z0-9])?)|(did:[a-z]+:[a-zA-Z0-9._:%\\-]*[a-zA-Z0-9._\\-])$';
export const PATTERN_ATTR = '^[a-zA-Z0-9\\-_\\/]{2,32}$';
export const PATTERN_ATTR_DESC =
    '^[[\\p{L}\\p{Mn}\\p{Mc}\\p{N}\\-_\\/\\s]--[\\u{2139}\\u{FE0F}]]{0,128}$';
export const PATTERN_API_KEY = '^[a-zA-Z0-9_\\/\\-]{2,24}$';
export const PATTERN_CITY = '^[a-zA-Z0-9À-ÿ\\-\\p{Zs}]{0,48}$';
// Technically, the API accepts `PATTERN_CLIENT_ID_EPHEMERAL` everywhere to make resolving via URL possible.
// However, we don't want to allow that in the UI, as it would be very confusing. We also want to reject IDs
// that could potentially overlap with any `dyn$*` clients created via DCR to avoid conflicts.
// Via direct API call, it's still possible to create conflicts, but this has to be very intentional.
export const PATTERN_CLIENT_ID_NEW = '^[a-zA-Z0-9._\\-]{2,256}$';
export const PATTERN_CLIENT_ID_EPHEMERAL = "^[a-zA-Z0-9,.:\\/_\\-&?=~#!$'\\(\\)*+%]{2,256}$";
export const PATTERN_CLIENT_NAME = '^[\\p{L}\\p{M}\\p{N}\\p{Zs}\\(\\)._\\-]{2,128}$';
// A URI with a non-empty host part (optional scheme + host, optionally with a port): unlike
// PATTERN_URI, degenerate values such as `https://`, `/` or `javascript:alert(1)` are rejected.
// A fragment (`#`) and a `,` are rejected too, since (post-logout) redirect URIs are stored
// comma-joined. Matches the backend `RE_CLIENT_URI`.
export const PATTERN_CLIENT_URI =
    "^(?:[a-zA-Z][a-zA-Z0-9+.\\-]*://)?[a-zA-Z0-9](?:[a-zA-Z0-9._\\-]{0,253}[a-zA-Z0-9])?(?::[0-9]{1,5})?(?:[\\/?][a-zA-Z0-9.:\\/_\\-&?=~!$'\\(\\)*+%@]*)?$";
// export const PATTERN_DATE_STR = '[0-9]{4}\\-[0-9]{2}-[0-9]{2}$';
// export const PATTERN_CODE_CHALLENGE = '^[a-zA-Z0-9\\-._~]{43,128}$';
export const PATTERN_CONTACT = '^[a-zA-Z0-9\\+.@\\/:-]{0,48}$';
export const PATTERN_CSS_VALUE_LOOSE = '^[a-z0-9\\-,.#\\(\\)%\\/\\s]+$';
// export const PATTERN_FLOW = '^(authorization_code|client_credentials|password|refresh_token)$';
export const PATTERN_GROUP =
    '^[[\\p{L}\\p{Mn}\\p{Mc}\\p{N}\\-_\\/,:*\\p{Zs}]--[\\u{2139}\\u{FE0F}]]{2,64}$';
// KV namespaces and access key names are used in URL paths and stay ASCII only
export const PATTERN_KV_NAME = '^[a-zA-Z0-9\\-_\\/,:*\\p{Zs}]{2,64}$';
export const PATTERN_KV_KEY = '^[a-zA-Z0-9\\-\\._\\~]{2,64}$';
export const PATTERN_ROLE =
    '^[[\\p{L}\\p{Mn}\\p{Mc}\\p{N}\\-_\\/,:*.]--[\\u{2139}\\u{FE0F}]]{2,64}$';
// OAuth scopes must stay ASCII (RFC 6749 section 3.3)
export const PATTERN_SCOPE = '^[a-zA-Z0-9\\-_\\/,:*.]{2,64}$';
// export const PATTERN_IPV4 = '^(?:25[0-5]|2[0-4]\\d|1\\d\\d|[1-9]\\d|\\d)(?:\\.(?:25[0-5]|2[0-4]\\d|1\\d\\d|[1-9]\\d|\\d)){3}$';
export const PATTERN_LINUX_HOSTNAME = '^[a-zA-Z0-9][a-zA-Z0-9\\-.]*[a-zA-Z0-9]$';
export const PATTERN_LINUX_USERNAME = '^[a-z][a-z0-9_\\-]{1,61}$';
export const PATTERN_LOWERCASE = '^[a-z0-9\\-_\\/]{2,128}$';
export const PATTERN_ORIGIN = '^(http|https)://[a-z0-9.:\\-]+$';
export const PATTERN_OTP_CODE = '^[0-9\\s]*$';
// export const PATTERN_PEM = '^(-----BEGIN CERTIFICATE-----)[a-zA-Z0-9+\\/=\\n]+(-----END CERTIFICATE-----)$';
export const PATTERN_PHONE = '^\\+[0-9]{0,32}$';
export const PATTERN_SCOPE_SPACE = '^[a-zA-Z0-9\\-_\\/:\\p{Zs}*]{0,512}$';
export const PATTERN_STREET = '^[a-zA-Z0-9À-ÿ\\-.\\p{Zs}]{0,48}$';
export const PATTERN_URI = "^[a-zA-Z0-9,.:\\/_\\-&?=~#!$'\\(\\)*+%@]*$";
// Like PATTERN_URI but without `#`: an RFC 8707 resource indicator must be an absolute URI
// without a fragment (RFC 8707 §2). Matches the backend `RE_RESOURCE`.
export const PATTERN_RESOURCE = "^[a-zA-Z0-9,.:\\/_\\-&?=~!$'\\(\\)*+%@]*$";
export const PATTERN_USER_NAME = "^[\\p{L}\\p{M}\\p{N}\\p{Zs}'.\\-]{1,32}$";
