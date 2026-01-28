import {
    ACCESS_TOKEN,
    AUTH_ENDPOINT,
    CLIENT_ID,
    CSRF_TOKEN,
    ID_TOKEN,
    LOGOUT_URL,
    PKCE_VERIFIER,
    PKCE_VERIFIER_UPSTREAM,
    POST_LOGOUT_REDIRECT_URI,
    PROVIDER_TOKEN,
    REDIRECT_URI,
} from './constants.js';
import { decode, encode } from 'base64-arraybuffer';
import type { PasswordPolicyResponse } from '$api/types/password_policy.ts';
import type { EventLevel } from '$api/types/events.ts';
import { generateNonce, generatePKCE } from '$utils/pkce';
import { fetchGet } from '$api/fetch';

export function buildWebIdUri(userId: string) {
    return `${window.location.origin}/auth/${userId}/profile#me`;
}

export function isBrowser() {
    return typeof window !== 'undefined';
}

export function isDefaultScope(name: string) {
    return (
        name === 'openid' ||
        name === 'profile' ||
        name === 'email' ||
        name === 'groups' ||
        name === 'address' ||
        name === 'phone'
    );
}

export const redirectToLogin = (state?: string) => {
    generatePKCE().then(pkce => {
        if (pkce) {
            localStorage.setItem(PKCE_VERIFIER, pkce.verifier);
            // If we were able to generate PKCE, nonce generation will always succeed as well.
            // `genKey()` is just a fallback to make TS happy.
            let nonce = generateNonce() || genKey(64);
            const s = state || 'admin';
            const redirect_uri = `${window.location.origin}${REDIRECT_URI}`
                .replaceAll(':', '%3A')
                .replaceAll('/', '%2F');
            window.location.href = `${AUTH_ENDPOINT}?client_id=${CLIENT_ID}&redirect_uri=${redirect_uri}&response_type=code&code_challenge=${pkce.challenge}&code_challenge_method=S256&scope=openid+profile+email&nonce=${nonce}&state=${s}`;
        }
    });
};

export const redirectToLogout = () => {
    const url = `${LOGOUT_URL}?post_logout_redirect_uri=${window.location.origin}${POST_LOGOUT_REDIRECT_URI}`;
    // const url = `${LOGOUT_URL}?id_token_hint=${getIdToken()}&post_logout_redirect_uri=${window.location.origin}${POST_LOGOUT_REDIRECT_URI}`;
    window.location.href = url;
};

export const saveCsrfToken = (csrf: string) => {
    // This obfuscation is neither special nor any real protection.
    // The only thing it prevents is grabbing the token as it is in
    // plaintext from the storage without even thinking about it.
    // With this very simple obfuscation, you can only grab a valid
    // token if you forge a targeted attack against Rauthy specifically.
    //
    // Apart from that, there are other ways to handel a CSRF token, but
    // all of them have their downsides and none is fully secure against
    // browser extensions grabbing them, apart from managing it in a
    // WebWorker, which will have huge UX downsides.
    //
    // In the end, this token is only an additional defense in depth
    // in combination with many other methods applied in the backend
    // and on cookie level, and everything would be secure even without
    // this token at all.
    localStorage.setItem(CSRF_TOKEN, 'o/' + genKey(39) + csrf + genKey(43));
    // if (csrf != getCsrfToken()) {
    //     console.error('csrf deobfuscation went wrong!');
    // }
};

export const getCsrfToken = () => {
    let obfs = localStorage.getItem(CSRF_TOKEN) || '';
    if (obfs.startsWith('o/')) {
        return obfs.slice(41, obfs.length - 43);
    }
    return obfs;
};

// export const saveIdToken = (token: string) => {
//     localStorage.setItem(ID_TOKEN, token);
// }
//
// export const saveAccessToken = (token: string) => {
//     localStorage.setItem(ACCESS_TOKEN, token);
// }

export const saveProviderToken = (token: string) => {
    localStorage.setItem(PROVIDER_TOKEN, token);
};
export const getProviderToken = () => {
    return localStorage.getItem(PROVIDER_TOKEN) || '';
};
// export const deleteProviderToken = () => {
//     localStorage.removeItem(PROVIDER_TOKEN);
// }

export const getVerifierFromStorage = () => {
    return localStorage.getItem(PKCE_VERIFIER) || '';
};

export const getVerifierUpstreamFromStorage = () => {
    return localStorage.getItem(PKCE_VERIFIER_UPSTREAM) || '';
};

export const deleteVerifierFromStorage = () => {
    localStorage.removeItem(PKCE_VERIFIER);
};

export const purgeStorage = () => {
    // the old value, keep around for a while until it's cleaned up everywhere
    localStorage.removeItem('csrf_token');
    localStorage.removeItem(CSRF_TOKEN);
    localStorage.removeItem(ID_TOKEN);
    localStorage.removeItem(ACCESS_TOKEN);
    localStorage.removeItem(PKCE_VERIFIER);
    localStorage.removeItem(PKCE_VERIFIER_UPSTREAM);
    localStorage.removeItem(PROVIDER_TOKEN);
};

export function arrBufToBase64UrlSafe(buffer: ArrayBuffer) {
    let base64 = encode(buffer);
    const enc = {
        '+': '-',
        '/': '_',
        '=': '',
    };
    return base64.replace(/[+/=]/g, m => enc[m as '+' | '/' | '=']);
}

// export function base64UrlSafeToArrBuf(base64url: string) {
// 	const dec = {
// 		'-': '+',
// 		_: '/',
// 		'.': '='
// 	};
// 	const base64 = base64url.replace(/[-_.]/g, (m) => dec[m as '-' | '_' | '.']);
// 	return decode(base64);
// }

export function eventColor(level: EventLevel) {
    switch (level) {
        case 'info':
            return 'rgba(52,255,109,0.7)';
        case 'notice':
            return 'rgba(51,69,255,0.7)';
        case 'warning':
            return 'rgba(255,211,51,0.7)';
        case 'critical':
            return 'rgba(255,46,46,0.7)';
    }
}

export const formatUtcTsFromDateInput = (date: string, time?: string) => {
    let d: number;
    if (time) {
        d = Date.parse(`${date}T${time}`);
    } else {
        d = Date.parse(date);
    }
    if (isNaN(d)) {
        return;
    }
    return d / 1000;
};

export const formatDateFromTs = (ts: number, fmtIso?: boolean) => {
    const utcOffsetMinutes = -new Date().getTimezoneOffset();
    const d = new Date((ts + utcOffsetMinutes * 60) * 1000);

    let dd: number | string = d.getUTCDate();
    if (dd < 10) {
        dd = '0' + dd;
    }
    let mm: number | string = d.getUTCMonth() + 1;
    if (mm < 10) {
        mm = '0' + mm;
    }
    const yyyy = d.getUTCFullYear();

    let hr: number | string = d.getUTCHours();
    if (hr < 10) {
        hr = '0' + hr;
    }
    let mn: number | string = d.getUTCMinutes();
    if (mn < 10) {
        mn = '0' + mn;
    }
    let sc: number | string = d.getUTCSeconds();
    if (sc < 10) {
        sc = '0' + sc;
    }

    if (fmtIso) {
        return `${yyyy}-${mm}-${dd}T${hr}:${mn}:${sc}`;
    }
    return `${yyyy}/${mm}/${dd} ${hr}:${mn}:${sc}`;
};

export const generatePassword = (policy: PasswordPolicyResponse) => {
    const length = policy.length_min > 20 ? policy.length_min : 20;
    const lowerCaseNeeded = policy.include_lower_case || 1;
    const upperCaseNeeded = policy.include_upper_case || 1;
    const digitNeeded = policy.include_digits || 1;
    const specialNeeded = policy.include_digits || 1;

    while (true) {
        let lowerCaseIncluded = 0;
        let upperCaseIncluded = 0;
        let digitIncluded = 0;
        let specialIncluded = 0;
        let pwdArr = [];

        for (let i = 0; i < length; i += 1) {
            let nextNumber = 60;
            while ((nextNumber > 57 && nextNumber < 65) || (nextNumber > 90 && nextNumber < 97)) {
                nextNumber = Math.floor(Math.random() * 74) + 33;
            }

            // check if a lower case char was already included
            if (nextNumber >= 91 && nextNumber <= 122) {
                lowerCaseIncluded += 1;
            }

            // check if a upper case char was already included
            if (nextNumber >= 65 && nextNumber <= 90) {
                upperCaseIncluded += 1;
            }

            // check if a digit was already included
            if (nextNumber >= 48 && nextNumber <= 57) {
                digitIncluded += 1;
            }

            // check if a special char was already included
            if (nextNumber >= 33 && nextNumber <= 47) {
                specialIncluded += 1;
            }

            pwdArr.push(String.fromCharCode(nextNumber));
        }

        // If not all types are included, start fresh and try again -> most random approach
        if (
            lowerCaseIncluded < lowerCaseNeeded ||
            upperCaseIncluded < upperCaseNeeded ||
            specialIncluded < specialNeeded ||
            digitIncluded < digitNeeded
        ) {
            continue;
        }

        return pwdArr.join('');
    }
};

// Returns a short random key, which can be used in components to identify them uniquely.
export const genKey = (length?: number) => {
    let key = [];
    length = length || 8;

    for (let i = 0; i < length; i += 1) {
        let nextNumber = 95;
        while (nextNumber > 90 && nextNumber < 97) {
            nextNumber = Math.floor(Math.random() * 57) + 65;
        }
        key.push(String.fromCharCode(nextNumber));
    }

    return key.join('');
};

/**
 * Tries to get the cookie with the given name, if it is accessible
 * @param cname {string} The cookie name to get
 * @returns {string} The cookie value, if it exists
 */
export function getCookie(cname: string): string {
    let name = cname + '=';
    let decodedCookie = decodeURIComponent(document.cookie);
    let ca = decodedCookie.split(';');
    for (let i = 0; i < ca.length; i++) {
        let c = ca[i];
        while (c.charAt(0) === ' ') {
            c = c.substring(1);
        }
        if (c.indexOf(name) === 0) {
            return c.substring(name.length, c.length);
        }
    }
    return '';
}

export function prettyFileSize(size: number) {
    if (size > 1024 * 1024 * 1024) {
        return `${(size / 1024 / 1024 / 1024).toFixed(2)} GiB`;
    } else if (size > 1024 * 1024) {
        return `${(size / 1024 / 1024).toFixed(2)} MiB`;
    } else if (size > 1024) {
        return `${(size / 1024).toFixed(2)} KiB`;
    } else {
        return `${size} B`;
    }
}

export async function fetchTimezones() {
    let res = await fetchGet<string[]>('/auth/v1/timezones');
    if (res.body) {
        return res.body;
    } else {
        console.error(res.error);
        return ['UTC'];
    }
}

/** races a promise against a given timeout and throws an exception if exceeded */
export function promiseTimeout<T>(prom: Promise<T>, time: number): Promise<T | undefined> {
    let timer: any;
    return Promise.race<T | undefined>([
        prom,
        new Promise((_r, rej) => {
            timer = setTimeout(rej, time, 'timeout');
        }),
    ]).finally(() => clearTimeout(timer));
}

// async sleep in ms
export const sleepAwait = async (ms: number) => await new Promise(x => setTimeout(x, ms));
