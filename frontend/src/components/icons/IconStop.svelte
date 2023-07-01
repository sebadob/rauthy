<script>
	export let color = 'var(--col-err)';
  export let opacity = 0.9;
  export let width = 20;
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
  <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12" />
</svg>
