<script>
    import {onMount} from "svelte";
    import {getAttr, getScopes} from "../../../utils/dataFetchingAdmin.js";
    import ScopeTileAddNew from "./ScopeTileAddNew.svelte";
    import ScopeTile from "./ScopeTile.svelte";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import Pagination from "$lib/Pagination.svelte";

    let attrs = [];
    let err = '';
    let scopes = [];
    let resScopes = [];
    let resScopesPaginated = [];
    let refresh;

    let searchOptions = [
        {
            label: 'Name',
            callback: (item, search) => item.name.toLowerCase().includes(search.toLowerCase()),
        },
        {
            label: 'ID',
            callback: (item, search) => item.id.toLowerCase().includes(search.toLowerCase()),
        },
    ];
    let orderOptions = [
        {
            label: 'Name',
            callback: (a, b) => a.name.localeCompare(b.name),
        },
        {
            label: 'ID',
            callback: (a, b) => a.id.localeCompare(b.id),
        },
    ];

    onMount(() => {
        fetchScopes();
        fetchAttr();
    });

    async function fetchAttr() {
        let res = await getAttr();
        if (!res.ok) {
            err = 'Error fetching attributes: ' + res.body.message;
        } else {
            let a = await res.json();
            attrs = [...a.values];
        }
    }

    async function fetchScopes() {
        let res = await getScopes();
        let body = await res.json();
        if (res.ok) {
            scopes = [...body];
        } else {
            err = body.message;
        }
    }

    function onSave() {
        fetchScopes();
    }

</script>

{err}

<div class="content">
    <OrderSearchBar
            items={scopes}
            bind:resItems={resScopes}
            searchOptions={searchOptions}
            orderOptions={orderOptions}
    />

    <ScopeTileAddNew onSave={onSave}/>

    <div id="scopes">
        {#each resScopes as scope (scope.id)}
            <div>
                <ScopeTile bind:attrs bind:scope onSave={onSave}/>
            </div>
        {/each}
    </div>

    <Pagination
            bind:items={resScopes}
            bind:resItems={resScopesPaginated}
    />

    <div style="height: 20px"></div>
</div>

<style>
    #scopes div:nth-of-type(2n + 1) {
        background: linear-gradient(90deg, var(--col-ghigh) 35rem, var(--col-bg) 50rem);
    }
</style>
