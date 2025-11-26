<script lang="ts">
	import IconBarsArrowDown from '$icons/IconBarsArrowDown.svelte';
	import IconBarsArrowUp from '$icons/IconBarsArrowUp.svelte';
	import Button from '$lib5/button/Button.svelte';
	import Tooltip from '$lib5/Tooltip.svelte';
	import Options from '$lib5/Options.svelte';
	import { useI18nAdmin } from '$state/i18n_admin.svelte';

	let {
		ref = $bindable(),
		onChange,
		options = [],
		borderless,
		firstDirReverse,
	}: {
		ref?: undefined | HTMLButtonElement;
		onChange: (option: string, direction: 'up' | 'down') => void;
		options: string[];
		borderless?: boolean;
		firstDirReverse?: boolean;
	} = $props();

	let ta = useI18nAdmin();

	let selected = $state(options[0]);
	let direction: 'up' | 'down' = $state(firstDirReverse ? 'down' : 'up');

	$effect(() => {
		onChange(selected, direction);
	});
</script>

<div class="container">
	<Tooltip text={ta.search.orderBy}>
		<Options
			bind:ref
			ariaLabel={ta.search.orderBy}
			{options}
			bind:value={selected}
			{borderless}
		/>
	</Tooltip>

	<div class="btn">
		{#if direction === 'up'}
			<Button
				ariaLabel={ta.search.orderChangeToDesc}
				invisible
				onclick={() => (direction = 'down')}
			>
				<Tooltip text={ta.search.orderChangeToDesc}>
					<IconBarsArrowUp />
				</Tooltip>
			</Button>
		{:else}
			<Button
				ariaLabel={ta.search.orderChangeToAsc}
				invisible
				onclick={() => (direction = 'up')}
			>
				<Tooltip text={ta.search.orderChangeToAsc}>
					<IconBarsArrowDown />
				</Tooltip>
			</Button>
		{/if}
	</div>
</div>

<style>
	.btn {
		margin: 0.25rem 1rem -0.25rem 0.5rem;
	}

	.container {
		display: flex;
		align-items: center;
	}
</style>
