<script lang="ts">
	import A from '$lib5/A.svelte';
	import type { Snippet } from 'svelte';
	import { useTrigger } from '$state/callback.svelte';

	let {
		compact,
		params,
		route,
		icon,
		highlightIncludes,
		children,
	}: {
		compact: boolean;
		params: string;
		route: string;
		icon: Snippet<[string]>;
		highlightIncludes?: string;
		children: Snippet;
	} = $props();

	const urlPrefix = '/auth/v1/admin';

	let tr = useTrigger();

	let width = $derived(compact ? '1.5rem' : '1.2rem');
	let href = $derived(`${urlPrefix}${route}${params}`);

	function onclick() {
		// This is a bit of a hacky solution.
		// Our main issue, is that we are trying to solve a chicken-and-egg problem here.
		// The focus trigger function is set inside the component we are linking to, so it can only
		// exist AFTER our link was opened. If we only await the next animation frame, we have a fail
		// because of a race condition. The timeout of 100ms should be more than enough though.
		// The trigger does not depend on data we need to load inside the component, so it should be fine.
		requestAnimationFrame(() => {
			setTimeout(() => {
				tr.trigger('navMain');
			}, 100);
		});
	}
</script>

<A
	{href}
	hideUnderline
	{highlightIncludes}
	{onclick}
>
	{#if compact}
		<div class="compact">
			<div class="iconCompact">
				{@render icon(width)}
			</div>
			<span>
				{@render children()}
			</span>
		</div>
	{:else}
		<div class="wide">
			<div
				class="iconWide"
				style:width
			>
				{@render icon(width)}
			</div>
			<div>
				{@render children()}
			</div>
		</div>
	{/if}
</A>

<style>
	.compact {
		display: flex;
		flex-direction: column;
		align-items: center;
		font-size: 0.7rem;
		word-break: break-word;
	}

	.compact > span {
		color: hsla(var(--text) / 0.66);
	}

	.iconCompact {
		margin-bottom: -0.2rem;
	}

	.iconWide {
		transform: translateY(0.15rem);
	}

	.wide {
		display: flex;
		align-items: center;
		gap: 0.5rem;
		overflow: clip;
	}
</style>
