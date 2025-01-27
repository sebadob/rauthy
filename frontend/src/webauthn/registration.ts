import {fetchPost} from "$api/fetch.ts";
import {arrBufToBase64UrlSafe, base64UrlSafeToArrBuf} from "./utils";
import type {CreationChallengeResponse, WebauthnRegFinishRequest, WebauthnRegStartRequest} from "$webauthn/types.ts";
import {promiseTimeout} from "$utils/helpers.ts";

export interface WebauthnRegResult {
    error?: string,
}

export async function webauthnReg(
    userId: string,
    passkeyName: string,
    errorI18nInvalidKey: string,
    errorI18nTimeout: string,
): Promise<WebauthnRegResult> {
    let payloadStart: WebauthnRegStartRequest = {
        passkey_name: passkeyName,
    };
    let resStart = await fetchPost<CreationChallengeResponse>(
        `/auth/v1/users/${userId}/webauthn/register/start`,
        payloadStart
    );
    if (!resStart.body) {
        return {
            error: resStart.error.message || 'did not receive any registration data',
        };
    }

    // We need to apply a small hack to make TS happy.
    // The browser expects ArrayBuffers in some places, but the backend sends them as base64 encoded data,
    // which we will decode properly in the following lines.
    let options = resStart.body as unknown as CredentialCreationOptions;
    if (!options.publicKey) {
        let error = 'no publicKey in options from the backend';
        console.error(error);
        return {error};
    }

    // the navigator credentials engine needs some values as array buffers
    options.publicKey.challenge = base64UrlSafeToArrBuf(options.publicKey.challenge);
    options.publicKey.user.id = base64UrlSafeToArrBuf(options.publicKey.user.id);

    if (options.publicKey.excludeCredentials) {
        for (let cred of options.publicKey.excludeCredentials) {
            cred.id = base64UrlSafeToArrBuf(cred.id);
        }
    }

    // prompt for the passkey and get its public key
    let exp = (options.publicKey.timeout || 60000) - 1000;
    const expTime = new Date().getTime() + exp;
    let credential: Credential;
    try {
        const cred = await promiseTimeout(navigator.credentials.create(options), exp);
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

    // the backend expects base64 url safe string instead of array buffers
    let payloadFinish: WebauthnRegFinishRequest = {
        passkey_name: passkeyName,
        data: {
            id: credential.id,
            // @ts-ignore the `rawId` actually exists
            rawId: arrBufToBase64UrlSafe(credential.rawId),
            response: {
                // @ts-ignore the `response.attestationObject` actually exists
                attestationObject: arrBufToBase64UrlSafe(credential.response.attestationObject),
                // @ts-ignore the `response.clientDataJSON` actually exists
                clientDataJSON: arrBufToBase64UrlSafe(credential.response.clientDataJSON),
            },
            // @ts-ignore the `response.getClientExtensionResults()` actually exists
            extensions: credential.getClientExtensionResults(),
            type: credential.type,
        },
    }

    let resFinish = await fetchPost(
        `/auth/v1/users/${userId}/webauthn/register/finish`,
        payloadFinish,
    );
    if (resFinish.status === 201) {
        return {};
    } else {
        return {
            error: resFinish.error.message || 'Registration failed',
        };
    }
}
