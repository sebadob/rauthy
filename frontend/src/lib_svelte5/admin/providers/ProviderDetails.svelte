<script lang="ts">
    import type {ProviderResponse} from "$api/types/auth_provider.ts";
    import ProviderConfig from "./ProviderConfig.svelte";
    import ProviderDelete from "../../../components/admin/providers/ProviderDelete.svelte";
    import Tabs from "$lib5/tabs/Tabs.svelte";

    let {
        provider,
        onSave = $bindable(),
    }: {
        provider: ProviderResponse,
        onSave: () => void,
    } = $props();

    const tabs = [
        'Config',
        'Delete',
    ];
    let selected = $state(tabs[0]);

</script>

<div class="flex">
    <Tabs {tabs} bind:selected/>
</div>

{#if selected === 'Config'}
    <ProviderConfig {provider} bind:onSave/>
{:else if selected === 'Delete'}
    <ProviderDelete {provider} bind:onSave/>
{/if}
