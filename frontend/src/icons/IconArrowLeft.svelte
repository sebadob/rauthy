<script lang="ts">
    import type { IconProps } from './props.ts';

    let { opacity = 0.9, width = '1.5rem', color = 'currentColor' }: IconProps = $props();
</script>

<svg fill="none" viewBox="0 0 24 24" stroke={color} stroke-width={2} {width} {opacity}>
    <path stroke-linecap="round" stroke-linejoin="round" d="M10.5 19.5 3 12m0 0 7.5-7.5M3 12h18" />
</svg>
