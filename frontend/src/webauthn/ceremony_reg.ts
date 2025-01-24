import {fetchPost} from "$api/fetch.ts";
import {arrBufToBase64UrlSafe, base64UrlSafeToArrBuf} from "./utils";
import type {CreationChallengeResponse, WebauthnRegFinishRequest, WebauthnRegStartRequest} from "$webauthn/types.ts";

export interface WebauthnRegResult {
    success: boolean,
    msg: string,
}

export async function webauthnReg(userId: string, passkeyName: string): Promise<WebauthnRegResult> {
    let payloadStart: WebauthnRegStartRequest = {
        passkey_name: passkeyName,
    };
    let resStart = await fetchPost<CreationChallengeResponse>(
        `/auth/v1/users/${userId}/webauthn/register/start`,
        payloadStart
    );
    if (!resStart.body) {
        return {
            success: false,
            msg: resStart.error.message || 'did not receive any registration data',
        };
    }

    // We need to apply a small hack to make TS happy.
    // The browser expects ArrayBuffers in some places, but the backend sends them as base64 encoded data,
    // which we will decode properly in the following lines.
    let options = resStart.body as unknown as CredentialCreationOptions;
    if (!options.publicKey) {
        let msg = 'no publicKey in options from the backend';
        console.error(msg);
        return {
            success: false,
            msg,
        };
    }

    // the navigator credentials engine needs some values as array buffers
    options.publicKey.challenge = base64UrlSafeToArrBuf(options.publicKey.challenge);
    options.publicKey.user.id = base64UrlSafeToArrBuf(options.publicKey.user.id);
    // options.publicKey.excludeCredentials = options.publicKey.excludeCredentials

    if (options.publicKey.excludeCredentials) {
        for (let cred of options.publicKey.excludeCredentials) {
            cred.id = base64UrlSafeToArrBuf(cred.id);
        }
    }

    // prompt for the passkey and get its public key
    let challengePk: Credential;
    try {
        const cred = await navigator.credentials.create(options);
        if (cred) {
            challengePk = cred;
        } else {
            return {
                success: false,
                msg: 'Credential Creation Error',
            };
        }
    } catch (e) {
        console.error(e);
        return {
            success: false,
            msg: 'Credential Creation Error',
        };
    }
    // TODO remove after proper type has been created
    console.log('challengePk', challengePk);

    // the backend expects base64 url safe string instead of array buffers
    let payloadFinish: WebauthnRegFinishRequest = {
        passkey_name: passkeyName,
        data: {
            id: challengePk.id,
            // @ts-ignore the `rawId` actually exists
            rawId: arrBufToBase64UrlSafe(challengePk.rawId),
            response: {
                // @ts-ignore the `response.attestationObject` actually exists
                attestationObject: arrBufToBase64UrlSafe(challengePk.response.attestationObject),
                // @ts-ignore the `response.clientDataJSON` actually exists
                clientDataJSON: arrBufToBase64UrlSafe(challengePk.response.clientDataJSON),
            },
            type: challengePk.type,
        },
    }

    let resFinish = await fetchPost(
        `/auth/v1/users/${userId}/webauthn/register/finish`,
        payloadFinish,
    );
    if (resFinish.status === 201) {
        return {
            success: true,
            msg: 'Registration successful',
        };
    } else {
        return {
            success: false,
            msg: resFinish.error.message || 'Registration failed',
        };
    }
}
