<script lang="ts">
    import type {IconProps} from "$icons/props";

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
            d="M13.19 8.688a4.5 4.5 0 0 1 1.242 7.244l-4.5 4.5a4.5 4.5 0 0 1-6.364-6.364l1.757-1.757m13.35-.622
            1.757-1.757a4.5 4.5 0 0 0-6.364-6.364l-4.5 4.5a4.5 4.5 0 0 0 1.242 7.244"
    />
</svg>

