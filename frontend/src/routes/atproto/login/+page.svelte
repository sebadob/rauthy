<script>
	import { onMount } from 'svelte';
	import Button from '$lib/Button.svelte';
	import Input from '$lib/inputs/Input.svelte';
	import * as yup from 'yup';
	import { PKCE_VERIFIER_UPSTREAM, REGEX_URI, REGEX_AT_ID } from '../../../utils/constants.js';
	import {
		extractFormErrors,
		formatDateFromTs,
		getQueryParams,
		saveProviderToken
	} from '../../../utils/helpers.js';

	import getPkce from 'oauth-pkce';
	import BrowserCheck from '../../../components/BrowserCheck.svelte';
	import IconHome from '$lib/icons/IconHome.svelte';
	import Logo from '../../../components/atproto/Logo.svelte';
	import { postAtprotoLogin } from '../../../utils/dataFetching.js';

	let redirectUri = '';

	let state = '';
	let pkce_challenge = '';

	let isLoading = false;
	let err = '';
	let tooManyRequests = false;

	let formValues = { at_id: '' };
	let formErrors = {};

	let schema = yup.object().shape({
		at_id: yup
			.string()
			.required('AT Identifier is required')
			.matches(REGEX_AT_ID, 'AT Identifier must be valid')
			.max(128),
		state: yup
			.string()
			.required('state is required')
			.matches(REGEX_URI, 'state must be valid')
			.max(128),
		redirect_uri: yup
			.string()
			.required('redirect URI is required')
			.matches(REGEX_URI, 'redirect URI must be valid')
			.max(128),
		pkce_challenge: yup
			.string()
			.required('PKCE challenge is required')
			.matches(REGEX_URI, 'PKCE challenge must be valid')
			.max(128)
	});

	onMount(async () => {
		state = generateRandomString();

		const params = getQueryParams();
		redirectUri = params.redirect_uri || 'https://atproto.com';

		getPkce(64, (error, { challenge, verifier }) => {
			if (!error) {
				localStorage.setItem(PKCE_VERIFIER_UPSTREAM, verifier);
				pkce_challenge = challenge;
			}
		});
	});

	async function onSubmit() {
		console.log('onSubmit');
		err = '';

		console.log('validate');
		try {
			await schema.validate(formValues, { abortEarly: false });
			formErrors = {};
		} catch (err) {
			formErrors = extractFormErrors(err);
			return;
		}
		console.log('validate finished');

		const req = {
			at_identifier: formValues.at_id,
			state: state,
			redirect_uri: redirectUri,
			pkce_challenge
		};
		console.log('postAtprotoLogin');
		let res = await postAtprotoLogin(req);
		console.log('postAtprotoLogin finished');

		if (res.status === 202) {
			console.log('OK');
			const xsrfToken = await res.text();
			saveProviderToken(xsrfToken);

			// -> all good
			window.location.replace(res.headers.get('location'));
		} else if (res.status === 429) {
			// 429 -> too many failed logins
			let notBefore = Number.parseInt(res.headers.get('x-retry-not-before'));
			let nbfDate = formatDateFromTs(notBefore);
			let diff = notBefore * 1000 - new Date().getTime();

			tooManyRequests = true;
			err = `Too many invalid inputs. Locked until: ${nbfDate}`;

			formValues.at_id = '';

			setTimeout(() => {
				tooManyRequests = false;
				err = '';
			}, diff);
		} else {
			let body = await res.json();
			err = body.message;
		}
		isLoading = false;
	}

	function generateRandomString(length = 64) {
		const characters = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
		const randomValues = new Uint32Array(length);

		window.crypto.getRandomValues(randomValues);

		let result = '';
		randomValues.forEach((value) => {
			result += characters.charAt(value % characters.length);
		});
		return result;
	}
</script>

<svelte:head>
	<title>ATProto Login</title>
</svelte:head>

<BrowserCheck>
	<div class="container">
		<div class="head">
			<div class="logo">
				<Logo />
			</div>
			<a class="home" href="https://atproto.com">
				<IconHome opacity={0.5} />
			</a>
		</div>

		<div class="name">
			<h2>ATProto</h2>
		</div>

		<Input
			type="text"
			name="atIdentifier"
			bind:value={formValues.at_id}
			bind:error={formErrors.at_id}
			autocomplete="off"
			placeholder="handle or DID"
			disabled={tooManyRequests}
			on:enter={onSubmit}
		>
			{'AT Identifier'.toUpperCase()}
		</Input>

		{#if !tooManyRequests}
			<div class="btn flex-col">
				<Button on:click={onSubmit} bind:isLoading>
					{'Login'.toUpperCase()}
				</Button>
			</div>
		{/if}

		{#if err}
			<div class="errMsg errMsgApi">
				{err}
			</div>
		{/if}
	</div>
</BrowserCheck>

<style>
	.btn {
		margin: 5px 0;
		display: flex;
	}

	.container {
		display: flex;
		flex-direction: column;
		justify-content: center;
		max-width: 19rem;
		padding: 20px;
		border: 1px solid var(--col-gmid);
		border-radius: 5px;
		box-shadow: 5px 5px 5px rgba(128, 128, 128, 0.1);
	}

	.errMsg {
		max-width: 15rem;
		margin: -10px 10px 5px 5px;
		color: var(--col-err);
	}

	.flex-col {
		display: flex;
		flex-direction: column;
	}

	.head {
		display: flex;
		justify-content: space-between;
	}

	.home {
		margin-right: 5px;
		cursor: pointer;
	}

	.name {
		margin: -10px 5px 0 5px;
	}

	.logo {
		margin: 0 0.25rem;
		width: 84px;
		height: 84px;
		display: flex;
		justify-content: center;
		align-items: center;
	}
</style>
