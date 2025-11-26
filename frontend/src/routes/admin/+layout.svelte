<script lang="ts">
	import type { Snippet } from 'svelte';
	import Main from '$lib5/Main.svelte';
	import Button from '$lib5/button/Button.svelte';
	import NavSide from '$lib5/nav/NavSide.svelte';
	import { useSession } from '$state/session.svelte.ts';
	import { fetchGet } from '$api/fetch.ts';
	import Events from '$lib5/admin/events/Events.svelte';
	import { initI18nAdmin, useI18nAdmin } from '$state/i18n_admin.svelte.ts';

	let {
		children,
	}: {
		children: Snippet;
	} = $props();

	initI18nAdmin();

	let ta = useI18nAdmin();
	let session = useSession('admin');

	let innerWidth: undefined | number = $state();

	let isAdmin = $state(false);
	let needsAdminRole = $state(false);
	let mfaReqErr = $state(false);

	$effect(() => {
		let s = session.get();
		if (s) {
			let isAdm = !!s?.roles?.includes('rauthy_admin');
			if (isAdm) {
				isAdmin = true;
				// async check for admin access speeds up FCP and is still fast enough for a good UX
				checkAdminAccess();
			} else {
				needsAdminRole = true;
			}
		}
	});

	async function checkAdminAccess() {
		let res = await fetchGet('/auth/v1/auth_check_admin');
		if (res.status === 406) {
			mfaReqErr = true;
		}
	}
</script>

<svelte:window bind:innerWidth />

<svelte:head>
	<title>Rauthy Admin</title>
</svelte:head>

{#if mfaReqErr}
	<div class="noAdmin">
		<div>
			<div class="text">
				{@html ta.error.noAdmin}
			</div>
			<Button onclick={() => (window.location.href = '/auth/v1/account')}
				>{ta.common.account}</Button
			>
		</div>
	</div>
{:else if needsAdminRole}
	<div class="noAdmin">
		<div>
			<div class="text">
				{@html ta.error.needsAdminRole}
			</div>
			<Button onclick={() => (window.location.href = '/auth/v1/')}>{ta.common.back}</Button>
		</div>
	</div>
{:else if isAdmin}
	<NavSide />
	<Main>
		<div class="content">
			{@render children()}
		</div>
		{#if innerWidth && innerWidth > 1024}
			<div class="events">
				<Events />
			</div>
		{/if}
	</Main>
{/if}

<style>
	.noAdmin {
		width: 100dvw;
		display: flex;
		flex-direction: column;
		justify-content: center;
		align-items: center;
	}

	.content {
		flex: 1;
		display: flex;
	}

	.text {
		margin-bottom: 1rem;
	}
</style>
