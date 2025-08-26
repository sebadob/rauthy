import { arrBufToBase64UrlSafe, base64UrlSafeToArrBuf } from './utils';
import type { WebauthnRegFinishRequest, WebauthnRegStartRequest } from '$webauthn/types.ts';
import { getCsrfToken, promiseTimeout } from '$utils/helpers';

export interface WebauthnRegResult {
	error?: string;
}

export async function webauthnReg(
	userId: string,
	passkeyName: string,
	errorI18nInvalidKey: string,
	errorI18nTimeout: string,
	magicLinkId?: string,
	pwdCsrfToken?: string,
	mfaModTokenId?: string
): Promise<WebauthnRegResult> {
	let payloadStart: WebauthnRegStartRequest = {
		passkey_name: passkeyName,
		magic_link_id: magicLinkId,
		mfa_mod_token_id: mfaModTokenId
	};
	let headers: HeadersInit = {
		'Content-Type': 'application/json',
		Accept: 'application/json'
	};
	if (pwdCsrfToken) {
		headers['x-pwd-csrf-token'] = pwdCsrfToken;
	} else {
		headers['x-csrf-token'] = getCsrfToken();
	}

	let resStart = await fetch(`/auth/v1/users/${userId}/webauthn/register/start`, {
		method: 'POST',
		headers,
		body: JSON.stringify(payloadStart)
	});
	let body = await resStart.json();
	if ('error' in body) {
		return {
			error: body.error.message || 'did not receive any registration data'
		};
	}

	// We need to apply a small hack to make TS happy.
	// The browser expects ArrayBuffers in some places, but the backend sends them as base64 encoded data,
	// which we will decode properly in the following lines.
	let options = body as unknown as CredentialCreationOptions;
	if (!options.publicKey) {
		let error = 'no publicKey in options from the backend';
		console.error(error, options);
		return { error };
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
				error: errorI18nInvalidKey
			};
		}
	} catch (e) {
		console.error(e);
		const timeout = new Date().getTime() >= expTime;
		return {
			error: timeout ? errorI18nTimeout : errorI18nInvalidKey
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
				clientDataJSON: arrBufToBase64UrlSafe(credential.response.clientDataJSON)
			},
			// @ts-ignore the `response.getClientExtensionResults()` actually exists
			extensions: credential.getClientExtensionResults(),
			type: credential.type
		},
		magic_link_id: magicLinkId
	};

	let resFinish = await fetch(`/auth/v1/users/${userId}/webauthn/register/finish`, {
		method: 'POST',
		headers,
		body: JSON.stringify(payloadFinish)
	});
	if (resFinish.status === 201) {
		return {};
	} else {
		let body = await resFinish.json();
		return {
			error: body.error?.message || 'Registration failed'
		};
	}
}
