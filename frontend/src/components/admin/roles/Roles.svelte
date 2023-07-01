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

    {#each resRoles as role, i (role.id)}
        <RoleTile idx={i} bind:role onSave={onSave}/>
    {/each}

    <div style="height: 20px"></div>
</div>
