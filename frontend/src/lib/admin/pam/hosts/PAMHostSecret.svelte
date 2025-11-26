<script lang="ts">
	import Modal from '$lib/Modal.svelte';
	import Button from '$lib/button/Button.svelte';
	import { fetchPost, fetchPut } from '$api/fetch';
	import type { PamHostSecretResponse } from '$api/types/pam';
	import HiddenValueArea from '$lib/HiddenValueArea.svelte';
	import { useI18nAdmin } from '$state/i18n_admin.svelte';
	import { useI18n } from '$state/i18n.svelte';

	let {
		hostId,
	}: {
		hostId: string;
	} = $props();

	let t = useI18n();
	let ta = useI18nAdmin();

	let closeModal: undefined | (() => void) = $state();
	let showModal = $state(false);

	let err = $state('');
	let secret = $state('');
	let url = $derived(`/auth/v1/pam/hosts/${hostId}/secret`);

	async function fetchSecret() {
		let res = await fetchPost<PamHostSecretResponse>(url);
		if (res.body) {
			secret = res.body.secret;
		} else {
			err = res.error?.message || 'Error';
		}
	}

	function onclick() {
		fetchSecret();
		showModal = true;
	}

	async function rotate() {
		let res = await fetchPut<PamHostSecretResponse>(url);
		if (res.body) {
			secret = res.body.secret;
		} else {
			err = res.error?.message || 'Error';
		}
	}
</script>

<div class="container">
	<Button
		level={2}
		{onclick}
	>
		{ta.pam.secretShow}
	</Button>
	<Modal
		bind:showModal
		bind:closeModal
	>
		<h1>Secret</h1>

		{#key secret}
			<HiddenValueArea
				ariaLabel="Secret"
				rows={3}
				value={secret}
			/>
		{/key}

		<div class="btn">
			<Button onclick={rotate}>
				{ta.pam.secretRotate}
			</Button>
			<Button
				level={2}
				onclick={() => closeModal?.()}
			>
				{t.common.close}
			</Button>
		</div>

		{#if err}
			<div class="err">
				{err}
			</div>
		{/if}
	</Modal>
</div>

<style>
	.btn {
		margin-top: 1rem;
		display: flex;
		align-items: center;
		gap: 0.5rem;
	}

	.container {
		margin: 1rem 0;
	}
</style>
