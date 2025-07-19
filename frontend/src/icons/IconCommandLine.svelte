<script lang="ts">
    import type {IconProps} from "./props.ts";

    let {
        opacity = 0.9,
        width = '1.5rem',
        color = 'currentColor',
    }: IconProps = $props();
</script>

<svg
        viewBox="0 0 24 24"
        stroke={color}
        stroke-width={2}
        {width}
        {opacity}
>
    <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="m6.75 7.5 3 2.25-3 2.25m4.5 0h3m-9 8.25h13.5A2.25 2.25 0 0 0 21 18V6a2.25 2.25 0 0
            0-2.25-2.25H5.25A2.25 2.25 0 0 0 3 6v12a2.25 2.25 0 0 0 2.25 2.25Z"
    />
</svg>
