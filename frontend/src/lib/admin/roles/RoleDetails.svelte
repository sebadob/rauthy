<script lang="ts">
    import EntityDelete from '#lib/admin/EntityDelete.svelte';
    import EntityHeader from '#lib/admin/EntityHeader.svelte';
    import RoleUsers from './RoleUsers.svelte';
    import Tabs from '#lib/tabs/Tabs.svelte';
    import IconCheckBadge from '#icons/IconCheckBadge.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import type { RoleResponse } from '#api/types/roles.ts';
    import RoleConfig from '#lib/admin/roles/RoleConfig.svelte';

    let {
        role,
        roles,
        onSave,
    }: {
        role: RoleResponse;
        roles: RoleResponse[];
        onSave: () => void;
    } = $props();

    let ta = useI18nAdmin();
    const tabs = [ta.nav.config, ta.nav.users];
    let selected = $state(tabs[0]);
</script>

<EntityHeader title={role.name} subtitle={ta.nav.roles}>
    {#snippet actions()}
        {#key role.id}
            <EntityDelete
                url={`/auth/v1/roles/${role.id}`}
                name={role.name}
                message={ta.roles.delete1}
                disabledReason={role.name === 'rauthy_admin'
                    ? ta.roles.adminNoMod.replace(/<[^>]*>/g, '')
                    : undefined}
                onDelete={onSave}
            />
        {/key}
    {/snippet}
    <IconCheckBadge width="28px" />
</EntityHeader>

<Tabs {tabs} bind:selected />

{#if selected === ta.nav.config}
    <RoleConfig {role} {roles} {onSave} />
{:else}
    <RoleUsers {role} />
{/if}
