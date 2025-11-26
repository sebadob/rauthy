<script lang="ts">
	import { formatUtcTsFromDateInput } from '$utils/helpers';
	import Button from '$lib5/button/Button.svelte';
	import { slide } from 'svelte/transition';
	import IconCheck from '$icons/IconCheck.svelte';
	import LabeledValue from '$lib5/LabeledValue.svelte';
	import InputCheckbox from '$lib5/form/InputCheckbox.svelte';
	import InputDateTimeCombo from '$lib5/form/InputDateTimeCombo.svelte';
	import type { ApiKeyAccess, ApiKeyRequest, ApiKeyResponse } from '$api/types/api_keys.ts';
	import { useI18n } from '$state/i18n.svelte';
	import { useI18nAdmin } from '$state/i18n_admin.svelte';
	import { fmtDateInput, fmtTimeInput } from '$utils/form';
	import { fetchPut } from '$api/fetch';
	import ApiKeyMatrix from '$lib5/admin/api_keys/ApiKeyMatrix.svelte';

	let {
		key = $bindable(),
		onSave,
	}: {
		key: ApiKeyResponse;
		onSave: () => void;
	} = $props();

	let t = useI18n();
	let ta = useI18nAdmin();

	const minDate = fmtDateInput();

	let err = $state('');
	let success = $state(false);

	let doesExpire = $state(!!key.expires);
	let expDate = $state(fmtDateInput());
	let expTime = $state(fmtTimeInput());

	let finalizeMatrix: undefined | (() => ApiKeyAccess[]) = $state();

	$effect(() => {
		if (key.name) {
			doesExpire = !!key.expires;
		}
	});

	$effect(() => {
		if (doesExpire) {
			let dt: Date;
			if (key.expires) {
				dt = new Date(key.expires * 1000);
			} else {
				dt = new Date();
			}
			expDate = fmtDateInput(dt);
			expTime = fmtTimeInput(dt);
		}
	});

	async function onSubmit() {
		err = '';

		if (!finalizeMatrix) {
			console.error('finalizeMatrix is undefined');
			return;
		}

		let payload: ApiKeyRequest = {
			name: key.name,
			access: finalizeMatrix(),
		};
		if (payload.access.length === 0) {
			err = 'Grant the API Key at least one permission';
			return;
		}

		if (doesExpire) {
			let exp = formatUtcTsFromDateInput(expDate, expTime);
			if (!exp) {
				err = 'Invalid Date Input: User Expires';
				return;
			}
			payload.exp = exp;
		}

		let res = await fetchPut(`/auth/v1/api_keys/${key.name}`, payload);
		if (res.error) {
			err = res.error.message;
		} else {
			onSave();
			success = true;
			setTimeout(() => {
				success = false;
				onSave();
			}, 2000);
		}
	}
</script>

<LabeledValue label={ta.api_key.keyName}>
	{key.name}
</LabeledValue>

<InputCheckbox
	ariaLabel={ta.api_key.limitedValidity}
	bind:checked={doesExpire}
>
	{ta.api_key.limitedValidity}
</InputCheckbox>

{#if doesExpire}
	<div transition:slide={{ duration: 150 }}>
		<InputDateTimeCombo
			label={ta.api_key.expires}
			bind:value={expDate}
			bind:timeValue={expTime}
			withTime
			min={minDate}
			required
		/>
	</div>
{/if}

<ApiKeyMatrix
	{key}
	bind:finalize={finalizeMatrix}
/>

<div class="btn">
	<Button onclick={onSubmit}>
		{t.common.save}
	</Button>

	{#if success}
		<IconCheck />
	{/if}

	{#if err}
		<div class="err">
			{err}
		</div>
	{/if}
</div>

<style>
	.btn {
		margin-top: 1rem;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}
</style>
