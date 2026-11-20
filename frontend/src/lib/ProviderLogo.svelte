<script lang="ts">
    import EntityDelete from '#lib/admin/EntityDelete.svelte';
    import { useI18n } from '#state/i18n.svelte.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';

    let {
        providerId,
        providerName,
        imageUrl,
        onDelete,
    }: {
        providerId: string;
        providerName: string;
        imageUrl: string;
        onDelete: () => void;
    } = $props();

    let exists = $state(false);
    const t = useI18n();
    const ta = useI18nAdmin();
</script>

<div class="container">
    <img
        src={imageUrl}
        width={20}
        height={20}
        alt={ta.clients.branding.logo}
        onload={() => (exists = true)}
        onerror={() => (exists = false)}
    />
    {#if exists}
        <EntityDelete
            url={`/auth/v1/providers/${providerId}/img`}
            name={providerName}
            label={`${t.common.delete} · ${ta.clients.branding.logo}`}
            {onDelete}
            iconOnly
        />
    {/if}
</div>

<style>
    .container {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }
</style>
