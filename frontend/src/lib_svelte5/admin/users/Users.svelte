<script lang="ts">
    import {onMount} from "svelte";
    import {fetchGet} from "$api/fetch.ts";
    import type {UserResponse, UserResponseSimple} from "$api/types/user.ts";
    import OrderSearchBar from "$lib5/search_bar/OrderSearchBar.svelte";
    import type {GroupResponse} from "$api/types/groups.ts";
    import type {RoleResponse} from "$api/types/roles.ts";
    import Pagination from "$lib5/pagination/Pagination.svelte";
    import NavSub from "$lib5/nav/NavSub.svelte";
    import ButtonAddModal from "$lib5/button/ButtonAddModal.svelte";
    import {useParam} from "$state/param.svelte.ts";
    import NavButtonTile from "$lib5/nav/NavButtonTile.svelte";
    import ContentAdmin from "$lib5/ContentAdmin.svelte";
    import UserAddNew from "$lib5/admin/users/UserAddNew.svelte";
    import UserDetails from "$lib5/admin/users/UserDetails.svelte";
    import PaginationServerSide from "$lib5/pagination/PaginationServerSide.svelte";
    import {PAGE_SIZE_DEFAULT} from "$lib5/pagination/props.ts";

    let closeModal: undefined | (() => void) = $state();
    let err = $state('');

    let users: UserResponseSimple[] = $state([]);
    let usersFiltered: UserResponseSimple[] = $state([]);
    let usersPaginated: UserResponseSimple[] = $state([]);
    let uid = useParam('uid');
    let userId: undefined | string = $state();

    let groups: GroupResponse[] = $state([]);
    let roles: RoleResponse[] = $state([]);

    let firstFetchHeaders: undefined | Headers = $state();

    let searchOptions = $state(['E-Mail', 'ID']);
    let searchOption = $state(searchOptions[0]);
    let searchValue = $state('');
    let orderOptions = $state(['E-Mail', 'ID', 'Created', 'Last Login']);

    onMount(() => {
        fetchUsers('page_size=' + PAGE_SIZE_DEFAULT);
        fetchRoles();
        fetchGroups();
    })

    $effect(() => {
        userId = users.find(u => u.id === uid.get())?.id;
    });

    $effect(() => {
        let search = searchValue.toLowerCase();
        if (!search) {
            usersFiltered = users;
        } else if (searchOption === searchOptions[0]) {
            usersFiltered = users.filter(u => u.email?.toLowerCase().includes(search));
        } else if (searchOption === searchOptions[1]) {
            usersFiltered = users.filter(u => u.id.toLowerCase().includes(search));
        }
    });

    async function fetchUsers(urlParams?: string): Promise<[number, Headers]> {
        let url = `/auth/v1/users`;
        if (urlParams) {
            url += `?${urlParams}`
        }

        let res = await fetchGet<UserResponse[]>(url);
        if (res.error) {
            err = 'Error fetching users: ' + res.error.message;
        } else if (res.body) {
            if (res.status === 206) {
                firstFetchHeaders = res.headers;
            } else {
                firstFetchHeaders = undefined;
            }
            users = res.body;
        }

        return [res.status, res.headers];
    }

    async function fetchRoles() {
        let res = await fetchGet<RoleResponse[]>('/auth/v1/roles');
        if (res.body) {
            roles = res.body.toSorted((a, b) => a.name.localeCompare(b.name));
        } else {
            err = res.error?.message || 'Error';
        }
    }

    async function fetchGroups() {
        let res = await fetchGet<GroupResponse[]>('/auth/v1/groups');
        if (res.body) {
            groups = res.body.toSorted((a, b) => a.name.localeCompare(b.name));
        } else {
            err = res.error?.message || 'Error';
        }
    }

    function onChangeOrder(option: string, direction: 'up' | 'down') {
        let up = direction === 'up';

        // ['E-Mail', 'ID', 'Created', 'Last Login']
        if (option === orderOptions[0]) {
            users.sort((a, b) => up ? a.email.localeCompare(b.email) : b.email.localeCompare(a.email));
        } else if (option === orderOptions[1]) {
            users.sort((a, b) => up ? a.id.localeCompare(b.id) : b.id.localeCompare(a.id));
        } else if (option === orderOptions[2]) {
            users.sort((a, b) => up ? a.created_at - b.created_at : b.created_at - a.created_at);
        } else if (option === orderOptions[3]) {
            users.sort((a, b) => {
                // 9999999999 as default to make ordering correct with a unix timestamp (seconds)
                // otherwise undefined values will screw ordering up
                let al = a.last_login || 9999999999;
                let bl = a.last_login || 9999999999;
                if (up) {
                    return al - bl;
                } else {
                    return bl - al;
                }
            });
        }
    }

    async function onAddNew(id: string) {
        closeModal?.();
        await fetchUsers();
        uid.set(id);
    }

    function onSave() {
        fetchUsers();
        fetchRoles();
        fetchGroups();
        searchValue = '';
    }
</script>

{#snippet navTile(id: string, email: string)}
    <NavButtonTile onclick={() => uid.set(id)} selected={uid.get() === id}>
        {email}
    </NavButtonTile>
{/snippet}

<NavSub
        paddingTop="2.1rem"
        buttonTilesAriaControls="users"
        width="min(23rem, 100dvw)"
        thresholdNavSub={700}
>
    <ButtonAddModal level={roles.length === 0 ? 1 : 2} bind:closeModal alignRight>
        <UserAddNew onSave={onAddNew} {roles} {groups}/>
    </ButtonAddModal>
    <OrderSearchBar
            bind:value={searchValue}
            {searchOptions}
            bind:searchOption
            {orderOptions}
            {onChangeOrder}
            searchWidth="min(23rem, calc(100dvw - .5rem))"
    />

    {#snippet buttonTiles()}
        <div style:height=".5rem"></div>
        {#if firstFetchHeaders}
            {#each usersFiltered as user (user.id)}
                {@render navTile(user.id, user.email)}
            {/each}
        {:else}
            {#each usersPaginated as user (user.id)}
                {@render navTile(user.id, user.email)}
            {/each}
        {/if}

        {#if firstFetchHeaders}
            <PaginationServerSide
                    sspFetch={fetchUsers}
                    idxTotalCount="x-user-count"
                    itemsLength={users.length}
                    {firstFetchHeaders}
            />
        {:else}
            <Pagination
                    bind:items={usersFiltered}
                    bind:itemsPaginated={usersPaginated}
            />
        {/if}
    {/snippet}
</NavSub>

<ContentAdmin>
    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}

    <div id="users">
        {#if userId}
            <UserDetails {userId} {roles} {groups} {onSave}/>
        {/if}
    </div>
</ContentAdmin>

<style>
</style>
