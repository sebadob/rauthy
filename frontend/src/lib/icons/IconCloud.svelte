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
            d="M2.25 15a4.5 4.5 0 0 0 4.5 4.5H18a3.75 3.75 0 0 0 1.332-7.257 3 3 0 0 0-3.758-3.848 5.25 5.25 0 0
            0-10.233 2.33A4.502 4.502 0 0 0 2.25 15Z"
    />
</svg>

