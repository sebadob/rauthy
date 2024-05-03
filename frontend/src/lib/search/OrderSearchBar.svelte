<script>
    import OrderBy from "./OrderBy.svelte";
    import SearchBar from "./SearchBar.svelte";

    export let items = [];
    export let resItems;
    export let searchOptions = [];
    export let orderOptions = [];
    export let firstDirReverse = false;
    export let useServerSideIdx = '';
    export let isSearchFiltered = false;

    let searchItems = [];
</script>

<div class="container">
    <div>
        <OrderBy
                items={searchItems}
                bind:resItems={resItems}
                options={orderOptions}
                firstDirReverse={firstDirReverse}
        />
    </div>

    {#if orderOptions.length > 0}
        <div class="divider"></div>
    {/if}

    <div>
        <SearchBar
                bind:items
                bind:resItems={searchItems}
                options={searchOptions}
                bind:useServerSideIdx
                bind:isSearchFiltered
        />
    </div>
</div>

<style>
    .divider {
        margin: 0 1rem;
    }

    .container {
        display: flex;
        align-items: center;
        padding: 10px 0 20px 0;
    }
</style>
