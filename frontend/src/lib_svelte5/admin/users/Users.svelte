<script lang="ts">
    import {onMount} from "svelte";
    import {fetchGet} from "$api/fetch.ts";
    import type {UserResponse} from "$api/types/user.ts";
    import OrderSearchBar from "$lib5/search_bar/OrderSearchBar.svelte";
    import type {GroupResponse} from "$api/types/groups.ts";
    import type {RoleResponse} from "$api/types/roles.ts";
    import PaginationServer from "$lib/PaginationServer.svelte";
    import Pagination from "$lib5/Pagination.svelte";
    import NavSub from "$lib5/nav/NavSub.svelte";
    import ButtonAddModal from "$lib5/button/ButtonAddModal.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import {useParam} from "$state/param.svelte.ts";
    import NavButtonTile from "$lib5/nav/NavButtonTile.svelte";
    import ContentAdmin from "$lib5/ContentAdmin.svelte";
    import UserAddNew from "$lib5/admin/users/UserAddNew.svelte";

    let ta = useI18nAdmin();

    let closeModal: undefined | (() => void) = $state();
    let err = $state('');

    let users: UserResponse[] = $state([]);
    let usersFiltered: UserResponse[] = $state([]);
    let usersPaginated: UserResponse[] = $state([]);
    let usersCountTotal = $state(0);
    let uid = useParam('uid');
    let user: undefined | UserResponse = $state();

    let groups: GroupResponse[] = $state([]);
    let roles: RoleResponse[] = $state([]);

    let useServerSideIdx = $state('');
    let isSearchFiltered = $state(false);

    let sspPageSize = $state(15);
    let sspContinuationToken = $state('');
    let sspPage = $state(1);

    let searchOptions = $state(['E-Mail', 'ID']);
    let searchOption = $state(searchOptions[0]);
    let searchValue = $state('');
    let orderOptions = $state(['E-Mail', 'ID', 'Created', 'Last Login']);

    onMount(() => {
        fetchUsers();
        fetchRoles();
        fetchGroups();
    })

    $effect(() => {
        user = users.find(u => u.id === uid.get());
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

    async function fetchUsers(useSsp?: boolean, offset?: number, backwards?: boolean, pageSize?: number) {
        let url = '/auth/v1/users';
        if (useSsp === true) {
            url += `page_size=${pageSize || sspPageSize}`;
            if (offset) {
                url += `&offset=${offset}`;
            }
            if (backwards) {
                url += `&backwards=${backwards}`;
                if (sspPage !== 2 && sspContinuationToken) {
                    url += `&continuation_token=${sspContinuationToken}`;
                }
            }
        }
        let res = await fetchGet<UserResponse[]>(url);
        if (res.error) {
            err = 'Error fetching users: ' + res.error.message;
        } else if (res.body) {
            // HTTP 206 --> Backend is using SSP
            if (res.status === 206) {
                // we get a few headers during SSP we can use for the navigation
                let xPageSize = res.headers.get('x-page-size');
                if (!xPageSize) {
                    console.error('Did not receive x-page-size with SSP');
                    return;
                }
                sspPageSize = Number.parseInt(xPageSize);
                sspContinuationToken = res.headers.get('x-continuation-token') || '';
                // sspPageCount = res.headers.get('x-page-count');
                useServerSideIdx = 'session';
            } else {
                useServerSideIdx = '';
            }

            users = res.body;
        }
    }

    // // Callback function for <PaginationServer>
    // // Fetches the next page during server side pagination with the given offset and direction.
    // async function fetchUsersSsp(offset, backwards) {
    //     await fetchUsers(true, offset, backwards);
    //     if (backwards) {
    //         sspPage -= 1;
    //     } else {
    //         sspPage += 1;
    //     }
    // }

    // Callback function for <PaginationServer> to make page size switches work
    async function sspPageSizeChange(pageSize: number) {
        sspContinuationToken = '';
        await fetchUsers(true, 0, false, pageSize);
        sspPage = 1;
    }

    async function fetchRoles() {
        let res = await fetchGet<RoleResponse[]>('/auth/v1/roles');
        if (res.body) {
            roles = res.body;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    async function fetchGroups() {
        let res = await fetchGet<GroupResponse[]>('/auth/v1/groups');
        if (res.body) {
            groups = res.body;
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
        {#if useServerSideIdx}
            {#each usersFiltered as user (user.id)}
                {@render navTile(user.id, user.email)}
            {/each}
        {:else}
            {#each usersPaginated as user (user.id)}
                {@render navTile(user.id, user.email)}
            {/each}
        {/if}

        <!--
        Even with server side pagination, we must use it client side if we
        have a filtered search result. Otherwise, switching pages would
        overwrite the filtered data.
        -->
        {#if useServerSideIdx}
            <PaginationServer
                    itemsTotal={usersCountTotal}
                    bind:sspPage
                    bind:sspPageSize
                    bind:sspContinuationToken
                    sspPageSizeChange={sspPageSizeChange}
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
        {#if user}
            {user.email} TODO
        {/if}
    </div>
</ContentAdmin>

<style>
</style>
