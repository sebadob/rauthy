import {isBrowser} from "$utils/helpers";

export interface PKCE {
    challenge: string,
    verifier: string,
    // technically not part of PKCE, but we keep the secure random gen all in one place
    nonce: string,
}

function b64UrlEncode(string: string) {
    return btoa(string).replace(/\+/g, '-').replace(/\//g, '_').replace(/=+$/, '');
}

export async function generatePKCE(): Promise<undefined | PKCE> {
    if (!isBrowser() || window.crypto?.subtle == undefined) {
        return;
    }

    let random = new Uint8Array(48);
    random = window.crypto.getRandomValues(random);
    let str = String.fromCharCode.apply(null, Array.from(random));
    let verifier = b64UrlEncode(str);

    let hash = await crypto.subtle.digest(
        {name: "SHA-256"},
        new TextEncoder().encode(verifier)
    );
    let strHash = String.fromCharCode.apply(null, Array.from(new Uint8Array(hash)));
    let challenge = b64UrlEncode(strHash);

    let randomNonce = new Uint8Array(24);
    randomNonce = window.crypto.getRandomValues(randomNonce);
    let strNonce = String.fromCharCode.apply(null, Array.from(randomNonce));
    let nonce = b64UrlEncode(str);

    return {challenge, verifier, nonce};
}
