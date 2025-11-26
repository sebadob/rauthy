<script lang="ts">
	import { onMount } from 'svelte';
	import Button from '$lib5/button/Button.svelte';
	import IconCheck from '$icons/IconCheck.svelte';
	import { fetchGet, fetchPost } from '$api/fetch';
	import type { EncKeyMigrateRequest, EncKeysResponse } from '$api/types/enc_keys.ts';
	import { useI18nAdmin } from '$state/i18n_admin.svelte';
	import Options from '$lib5/Options.svelte';

	let ta = useI18nAdmin();

	let err = $state('');
	let isLoading = $state(true);
	let activeKey = $state('');
	let migrateKey = $state('');
	let keys: string[] = $state([]);
	let success = $state(false);

	onMount(async () => {
		let res = await fetchGet<EncKeysResponse>('/auth/v1/encryption/keys');
		if (res.body) {
			keys = res.body.keys;
			activeKey = res.body.active;
			migrateKey = res.body.active;
		} else {
			err = res.error?.message || 'Error';
		}
		isLoading = false;
	});

	async function migrate() {
		isLoading = true;

		let payload: EncKeyMigrateRequest = {
			key_id: migrateKey,
		};
		let res = await fetchPost('/auth/v1/encryption/migrate', payload);
		if (res.error) {
			err = res.error.message;
		} else {
			success = true;
			setTimeout(() => {
				success = false;
			}, 4000);
		}

		isLoading = false;
	}
</script>

<h3>{ta.docs.encKeys.header}</h3>

<p>{ta.docs.encKeys.p1}</p>
<p>{ta.docs.encKeys.p2}</p>

<p>
	<span class="font-label">
		{ta.docs.encKeys.keyActive}:
	</span>
	<span class="active font-mono">
		{activeKey}
	</span>
</p>

<p>
	{ta.docs.encKeys.keysAvailable}:
</p>

<ul>
	{#each keys as key}
		<li class="font-mono">
			{key}
		</li>
	{/each}
</ul>

<p>{ta.docs.encKeys.p3}</p>

{#if keys.length > 1}
	<p>{ta.docs.encKeys.migrateToKey}:</p>

	<Options
		ariaLabel={ta.docs.encKeys.migrateToKey}
		options={keys}
		bind:value={migrateKey}
	/>

	<div class="btn flex gap-05">
		<Button
			onclick={migrate}
			{isLoading}
		>
			{ta.docs.encKeys.migrate}
		</Button>

		{#if success}
			<IconCheck />
		{/if}
	</div>
{:else}
	<p class="err">{ta.docs.encKeys.pNotPossible}</p>
{/if}

{#if err}
	<div class="err">
		{err}
	</div>
{/if}

<style>
	.active {
		color: hsl(var(--text-high));
		font-weight: bold;
	}

	.btn {
		margin: 1rem 0;
	}
</style>
