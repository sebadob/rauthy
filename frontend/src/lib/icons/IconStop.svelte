<script>
    /**
     * @typedef {Object} Props
     * @property {string} [color]
     * @property {number} [opacity]
     * @property {number} [width]
     */

    /** @type {Props} */
    let { color = 'var(--col-text)', opacity = 0.9, width = 20 } = $props();
</script>

<svg
        fill="none"
        viewBox="0 0 24 24"
        stroke="currentColor"
        stroke-width={2}
        width={width}
        color={color}
        opacity={opacity}
>
    <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
</svg>
