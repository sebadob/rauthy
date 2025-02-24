<script lang="ts">
    import {useI18n} from "$state/i18n.svelte.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import Tabs from "$lib5/tabs/Tabs.svelte";
    import type {UserResponse} from "$api/types/user.ts";
    import type {RoleResponse} from "$api/types/roles.ts";
    import type {GroupResponse} from "$api/types/groups.ts";

    let {
        user,
        users,
        roles,
        groups,
        onSave,
    }: {
        user: UserResponse,
        users: UserResponse[],
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

    $effect(() => {
        if (user.id) {
            requestAnimationFrame(() => {
                focusFirst?.();
            });
        }
    });

</script>

<div class="flex">
    <Tabs {tabs} bind:selected bind:focusFirst/>
</div>

{#if selected === tabs[0]}
    TODO info
{:else if selected === tabs[1]}
    TODO attributes
{:else if selected === tabs[2]}
    TODO password
{:else if selected === tabs[3]}
    TODO mfa
{:else if selected === tabs[4]}
    TODO devices
{:else if selected === tabs[5]}
    TODO logout
{:else if selected === tabs[6]}
    TODO delete
{/if}
