<script>
    import {onMount} from "svelte";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import Pagination from "$lib/Pagination.svelte";

    let err = '';
    let blacklist = [];
    let resBlacklist = [];
    let resBlacklistPaginated = [];
    let refresh;

    const searchOptions = [
        {
            label: 'IP',
            callback: (item, search) => item.ip.includes(search),
        },
    ];

    onMount(() => {
        fetchBlacklist();
    });

    async function fetchBlacklist() {
        // let res = await getScopes();
        // let body = await res.json();
        // if (res.ok) {
        //     scopes = [...body];
        // } else {
        //     err = body.message;
        // }
    }

    function onSave() {
        fetchBlacklist();
    }

</script>

{err}

<div class="content">
    <OrderSearchBar
            items={blacklist}
            bind:resItems={resBlacklist}
            searchOptions={searchOptions}
    />

<!--    <ScopeTileAddNew onSave={onSave}/>-->

    <div id="blacklist">
        {#each resBlacklistPaginated as entry (entry.ip)}
            <div>
                {entry.ip}
<!--                <ScopeTile bind:attrs bind:scope onSave={onSave}/>-->
            </div>
        {/each}
    </div>

    <Pagination
            bind:items={resBlacklist}
            bind:resItems={resBlacklistPaginated}
    />

    <div style="height: 20px"></div>
</div>

<style>
    #blacklist div:nth-of-type(2n + 1) {
        background: linear-gradient(90deg, var(--col-ghigh) 35rem, var(--col-bg) 50rem);
    }
</style>
