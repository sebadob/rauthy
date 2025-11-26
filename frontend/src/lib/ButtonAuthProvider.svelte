<script lang="ts">
	import Button from '$lib5/button/Button.svelte';
	import type { AuthProviderTemplate } from '$api/templates/AuthProvider.ts';

	let {
		ariaLabel,
		provider,
		onclick,
		isLoading,
	}: {
		ariaLabel: string;
		provider: AuthProviderTemplate;
		onclick: (providerId: string) => void;
		isLoading: boolean;
	} = $props();

	let showIcon = $state(false);
</script>

<Button
	{ariaLabel}
	level={2}
	onclick={() => onclick(provider.id)}
	{isLoading}
>
	<div class="inline">
		<img
			src={`/auth/v1/providers/${provider.id}/img?updated=${provider.updated}`}
			alt="Provider Logo"
			width="20"
			height="20"
			aria-hidden={!showIcon}
			onload={() => (showIcon = true)}
		/>
		<span class="name">
			{provider.name}
		</span>
	</div>
</Button>

<style>
	img {
		transform: translateY(0.15rem);
	}

	img[aria-hidden='true'] {
		display: none;
	}

	.inline {
		display: flex;
		justify-content: center;
		gap: 0.5rem;
	}

	.name {
		margin-bottom: -0.1rem;
	}
</style>
