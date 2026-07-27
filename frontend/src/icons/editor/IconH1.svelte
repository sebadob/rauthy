<script lang="ts">
    import type { IconProps } from '$icons/props';

    let { opacity = 0.9, width = '1.5rem', color = 'currentColor' }: IconProps = $props();
</script>

<svg fill="none" viewBox="0 0 24 24" stroke={color} stroke-width={2} {width} {opacity}>
    <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="M2.243 4.493v7.5m0 0v7.502m0-7.501h10.5m0-7.5v7.5m0 0v7.501m4.501-8.627 2.25-1.5v10.126m0
            0h-2.25m2.25 0h2.25"
    />
</svg>
