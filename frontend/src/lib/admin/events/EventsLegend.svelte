<script lang="ts">
	import { eventColor } from '$utils/helpers';
	import { useI18n } from '$state/i18n.svelte';

	let { wide }: { wide: boolean } = $props();

	let t = useI18n();
</script>

<fieldset aria-hidden="true">
	<legend class="font-label">{t.common.legend}</legend>
	<ul class:wide>
		<li style:border-color={eventColor('info')}>Info</li>
		<li style:border-color={eventColor('notice')}>Notice</li>
		<li style:border-color={eventColor('warning')}>Warning</li>
		<li style:border-color={eventColor('critical')}>Critical</li>
	</ul>
</fieldset>

<style>
	fieldset {
		line-height: 1rem;
		border: none;
	}

	legend {
		font-size: 0.9rem;
		color: hsla(var(--text) / 0.5);
	}

	ul {
		padding-right: 0.5rem;
		display: flex;
		flex-direction: column;
		justify-content: space-between;
	}

	li {
		margin: 0;
		padding-left: 0.5rem;
		list-style: none;
		border-left: 2px solid transparent;
		color: hsla(var(--text) / 0.7);
	}

	.wide {
		flex-direction: row;
	}
</style>
