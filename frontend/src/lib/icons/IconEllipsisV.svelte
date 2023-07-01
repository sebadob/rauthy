<script>
	export let color;
  export let opacity = 0.9;
  export let width = 24;
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
      d="M12 6.75a.75.75 0 110-1.5.75.75 0 010 1.5zM12 12.75a.75.75 0 110-1.5.75.75 0 010 1.5zM12 18.75a.75.75 0 110-1.5.75.75 0 010 1.5z"
  />
</svg>
