<script>
    import {onMount} from "svelte";
    import {getGroups} from "../../../utils/dataFetchingAdmin.js";
    import GroupTileAddNew from "./GroupTileAddNew.svelte";
    import GroupTile from "./GroupTile.svelte";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import Pagination from "$lib/Pagination.svelte";

    let err = '';
    let groups = [];
    let resGroups = [];
    let resGroupsPaginated = [];

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
        let res = await getGroups();
        let body = await res.json();
        if (res.ok) {
            groups = [...body];
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
            items={groups}
            bind:resItems={resGroups}
            searchOptions={searchOptions}
            orderOptions={orderOptions}
    />

    <GroupTileAddNew onSave={onSave}/>

    <div id="groups">
        {#each resGroupsPaginated as group (group.id)}
            <div>
                <GroupTile bind:group onSave={onSave}/>
            </div>
        {/each}
    </div>

    <Pagination
            bind:items={resGroups}
            bind:resItems={resGroupsPaginated}
    />

    <div style="height: 20px"></div>
</div>

<style>
    #groups div:nth-of-type(2n + 1) {
        background: linear-gradient(90deg, var(--col-ghigh) 35rem, var(--col-bg) 50rem);
    }
</style>
