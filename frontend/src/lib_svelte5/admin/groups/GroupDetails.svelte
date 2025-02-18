<script lang="ts">
    import {useI18n} from "$state/i18n.svelte.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import Tabs from "$lib5/tabs/Tabs.svelte";
    import type {GroupResponse} from "$api/types/groups.ts";
    import GroupConfig from "$lib5/admin/groups/GroupConfig.svelte";
    import GroupDelete from "$lib5/admin/groups/GroupDelete.svelte";

    let {
        group,
        groups,
        onSave,
    }: {
        group: GroupResponse,
        groups: GroupResponse[],
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
        if (group.id) {
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
    <GroupConfig {group} {groups} {onSave}/>
{:else if selected === t.common.delete}
    <GroupDelete {group} {onSave}/>
{/if}
