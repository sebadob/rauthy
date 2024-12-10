<script>
    import { run } from 'svelte/legacy';

    import IconPlus from "$lib/icons/IconPlus.svelte";
    import {slide} from "svelte/transition";
    import SearchBar from "../search/SearchBar.svelte";



    /**
     * @typedef {Object} Props
     * @property {any} [items]
     * @property {any} [onSelect]
     * @property {number} [mindWidth]
     * @property {any} [maxHeight]
     * @property {number} [searchThreshold]
     */

    /** @type {Props} */
    let {
        items = $bindable([]),
        onSelect = (item) => {
    },
        mindWidth = 130,
        maxHeight = items.length > 4 ? 175 : 120,
        searchThreshold = 5
    } = $props();

    let resItems = $state([]);
    let show = $state(false);

    run(() => {
        if (items.length <= searchThreshold) {
            resItems = items;
        }
    });

    function handleSelect(item) {
        show = false;
        resItems = items;
        onSelect(item);
    }
</script>

<div class="wrapper">
    <div
            role="button"
            tabindex="0"
            class="icon"
            onclick={() => show = !show}
            onkeypress={() => show = !show}
    >
        <IconPlus/>
    </div>

    {#if show}
        <div
                class="itemsContainer"
                style="min-width: {mindWidth}px; max-height: {maxHeight}px"
                transition:slide|global={{ duration: 200 }}
        >
            {#if items.length > searchThreshold}
                <div class="search">
                    <SearchBar bind:items bind:resItems maxBarWidth={`${mindWidth}px`}/>
                </div>
            {/if}

            <div
                    class="items noselect font-label"
                    style="width: {mindWidth}; max-height: {items.length > searchThreshold ? maxHeight - 29 : maxHeight}px"
            >
                {#each resItems as item}
                    <div
                            role="button"
                            tabindex="0"
                            class="item"
                            onclick={() => handleSelect(item)}
                            onkeypress={() => handleSelect(item)}
                    >
                        {item}
                    </div>
                {/each}
            </div>
        </div>
    {/if}
</div>

<style>
    .icon {
        margin: 0 2px;
        padding: 1px 1px 0 1px;
        color: var(--col-act2a);
        border: 1px solid var(--col-act2);
        border-radius: 5px;
        cursor: pointer;
    }

    .item {
        padding: 3px 5px;
        cursor: pointer;
    }

    .item:hover {
        background: var(--col-act1);
        color: white;
    }

    .itemsContainer {
        position: absolute;
        top: 0;
        left: 30px;
        background: var(--col-bg);
        border: 1px solid var(--col-acnt);
        border-radius: 5px;
        box-shadow: 3px 3px 3px rgba(0, 0, 0, .15);
        overflow: hidden;
        z-index: 1;
    }

    .items {
        overflow-y: auto;
    }

    .search {
        height: 27px;
        margin: -6px -6px 10px -6px;
        border-radius: 3px;
    }

    .wrapper {
        position: relative;
    }
</style>
