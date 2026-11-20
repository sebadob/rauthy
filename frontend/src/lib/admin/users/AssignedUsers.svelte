<script lang="ts">
    import { fetchGet, fetchPatch } from '#api/fetch.ts';
    import type { PatchOp } from '#api/types/generic.ts';
    import type { UserResponse, UserResponseSimple } from '#api/types/user.ts';
    import Button from '#lib/button/Button.svelte';
    import IconTrash from '#icons/IconTrash.svelte';
    import Pagination from '#lib/pagination/Pagination.svelte';
    import SearchBar from '#lib/search_bar/SearchBar.svelte';
    import Tooltip from '#lib/Tooltip.svelte';
    import UserPicture from '#lib/UserPicture.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { useSession } from '#state/session.svelte.ts';

    let {
        name,
        usersUrl,
        patchKey,
        canDelete,
        msgEmpty,
        msgRemove,
    }: {
        // the group / role name every listed user is assigned to
        name: string;
        // endpoint returning all users that are assigned to `name`
        usersUrl: string;
        // the user field a remove PATCH operation modifies
        patchKey: 'groups' | 'roles';
        canDelete: boolean;
        msgEmpty: string;
        msgRemove: string;
    } = $props();

    let ta = useI18nAdmin();
    let session = useSession('admin');

    let err = $state('');
    let users: UserResponseSimple[] = $state([]);
    let usersFiltered: UserResponseSimple[] = $state([]);
    let usersPaginated: UserResponseSimple[] = $state([]);
    let searchValue = $state('');

    $effect(() => {
        // runs on mount and whenever another group / role is selected in the nav
        let url = usersUrl;
        err = '';
        users = [];
        void fetchUsers(url);
    });

    $effect(() => {
        let search = searchValue.toLowerCase();
        if (!search) {
            usersFiltered = users;
        } else {
            usersFiltered = users.filter(u =>
                [u.email, u.id, u.given_name, u.family_name].some(v =>
                    v?.toLowerCase().includes(search),
                ),
            );
        }
    });

    async function fetchUsers(url: string) {
        let res = await fetchGet<UserResponseSimple[]>(url);
        if (url !== usersUrl) {
            // a late answer for a previously selected id must not overwrite the current list
            return;
        }
        if (res.body) {
            users = res.body.toSorted((a, b) => a.email.localeCompare(b.email));
        } else {
            err = res.error?.message || 'Error';
        }
    }

    function fallbackCharacters(user: UserResponseSimple) {
        let chars = user.given_name ? user.given_name[0] : user.email[0];
        if (user.family_name) {
            chars += user.family_name[0];
        }
        return chars;
    }

    async function onRemove(userId: string) {
        const assignment = name;
        const url = usersUrl;
        err = '';

        // the list only holds the minified `UserResponseSimple`, but a PATCH operates on the whole
        // membership set, so the user's current groups / roles are read from the backend first
        let resUser = await fetchGet<UserResponse>(`/auth/v1/users/${userId}`);
        if (!resUser.body) {
            err = resUser.error?.message || 'Error';
            return;
        }

        let current = patchKey === 'groups' ? (resUser.body.groups ?? []) : resUser.body.roles;
        let remaining = current.filter(v => v !== assignment);

        // anti-lockout: an admin must not strip the `rauthy_admin` role from its own account
        if (
            patchKey === 'roles' &&
            session.get()?.user_id === userId &&
            current.includes('rauthy_admin') &&
            !remaining.includes('rauthy_admin')
        ) {
            err = `${ta.users.antiLockout.rule}: ${ta.users.antiLockout.rauthyAdmin}`;
            return;
        }

        // same PATCH payload shape the user details tab builds: an emptied set is `del`-eted
        let payload: PatchOp = { put: [], del: [] };
        if (remaining.length > 0) {
            payload.put.push({ key: patchKey, value: remaining });
        } else {
            payload.del.push(patchKey);
        }

        let res = await fetchPatch(`/auth/v1/users/${userId}`, payload);
        if (res.error) {
            err = res.error.message;
        } else {
            await fetchUsers(url);
        }
    }
</script>

<div class="search">
    <SearchBar bind:value={searchValue} width="min(36rem, 100%)" />
</div>

{#if err}
    <div class="err">
        {err}
    </div>
{/if}

{#if users.length === 0}
    <p class="empty">{msgEmpty}</p>
{:else}
    <div class="usersList">
        {#each usersPaginated ?? [] as user (user.id)}
            <div class="row">
                <div class="picture">
                    <UserPicture
                        fallbackCharacters={fallbackCharacters(user)}
                        userId={user.id}
                        pictureId={user.picture_id}
                        size="small"
                        disableUpload
                    />
                </div>
                <div class="userData">
                    <div>{user.email}</div>
                    <div class="names">
                        {user.given_name}
                        {user.family_name}
                        <span class="uid font-mono">{user.id}</span>
                    </div>
                </div>
                {#if canDelete}
                    <Button ariaLabel={msgRemove} invisible onclick={() => onRemove(user.id)}>
                        <Tooltip text={msgRemove}>
                            <IconTrash width="1.15rem" />
                        </Tooltip>
                    </Button>
                {/if}
            </div>
        {/each}
    </div>
{/if}

<Pagination bind:items={usersFiltered} bind:itemsPaginated={usersPaginated} />

<style>
    /* a fixed list width makes every row the same size, so rows and their hover highlight align */
    .usersList {
        display: flex;
        flex-direction: column;
        gap: 0.125rem;
        width: min(36rem, 100%);
    }

    .row {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 0.5rem;
        padding: 0.25rem 0.5rem;
        border-radius: var(--border-radius);
        overflow: clip;
    }

    .row:hover {
        background: hsl(var(--bg-high));
    }

    .picture {
        flex: none;
    }

    .userData {
        display: flex;
        flex: 1 1 auto;
        flex-direction: column;
        align-items: flex-start;
        min-width: 0;
        line-height: 1.1rem;
        overflow-x: clip;
    }

    .names {
        opacity: 0.65;
        font-size: 0.8rem;
    }

    .search {
        margin: 1rem 0 0.5rem 0;
    }

    .uid {
        margin-left: 0.5rem;
    }

    .empty {
        margin: 1rem 0.5rem;
    }
</style>
