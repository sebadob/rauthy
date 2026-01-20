export const PATTERN_ALNUM = '^[a-zA-Z0-9]*$';
// export const PATTERN_ALNUM_64 = '^[a-zA-Z0-9]{64}$';
export const PATTERN_ATPROTO_ID =
    '^(([a-zA-Z0-9]([a-zA-Z0-9\\-]{0,61}[a-zA-Z0-9])?\\.)+[a-zA-Z]([a-zA-Z0-9\\-]{0,61}[a-zA-Z0-9])?)|(did:[a-z]+:[a-zA-Z0-9._:%\\-]*[a-zA-Z0-9._\\-])$';
export const PATTERN_ATTR = '^[a-zA-Z0-9\\-_\\/]{2,32}$';
export const PATTERN_ATTR_DESC = '^[a-zA-Z0-9\\-_\\/\\s]{0,128}$';
export const PATTERN_API_KEY = '^[a-zA-Z0-9_\\/\\-]{2,24}$';
export const PATTERN_CITY = '^[a-zA-Z0-9À-ÿ\\-]{0,48}$';
// export const PATTERN_CLIENT_ID_EPHEMERAL = '^[a-zA-Z0-9,.:\\/_\\-&?=~#!$\'\\(\\)*+%]{2,256}$';
export const PATTERN_CLIENT_NAME =
    '^[a-zA-Z0-9À-ɏ\\-\\s\\u3041-\\u3096\\u30A0-\\u30FF\\u3400-\\u4DB5\\u4E00-\\u9FCB\\uF900-\\uFA6A\\u2E80-\\u2FD5\\uFF66-\\uFF9F\\uFFA1-\\uFFDC\\u31F0-\\u31FF]{2,128}$';
// export const PATTERN_DATE_STR = '[0-9]{4}\\-[0-9]{2}-[0-9]{2}$';
// export const PATTERN_CODE_CHALLENGE = '^[a-zA-Z0-9\\-._~]{43,128}$';
export const PATTERN_CONTACT = '^[a-zA-Z0-9\\+.@\\/:-]{0,48}$';
export const PATTERN_CSS_VALUE_LOOSE = '^[a-z0-9\\-,.#\\(\\)%\\/\\s]+$';
// export const PATTERN_FLOW = '^(authorization_code|client_credentials|password|refresh_token)$';
export const PATTERN_GROUP = '^[a-zA-Z0-9\\-_\\/,:*\\s]{2,64}$';
export const PATTERN_ROLE_SCOPE = '^[a-zA-Z0-9\\-_\\/,:*.]{2,64}$';
// export const PATTERN_IPV4 = '^(?:25[0-5]|2[0-4]\\d|1\\d\\d|[1-9]\\d|\\d)(?:\\.(?:25[0-5]|2[0-4]\\d|1\\d\\d|[1-9]\\d|\\d)){3}$';
export const PATTERN_LINUX_HOSTNAME = '^[a-zA-Z0-9][a-zA-Z0-9\\-.]*[a-zA-Z0-9]$';
export const PATTERN_LINUX_USERNAME = '^[a-z][a-z0-9_\\-]{1,61}$';
export const PATTERN_LOWERCASE = '^[a-z0-9\\-_\\/]{2,128}$';
export const PATTERN_ORIGIN = '^(http|https)://[a-z0-9.:\\-]+$';
// export const PATTERN_PEM = '^(-----BEGIN CERTIFICATE-----)[a-zA-Z0-9+\\/=\\n]+(-----END CERTIFICATE-----)$';
export const PATTERN_PHONE = '^\\+[0-9]{0,32}$';
export const PATTERN_SCOPE_SPACE = '^[a-zA-Z0-9\\-_\\/:\\s*]{0,512}$';
export const PATTERN_STREET = '^[a-zA-Z0-9À-ÿ\\-.\\s]{0,48}$';
export const PATTERN_URI = "^[a-zA-Z0-9,.:\\/_\\-&?=~#!$'\\(\\)*+%@]*$";
export const PATTERN_USER_NAME =
    "^[a-zA-Z0-9À-ɏ\\-'\\s\\u3041-\\u3096\\u30A0-\\u30FF\\u3400-\\u4DB5\\u4E00-\\u9FCB\\uF900-\\uFA6A\\u2E80-\\u2FD5\\uFF66-\\uFF9F\\uFFA1-\\uFFDC\\u31F0-\\u31FF]{1,32}$";
