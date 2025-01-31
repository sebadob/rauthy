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
        stroke="currentColor"
        stroke-width={2}
        {width}
        {color}
        {opacity}
>
    <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M8.25 4.5l7.5 7.5-7.5 7.5"
    />
</svg>
