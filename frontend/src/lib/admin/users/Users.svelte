<script lang="ts">
    import AdminListHeading from '#lib/admin/AdminListHeading.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { onMount, untrack } from 'svelte';
    import { fetchGet } from '#api/fetch.ts';
    import type { UserResponse, UserResponseSimple } from '#api/types/user.ts';
    import OrderSearchBar from '#lib/search_bar/OrderSearchBar.svelte';
    import type { GroupResponse } from '#api/types/groups.ts';
    import type { RoleResponse } from '#api/types/roles.ts';
    import Pagination from '#lib/pagination/Pagination.svelte';
    import NavSub from '#lib/nav/NavSub.svelte';
    import ButtonAddModal from '#lib/button/ButtonAddModal.svelte';
    import { useParam } from '#state/param.svelte.ts';
    import type { UnsavedField } from '#utils/unsaved.ts';
    import NavButtonTile from '#lib/nav/NavButtonTile.svelte';
    import ContentAdmin from '#lib/ContentAdmin.svelte';
    import UserAddNew from '#lib/admin/users/UserAddNew.svelte';
    import UserDetails from '#lib/admin/users/UserDetails.svelte';
    import PaginationServerSide from '#lib/pagination/PaginationServerSide.svelte';
    import { loadPageSize, type PageSize } from '#lib/pagination/props.ts';
    import { fetchSearchServer, type SearchParamsIdxUser } from '#utils/search.ts';
    import UserPicture from '#lib/UserPicture.svelte';
    import type { AuthProviderTemplate } from '#api/templates/AuthProvider.ts';
    import type { UserValuesConfig } from '#api/templates/UserValuesConfig.ts';
    import SendMail from '#lib/SendMail.svelte';
    import EmailJobs from '#lib/EmailJobs.svelte';

    let ta = useI18nAdmin();
    const t = useI18n();
    // Both name fields support server-side lookup. Only given and family names are in the list.
    const SEARCH_EMAIL = t.common.email;
    const SEARCH_ID = 'ID';
    const SEARCH_NAME = ta.users.searchName ?? 'Name';
    const SEARCH_USERNAME = ta.users.searchUsername ?? 'Username';
    // Limit how often typing can trigger a server search.
    const SERVER_SEARCH_THROTTLE_MS = 250;

    let closeModal: undefined | (() => void) = $state();
    let err = $state('');

    let userValuesConfig: undefined | UserValuesConfig = $state();
    let users: UserResponseSimple[] = $state([]);
    let usersFiltered: UserResponseSimple[] = $state([]);
    let usersPaginated: UserResponseSimple[] = $state([]);
    let uid = useParam('uid');
    let userId: undefined | string = $state();
    let fields: UnsavedField[] = $state([]);
    // set by the guard inside UserDetails; a `uid` change is applied to the reactive state
    // before `goto` resolves, so switching users must be asked for here, not in beforeNavigate
    let guard: (run: () => void) => void = $state(run => run());

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
    let orderOptions = $state([t.common.email, 'ID', t.account.userCreated, ta.users.lastLogin]);

    let serverSearchTimer: undefined | ReturnType<typeof setTimeout> = undefined;
    // timestamp of the last fired server-side search - the throttle's anchor
    let serverSearchTs = 0;
    let serverSearchId = 0;

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
        const search = searchValue.toLowerCase();
        const option = searchOption;
        const paginated = useServerSide;

        if (paginated || option === SEARCH_USERNAME) {
            const searchId = ++serverSearchId;
            if (search.length < 3) {
                untrack(() => {
                    if (isSearchedServer && paginated) {
                        fetchUsers('page_size=' + sspPageSize);
                    }
                    isSearchedServer = false;
                });
                if (!paginated) usersFiltered = users;
                return;
            }

            const delay = Math.max(0, SERVER_SEARCH_THROTTLE_MS - (Date.now() - serverSearchTs));
            const run = () => {
                void searchServer(search, option, paginated, searchId);
            };
            if (delay) serverSearchTimer = setTimeout(run, delay);
            else untrack(run);
            return () => {
                clearTimeout(serverSearchTimer);
                serverSearchId++;
            };
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

    async function searchServer(q: string, option: string, paginated: boolean, id: number) {
        serverSearchTs = Date.now();
        firstFetchHeaders = undefined;
        isSearchedServer = true;

        let idx: SearchParamsIdxUser;
        if (option === SEARCH_EMAIL) {
            idx = 'email';
        } else if (option === SEARCH_ID) {
            idx = 'id';
        } else if (option === SEARCH_NAME) {
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
            if (paginated) {
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
        guard(() => uid.set(id));
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
        onclick={() => guard(() => uid.set(user.id))}
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
                <div class="admin-row-title">
                    {[user.given_name, user.family_name].filter(Boolean).join(' ') || user.email}
                </div>
                <div class="muted">{user.email}</div>
            </div>
        </div>
    </NavButtonTile>
{/snippet}

<NavSub
    detail={!!userId}
    onBack={() => guard(() => uid.set(undefined))}
    error={err}
    buttonTilesAriaControls="users"
>
    <AdminListHeading
        title={ta.nav.users}
        count={useServerSide ? users.length : usersFiltered.length}
    />
    <div class="topBtns">
        <SendMail {groups} {roles} />
        <EmailJobs />
        <ButtonAddModal level={roles.length === 0 ? 1 : 2} bind:closeModal alignRight>
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
    {/snippet}
    {#snippet listFooter()}
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
            <UserDetails
                {userValuesConfig}
                {userId}
                {providers}
                {roles}
                {groups}
                bind:fields
                bind:guard
                {onSave}
            />
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

    .navBtn {
        display: flex;
        align-items: center;
        gap: 12px;
    }
</style>
