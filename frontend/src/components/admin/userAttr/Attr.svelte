<script>
    import {onMount} from "svelte";
    import {getAttr} from "../../../utils/dataFetchingAdmin.js";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import UserAttrTileAddNew from "./AttrTileAddNew.svelte";
    import UserAttrTile from "./AttrTile.svelte";

    let msg = '';
    let attr = [];
    let resAttr = [];

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
    }

</script>

{msg}

<div class="content">
    <OrderSearchBar
            items={attr}
            bind:resItems={resAttr}
            searchOptions={searchOptions}
            orderOptions={orderOptions}
    />

    <UserAttrTileAddNew onSave={onSave}/>

    {#each resAttr as attr, i (attr.name)}
        <UserAttrTile idx={i} bind:attr onSave={onSave}/>
    {/each}

    <div style="height: 20px"></div>
</div>
