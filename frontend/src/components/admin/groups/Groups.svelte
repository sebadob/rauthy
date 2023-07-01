<script>
    import {onMount} from "svelte";
    import {getGroups} from "../../../utils/dataFetchingAdmin.js";
    import GroupTileAddNew from "./GroupTileAddNew.svelte";
    import GroupTile from "./GroupTile.svelte";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";

    let err = '';
    let groups = [];
    let resGroups = [];

    let searchOptions = [
        {
            label: 'NAME',
            callback: (item, search) => item.name.toLowerCase().includes(search.toLowerCase()),
        },
        {
            label: 'ID',
            callback: (item, search) => item.id.toLowerCase().includes(search.toLowerCase()),
        },
    ];
    let orderOptions = [
        {
            label: 'NAME',
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

    {#each resGroups as group, i (group.id)}
        <GroupTile idx={i} bind:group onSave={onSave}/>
    {/each}

    <div style="height: 20px"></div>
</div>
