<script lang="ts">
    import { useI18n } from '#state/i18n.svelte.ts';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import EntityHeader from '#lib/admin/EntityHeader.svelte';
    import IconOffice from '#icons/IconOffice.svelte';
    import Tabs from '#lib/tabs/Tabs.svelte';
    import type { ClientResponse } from '#api/types/clients.ts';
    import ClientConfig from '#lib/admin/clients/ClientConfig.svelte';
    import EntityDelete from '#lib/admin/EntityDelete.svelte';
    import ClientBranding from '#lib/admin/clients/branding/ClientBranding.svelte';

    let {
        client,
        clients,
        scopesAll,
        onSave,
    }: {
        client: ClientResponse;
        clients: ClientResponse[];
        scopesAll: string[];
        onSave: () => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    const tabs = [
        ta.clients.settings.general,
        t.authorize.login,
        ta.clients.settings.tokens,
        ta.clients.settings.integrations,
        'Branding',
    ];
    let selected = $state(tabs[0]);
    let section = $derived(tabs.indexOf(selected));
</script>

<EntityHeader title={client.name || client.id} subtitle={client.id}>
    {#snippet actions()}
        {#key client.id}
            <EntityDelete
                url={`/auth/v1/clients/${client.id}`}
                name={client.id}
                message={ta.clients.delete1}
                onDelete={onSave}
            />
        {/key}
    {/snippet}
    <IconOffice width="1.75rem" />
</EntityHeader>

<div class="flex">
    <Tabs {tabs} bind:selected />
</div>

<div class="details">
    <div hidden={section >= 4}>
        <ClientConfig
            {client}
            {clients}
            {scopesAll}
            {onSave}
            bind:section={() => section, value => (selected = tabs[value])}
        />
    </div>
    {#if selected === 'Branding'}
        <ClientBranding {client} />
    {/if}
</div>
