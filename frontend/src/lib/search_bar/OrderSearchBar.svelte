<script lang="ts">
    import OrderBy from '$lib5/search_bar/OrderBy.svelte';
    import SearchBar from '$lib5/search_bar/SearchBar.svelte';

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

<div class="container">
    <OrderBy
        bind:ref
        options={orderOptions}
        onChange={onChangeOrder}
        {borderless}
        {firstDirReverse}
    />
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
</div>

<style>
    .container {
        margin: 0 0.25rem;
        min-width: 20rem;
        max-width: 40rem;
        display: flex;
        align-items: center;
        column-gap: 0.5rem;
        row-gap: 0.25rem;
        flex-wrap: wrap;
    }
</style>
