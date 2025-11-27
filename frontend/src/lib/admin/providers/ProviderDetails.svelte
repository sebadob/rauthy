<script lang="ts">
    import type { ProviderResponse } from '$api/types/auth_provider.ts';
    import ProviderConfig from './ProviderConfig.svelte';
    import ProviderDelete from './ProviderDelete.svelte';
    import Tabs from '$lib5/tabs/Tabs.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';

    let {
        provider = $bindable(),
        onSave = $bindable(),
    }: {
        provider: ProviderResponse;
        onSave: () => void;
    } = $props();

    let ta = useI18nAdmin();

    const tabs = [ta.tabs.config, ta.tabs.delete];
    let selected = $state(tabs[0]);
</script>

<div class="flex">
    <Tabs {tabs} bind:selected />
</div>

{#if selected === ta.tabs.config}
    <ProviderConfig bind:provider bind:onSave />
{:else if selected === ta.tabs.delete}
    <ProviderDelete {provider} bind:onSave />
{/if}
