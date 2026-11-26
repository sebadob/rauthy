import type { ProviderLoginRequest } from '#api/types/auth_provider.ts';
import { generatePKCE } from '#utils/pkce.ts';
import { fetchSolvePow } from '#utils/pow.ts';
import { fetchPost, type IResponse } from '#api/fetch.ts';
import { saveProviderToken } from '#utils/helpers.ts';
import { PKCE_VERIFIER_UPSTREAM, PROVIDER_LOGIN_PAGE } from '#utils/constants.ts';

/**
 * The payload does not need to contain `pkce_challenge` and `pow`, as they will be
 * generated inside this function. Will return the response, if the login request fails,
 * and do a redirect otherwise.
 */
export async function execProviderLogin(
    payload: ProviderLoginRequest,
): Promise<undefined | IResponse<string>> {
    let pkce = await generatePKCE();
    if (!pkce) {
        return;
    }
    payload.pkce_challenge = pkce.challenge;
    localStorage.setItem(PKCE_VERIFIER_UPSTREAM, pkce.verifier);

    let pow = await fetchSolvePow();
    if (!pow) {
        console.error('Error generating PoW - are you in a secure browser context?');
        return;
    }
    payload.pow = pow;

    let res = await fetchPost<string>('/auth/v1/providers/login', payload);

    if (res.text) {
        saveProviderToken(res.text);
        // the callback returns here if the consent turns out to be missing
        sessionStorage.setItem(PROVIDER_LOGIN_PAGE, window.location.href);

        let loc = res.headers.get('location');
        if (!loc) {
            console.error('no location header set for provider login');
            return;
        }
        window.location.href = loc;
    } else {
        return res;
    }
}
