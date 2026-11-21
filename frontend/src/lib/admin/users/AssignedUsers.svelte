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

<div class="members">
    <SearchBar bind:value={searchValue} width="100%" />

    {#if err}
        <p class="err" role="alert">{err}</p>
    {/if}

    {#if users.length === 0}
        <p class="ui-muted-copy">{msgEmpty}</p>
    {:else}
        <ul class="users-list" role="list">
            {#each usersPaginated as user (user.id)}
                {@const name = [user.given_name, user.family_name].filter(Boolean).join(' ')}
                <li class="member">
                    <div class="identity">
                        <UserPicture
                            fallbackCharacters={fallbackCharacters(user)}
                            userId={user.id}
                            pictureId={user.picture_id}
                            size="small"
                            disableUpload
                        />
                        <div class="user-data">
                            <div class="name">{name || user.email}</div>
                            {#if name}<div class="secondary">{user.email}</div>{/if}
                            <div class="secondary font-mono">{user.id}</div>
                        </div>
                    </div>
                    {#if canDelete}
                        <Button ariaLabel={msgRemove} level={-3} onclick={() => onRemove(user.id)}>
                            <Tooltip text={msgRemove}>
                                <IconTrash width="1.125rem" />
                            </Tooltip>
                        </Button>
                    {/if}
                </li>
            {/each}
        </ul>
    {/if}

    <Pagination bind:items={usersFiltered} bind:itemsPaginated={usersPaginated} />
</div>

<style>
    .members,
    .users-list {
        display: grid;
        gap: 0.5rem;
        min-width: 0;
    }

    .users-list {
        padding: 0;
        margin: 0;
        list-style: none;
    }

    .member,
    .identity {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        min-width: 0;
    }

    .member {
        justify-content: space-between;
        padding: 0.75rem;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-surface);
    }

    .member:hover,
    .member:focus-within {
        background: var(--ui-container);
    }

    .identity :global(.avatar) {
        --avatar-background: var(--ui-tonal);
    }

    .identity :global(.avatar span) {
        color: var(--ui-primary);
        font-size: 0.875rem;
        line-height: 1;
    }

    .user-data {
        min-width: 0;
        overflow-wrap: anywhere;
    }

    .name {
        color: var(--ui-text);
        font-size: var(--ui-control-font-size);
        font-weight: 600;
    }

    .secondary {
        color: var(--ui-muted);
        font-size: 0.8125rem;
        line-height: 1.4;
    }
</style>
