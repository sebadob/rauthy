export const PATTERN_ALNUM = '^[a-zA-Z0-9]$';
export const PATTERN_ALNUM_64 = '^[a-zA-Z0-9]{64}$';
export const PATTERN_CITY = '^[a-zA-Z0-9À-ÿ\\-]{0,48}$';
export const PATTERN_CLIENT_NAME = '^[a-zA-Z0-9À-ſ\\-\\s\\u3041-\\u3096\\u30A0-\\u30FF\\u3400-\\u4DB5\\u4E00-\\u9FCB\\uF900-\\uFA6A\\u2E80-\\u2FD5\\uFF66-\\uFF9F\\uFFA1-\\uFFDC\\u31F0-\\u31FF]{2,128}$';
// export const PATTERN_DATE_STR = '[0-9]{4}\\-[0-9]{2}-[0-9]{2}$';
export const PATTERN_PHONE = '^\\+[0-9]{0,32}$';
export const PATTERN_STREET = '^[a-zA-Z0-9À-ÿ\\-.\\s]{0,48}$';
export const PATTERN_URI = '^[a-zA-Z0-9,.:\\/_\\-&?=~#!$\'\\(\\)*+%]*$';
export const PATTERN_USER_NAME = '^[a-zA-Z0-9À-ſ\\-\\s]{1,32}$';