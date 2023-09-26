<script>
    import {onMount} from "svelte";
    import {getGroups, getRoles, getUsers} from "../../../utils/dataFetchingAdmin.js";
    import UserTile from "./UserTile.svelte";
    import {globalGroups, globalGroupsNames, globalRoles, globalRolesNames,} from "../../../stores/admin.js";
    import UserTileAddNew from "./UserTileAddNew.svelte";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import Pagination from "$lib/Pagination.svelte";

    let msg = '';

    let users = [];
    let resUsers = [];
    let resUsersPaginated = [];

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

    async function fetchUsers() {
        let res = await getUsers();
        if (!res.ok) {
            msg = 'Error fetching users: ' + res.body.message;
        } else {
            let u = await res.json();
            console.log(u);
            users = [...u];
            resUsers = [...u];
        }
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
    }
</script>

{msg}

<div class="content">
    <OrderSearchBar
            items={users}
            bind:resItems={resUsers}
            bind:searchOptions
            bind:orderOptions
    />

    <UserTileAddNew onSave={onSave}/>

    <div id="users">
        {#each resUsersPaginated as user (user.id)}
            <div>
                <UserTile bind:user onSave={onSave}/>
            </div>
        {/each}
    </div>

    <Pagination
            bind:items={resUsers}
            bind:resItems={resUsersPaginated}
    />
</div>

<style>
    #users div:nth-of-type(2n + 1) {
        background: linear-gradient(90deg, var(--col-ghigh) 35rem, var(--col-bg) 50rem);
    }
</style>
