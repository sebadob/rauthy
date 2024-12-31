<script>
    import OrderBy from "./OrderBy.svelte";
    import SearchBar from "./SearchBar.svelte";

    /**
     * @typedef {Object} Props
     * @property {any} [items]
     * @property {any} resItems
     * @property {any} [searchOptions]
     * @property {any} [orderOptions]
     * @property {boolean} [firstDirReverse]
     * @property {string} [useServerSideIdx]
     * @property {boolean} [isSearchFiltered]
     * @property {string} [search]
     */

    /** @type {Props} */
    let {
        items = $bindable([]),
        resItems = $bindable(),
        searchOptions = $bindable([]),
        orderOptions = $bindable([]),
        firstDirReverse = false,
        useServerSideIdx = $bindable(''),
        isSearchFiltered = $bindable(false),
        search = $bindable('')
    } = $props();

    let searchItems = $state([]);
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
                bind:search
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
