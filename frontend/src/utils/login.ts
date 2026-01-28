import type { ProviderLoginRequest } from '$api/types/auth_provider';
import { generatePKCE } from '$utils/pkce';
import { fetchSolvePow } from '$utils/pow';
import { fetchPost } from '$api/fetch';
import { saveProviderToken } from '$utils/helpers';
import { PKCE_VERIFIER_UPSTREAM } from '$utils/constants';

/**
 * The payload does not need to contain `pkce_challenge` and `pow`, as they will be
 * generated inside this function. Will return an error string, if anything fails,
 * and do a redirect otherwise.
 */
export async function execProviderLogin(payload: ProviderLoginRequest) {
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

        let loc = res.headers.get('location');
        if (!loc) {
            console.error('no location header set for provider login');
            return;
        }
        window.location.href = loc;
    } else {
        return res.error?.message || 'Error';
    }
}
