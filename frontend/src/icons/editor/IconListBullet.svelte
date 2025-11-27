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
        d="M8.25 6.75h12M8.25 12h12m-12 5.25h12M3.75 6.75h.007v.008H3.75V6.75Zm.375 0a.375.375 0 1 1-.75 0 .375.375
            0 0 1 .75 0ZM3.75 12h.007v.008H3.75V12Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Zm-.375
            5.25h.007v.008H3.75v-.008Zm.375 0a.375.375 0 1 1-.75 0 .375.375 0 0 1 .75 0Z"
    />
</svg>
