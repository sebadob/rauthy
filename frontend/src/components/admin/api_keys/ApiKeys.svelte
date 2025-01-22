<script>
    import {onMount} from "svelte";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import Pagination from "$lib/Pagination.svelte";
    import Button from "$lib/Button.svelte";
    import {slide} from "svelte/transition";
    import {getApiKeys} from "../../../utils/dataFetchingAdmin.js";
    import ApiKeyAddNew from "./ApiKeyAddNew.svelte";
    import ApiKeyTile from "./ApiKeyTile.svelte";

    let err = $state('');
    let keys = $state([]);
    let resKeys = $state([]);
    let resKeysPaginated = $state([]);
    let refresh;
    let showAddNew = $state(false);

    const searchOptions = [
        {
            label: 'Name',
            callback: (item, search) => item.name.includes(search),
        },
    ];
    let orderOptions = [
        {
            label: 'Name',
            callback: (a, b) => a.name.localeCompare(b.name),
        },
    ];

    onMount(() => {
        fetchApiKeys();
    });

    async function fetchApiKeys() {
        let res = await getApiKeys();
        let body = await res.json();
        if (res.ok) {
            keys = body.keys;
            showAddNew = false;
        } else {
            err = body.message;
        }
    }

</script>

{err}

<div class="content">
    <div class="top">
        <OrderSearchBar
                items={keys}
                bind:resItems={resKeys}
                searchOptions={searchOptions}
                orderOptions={orderOptions}
        />

        <div class="addNew">
            <Button on:click={() => showAddNew = !showAddNew} level={3}>NEW KEY</Button>
        </div>
    </div>
    {#if showAddNew}
        <div transition:slide>
            <ApiKeyAddNew onSave={fetchApiKeys} apiKeys={keys}/>
        </div>
    {/if}

    <div id="keys">
        {#if keys.length === 0}
            <div>
                No Api Keys
            </div>
        {:else}
            {#each resKeysPaginated as apiKey, i (apiKey.name)}
                <ApiKeyTile bind:apiKey={resKeysPaginated[i]} onSave={fetchApiKeys}/>
            {/each}
        {/if}
    </div>

    {#if keys.length > 0}
        <Pagination
                bind:items={resKeys}
                bind:resItems={resKeysPaginated}
        />
    {/if}

    <div style="height: 20px"></div>
</div>

<style>
    #keys div:nth-of-type(2n + 1) {
        background: linear-gradient(90deg, hsla(var(--bg-high) / .25) 10rem, hsl(var(--bg)) 50rem);
    }

    .addNew {
        margin-bottom: .6rem;
    }

    .top {
        display: inline-flex;
        align-items: center;
        gap: 1rem;
    }
</style>
