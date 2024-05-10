<script>
    import {onMount} from "svelte";
    import {getGroups, getRoles, getUsers, getUsersSsp} from "../../../utils/dataFetchingAdmin.js";
    import UserTile from "./UserTile.svelte";
    import {globalGroups, globalGroupsNames, globalRoles, globalRolesNames,} from "../../../stores/admin.js";
    import UserTileAddNew from "./UserTileAddNew.svelte";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import Pagination from "$lib/Pagination.svelte";
    import PaginationServer from "$lib/PaginationServer.svelte";

    let msg = '';
    let isInitialized = false;

    let users = [];
    let resUsers = [];
    let resUsersPaginated = [];
    let useServerSideIdx = '';
    let isSearchFiltered = false;
    let search = '';

    let usersCountTotal = 0;
    let sspPageSize = 15;
    let sspContinuationToken = '';
    let sspPage = 1;

    let searchOptions = [
        {
            label: 'E-Mail',
            callback: (item, search) => item.email.toLowerCase().includes(search.toLowerCase()),
        },
        {
            label: 'ID',
            callback: (item, search) => item.id.toLowerCase().includes(search.toLowerCase()),
        },
    ];
    let orderOptions = [
        {
            label: 'E-Mail',
            callback: (a, b) => a.email.localeCompare(b.email),
        },
        {
            label: 'ID',
            callback: (a, b) => a.id.localeCompare(b.id),
        },
    ];

    onMount(async () => {
        fetchUsers();
        fetchRoles();
        fetchGroups();
    })

    async function fetchUsers(useSsp, offset, backwards, pageSize) {
        let res;
        if (useSsp === true) {
            if (backwards && sspPage === 2) {
                // In this case, just do a normal fetch of the very first entry to have a clean start again
                res = await getUsersSsp(pageSize || sspPageSize, offset, undefined, false);
            } else {
                res = await getUsersSsp(pageSize || sspPageSize, offset, sspContinuationToken, backwards);
            }
        } else {
            res = await getUsers();
        }
        if (!res.ok) {
            msg = 'Error fetching users: ' + res.body.message;
        } else {
            const isSsp = res.status === 206;
            if (isSsp) {
                // we get a few headers during SSP we can use for the navigation
                sspPageSize = Number.parseInt(res.headers.get('x-page-size'), 10);
                // sspPageCount = res.headers.get('x-page-count');
                useServerSideIdx = 'user';

                const token = res.headers.get('x-continuation-token');
                if (!token && !backwards) {
                    // In this case, we went 1 page too far. Because of the dynamic pagination,
                    // we can't know beforehand which page will be the last existing.
                    // If we went too far, we will not save the continuation token and just go back.
                }

                sspContinuationToken = res.headers.get('x-continuation-token');
            } else {
                useServerSideIdx = '';
            }

            usersCountTotal = res.headers.get('x-user-count');

            let u = await res.json();
            users = [...u];
            resUsers = [...u];
        }
    }

    // Callback function for <PaginationServer>
    // Fetches the next page during server side pagination with the given offset and direction.
    async function fetchUsersSsp(offset, backwards) {
        await fetchUsers(true, offset, backwards);
        if (backwards) {
            sspPage -= 1;
        } else {
            sspPage += 1;
        }
    }

    // Callback function for <PaginationServer> to make page size switches work
    async function sspPageSizeChange(pageSize) {
        sspContinuationToken = '';
        await fetchUsers(true, 0, false, pageSize);
        sspPage = 1;
    }

    async function fetchRoles() {
        let res = await getRoles();
        if (!res.ok) {
            msg = 'Error fetching roles: ' + res.body.message;
        } else {
            let roles = await res.json();
            globalRoles.set(roles);
            globalRolesNames.set(roles.map(r => r.name));
        }
    }

    async function fetchGroups() {
        let res = await getGroups();
        if (!res.ok) {
            msg = 'Error fetching groups: ' + res.body.message;
        } else {
            let groups = await res.json();
            globalGroups.set(groups);
            globalGroupsNames.set(groups.map(g => g.name));
        }
    }

    function onSave() {
        fetchUsers();
        fetchRoles();
        fetchGroups();
        search = '';
    }
</script>

{msg}

<div class="content">
    <OrderSearchBar
            items={users}
            bind:resItems={resUsers}
            bind:searchOptions
            bind:orderOptions
            bind:useServerSideIdx
            bind:isSearchFiltered
            bind:search
    />

    <UserTileAddNew onSave={onSave}/>

    <div id="users">
        {#if useServerSideIdx && !isSearchFiltered}
            {#each users as user (user.id)}
                <div>
                    <UserTile userId={user.id} userEmail={user.email} onSave={onSave}/>
                </div>
            {/each}
        {:else}
            {#each resUsersPaginated as user (user.id)}
                <div>
                    <UserTile userId={user.id} userEmail={user.email} onSave={onSave}/>
                </div>
            {/each}
        {/if}
    </div>

    <!--
    Even with server side pagination, we must use it client side if we
    have a filtered search result. Otherwise, switching pages would
    overwrite the filtered data.
    -->
    {#if useServerSideIdx && !isSearchFiltered}
        <PaginationServer
                itemsTotal={usersCountTotal}
                bind:sspPage
                bind:sspPageSize
                bind:sspContinuationToken
                fetchPageCallback={fetchUsersSsp}
                sspPageSizeChange={sspPageSizeChange}
        />
    {:else}
        <Pagination bind:items={resUsers} bind:resItems={resUsersPaginated}/>
    {/if}
</div>

<style>
    #users div:nth-of-type(2n + 1) {
        background: linear-gradient(90deg, var(--col-ghigh) 35rem, var(--col-bg) 50rem);
    }
</style>
