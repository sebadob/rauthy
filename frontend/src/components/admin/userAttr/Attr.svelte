<script>
    import {onMount} from "svelte";
    import {getAttr} from "../../../utils/dataFetchingAdmin.js";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import UserAttrTileAddNew from "./AttrTileAddNew.svelte";
    import UserAttrTile from "./AttrTile.svelte";
    import Pagination from "$lib/Pagination.svelte";

    let msg = $state('');
    let attr = $state([]);
    let resAttr = $state([]);
    let resAttrPaginated = $state([]);
    let search = $state('');

    let searchOptions = [
        {
            label: 'NAME',
            callback: (item, search) => item.name.toLowerCase().includes(search.toLowerCase()),
        },
    ];
    let orderOptions = [
        {
            label: 'NAME',
            callback: (a, b) => a.name.localeCompare(b.name),
        },
    ];

    onMount(() => {
        fetchAttr();
    })

    async function fetchAttr() {
        let res = await getAttr();
        if (!res.ok) {
            msg = 'Error fetching user attr: ' + res.body.message;
        } else {
            let u = await res.json();
            attr = [...u.values];
        }
    }

    function onSave() {
        // Just a workaround for now until the order and search components have been migrated to svelte5.
        // If the last existing `attr` has been deleted, the list would not update.
        if (attr.length < 2) {
            attr = [];
            resAttr = [];
            resAttrPaginated = [];
        }

        fetchAttr();
        search = '';
    }

</script>

{msg}

<div class="content">
    <OrderSearchBar
            items={attr}
            bind:resItems={resAttr}
            searchOptions={searchOptions}
            orderOptions={orderOptions}
            bind:search
    />

    <UserAttrTileAddNew onSave={onSave}/>

    <div id="attrs">
        {#each resAttrPaginated as attr, i (attr.name)}
            <div>
                <UserAttrTile bind:attr={resAttrPaginated[i]} onSave={onSave}/>
            </div>
        {/each}
    </div>

    <Pagination
            bind:items={resAttr}
            bind:resItems={resAttrPaginated}
    />

    <div style="height: 20px"></div>
</div>

<style>
    #attrs div:nth-of-type(2n + 1) {
        background: linear-gradient(90deg, var(--col-ghigh) 35rem, var(--col-bg) 50rem);
    }
</style>
