<script>
    import {onMount} from "svelte";
    import {getRoles} from "../../../utils/dataFetchingAdmin.js";
    import RoleTile from "./RoleTile.svelte";
    import RoleTileAddNew from "./RoleTileAddNew.svelte";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";

    let err = '';
    let roles = [];
    let resRoles = [];

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
    }

</script>

{err}

<div class="content">
    <OrderSearchBar
            items={roles}
            bind:resItems={resRoles}
            searchOptions={searchOptions}
            orderOptions={orderOptions}
    />

    <RoleTileAddNew onSave={onSave}/>

    <div id="roles">
        {#each resRoles as role (role.id)}
            <div>
                <RoleTile bind:role onSave={onSave}/>
            </div>
        {/each}
    </div>

    <div style="height: 20px"></div>
</div>

<style>
    #roles div:nth-of-type(2n + 1) {
        background: linear-gradient(90deg, var(--col-ghigh) 35rem, var(--col-bg) 50rem);
    }
</style>
