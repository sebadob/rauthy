<script lang="ts">
	import type { IconProps } from '$icons/props';

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
		d="M3.75 6.75h16.5M3.75 12h16.5m-16.5 5.25H12"
	/>
</svg>
