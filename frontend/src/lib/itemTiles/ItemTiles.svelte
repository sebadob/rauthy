<script>
    import AddItemTile from "./AddItemTile.svelte";
    import DeleteItemTile from "./DeleteItemTile.svelte";
    import {onMount, tick} from "svelte";

    /**
     * @typedef {Object} Props
     * @property {any} [items]
     * @property {any} [options]
     * @property {number} [searchThreshold]
     */

    /** @type {Props} */
    let {items = $bindable(), options = [], searchThreshold = $bindable(4)} = $props();

    let missing = $state([]);

    onMount(() => {
        computeMissing();
    });

    async function addItem(item) {
        if (!items) {
            items = [];
        }
        items.push(item);
        items = [...items];
        await computeMissing();
    }

    async function deleteItem(item) {
        items = [...items.filter(i => i !== item)];
        await computeMissing();
    }

    async function computeMissing() {
        await tick();
        if (!items) {
            items = [];
        }
        missing = [...options.filter(i => !items.includes(i))];
    }

</script>

<div class="container">
    {#if items?.length > 0}
        {#each items as item, i}
            <DeleteItemTile bind:label={items[i]} onDelete={deleteItem}/>
        {/each}
    {/if}

    {#if missing && missing.length > 0}
        <AddItemTile
                bind:items={missing}
                bind:searchThreshold
                onSelect={addItem}
        />
    {/if}
</div>

<style>
    .container {
        display: flex;
        flex-wrap: wrap;
        gap: .5rem 0;
    }
</style>
