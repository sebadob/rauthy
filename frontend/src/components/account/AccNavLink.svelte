<script>
  /**
   * @typedef {Object} Props
   * @property {string} [selected]
   * @property {string} [label]
   */

  /** @type {Props} */
  let { selected = $bindable(''), label = '' } = $props();
    let active = $derived(selected === label);
</script>

<div class="link noselect" onclick={() => selected = label} onkeypress={() => selected = label}>
  <span class:active>
    {label}
  </span>
</div>

<style>
    .active {
        color: var(--col-ok);
    }

    .link {
        margin: 10px 20px;
        font-weight: bold;
    }
</style>
