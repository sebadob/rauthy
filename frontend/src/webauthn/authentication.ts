import { arrBufToBase64UrlSafe, promiseTimeout } from '$utils/helpers';
import { fetchPost } from '$api/fetch';
import type {
    MfaPurpose,
    WebauthnAdditionalData,
    WebauthnAuthFinishRequest,
    WebauthnAuthStartRequest,
    WebauthnAuthStartResponse,
} from './types.ts';
import { base64UrlSafeToArrBuf } from './utils';
import { untrack } from 'svelte';

export interface WebauthnAuthResult {
    error?: string;
    data?: WebauthnAdditionalData;
}

export async function webauthnAuth(
    userId: string,
    purpose: MfaPurpose,
    errorI18nInvalidKey: string,
    errorI18nTimeout: string,
): Promise<WebauthnAuthResult> {
    let payloadStart: WebauthnAuthStartRequest = {
        purpose,
    };
    let res = await fetchPost<WebauthnAuthStartResponse>(
        `/auth/v1/users/${userId}/webauthn/auth/start`,
        payloadStart,
    );
    if (res.error) {
        console.error(res.error);
        return {
            error: res.error.message || 'Error starting the Authentication',
        };
    }
    if (!res.body) {
        let error = 'Did not receive a valid webauthn body';
        console.error(error);
        return { error };
    }

    let resp = res.body;
    // We need to apply a small hack to make TS happy.
    // The browser expects ArrayBuffers in some places, but the backend sends them as base64 encoded data,
    // which we will decode properly in the following lines.
    let challenge = resp.rcr as unknown as CredentialRequestOptions;
    if (!challenge.publicKey) {
        let error = 'no publicKey in challenge from the backend';
        console.error(error);
        return {
            error,
        };
    }

    // convert base64 into ArrayBuffers
    challenge.publicKey.challenge = base64UrlSafeToArrBuf(challenge.publicKey.challenge);
    if (challenge.publicKey.allowCredentials) {
        for (let cred of challenge.publicKey.allowCredentials) {
            cred.id = base64UrlSafeToArrBuf(cred.id);
        }
    }

    // prompt for the passkey and get its public key
    const exp = (resp.exp - 1) * 1000;
    const expTime = new Date().getTime() + exp;
    let credential: Credential;
    try {
        const cred = await promiseTimeout(navigator.credentials.get(challenge), exp);
        if (cred) {
            credential = cred;
        } else {
            return {
                error: errorI18nInvalidKey,
            };
        }
    } catch (e) {
        const timeout = new Date().getTime() >= expTime;
        return {
            error: timeout ? errorI18nTimeout : errorI18nInvalidKey,
        };
    }

    // The backend expects base64 url safe strings instead of array buffers.
    // The values we need to modify are not publicly exported in the TS type though, but they exist.
    let payloadFinish: WebauthnAuthFinishRequest = {
        code: resp.code,
        data: {
            id: credential.id,
            // @ts-ignore the `response.rawId` actually exists
            rawId: arrBufToBase64UrlSafe(credential.rawId),
            response: {
                // @ts-ignore the `response.authenticatorData` actually exists
                authenticatorData: arrBufToBase64UrlSafe(credential.response.authenticatorData),
                // @ts-ignore the `response.clientDataJSON` actually exists
                clientDataJSON: arrBufToBase64UrlSafe(credential.response.clientDataJSON),
                // @ts-ignore the `response.signature` actually exists
                signature: arrBufToBase64UrlSafe(credential.response.signature),
            },
            // @ts-ignore the `response.getClientExtensionResults()` actually exists
            extensions: credential.getClientExtensionResults(),
            type: credential.type,
        },
    };

    // finish the ceremony
    let resFinish = await fetchPost<WebauthnAdditionalData>(
        `/auth/v1/users/${userId}/webauthn/auth/finish`,
        payloadFinish,
    );
    // 202 -> normal success
    // 205 -> Webauthn success during login, but needs user values updates
    // 206 -> Webauthn success during login, but needs additional ToS update accept
    if (resFinish.status === 202 || resFinish.status === 206) {
        return {
            data: resFinish.body,
        };
    } else if (resFinish.status === 205) {
        return {};
    } else {
        console.error(resFinish);
        return {
            error: resFinish.error?.message || 'Authentication Error',
        };
    }
}
