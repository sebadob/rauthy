<script lang="ts">
	import { getProviderToken, getVerifierUpstreamFromStorage } from '$utils/helpers';
	import { onMount } from 'svelte';
	import LangSelector from '$lib5/LangSelector.svelte';
	import { useParam } from '$state/param.svelte';
	import ThemeSwitch from '$lib5/ThemeSwitch.svelte';
	import { fetchPost } from '$api/fetch';
	import type { WebauthnLoginResponse } from '$api/types/authorize.ts';
	import type { AtprotoCallbackRequest } from '$api/types/atproto.ts';

	let error = $state('');

	onMount(async () => {
		if (window.location.href.startsWith('http://127.0.0.1')) {
			const next = new URL(window.location.href);

			next.host = 'localhost';

			window.location.replace(next.toString());
		}

		let pErr = useParam('error').get();
		if (pErr) {
			// if we have any error, do not proceed like normal and only show the error
			let desc = useParam('error_description').get();
			error = `${pErr}: ${desc}`;
			return;
		}

		let state = useParam('state').get();
		if (!state) {
			error = "'state' is missing in URL";
			return;
		}
		let code = useParam('code').get();
		if (!code) {
			error = "'code' is missing in URL";
			return;
		}
		let iss = useParam('iss').get();
		if (!iss) {
			error = "'iss' is missing in URL";
			return;
		}

		let payload: AtprotoCallbackRequest = {
			state,
			code,
			iss,
			pkce_verifier: getVerifierUpstreamFromStorage(),
			xsrf_token: getProviderToken()
		};

		const url = '/auth/v1/atproto/callback';

		let res = await fetchPost<undefined | WebauthnLoginResponse>(url, payload);

		console.error(res.status);

		if (res.status === 200) {
			// -> all good
			window.location.replace(res.headers.get('location') || '/auth/v1/account');
		} else if (res.status === 204) {
			// in case of a 204, we have done a user federation on an existing account -> just redirect
			window.location.replace('/auth/v1/account');
		} else if (res.status === 403) {
			// we will get a forbidden if for instance the user already exists but without
			// any upstream provider link (or the wrong one)
			error = res.error?.message || 'HTTP 403 Forbidden';
		} else {
			error = `HTTP ${res.status}: ${res.error?.message}`;
		}
	});
</script>

<svelte:head>
	<title>Callback</title>
</svelte:head>

{#if error}
	<div class="err">
		{error}
	</div>
{/if}

<ThemeSwitch absolute />
<LangSelector absolute />

<style>
</style>
