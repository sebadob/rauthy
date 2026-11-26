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
    import UnsavedGuard from '#lib/admin/UnsavedGuard.svelte';
    import type { UnsavedField } from '#utils/unsaved.ts';
    import { page } from '$app/state';

    let {
        client,
        clients,
        scopesAll,
        guard = $bindable((run: () => void) => run()),
        onSave,
    }: {
        client: ClientResponse;
        clients: ClientResponse[];
        scopesAll: string[];
        // set by UnsavedGuard; the list wraps its own `cid` changes with it
        guard?: (run: () => void) => void;
        onSave: () => void;
    } = $props();

    let t = useI18n();
    let ta = useI18nAdmin();

    const TAB_BRANDING = ta.clients.branding.title;
    const tabs = [
        ta.clients.settings.general,
        t.authorize.login,
        ta.clients.settings.tokens,
        ta.clients.settings.integrations,
        TAB_BRANDING,
    ];
    let selected = $state(tabs[0]);
    let section = $derived(tabs.indexOf(selected));

    let configFields: UnsavedField[] = $state([]);
    let brandingFields: UnsavedField[] = $state([]);
    let resetConfig: () => void = $state(() => {});
    let resetBranding: () => void = $state(() => {});

    let fields = $derived(section < 4 ? configFields : brandingFields);

    function discardChanges() {
        if (section < 4) {
            resetConfig();
        } else {
            resetBranding();
        }
    }

    // re-selecting this very client keeps the form mounted. compare against the current
    // URL: a tile click already updates the `cid` param before `goto` resolves
    function isSameTarget(to: URL): boolean {
        return (
            to.pathname === page.url.pathname &&
            to.searchParams.get('cid') === page.url.searchParams.get('cid')
        );
    }

    function select(label: string) {
        if (label === selected) {
            return;
        }
        guard(() => (selected = label));
    }
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
    <Tabs {tabs} bind:selected={() => selected, select} />
</div>

<div class="details">
    <div hidden={section >= 4}>
        <ClientConfig
            {client}
            {clients}
            {scopesAll}
            bind:fields={configFields}
            bind:reset={resetConfig}
            {onSave}
            bind:section={() => section, value => (selected = tabs[value])}
        />
    </div>
    {#if selected === TAB_BRANDING}
        <ClientBranding {client} bind:fields={brandingFields} bind:reset={resetBranding} />
    {/if}
</div>

<UnsavedGuard {fields} bind:guard {isSameTarget} onDiscard={discardChanges} />
