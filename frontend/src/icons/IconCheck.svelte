<script lang="ts">
	import type { IconProps } from './props.ts';

	let { opacity = 0.9, width = '1.5rem', color = 'hsl(var(--action))' }: IconProps = $props();
</script>

<svg
	fill="none"
	viewBox="0 0 24 24"
	stroke="currentColor"
	stroke-width={2}
	{width}
	{color}
	{opacity}
>
	<path
		stroke-linecap="round"
		stroke-linejoin="round"
		d="M4.5 12.75l6 6 9-13.5"
	/>
</svg>
