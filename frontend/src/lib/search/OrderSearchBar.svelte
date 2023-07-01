<script>
    import OrderBy from "./OrderBy.svelte";
    import SearchBar from "./SearchBar.svelte";

    export let items = [];
    export let resItems;
    export let searchOptions = [];
    export let orderOptions = [];
    export let firstDirReverse = false;

    let searchItems = [];
</script>

<div class="container">
    <div>
        <OrderBy items={searchItems} bind:resItems={resItems} options={orderOptions} firstDirReverse={firstDirReverse}/>
    </div>

    <div class="divider"></div>

    <div>
        <SearchBar items={items} bind:resItems={searchItems} options={searchOptions}/>
    </div>
</div>

<style>
    .divider {
        margin: 0 17px;
    }

    .container {
        display: flex;
        align-items: center;
        padding: 10px 0 20px 0;
    }
</style>
