<script lang="ts">
    import { onDestroy, onMount } from 'svelte';
    import { fetchGet } from '#api/fetch.ts';
    import type { UserResponse, UserResponseSimple } from '#api/types/user.ts';
    import OrderSearchBar from '#lib/search_bar/OrderSearchBar.svelte';
    import type { GroupResponse } from '#api/types/groups.ts';
    import type { RoleResponse } from '#api/types/roles.ts';
    import Pagination from '#lib/pagination/Pagination.svelte';
    import NavSub from '#lib/nav/NavSub.svelte';
    import ButtonAddModal from '#lib/button/ButtonAddModal.svelte';
    import { useParam } from '#state/param.svelte.ts';
    import NavButtonTile from '#lib/nav/NavButtonTile.svelte';
    import ContentAdmin from '#lib/ContentAdmin.svelte';
    import UserAddNew from '#lib/admin/users/UserAddNew.svelte';
    import UserDetails from '#lib/admin/users/UserDetails.svelte';
    import PaginationServerSide from '#lib/pagination/PaginationServerSide.svelte';
    import { loadPageSize, type PageSize } from '#lib/pagination/props.ts';
    import { fetchSearchServer, type SearchParamsIdxUser } from '#utils/search.ts';
    import UserPicture from '#lib/UserPicture.svelte';
    import type { AuthProviderTemplate } from '#api/templates/AuthProvider.ts';
    import { useTrigger } from '#state/callback.svelte.ts';
    import type { UserValuesConfig } from '#api/templates/UserValuesConfig.ts';
    import Button from '#lib/button/Button.svelte';
    import IconEnvelope from '#icons/IconEnvelope.svelte';
    import SendMail from '#lib/SendMail.svelte';
    import EmailJobs from '#lib/EmailJobs.svelte';

    // The search bar's options: 'Name' matches given_name / family_name, 'Username' matches
    // preferred_username, and both are server-side idx values as well
    const SEARCH_EMAIL = 'E-Mail';
    const SEARCH_ID = 'ID';
    const SEARCH_NAME = 'Name';
    const SEARCH_USERNAME = 'Username';
    // Server-side searches are fired while typing - this is the minimum time between two queries
    const SERVER_SEARCH_THROTTLE_MS = 250;

    let refAddNew: undefined | HTMLButtonElement = $state();
    let tr = useTrigger();
    tr.set('navMain', () => refAddNew?.focus());

    let closeModal: undefined | (() => void) = $state();
    let err = $state('');

    let userValuesConfig: undefined | UserValuesConfig = $state();
    let users: UserResponseSimple[] = $state([]);
    let usersFiltered: UserResponseSimple[] = $state([]);
    let usersPaginated: UserResponseSimple[] = $state([]);
    let uid = useParam('uid');
    let userId: undefined | string = $state();

    let providers: AuthProviderTemplate[] = $state([]);
    let groups: GroupResponse[] = $state([]);
    let roles: RoleResponse[] = $state([]);

    let useServerSide = $state(false);
    let firstFetchHeaders: undefined | Headers = $state();
    let sspPageSize: PageSize = $state(loadPageSize());
    let isSearchedServer = $state(false);

    // 'Name' is filtered in memory as long as the whole list is loaded, 'Username' always needs a
    // server-side search
    let searchOptions = $state([SEARCH_EMAIL, SEARCH_ID, SEARCH_NAME, SEARCH_USERNAME]);
    let searchOption = $state(SEARCH_EMAIL);
    let searchValue = $state('');
    let orderOptions = $state(['E-Mail', 'ID', 'Created', 'Last Login']);

    let serverSearchTimer: undefined | ReturnType<typeof setTimeout> = undefined;
    // timestamp of the last fired server-side search - the throttle's anchor
    let serverSearchTs = 0;
    let serverSearchId = 0;

    onDestroy(() => clearTimeout(serverSearchTimer));

    onMount(() => {
        fetchUsers('page_size=' + sspPageSize);
        fetchUserValuesConfig();
        fetchAuthProviders();
        fetchRoles();
        fetchGroups();
    });

    $effect(() => {
        userId = users.find(u => u.id === uid.get())?.id;
    });

    $effect(() => {
        let search = searchValue.toLowerCase();
        // preferred_username is not part of the in-memory payload -> server-side in any mode, and
        // with server side pagination 'Name' is searched on the backend as well
        let serverSide = useServerSide || searchOption === SEARCH_USERNAME;

        if (serverSide) {
            clearTimeout(serverSearchTimer);
            if (search.length < 3) {
                if (isSearchedServer) {
                    isSearchedServer = false;
                    if (useServerSide) {
                        fetchUsers('page_size=' + sspPageSize);
                    } else {
                        usersFiltered = users;
                    }
                }
                return;
            }

            let elapsed = Date.now() - serverSearchTs;
            if (elapsed >= SERVER_SEARCH_THROTTLE_MS) {
                searchServer(search);
            } else {
                // trailing query so the last typed value is the one that ends up being shown
                serverSearchTimer = setTimeout(
                    () => searchServer(search),
                    SERVER_SEARCH_THROTTLE_MS - elapsed,
                );
            }
            return;
        }

        isSearchedServer = false;

        if (!search) {
            usersFiltered = users;
        } else if (searchOption === SEARCH_EMAIL) {
            usersFiltered = users.filter(u => u.email?.toLowerCase().includes(search));
        } else if (searchOption === SEARCH_ID) {
            usersFiltered = users.filter(u => u.id.toLowerCase().includes(search));
        } else {
            usersFiltered = users.filter(
                u =>
                    u.given_name?.toLowerCase().includes(search) ||
                    u.family_name?.toLowerCase().includes(search),
            );
        }
    });

    async function searchServer(q: string) {
        // throttled queries can resolve out of order - only apply the newest answer
        let id = ++serverSearchId;
        serverSearchTs = Date.now();
        firstFetchHeaders = undefined;
        isSearchedServer = true;

        let idx: SearchParamsIdxUser;
        if (searchOption === SEARCH_EMAIL) {
            idx = 'email';
        } else if (searchOption === SEARCH_ID) {
            idx = 'id';
        } else if (searchOption === SEARCH_NAME) {
            idx = 'name';
        } else {
            idx = 'username';
        }

        let res = await fetchSearchServer<UserResponseSimple[]>({
            ty: 'user',
            idx,
            q,
        });
        if (id !== serverSearchId) {
            return;
        }
        if (res.body) {
            // in-memory mode keeps the full `users` list intact and only replaces the filter
            if (useServerSide) {
                users = res.body;
            } else {
                usersFiltered = res.body;
            }
        } else {
            console.error(res.error);
        }
    }

    async function fetchUserValuesConfig() {
        let res = await fetchGet<UserValuesConfig>('/auth/v1/users/values_config');
        if (res.body) {
            userValuesConfig = res.body;
        } else {
            console.error(res);
        }
    }

    async function fetchUsers(urlParams?: string): Promise<[number, Headers]> {
        let url = `/auth/v1/users`;
        if (urlParams) {
            url += `?${urlParams}`;
        }

        let res = await fetchGet<UserResponse[]>(url);
        if (res.error) {
            err = 'Error fetching users: ' + res.error.message;
        } else if (res.body) {
            if (res.status === 206) {
                useServerSide = true;
                firstFetchHeaders = res.headers;
            } else {
                useServerSide = false;
                firstFetchHeaders = undefined;
            }
            users = res.body;
        }

        return [res.status, res.headers];
    }

    async function fetchAuthProviders() {
        let res = await fetchGet<AuthProviderTemplate[]>('/auth/v1/providers/minimal');
        if (res.body) {
            providers = res.body;
        } else {
            err = res.error?.message || 'Error';
        }
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

    function fallbackCharacters(user: UserResponseSimple) {
        let chars = '';
        if (user.given_name) {
            chars = user.given_name[0];
        } else {
            chars = user.email[0];
        }
        if (user.family_name) {
            chars += user.family_name[0];
        }
        return chars;
    }

    function onChangeOrder(option: string, direction: 'up' | 'down') {
        let up = direction === 'up';

        // ['E-Mail', 'ID', 'Created', 'Last Login']
        if (option === orderOptions[0]) {
            users.sort((a, b) =>
                up ? a.email.localeCompare(b.email) : b.email.localeCompare(a.email),
            );
        } else if (option === orderOptions[1]) {
            users.sort((a, b) => (up ? a.id.localeCompare(b.id) : b.id.localeCompare(a.id)));
        } else if (option === orderOptions[2]) {
            users.sort((a, b) => (up ? a.created_at - b.created_at : b.created_at - a.created_at));
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

{#snippet navTile(user: UserResponseSimple)}
    <NavButtonTile
        onclick={() => {
            uid.set(user.id);
            tr.trigger('navSubSub');
        }}
        selected={uid.get() === user.id}
        pictureLeft
    >
        <div class="navBtn">
            <div class="picture">
                <UserPicture
                    fallbackCharacters={fallbackCharacters(user)}
                    userId={user.id}
                    pictureId={user.picture_id}
                    size="small"
                    disableUpload
                />
            </div>
            <div class="tile">
                <div>
                    {user.email}
                </div>
                <div class="muted">
                    {user.given_name}
                    {user.family_name}
                </div>
            </div>
        </div>
    </NavButtonTile>
{/snippet}

<NavSub
    paddingTop="2.35rem"
    buttonTilesAriaControls="users"
    width="min(23rem, 100dvw)"
    thresholdNavSub={700}
>
    <div class="topBtns">
        <SendMail {groups} {roles} />
        <EmailJobs />
        <ButtonAddModal
            bind:ref={refAddNew}
            level={roles.length === 0 ? 1 : 2}
            bind:closeModal
            alignRight
        >
            <UserAddNew config={userValuesConfig} onSave={onAddNew} {roles} {groups} />
        </ButtonAddModal>
    </div>

    <OrderSearchBar
        bind:value={searchValue}
        {searchOptions}
        bind:searchOption
        {orderOptions}
        {onChangeOrder}
        searchWidth="min(23rem, calc(100dvw - .5rem))"
    />

    {#snippet buttonTiles()}
        <div class="usersList">
            {#if firstFetchHeaders}
                {#each users as user}
                    {@render navTile(user)}
                {/each}
            {:else}
                {#each usersPaginated as user (user.id)}
                    {@render navTile(user)}
                {/each}
            {/if}
        </div>

        {#if firstFetchHeaders}
            <PaginationServerSide
                bind:pageSize={sspPageSize}
                sspFetch={fetchUsers}
                idxTotalCount="x-user-count"
                itemsLength={users.length}
                {firstFetchHeaders}
            />
        {:else if useServerSide}
            <Pagination bind:items={users} bind:itemsPaginated={usersPaginated} />
        {:else}
            <Pagination bind:items={usersFiltered} bind:itemsPaginated={usersPaginated} />
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
        {#if userId && userValuesConfig}
            <UserDetails {userValuesConfig} {userId} {providers} {roles} {groups} {onSave} />
        {/if}
    </div>
</ContentAdmin>

<style>
    .muted {
        margin-bottom: -0.2rem;
        opacity: 0.65;
        font-size: 0.8rem;
    }

    .navBtn {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .topBtns {
        margin-left: 0.5rem;
        display: flex;
        justify-content: flex-end;
        gap: 0.5rem;
    }

    .usersList {
        margin-top: 0.5rem;
        max-height: calc(100dvh - 11.5rem);
        overflow-y: auto;
    }

    .tile {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        line-height: 1rem;
        overflow-x: clip;
    }
</style>
