<script>
    import IconCheck from '$icons/IconCheck.svelte';
    import IconStop from '$icons/IconStop.svelte';

    let { checked = $bindable() } = $props();
</script>

{#if checked}
    <IconCheck color="hsl(var(--action))" />
{:else}
    <IconStop color="hsl(var(--error))" />
{/if}
