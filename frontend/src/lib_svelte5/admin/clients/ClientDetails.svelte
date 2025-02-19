<script lang="ts">
    import {useI18n} from "$state/i18n.svelte.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import Tabs from "$lib5/tabs/Tabs.svelte";
    import type {ClientResponse} from "$api/types/clients.ts";

    let {
        client,
        clients,
        onSave,
    }: {
        client: ClientResponse,
        clients: ClientResponse[],
        onSave: () => void,
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    const tabs = [
        ta.nav.config,
        'Secret',
        'Branding',
        t.common.delete,
    ];
    let selected = $state(tabs[0]);

    let focusFirst: undefined | (() => void) = $state();

    $effect(() => {
        if (client.id) {
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
    TODO
    <!--    <GroupConfig {group} {groups} {onSave}/>-->
{:else if selected === 'Secret'}
    TODO
{:else if selected === 'Branding'}
    TODO
{:else if selected === t.common.delete}
    TODO
    <!--    <GroupDelete {group} {onSave}/>-->
{/if}
