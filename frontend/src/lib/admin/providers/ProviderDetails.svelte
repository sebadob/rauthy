<script lang="ts">
    import type { ProviderResponse } from '#api/types/auth_provider.ts';
    import ProviderConfig from './ProviderConfig.svelte';
    import ProviderDelete from './ProviderDelete.svelte';
    import EntityHeader from '#lib/admin/EntityHeader.svelte';
    import ProviderLogo from '#lib/ProviderLogo.svelte';
    import EntityDelete from '#lib/admin/EntityDelete.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { genKey } from '#utils/helpers.ts';

    let {
        provider = $bindable(),
        onSave,
    }: {
        provider: ProviderResponse;
        onSave: () => void;
    } = $props();

    let ta = useI18nAdmin();
    let t = useI18n();
    let revision = $state(genKey());
    let hasLogo = $state(false);
    let logoError = $state('');

    function refreshLogo() {
        hasLogo = false;
        revision = genKey();
    }
</script>

<EntityHeader title={provider.name} subtitle={ta.nav.providers}>
    {#snippet actions()}
        {#if hasLogo}
            <EntityDelete
                url={`/auth/v1/providers/${provider.id}/img`}
                name={provider.name}
                label={`${t.common.delete} · ${ta.clients.branding.logo}`}
                onDelete={refreshLogo}
                iconOnly
            />
        {/if}
        <ProviderDelete {provider} {onSave} />
    {/snippet}
    <ProviderLogo
        providerId={provider.id}
        {revision}
        bind:exists={hasLogo}
        bind:error={logoError}
        onUpload={refreshLogo}
    />
</EntityHeader>

{#if logoError}<p class="err" role="alert">{logoError}</p>{/if}

<ProviderConfig bind:provider {onSave} />
