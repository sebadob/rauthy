<script lang="ts">
    import ApiKeyConfig from "./ApiKeyConfig.svelte";
    import ApiKeySecret from "./ApiKeySecret.svelte";
    import Tabs from "$lib5/tabs/Tabs.svelte";
    import type {ApiKeyResponse} from "$api/types/api_keys.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import ApiKeyDelete from "$lib5/admin/api_keys/ApiKeyDelete.svelte";

    let {
        key = $bindable(),
        onSave = $bindable(),
    }: {
        key: ApiKeyResponse,
        onSave: () => void,
    } = $props();

    let ta = useI18nAdmin();

    const tabs = [
        ta.tabs.config,
        'Secret',
        ta.tabs.delete,
    ];
    let selected = $state(tabs[0]);
    let focusFirst: undefined | (() => void) = $state();

    $effect(() => {
        if (key.name) {
            focusFirst?.();
        }
    });

</script>

<div>
    <div class="flex">
        <Tabs {tabs} bind:selected bind:focusFirst/>
    </div>

    {#if selected === ta.tabs.config}
        <ApiKeyConfig bind:key {onSave}/>
    {:else if selected === 'Secret'}
        <ApiKeySecret {key}/>
    {:else if selected === ta.tabs.delete}
        <ApiKeyDelete {key} onSave={onSave}/>
    {/if}
</div>
