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
    } = $props();
</script>

<div class="admin-list-tools">
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
</style>
