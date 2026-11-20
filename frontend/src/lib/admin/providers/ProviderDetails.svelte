<script lang="ts">
    import type { ProviderResponse } from '#api/types/auth_provider.ts';
    import ProviderConfig from './ProviderConfig.svelte';
    import ProviderDelete from './ProviderDelete.svelte';
    import EntityHeader from '#lib/admin/EntityHeader.svelte';
    import IconShieldCheck from '#icons/IconShieldCheck.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';

    let {
        provider = $bindable(),
        onSave = $bindable(),
    }: {
        provider: ProviderResponse;
        onSave: () => void;
    } = $props();

    let ta = useI18nAdmin();
</script>

<EntityHeader title={provider.name} subtitle={ta.nav.providers}>
    {#snippet actions()}
        {#key provider.id}
            <ProviderDelete {provider} {onSave} />
        {/key}
    {/snippet}
    <IconShieldCheck width="28px" />
</EntityHeader>

<ProviderConfig bind:provider bind:onSave />
