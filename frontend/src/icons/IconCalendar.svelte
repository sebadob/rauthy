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
        d="M6.75 3v2.25M17.25 3v2.25M3 18.75V7.5a2.25 2.25 0 0 1 2.25-2.25h13.5A2.25 2.25 0 0 1 21
            7.5v11.25m-18 0A2.25 2.25 0 0 0 5.25 21h13.5A2.25 2.25 0 0 0 21 18.75m-18 0v-7.5A2.25 2.25
            0 0 1 5.25 9h13.5A2.25 2.25 0 0 1 21 11.25v7.5"
    />
</svg>
