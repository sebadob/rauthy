<script>
    export let selected = '';
    export let label = '';
    $: active = selected === label;
</script>

<div class="link noselect" on:click={() => selected = label} on:keypress={() => selected = label}>
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
