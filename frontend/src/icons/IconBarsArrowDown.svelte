<script lang="ts">
	import type { IconProps } from './props.ts';

	let { opacity = 0.9, width = '1.5rem', color = 'currentColor' }: IconProps = $props();
</script>

<svg
	fill="none"
	viewBox="0 0 24 24"
	stroke={color}
	stroke-width={2}
	{width}
	{opacity}
>
	<path
		stroke-linecap="round"
		stroke-linejoin="round"
		d="M3 4.5h14.25M3 9h9.75M3 13.5h9.75m4.5-4.5v12m0 0l-3.75-3.75M17.25 21L21 17.25"
	/>
</svg>
