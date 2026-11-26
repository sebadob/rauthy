<script lang="ts">
    import type { IconProps } from './props.ts';

    let { color = 'currentColor', opacity = 0.9, width = '1.5rem' }: IconProps = $props();
</script>

<svg
    viewBox="0 0 24 24"
    fill="none"
    stroke={color}
    stroke-width="1.75"
    {width}
    {opacity}
    aria-hidden="true"
>
    <path
        stroke-linecap="round"
        stroke-linejoin="round"
        d="m14.5 5.5 4 4M4 20l1-5 11-11a2.83 2.83 0 0 1 4 4L9 19l-5 1Z"
    />
</svg>
