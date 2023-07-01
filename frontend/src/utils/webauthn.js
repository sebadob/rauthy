import { webauthnAuthFinish, webauthnAuthStart } from "./dataFetching.js";
import { arrBufToBase64UrlSafe, base64UrlSafeToArrBuf } from "./helpers.js";

export async function webauthnAuth(uid, data) {
	let res = await webauthnAuthStart(uid, data);
	if (res.status === 200) {
		let resp = await res.json();
		let challenge = resp.rcr;

		// the navigator credentials engine needs some values as array buffers
		challenge.publicKey.challenge = base64UrlSafeToArrBuf(challenge.publicKey.challenge);
		for (let cred of challenge.publicKey.allowCredentials) {
			cred.id = base64UrlSafeToArrBuf(cred.id);
		}

		// prompt for the user security key and get its public key
		let challengePk
		try {
			challengePk = await navigator.credentials.get(challenge);
		} catch (e) {
			return {
				err: true,
				msg: 'Invalid Key used'
			};
		}

		// the backend expects base64 url safe string instead of array buffers
		let data = {
			code: resp.code,
			data: {
				id: challengePk.id,
				rawId: arrBufToBase64UrlSafe(challengePk.rawId),
				response: {
					authenticatorData: arrBufToBase64UrlSafe(challengePk.response.authenticatorData),
					clientDataJSON: arrBufToBase64UrlSafe(challengePk.response.clientDataJSON),
					signature: arrBufToBase64UrlSafe(challengePk.response.signature),
				},
				type: challengePk.type,
			}
		}

		// send the data to the backend
		res = await webauthnAuthFinish(uid, data);
		if (res.status === 202) {
			let body = await res.json();
			return {
				err: false,
				msg: 'Authentication successful',
				body,
			};
		} else {
			console.error(res);
		}
	} else {
		return {
			err: true,
			msg: 'Error starting the Authentication'
		};
	}
}
