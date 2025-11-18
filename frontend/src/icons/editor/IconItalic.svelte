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
            d="M5.248 20.246H9.05m0 0h3.696m-3.696 0 5.893-16.502m0 0h-3.697m3.697 0h3.803"
    />
</svg>

