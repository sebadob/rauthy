<script>
    import IconCheck from "./icons/IconCheck.svelte";
    import IconStop from "./icons/IconStop.svelte";

    /**
     * @typedef {Object} Props
     * @property {boolean} [check]
     */

    /** @type {Props} */
    let { check = false } = $props();
</script>

{#if check}
    <IconCheck color="var(--col-ok)"/>
{:else}
    <IconStop color="var(--col-err)"/>
{/if}
