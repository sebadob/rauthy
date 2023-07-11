<script>
    import {onMount} from "svelte";
    import {getGroups, getRoles, getUsers} from "../../../utils/dataFetchingAdmin.js";
    import UserTile from "./UserTile.svelte";
    import {globalGroups, globalGroupsNames, globalRoles, globalRolesNames,} from "../../../stores/admin.js";
    import UserTileAddNew from "./UserTileAddNew.svelte";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";

    let msg = '';

    let users = [];
    let resUsers = [];

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

    {#each resUsers as user, i (user.id)}
        <UserTile idx={i} bind:user onSave={onSave}/>
    {/each}
</div>
