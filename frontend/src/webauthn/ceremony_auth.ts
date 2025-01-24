import {arrBufToBase64UrlSafe} from "$utils/helpers.ts";
import {fetchPost} from "$api/fetch.ts";
import type {
    MfaPurpose,
    WebauthnAdditionalData,
    WebauthnAuthFinishRequest,
    WebauthnAuthStartResponse
} from "./types.ts";
import {base64UrlSafeToArrBuf} from "./utils.ts";

export interface WebauthnAuthResult {
    success: boolean,
    msg: string,
    data?: WebauthnAdditionalData,
}

export async function webauthnAuth(userId: string, purpose: MfaPurpose, errorI18nInvalidKey: string): Promise<WebauthnAuthResult> {
    let res = await fetchPost<WebauthnAuthStartResponse>(`/auth/v1/users/${userId}/webauthn/auth/start`, purpose);
    if (res.error) {
        console.error(res.error);
        return {
            success: false,
            msg: res.error.message || 'Error starting the Authentication'
        };
    }
    if (!res.body) {
        let msg = 'Did not receive a valid webauthn body';
        console.error(msg);
        return {
            success: false,
            msg,
        };
    }

    let resp = res.body;
    // We need to apply a small hack to make TS happy.
    // The browser expects ArrayBuffers in some places, but the backend sends them as base64 encoded data,
    // which we will decode properly in the following lines.
    let challenge = resp.rcr as unknown as CredentialRequestOptions;
    if (!challenge.publicKey) {
        let msg = 'no publicKey in challenge from the backend';
        console.error(msg);
        return {
            success: false,
            msg,
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
    let challengePk: Credential;
    try {
        const cred = await navigator.credentials.get(challenge);
        if (cred) {
            challengePk = cred;
        } else {
            return {
                success: false,
                msg: errorI18nInvalidKey || 'Invalid Key',
            };
        }
    } catch (e) {
        console.error(e);
        return {
            success: false,
            msg: errorI18nInvalidKey || 'Invalid Key',
        };
    }
    // TODO remove after proper type has been created
    console.log('challengePk', challengePk);

    // The backend expects base64 url safe strings instead of array buffers.
    // The values we need to modify are not publicly exported in the TS type though, but they exist.
    let payload: WebauthnAuthFinishRequest = {
        code: resp.code,
        data: {
            id: challengePk.id,
            rawId: arrBufToBase64UrlSafe(challengePk.rawId),
            response: {
                authenticatorData: arrBufToBase64UrlSafe(challengePk.response.authenticatorData),
                clientDataJSON: arrBufToBase64UrlSafe(challengePk.response.clientDataJSON),
                signature: arrBufToBase64UrlSafe(challengePk.response.signature),
            },
            // TODO check if this exists, we can probably get rid of it in the browser
            extensions: challengePk.extensions,
            type: challengePk.type,
        }
    }

    // finish the ceremony
    let resFinish = await fetchPost<WebauthnAdditionalData>(
        `/auth/v1/users/${userId}/webauthn/auth/finish`,
        payload,
    );
    if (resFinish.status === 202) {
        return {
            success: true,
            msg: 'Authentication successful',
            data: resFinish.body,
        };
    } else {
        console.error(res);
        return {
            success: false,
            msg: res.error || 'Authentication Error',
        };
    }
}
