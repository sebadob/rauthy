<script lang="ts">
	import Button from '$lib5/button/Button.svelte';
	import Input from '$lib5/form/Input.svelte';
	import { slide } from 'svelte/transition';
	import { formatUtcTsFromDateInput } from '$utils/helpers.js';
	import { fmtDateInput, fmtTimeInput } from '$utils/form';
	import type { ApiKeyAccess, ApiKeyRequest, ApiKeyResponse } from '$api/types/api_keys.ts';
	import { useI18n } from '$state/i18n.svelte';
	import { useI18nAdmin } from '$state/i18n_admin.svelte';
	import ApiKeyMatrix from '$lib5/admin/api_keys/ApiKeyMatrix.svelte';
	import { fetchPost } from '$api/fetch';
	import { PATTERN_API_KEY } from '$utils/patterns';
	import InputCheckbox from '$lib5/form/InputCheckbox.svelte';
	import InputDateTimeCombo from '$lib5/form/InputDateTimeCombo.svelte';
	import Form from '$lib5/form/Form.svelte';

	const minDate = fmtDateInput();

	let {
		keys,
		onSave,
	}: {
		keys: ApiKeyResponse[];
		onSave: () => void;
	} = $props();

	let t = useI18n();
	let ta = useI18nAdmin();

	let err = $state('');
	let doesExpire = $state(false);
	// IMPORTANT: do NOT give a default here -> will be initialized inside ApiKeyAccessMatrix!
	let finalizeMatrix: undefined | (() => ApiKeyAccess[]) = $state();

	let name = $state('');
	let expDate = $state(fmtDateInput());
	let expTime = $state(fmtTimeInput());

	async function onSubmit(form: HTMLFormElement, params: URLSearchParams) {
		err = '';

		for (let key of keys) {
			if (key.name === name) {
				err = 'Name already exists';
				return;
			}
		}

		let access = finalizeMatrix?.();
		if (!access) {
			console.error('access rights is undefined');
			return;
		}

		let payload: ApiKeyRequest = {
			name,
			access,
		};
		if (payload.access.length === 0) {
			err = 'Grant the new API Key at least one permission';
			return;
		}

		if (doesExpire) {
			if (!expDate || !expTime) {
				err = 'Disable expiry or provide an valid date and time';
				return;
			}

			let exp = formatUtcTsFromDateInput(expDate, expTime);
			if (!exp) {
				err = 'Invalid Date Input: User Expires';
				return;
			}
			payload.exp = exp;
		}

		let res = await fetchPost(form.action, payload);
		if (res.error) {
			err = res.error.message;
		} else {
			onSave();
		}
	}
</script>

<div class="container">
	<Form
		action="/auth/v1/api_keys"
		{onSubmit}
	>
		<Input
			label={ta.api_key.keyName}
			placeholder={ta.api_key.keyName}
			autocomplete="off"
			bind:value={name}
			required
			min="2"
			max="24"
			pattern={PATTERN_API_KEY}
		/>

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

		<ApiKeyMatrix bind:finalize={finalizeMatrix} />

		<Button type="submit">
			{t.common.save}
		</Button>

		<div class="err">
			{err}
		</div>
	</Form>
</div>

<style>
	.container {
		min-height: 34rem;
		text-align: left;
	}
</style>
