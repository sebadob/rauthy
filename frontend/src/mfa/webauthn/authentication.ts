import { promiseTimeout } from '#utils/helpers.ts';
import { fetchPost } from '#api/fetch.ts';
import type {
    WebauthnAdditionalData,
    WebauthnAuthFinishRequest,
    WebauthnAuthStartRequest,
    WebauthnAuthStartResponse,
} from '#mfa/webauthn/types.ts';
import { arrBufToBase64UrlSafe, base64UrlSafeToArrBuf } from './utils';
import type { MfaPurpose } from '#api/types/mfa.js';

export interface WebauthnAuthResult {
    error?: string;
    data?: WebauthnAdditionalData;
}

export async function webauthnAuth(
    purpose: MfaPurpose,
    errorI18nInvalidKey: string,
    errorI18nTimeout: string,
    resetUserId?: string,
): Promise<WebauthnAuthResult> {
    let startUri = '/auth/v1/users/webauthn_start';
    if (purpose === 'PasswordReset') {
        if (!resetUserId) {
            return { error: 'Missing user context for password reset' };
        }
        startUri = `/auth/v1/users/${encodeURIComponent(resetUserId)}/webauthn/auth/start`;
    }

    let payloadStart: WebauthnAuthStartRequest = {
        purpose,
    };
    let res = await fetchPost<WebauthnAuthStartResponse>(startUri, payloadStart);
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
    let credential: PublicKeyCredential;
    try {
        const cred = await promiseTimeout(navigator.credentials.get(challenge), exp);
        if (cred) {
            credential = cred as PublicKeyCredential;
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
    const response = credential.response as AuthenticatorAssertionResponse;
    let payloadFinish: WebauthnAuthFinishRequest = {
        code: resp.code,
        data: {
            id: credential.id,
            rawId: arrBufToBase64UrlSafe(credential.rawId),
            response: {
                authenticatorData: arrBufToBase64UrlSafe(response.authenticatorData),
                clientDataJSON: arrBufToBase64UrlSafe(response.clientDataJSON),
                signature: arrBufToBase64UrlSafe(response.signature),
                userHandle: response.userHandle
                    ? arrBufToBase64UrlSafe(response.userHandle)
                    : undefined,
            },
            extensions: credential.getClientExtensionResults(),
            type: credential.type,
        },
    };

    // finish the ceremony
    let resFinish = await fetchPost<WebauthnAdditionalData>(
        `/auth/v1/users/webauthn_finish`,
        payloadFinish,
    );
    // 202 -> normal success
    // 205 -> Webauthn success during login, but needs user values updates
    // 206 -> Webauthn success during login, but needs additional ToS update accept
    // 404 -> Usually only happens for direct Resident Key logins when the key is unknown
    if (resFinish.status === 202 || resFinish.status === 206) {
        return {
            data: resFinish.body,
        };
    } else if (resFinish.status === 205) {
        return {};
    } else if (resFinish.status === 404) {
        return {
            error: errorI18nInvalidKey,
        };
    } else {
        console.error(resFinish);
        return {
            error: resFinish.error?.message || 'Authentication Error',
        };
    }
}
