<script lang="ts">
    import ApiKeyConfig from "./ApiKeyConfig.svelte";
    import ApiKeyDelete from "../../../components/admin/api_keys/ApiKeyDelete.svelte";
    import ApiKeySecret from "./ApiKeySecret.svelte";
    import Tabs from "$lib5/tabs/Tabs.svelte";
    import type {ApiKeyResponse} from "$api/types/api_keys.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";

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

</script>

<div>
    <div class="flex">
        <Tabs {tabs} bind:selected/>
    </div>

    {#if selected === ta.tabs.config}
        <ApiKeyConfig bind:key {onSave}/>
    {:else if selected === 'Secret'}
        <ApiKeySecret {key}/>
    {:else if selected === ta.tabs.delete}
        <ApiKeyDelete {key} onSave={onSave}/>
    {/if}
</div>
