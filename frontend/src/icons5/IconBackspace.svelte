<script lang="ts">
    import type {IconProps} from "./props.ts";

    let {
        opacity = 0.9,
        width = '1.5rem',
        color = 'currentColor',
    }: IconProps = $props();
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
            d="M12 9.75L14.25 12m0 0l2.25 2.25M14.25 12l2.25-2.25M14.25 12L12 14.25m-2.58 4.92l-6.375-6.375a1.125 1.125 0 010-1.59L9.42 4.83c.211-.211.498-.33.796-.33H19.5a2.25 2.25 0 012.25 2.25v10.5a2.25 2.25 0 01-2.25 2.25h-9.284c-.298 0-.585-.119-.796-.33z"
    />
</svg>

