// A copy of the backend `validate_redirect_uri_shape()` (`src/data/src/entity/clients.rs`), so the
// UI can reject an invalid redirect URI with a translated error before sending it.
// Keep both in sync.

import type { I18nAdmin } from '../i18n/admin/interface.ts';

type I18nRedirectUri = I18nAdmin['validation']['redirectUri'];

// Query keys an authorization response sets itself (RFC 6749 §4.1.2 / §4.1.2.1, RFC 9207).
export const RESERVED_REDIRECT_QUERY_KEYS = [
    'code',
    'state',
    'error',
    'error_description',
    'error_uri',
    'iss',
];

export type RedirectUriShapeError =
    | { kind: 'fragment' }
    | { kind: 'comma' }
    | { kind: 'controlChar' }
    | { kind: 'reservedKey'; key: string };

/**
 * Rejects a `redirect_uri` with a fragment (RFC 6749 §3.1.2) or whose query already carries a
 * reserved key. A `,` is rejected as well, since redirect URIs are stored comma-joined.
 *
 * Query keys are split on both `&` and `;`, form-decoded, trimmed and compared
 * case-insensitively. A reserved key followed by `[` or `.` is rejected too, with `.` and ` `
 * compared as `_`. Keys with control characters are rejected as well.
 *
 * Returns `undefined` if the URI is valid.
 */
export function validateRedirectUriShape(uri: string): RedirectUriShapeError | undefined {
    if (uri.includes('#')) {
        return { kind: 'fragment' };
    }
    if (uri.includes(',')) {
        return { kind: 'comma' };
    }

    const idx = uri.indexOf('?');
    if (idx === -1) {
        return undefined;
    }

    for (const pair of uri.slice(idx + 1).split(/[&;]/)) {
        if (pair.length === 0) {
            continue;
        }
        const eq = pair.indexOf('=');
        const key = formDecode(eq === -1 ? pair : pair.slice(0, eq));

        if (RE_CONTROL.test(key)) {
            return { kind: 'controlChar' };
        }
        const reserved = reservedQueryKey(key);
        if (reserved) {
            return { kind: 'reservedKey', key: reserved };
        }
    }

    return undefined;
}

// Unicode `Cc`, like Rust's `char::is_control()`
const RE_CONTROL = /[\u0000-\u001F\u007F-\u009F]/;
// Unicode `White_Space`, like Rust's `str::trim()` - unlike JS `trim()`, without U+FEFF
const WS =
    '\\t\\n\\v\\f\\r \\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000';
const RE_TRIM = new RegExp(`^[${WS}]+|[${WS}]+$`, 'g');

// `application/x-www-form-urlencoded` decoding like the Rust `form_urlencoded` crate: `+` becomes
// a space, valid `%XX` sequences are decoded, everything else is kept, invalid UTF-8 is replaced.
function formDecode(s: string): string {
    const input = new TextEncoder().encode(s);
    const out = new Uint8Array(input.length);
    let len = 0;
    for (let i = 0; i < input.length; i++) {
        const b = input[i];
        if (b === 0x2b) {
            out[len++] = 0x20;
        } else if (
            b === 0x25 &&
            i + 2 < input.length &&
            isHex(input[i + 1]) &&
            isHex(input[i + 2])
        ) {
            out[len++] = parseInt(String.fromCharCode(input[i + 1], input[i + 2]), 16);
            i += 2;
        } else {
            out[len++] = b;
        }
    }
    // `ignoreBOM`, so a leading U+FEFF is kept like in Rust
    return new TextDecoder('utf-8', { ignoreBOM: true }).decode(out.subarray(0, len));
}

function isHex(b: number): boolean {
    return (b >= 0x30 && b <= 0x39) || (b >= 0x41 && b <= 0x46) || (b >= 0x61 && b <= 0x66);
}

// Returns the reserved key a decoded query `key` would be folded into by common parsers.
function reservedQueryKey(decoded: string): string | undefined {
    // ASCII-only lowercase like Rust's `to_ascii_lowercase()`
    const key = decoded.replace(RE_TRIM, '').replace(/[A-Z]/g, c => c.toLowerCase());
    // PHP turns `.` and ` ` into `_`, and `code[]` / `iss[0]` become arrays under the plain key
    const normalized = key.replace(/[. ]/g, '_');
    const bracket = normalized.indexOf('[');
    const base = bracket === -1 ? normalized : normalized.slice(0, bracket);

    return (
        RESERVED_REDIRECT_QUERY_KEYS.find(reserved => reserved === base) ||
        // `qs` with `allowDots` nests `iss.x` under `iss`
        RESERVED_REDIRECT_QUERY_KEYS.find(
            reserved =>
                key.startsWith(reserved) &&
                (key[reserved.length] === '[' || key[reserved.length] === '.'),
        )
    );
}

// Returns the translated error for an invalid `uri`, or `undefined` if it is valid.
export function redirectUriShapeErrorMsg(uri: string, i18n: I18nRedirectUri): string | undefined {
    const err = validateRedirectUriShape(uri);
    switch (err?.kind) {
        case undefined:
            return undefined;
        case 'fragment':
            return i18n.fragment;
        case 'comma':
            return i18n.comma;
        case 'controlChar':
            return i18n.controlChar;
        case 'reservedKey':
            return i18n.reservedKey.replace('{{ KEY }}', err.key);
    }
}

// Returns the translated error for the first invalid URI in `uris`, prefixed with the URI, or
// `undefined` if all are valid.
export function invalidRedirectUrisMsg(uris: string[], i18n: I18nRedirectUri): string | undefined {
    for (const uri of uris) {
        const msg = redirectUriShapeErrorMsg(uri, i18n);
        if (msg) {
            return `${uri}: ${msg}`;
        }
    }
    return undefined;
}
