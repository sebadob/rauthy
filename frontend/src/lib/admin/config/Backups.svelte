<script lang="ts">
	import type { BackupListing, BackupListings } from '$api/types/backup';
	import { fetchGet, fetchPost } from '$api/fetch';
	import { onMount } from 'svelte';
	import { useI18nAdmin } from '$state/i18n_admin.svelte';
	import { formatDateFromTs, prettyFileSize } from '$utils/helpers';
	import Button from '$lib/button/Button.svelte';
	import IconCheck from '$icons/IconCheck.svelte';
	import IconArrowPathSquare from '$icons/IconArrowPathSquare.svelte';
	import A from '$lib/A.svelte';

	let ta = useI18nAdmin();

	let local: BackupListing[] = $state([]);
	let s3: BackupListing[] = $state([]);

	let isDisabled = $state(false);
	let success = $state(false);

	onMount(() => {
		fetchListings();
	});

	async function postCreateBackup() {
		let res = await fetchPost('/auth/v1/backup');
		if (res.error) {
			console.error(res.error);
		} else {
			success = true;

			// S3 pushes are done async in the background
			setTimeout(() => {
				fetchListings();
			}, 1000);

			setTimeout(() => {
				success = false;
			}, 5000);
		}
	}

	async function fetchListings() {
		let res = await fetchGet<BackupListings>('/auth/v1/backup');
		if (res.body) {
			res.body.local.sort((a, b) => b.last_modified - a.last_modified);
			res.body.s3.sort((a, b) => b.last_modified - a.last_modified);

			local = res.body.local;
			s3 = res.body.s3;
		} else if (res.error) {
			if (res.error.error === 'NotFound') {
				isDisabled = true;
			} else {
				console.error(res);
			}
		}
	}
</script>

{#snippet header()}
	<div class="header row">
		<div>{ta.backup.name}</div>
		<div class="right">{ta.backup.lastModified}</div>
		<div class="right">{ta.backup.size}</div>
	</div>
{/snippet}

{#snippet backup(backup: BackupListing, hrefPrefix: string)}
	<div class="row">
		<A href={`${hrefPrefix}/${backup.name}`}>
			<span class="font-mono">
				{backup.name}
			</span>
		</A>
		<div class="font-mono muted right">
			{formatDateFromTs(backup.last_modified)}
		</div>
		<div class="font-mono muted right">
			{#if backup.size}
				{prettyFileSize(backup.size)}
			{:else}
				-
			{/if}
		</div>
	</div>
{/snippet}

{#if isDisabled}
	<p>{ta.backup.disabledDesc}</p>
{:else}
	<div class="flex gap-05">
		<Button onclick={postCreateBackup}>
			{ta.backup.createBackup}
		</Button>

		<div class="refresh">
			<Button
				invisible
				onclick={fetchListings}
			>
				<IconArrowPathSquare />
			</Button>
		</div>

		{#if success}
			<IconCheck />
		{/if}
	</div>

	{#if local.length === 0 && s3.length === 0}
		{ta.common.noEntries}
	{:else}
		<h2>{ta.backup.local}</h2>
		{@render header()}
		{#each local as b (b.name)}
			{@render backup(b, '/auth/v1/backup/local')}
		{/each}

		<h2>S3</h2>
		{@render header()}
		{#each s3 as b (b.name)}
			{@render backup(b, '/auth/v1/backup/s3')}
		{/each}
	{/if}
{/if}

<style>
	h2 {
		margin-top: 1.5rem;
	}

	.header {
		font-weight: bold;
	}

	.muted {
		color: hsla(var(--text) / 0.7);
	}

	.refresh {
		margin-bottom: -0.4rem;
	}

	.right {
		text-align: right;
	}

	.row {
		display: grid;
		grid-template-columns: 20rem 12rem 8rem;
	}
</style>
