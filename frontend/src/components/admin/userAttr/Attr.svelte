<script>
    import {onMount} from "svelte";
    import {getAttr} from "../../../utils/dataFetchingAdmin.js";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import UserAttrTileAddNew from "./AttrTileAddNew.svelte";
    import UserAttrTile from "./AttrTile.svelte";
    import Pagination from "$lib/Pagination.svelte";

    let msg = '';
    let attr = [];
    let resAttr = [];
    let resAttrPaginated = [];
    let search = '';

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

    onMount(async () => {
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
        {#each resAttrPaginated as attr (attr.name)}
            <div>
                <UserAttrTile bind:attr onSave={onSave}/>
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
