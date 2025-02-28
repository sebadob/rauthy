<script lang="ts">
    import {useI18n} from "$state/i18n.svelte.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import Tabs from "$lib5/tabs/Tabs.svelte";
    import type {RoleResponse} from "$api/types/roles.ts";
    import type {GroupResponse} from "$api/types/groups.ts";
    import UserInfo from "$lib5/admin/users/UserInfo.svelte";
    import {fetchGet} from "$api/fetch.ts";
    import type {UserResponse} from "$api/types/user.ts";
    import UserAttr from "$lib5/admin/users/UserAttr.svelte";
    import UserPassword from "./UserPassword.svelte";
    import UserMfa from "./UserMfa.svelte";
    import Devices from "$lib5/devices/Devices.svelte";
    import UserForceLogout from "./UserForceLogout.svelte";
    import UserDelete from "$lib5/admin/users/UserDelete.svelte";

    let {
        userId,
        roles,
        groups,
        onSave,
    }: {
        userId: string,
        roles: RoleResponse[],
        groups: GroupResponse[],
        onSave: () => void,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    const tabs = [
        t.account.navInfo,
        ta.users.attributes,
        t.common.password,
        t.account.navMfa,
        t.account.devices,
        t.account.navLogout,
        t.common.delete,
    ];
    let selected = $state(tabs[0]);

    let focusFirst: undefined | (() => void) = $state();

    let err = $state('');
    let user: undefined | UserResponse = $state();

    $effect(() => {
        fetchUser();
    });

    async function fetchUser() {
        let res = await fetchGet<UserResponse>(`/auth/v1/users/${userId}`);
        if (res.body) {
            user = res.body;
            requestAnimationFrame(() => {
                focusFirst?.();
            });
        } else {
            err = res.error?.message || 'Error fetching user';
        }
    }

</script>

{#if err}
    <div class="err">
        {err}
    </div>
{/if}

<div class="flex">
    <Tabs {tabs} bind:selected bind:focusFirst/>
</div>

{#if user}
    {#if selected === tabs[0]}
        <UserInfo bind:user {roles} {groups} {onSave}/>
    {:else if selected === tabs[1]}
        <UserAttr {user} {onSave}/>
    {:else if selected === tabs[2]}
        <UserPassword {user} {onSave}/>
    {:else if selected === tabs[3]}
        <UserMfa {user} {onSave}/>
    {:else if selected === tabs[4]}
        <Devices {userId}/>
    {:else if selected === tabs[5]}
        <UserForceLogout {userId}/>
    {:else if selected === tabs[6]}
        <UserDelete {userId} {onSave}/>
    {/if}
{/if}
