<script lang="ts">
    import { useI18n } from '$state/i18n.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import Tabs from '$lib5/tabs/Tabs.svelte';
    import type { RoleResponse } from '$api/types/roles.ts';
    import type { GroupResponse } from '$api/types/groups.ts';
    import UserInfo from '$lib5/admin/users/UserInfo.svelte';
    import { fetchGet } from '$api/fetch';
    import type { UserResponse } from '$api/types/user.ts';
    import UserAttr from '$lib5/admin/users/UserAttr.svelte';
    import UserPassword from './UserPassword.svelte';
    import UserMfa from './UserMfa.svelte';
    import Devices from '$lib5/devices/Devices.svelte';
    import UserForceLogout from './UserForceLogout.svelte';
    import UserDelete from '$lib5/admin/users/UserDelete.svelte';
    import type { AuthProviderTemplate } from '$api/templates/AuthProvider';
    import { useTrigger } from '$state/callback.svelte';
    import type { UserValuesConfig } from '$api/templates/UserValuesConfig';
    import { useSession } from '$state/session.svelte';

    let {
        userValuesConfig,
        userId,
        providers,
        roles,
        groups,
        onSave,
    }: {
        userValuesConfig: UserValuesConfig;
        userId: string;
        providers: AuthProviderTemplate[];
        roles: RoleResponse[];
        groups: GroupResponse[];
        onSave: () => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    let session = useSession('admin');
    // Group admins (#1538) get a reduced tab set: profile (restricted), MFA reset and
    // force-logout only. Attributes, password, devices and delete stay full-admin only,
    // matching the backend, which rejects those actions for a group admin anyway.
    let fullAdmin = $derived(session.isAdmin());

    const TAB_INFO = t.account.navInfo;
    const TAB_ATTR = ta.users.attributes;
    const TAB_PASSWORD = t.common.password;
    const TAB_MFA = t.account.navMfa;
    const TAB_DEVICES = t.account.devices;
    const TAB_LOGOUT = t.account.navLogout;
    const TAB_DELETE = t.common.delete;

    let tabs = $derived(
        fullAdmin
            ? [TAB_INFO, TAB_ATTR, TAB_PASSWORD, TAB_MFA, TAB_DEVICES, TAB_LOGOUT, TAB_DELETE]
            : [TAB_INFO, TAB_MFA, TAB_LOGOUT],
    );
    let selected = $state(TAB_INFO);

    let focusFirst: undefined | (() => void) = $state();
    useTrigger().set('navSubSub', () => {
        requestAnimationFrame(() => {
            focusFirst?.();
        });
    });

    let err = $state('');
    let user: undefined | UserResponse = $state();

    $effect(() => {
        fetchUser();
    });

    async function fetchUser() {
        err = '';
        let res = await fetchGet<UserResponse>(`/auth/v1/users/${userId}`);
        if (res.body) {
            user = undefined;
            requestAnimationFrame(() => {
                user = res.body;
            });
            focusFirst?.();
        } else {
            user = undefined;
            err = res.error?.message || 'Error fetching user';
        }
    }

    async function onSaveLocal() {
        onSave();
        await fetchUser();
    }
</script>

{#if err}
    <!-- a group admin reaching a user it cannot manage gets a clean notice instead of a
         raw API error and an empty tab bar (#1538) -->
    <div class="notice">
        {err}
    </div>
{:else}
    <div class="flex">
        <Tabs {tabs} bind:selected bind:focusFirst />
    </div>
{/if}

{#if user && !err}
    {#if selected === TAB_INFO}
        <UserInfo
            bind:user
            config={userValuesConfig}
            {providers}
            {roles}
            {groups}
            {fullAdmin}
            onSave={onSaveLocal}
        />
    {:else if selected === TAB_ATTR}
        <UserAttr {user} {onSave} />
    {:else if selected === TAB_PASSWORD}
        <UserPassword {user} {onSave} />
    {:else if selected === TAB_MFA}
        <UserMfa {user} {onSave} />
    {:else if selected === TAB_DEVICES}
        <Devices {userId} />
    {:else if selected === TAB_LOGOUT}
        <UserForceLogout {userId} />
    {:else if selected === TAB_DELETE}
        <UserDelete {userId} {onSave} />
    {/if}
{/if}

<style>
    .notice {
        margin: 1rem 0.5rem;
        padding: 1rem;
        border-radius: var(--border-radius);
        background: hsla(var(--bg-high) / 0.25);
        color: hsla(var(--text) / 0.9);
    }
</style>
