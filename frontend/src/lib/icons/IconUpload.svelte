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
      d="M3 16.5v2.25A2.25 2.25 0 005.25 21h13.5A2.25 2.25 0 0021 18.75V16.5m-13.5-9L12 3m0 0l4.5 4.5M12 3v13.5"
  />
</svg>
