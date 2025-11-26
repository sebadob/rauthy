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
		d="M12 6v6h4.5m4.5 0a9 9 0 1 1-18 0 9 9 0 0 1 18 0Z"
	/>
</svg>
