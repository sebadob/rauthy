<script lang="ts">
    import {useI18n} from "$state/i18n.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";
    import Tabs from "$lib5/tabs/Tabs.svelte";
    import type {RoleResponse} from "$api/types/roles.ts";
    import RoleConfig from "$lib5/admin/roles/RoleConfig.svelte";
    import RoleDelete from "$lib5/admin/roles/RoleDelete.svelte";

    let {
        role,
        roles,
        onSave,
    }: {
        role: RoleResponse,
        roles: RoleResponse[],
        onSave: () => void,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    const tabs = [
        ta.nav.config,
        t.common.delete,
    ];
    let selected = $state(tabs[0]);

    let focusFirst: undefined | (() => void) = $state();

    $effect(() => {
        if (role.id) {
            requestAnimationFrame(() => {
                focusFirst?.();
            });
        }
    });

</script>

<div class="flex">
    <Tabs {tabs} bind:selected bind:focusFirst/>
</div>

{#if selected === ta.nav.config}
    <RoleConfig {role} {roles} {onSave}/>
{:else if selected === t.common.delete}
    <RoleDelete {role} {onSave}/>
{/if}
