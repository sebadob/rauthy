import { isBrowser } from '$utils/helpers';

export interface PKCE {
    challenge: string;
    verifier: string;
}

function b64UrlEncode(string: string) {
    return btoa(string).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
}

export async function generatePKCE(): Promise<undefined | PKCE> {
    if (!isBrowser() || window.crypto?.subtle == undefined) {
        console.error('Error generating PKCE challenge - are you in a secure browser context?');
        return;
    }

    let random = new Uint8Array(64);
    random = window.crypto.getRandomValues(random);
    let str = String.fromCharCode.apply(null, Array.from(random));
    let verifier = b64UrlEncode(str);

    let hash = await crypto.subtle.digest({ name: 'SHA-256' }, new TextEncoder().encode(verifier));
    let strHash = String.fromCharCode.apply(null, Array.from(new Uint8Array(hash)));
    let challenge = b64UrlEncode(strHash);

    return { challenge, verifier };
}

export function generateNonce() {
    if (!isBrowser() || window.crypto?.subtle == undefined) {
        return;
    }

    let random = new Uint8Array(24);
    random = window.crypto.getRandomValues(random);
    let str = String.fromCharCode.apply(null, Array.from(random));
    return b64UrlEncode(str);
}
