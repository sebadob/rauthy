<script>
    import {onMount} from "svelte";
    import {getRoles} from "../../../utils/dataFetchingAdmin.js";
    import RoleTile from "./RoleTile.svelte";
    import RoleTileAddNew from "./RoleTileAddNew.svelte";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import Pagination from "$lib/Pagination.svelte";

    let err = '';
    let roles = [];
    let resRoles = [];
    let resRolesPaginated = [];
    let search = '';

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

    onMount(async () => {
        fetchData();
    });

    async function fetchData() {
        let res = await getRoles();
        let body = await res.json();
        if (res.ok) {
            roles = [...body];
        } else {
            err = body.message;
        }
    }

    function onSave() {
        fetchData();
        search = '';
    }

</script>

{err}

<div class="content">
    <OrderSearchBar
            items={roles}
            bind:resItems={resRoles}
            searchOptions={searchOptions}
            orderOptions={orderOptions}
            bind:search
    />

    <RoleTileAddNew onSave={onSave}/>

    <div id="roles">
        {#each resRolesPaginated as role (role.id)}
            <div>
                <RoleTile bind:role onSave={onSave}/>
            </div>
        {/each}
    </div>

    <Pagination
            bind:items={resRoles}
            bind:resItems={resRolesPaginated}
    />

    <div style="height: 20px"></div>
</div>

<style>
    #roles div:nth-of-type(2n + 1) {
        background: linear-gradient(90deg, var(--col-ghigh) 35rem, var(--col-bg) 50rem);
    }
</style>
