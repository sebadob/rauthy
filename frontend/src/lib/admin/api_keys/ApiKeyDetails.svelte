<script lang="ts">
    import ApiKeyConfig from './ApiKeyConfig.svelte';
    import ApiKeySecret from './ApiKeySecret.svelte';
    import Tabs from '#lib/tabs/Tabs.svelte';
    import type { ApiKeyResponse } from '#api/types/api_keys.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import EntityHeader from '#lib/admin/EntityHeader.svelte';
    import EntityDelete from '#lib/admin/EntityDelete.svelte';
    import IconKey from '#icons/IconKey.svelte';

    let {
        key = $bindable(),
        onSave = $bindable(),
    }: {
        key: ApiKeyResponse;
        onSave: () => void;
    } = $props();

    let ta = useI18nAdmin();

    const secretTab = ta.api_key.secret ?? 'Secret';
    const tabs = [ta.tabs.config, secretTab];
    let selected = $state(tabs[0]);
</script>

<EntityHeader title={key.name} subtitle={ta.nav.apiKeys}>
    {#snippet actions()}
        {#key key.name}
            <EntityDelete
                url={`/auth/v1/api_keys/${key.name}`}
                name={key.name}
                message={ta.api_key.delete1}
                onDelete={onSave}
            />
        {/key}
    {/snippet}
    <IconKey width="28px" />
</EntityHeader>

<div>
    <div class="flex">
        <Tabs {tabs} bind:selected />
    </div>

    {#if selected === ta.tabs.config}
        <ApiKeyConfig bind:key {onSave} />
    {:else if selected === secretTab}
        <ApiKeySecret {key} />
    {/if}
</div>
