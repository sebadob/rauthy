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
		d="M7.5 21 3 16.5m0 0L7.5 12M3 16.5h13.5m0-13.5L21 7.5m0 0L16.5 12M21 7.5H7.5"
	/>
</svg>
