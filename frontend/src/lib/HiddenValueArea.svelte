<script lang="ts">
	import IconClipboard from '$icons/IconClipboard.svelte';
	import IconEye from '$icons/IconEye.svelte';
	import IconEyeSlash from '$icons/IconEyeSlash.svelte';
	import Button from '$lib5/button/Button.svelte';
	import { onMount } from 'svelte';
	import { useI18n } from '$state/i18n.svelte';

	let {
		ariaLabel = '',
		value = '',
		rows = 10,
		cols = 60,
		show = $bindable(false),
		width = 'min(25rem, calc(100dvw - 2rem))',
		...rest
	}: {
		ariaLabel: string;
		value: string;
		rows?: number;
		cols?: number;
		show?: boolean;
		width?: string;
	} = $props();

	let t = useI18n();

	let hidden = $state('');
	let text = $state('');

	$effect(() => {
		if (show) {
			text = value;
		} else {
			text = hidden;
		}
	});

	onMount(() => {
		for (let i = 0; i < value.length; i++) {
			hidden = hidden + '*';
		}
		text = hidden;
		show = false;
	});

	function copyToClip() {
		navigator.clipboard.writeText(value);
	}

	function toggle() {
		show = !show;
	}
</script>

<div style:width={`${width}`}>
	<div class="iconsOuter">
		<div class="iconsInner">
			<Button
				ariaLabel={show ? t.common.hide : t.common.show}
				invisible
				onclick={toggle}
			>
				{#if show}
					<div title={t.common.hide}>
						<IconEye />
					</div>
				{:else}
					<div title={t.common.show}>
						<IconEyeSlash />
					</div>
				{/if}
			</Button>

			<Button
				ariaLabel={t.common.copyToClip}
				invisible
				onclick={copyToClip}
			>
				<div title={t.common.copyToClip}>
					<IconClipboard />
				</div>
			</Button>
		</div>
	</div>

	<textarea
		aria-label={ariaLabel}
		style:width
		style:padding-right="3.25rem"
		disabled
		bind:value={text}
		{rows}
		{cols}
		{...rest}
	></textarea>
</div>

<style>
	textarea {
		width: 100%;
		/*width: min(100%, calc(100dvw - 2rem));*/
		padding: 0.25rem 0.5rem;
		border: 1px solid hsl(var(--bg-high));
		border-radius: var(--border-radius);
		color: hsl(var(--text));
		background: hsl(var(--bg-high));
		outline: none;
		resize: none;
	}

	textarea:focus-visible {
		outline: hsl(var(--accent));
	}

	.iconsOuter {
		position: relative;
	}

	.iconsInner {
		position: absolute;
		top: 2px;
		right: 1px;
		cursor: pointer;
		opacity: 0.85;
	}
</style>
