<script lang="ts">
    import EntityDelete from '#lib/admin/EntityDelete.svelte';
    import EntityHeader from '#lib/admin/EntityHeader.svelte';
    import GroupUsers from './GroupUsers.svelte';
    import Tabs from '#lib/tabs/Tabs.svelte';
    import IconUserGroup from '#icons/IconUserGroup.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import type { GroupResponse } from '#api/types/groups.ts';
    import GroupConfig from '#lib/admin/groups/GroupConfig.svelte';

    let {
        group,
        groups,
        onSave,
    }: {
        group: GroupResponse;
        groups: GroupResponse[];
        onSave: () => void;
    } = $props();

    let ta = useI18nAdmin();
    const tabs = [ta.nav.config, ta.nav.users];
    let selected = $state(tabs[0]);
</script>

<EntityHeader title={group.name} subtitle={ta.nav.groups}>
    {#snippet actions()}
        {#key group.id}
            <EntityDelete
                url={`/auth/v1/groups/${group.id}`}
                name={group.name}
                message={ta.groups.delete1}
                onDelete={onSave}
            />
        {/key}
    {/snippet}
    <IconUserGroup width="28px" />
</EntityHeader>

<Tabs {tabs} bind:selected />

{#if selected === ta.nav.config}
    <GroupConfig {group} {groups} {onSave} />
{:else}
    <GroupUsers {group} />
{/if}
