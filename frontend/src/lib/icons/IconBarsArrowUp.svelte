<script>
    /**
     * @typedef {Object} Props
     * @property {number} [opacity]
     * @property {number} [width]
     */

    /** @type {Props} */
    let { opacity = 0.9, width = 24 } = $props();
</script>

<svg
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        stroke-width={2}
        width={width}
        opacity={opacity}
>
    <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M3 4.5h14.25M3 9h9.75M3 13.5h5.25m5.25-.75L17.25 9m0 0L21 12.75M17.25 9v12"
    />
</svg>
