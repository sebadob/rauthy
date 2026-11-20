<script lang="ts">
    import OrderBy from '#lib/search_bar/OrderBy.svelte';
    import SearchBar from '#lib/search_bar/SearchBar.svelte';

    let {
        ref = $bindable(),
        searchOptions,
        searchOption = $bindable(),
        datalist,
        pattern,
        value = $bindable(),
        searchWidth = 'min(25rem, calc(100dvw - 1rem))',
        onSearch = $bindable(),

        orderOptions,
        onChangeOrder,
        firstDirReverse,

        borderless,
        inline = false,
    }: {
        ref?: undefined | HTMLButtonElement;
        searchOptions?: string[];
        searchOption?: string;
        datalist?: string[];
        pattern?: string;
        value: string;
        searchWidth?: string;
        onSearch?: (value: string) => void;

        orderOptions: string[];
        onChangeOrder: (option: string, direction: 'up' | 'down') => void;
        firstDirReverse?: boolean;

        borderless?: boolean;
        inline?: boolean;
    } = $props();
</script>

<div class="admin-list-tools" class:inline>
    <SearchBar
        bind:value
        {datalist}
        options={searchOptions}
        bind:option={searchOption}
        {pattern}
        width={searchWidth}
        {borderless}
        {onSearch}
    />
    <OrderBy
        bind:ref
        options={orderOptions}
        onChange={onChangeOrder}
        {borderless}
        {firstDirReverse}
    />
</div>

<style>
    .admin-list-tools {
        display: flex;
        flex-direction: column;
        align-items: stretch;
        min-width: 0;
        max-width: 100%;
        gap: 8px;
        margin: 0;
    }
    .inline {
        flex-direction: row;
        flex-wrap: wrap;
        align-items: center;
    }
    .inline > :global(search) {
        flex: 1 1 min(100%, 15rem);
    }
    .inline > :global(.admin-sort) {
        margin-inline-start: auto;
    }
</style>
