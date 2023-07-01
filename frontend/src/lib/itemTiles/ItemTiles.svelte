<script>
    import AddItemTile from "./AddItemTile.svelte";
    import DeleteItemTile from "./DeleteItemTile.svelte";
    import {onMount, tick} from "svelte";

    export let items = [];
    export let options = [];
    export let offset = -55;
    export let offsetSearch = -100;
    export let searchThreshold = 4;

    let missing = [];

    onMount(() => {
        computeMissing();
    });

    async function addItem(item) {
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
        missing = [...options.filter(i => !items.includes(i))];
    }

</script>

<div class="container">
    {#if items.length > 0}
        {#each items as item}
            <DeleteItemTile bind:label={item} onDelete={deleteItem}/>
        {/each}
    {/if}

    {#if missing && missing.length > 0}
        <AddItemTile
                bind:items={missing}
                bind:searchThreshold
                onSelect={addItem}
                yOffset={items > searchThreshold ? offsetSearch : offset}
        />
    {/if}
</div>

<style>
    .container {
        display: flex;
    }
</style>
