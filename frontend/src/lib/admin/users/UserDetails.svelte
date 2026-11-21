<script lang="ts">
    import { useI18n } from '#state/i18n.svelte.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import EntityHeader from '#lib/admin/EntityHeader.svelte';
    import UserPicture from '#lib/UserPicture.svelte';
    import Tabs from '#lib/tabs/Tabs.svelte';
    import type { RoleResponse } from '#api/types/roles.ts';
    import type { GroupResponse } from '#api/types/groups.ts';
    import UserInfo from '#lib/admin/users/UserInfo.svelte';
    import { fetchGet } from '#api/fetch.ts';
    import type { UserResponse } from '#api/types/user.ts';
    import UserAttr from '#lib/admin/users/UserAttr.svelte';
    import UserAddToGroups from './UserAddToGroups.svelte';
    import UserPassword from './UserPassword.svelte';
    import UserMfa from './UserMfa.svelte';
    import Devices from '#lib/devices/Devices.svelte';
    import UserForceLogout from './UserForceLogout.svelte';
    import EntityDelete from '#lib/admin/EntityDelete.svelte';
    import type { AuthProviderTemplate } from '#api/templates/AuthProvider.ts';
    import type { UserValuesConfig } from '#api/templates/UserValuesConfig.ts';
    import { useSession } from '#state/session.svelte.ts';
    import { useParam } from '#state/param.svelte.ts';

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
    // Group admins get a reduced tab set: profile (restricted), password, MFA reset
    // and force-logout. Attributes, devices and delete stay full-admin only, matching the
    // backend, which rejects those actions for a group admin anyway. The principal acts as a
    // delegated group admin when it reached the admin UI without the full `rauthy_admin` role.
    let isGroupAdmin = $derived(!session.isAdmin());

    const TAB_INFO = t.account.navInfo;
    const TAB_ATTR = ta.users.attributes;
    const TAB_PASSWORD = t.common.password;
    const TAB_MFA = t.account.navMfa;
    const TAB_DEVICES = t.account.devices;
    const TAB_LOGOUT = t.account.navLogout;
    const sections = [
        { id: 'info', label: TAB_INFO, fullAdmin: false },
        { id: 'attributes', label: TAB_ATTR, fullAdmin: true },
        { id: 'password', label: TAB_PASSWORD, fullAdmin: false },
        { id: 'mfa', label: TAB_MFA, fullAdmin: false },
        { id: 'devices', label: TAB_DEVICES, fullAdmin: true },
        { id: 'logout', label: TAB_LOGOUT, fullAdmin: false },
    ];
    const sectionParam = useParam('userTab');
    let available = $derived(sections.filter(section => !section.fullAdmin || !isGroupAdmin));
    let tabs = $derived(available.map(section => section.label));
    let selected = $derived(
        available.find(section => section.id === sectionParam.get())?.label ?? TAB_INFO,
    );

    function select(label: string) {
        const section = available.find(section => section.label === label);
        sectionParam.set(section?.id === 'info' ? undefined : section?.id, 'replace');
    }

    let err = $state('');
    // a group admin opened an ordinary user that is not in any of its groups yet (HTTP 428):
    // instead of an error, we offer to add the user to a managed group
    let notManaged = $state(false);
    let user: undefined | UserResponse = $state();

    $effect(() => {
        fetchUser();
    });

    async function fetchUser() {
        const requestedId = userId;
        err = '';
        notManaged = false;
        let res = await fetchGet<UserResponse>(`/auth/v1/users/${requestedId}`);
        if (userId !== requestedId) return;
        if (res.body) {
            user = undefined;
            requestAnimationFrame(() => {
                if (userId === requestedId) user = res.body;
            });
        } else if (res.status === 428) {
            user = undefined;
            notManaged = true;
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

{#if user && !err}
    <EntityHeader
        title={[user.given_name, user.family_name].filter(Boolean).join(' ') || user.email}
        subtitle={user.email}
    >
        {#snippet actions()}
            {#if user && !isGroupAdmin}
                {#key user.id}
                    <EntityDelete
                        url={`/auth/v1/users/${user.id}`}
                        name={user.email}
                        message={ta.users.deleteUser}
                        disabledReason={session.get()?.user_id === user.id
                            ? ta.users.antiLockout.delete
                            : undefined}
                        onDelete={onSave}
                    />
                {/key}
            {/if}
        {/snippet}
        <UserPicture
            fallbackCharacters={(user.given_name?.[0] || user.email[0]) +
                (user.family_name?.[0] || '')}
            userId={user.id}
            pictureId={user.picture_id}
            size="large"
        />
    </EntityHeader>
{/if}

{#if err}
    <!-- a group admin reaching a user it cannot manage (e.g. another admin) gets a clean
         notice instead of a raw API error and an empty tab bar -->
    <div class="notice">
        {err}
    </div>
{:else if notManaged}
    <UserAddToGroups {userId} {groups} onAdded={onSaveLocal} />
{:else}
    <div class="flex">
        <Tabs {tabs} bind:selected={() => selected, select} />
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
            {isGroupAdmin}
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
