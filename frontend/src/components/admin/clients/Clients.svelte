<script>
    import {onMount} from "svelte";
    import {getClients, getScopes} from "../../../utils/dataFetchingAdmin.js";
    import {globalScopes, globalScopesNames} from "../../../stores/admin.js";
    import ClientTile from "./ClientTile.svelte";
    import ClientTileAddNew from "./ClientTileAddNew.svelte";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import Pagination from "$lib/Pagination.svelte";

    let msg = $state('');

    let clients = $state([]);
    let resClients = $state([]);
    let resClientsPaginated = $state([]);
    let search = $state('');

    let searchOptions = [
        {
            label: 'ID',
            callback: (item, search) => item.id.toLowerCase().includes(search.toLowerCase()),
        },
    ];
    let orderOptions = [
        {
            label: 'ID',
            callback: (a, b) => a.id.localeCompare(b.id),
        },
    ];

    onMount(async () => {
        fetchClients();
        fetchScopes();
    })

    async function fetchClients() {
        let res = await getClients();
        if (!res.ok) {
            msg = 'Error fetching clients: ' + res.body.message;
        } else {
            let c = await res.json();
            clients = [...c];
        }
    }

    async function fetchScopes() {
        let res = await getScopes();
        if (!res.ok) {
            msg = 'Error fetching scopes: ' + res.body.message;
        } else {
            let scopes = await res.json();
            globalScopes.set(scopes);
            globalScopesNames.set(scopes.map(s => s.name));
        }
    }

    function onSave() {
        fetchClients();
        fetchScopes();
        search = '';
    }
</script>

{msg}

<div class="content">
    <OrderSearchBar
            items={clients}
            bind:resItems={resClients}
            searchOptions={searchOptions}
            orderOptions={orderOptions}
            bind:search
    />

    <ClientTileAddNew onSave={onSave}/>

    <div id="clients">
        {#each resClientsPaginated as client (client.id)}
            <div>
                <ClientTile bind:client onSave={onSave}/>
            </div>
        {/each}
    </div>

    <Pagination
            bind:items={resClients}
            bind:resItems={resClientsPaginated}
    />

    <div style="height: 20px"></div>
</div>

<style>
    #clients div:nth-of-type(2n + 1) {
        background: linear-gradient(90deg, var(--col-ghigh) 35rem, var(--col-bg) 50rem);
    }
</style>
