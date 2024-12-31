<script>
  /**
   * @typedef {Object} Props
   * @property {string} [color]
   * @property {number} [opacity]
   * @property {number} [width]
   */

  /** @type {Props} */
  let { color = 'var(--col-text)', opacity = 0.9, width = 24 } = $props();
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
  <path
    stroke-linecap="round"
    stroke-linejoin="round"
    d="M16 12a4 4 0 10-8 0 4 4 0 008 0zm0 0v1.5a2.5 2.5 0 005 0V12a9 9 0 10-9 9m4.5-1.206a8.959 8.959 0 01-4.5 1.207"
  />
</svg>
