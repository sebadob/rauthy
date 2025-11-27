import { decode, encode } from 'base64-arraybuffer';

export function arrBufToBase64UrlSafe(buffer: ArrayBuffer): Base64URLString {
	let base64 = encode(buffer);
	const enc = {
		'+': '-',
		'/': '_',
		'=': '',
	};
	return base64.replace(/[+/=]/g, m => enc[m as '+' | '/' | '=']);
}

export function base64UrlSafeToArrBuf(base64url: ArrayBuffer | BufferSource | Base64URLString) {
	if (typeof base64url === 'string') {
		const dec = {
			'-': '+',
			_: '/',
			'.': '=',
		};
		const base64 = base64url.replace(/[-_.]/g, m => dec[m as '-' | '_' | '.']);
		return decode(base64);
	} else {
		return base64url as ArrayBuffer;
	}
}
