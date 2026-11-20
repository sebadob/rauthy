<script lang="ts">
    import AdminListHeading from '#lib/admin/AdminListHeading.svelte';
    import { onMount, untrack } from 'svelte';
    import { fetchPost } from '#api/fetch.ts';
    import type { ProviderResponse } from '#api/types/auth_provider.ts';
    import ContentAdmin from '#lib/ContentAdmin.svelte';
    import NavSub from '#lib/nav/NavSub.svelte';
    import { useParam } from '#state/param.svelte.ts';
    import NavButtonTile from '#lib/nav/NavButtonTile.svelte';
    import ButtonAddModal from '#lib/button/ButtonAddModal.svelte';
    import ProviderDetails from '#lib/admin/providers/ProviderDetails.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import ProviderAddNew from '#lib/admin/providers/ProviderAddNew.svelte';
    import { useTrigger } from '#state/callback.svelte.ts';
    import Pagination from '#lib/pagination/Pagination.svelte';
    import { PAGE_SIZE_DEFAULT, type PageSize } from '#lib/pagination/props.ts';

    let ta = useI18nAdmin();

    let refAddNew: undefined | HTMLButtonElement = $state();
    let tr = useTrigger();
    tr.set('navMain', () => refAddNew?.focus());

    let closeModal: undefined | (() => void) = $state();

    let pid = useParam('pid');

    let err = $state('');
    let providers: ProviderResponse[] = $state([]);
    let providersPaginated: ProviderResponse[] = $state([]);
    let page = $state(1);
    let pageSize: PageSize = $state(PAGE_SIZE_DEFAULT);
    let provider: undefined | ProviderResponse = $state();

    onMount(() => {
        fetchData();
    });

    $effect(() => {
        const index = providers.findIndex(p => p.id === pid.get());
        provider = providers[index];
        if (index >= 0) page = Math.floor(index / untrack(() => pageSize)) + 1;
    });

    async function fetchData() {
        let res = await fetchPost<ProviderResponse[]>('/auth/v1/providers');
        if (res.body) {
            providers = res.body;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    function onSave() {
        pid.set(undefined);
        closeModal?.();
        fetchData();
    }
</script>

<NavSub buttonTilesAriaControls="federation">
    <AdminListHeading title={ta.nav.providers} count={providers.length} />
    <ButtonAddModal bind:ref={refAddNew} level={providers.length === 0 ? 1 : 2} bind:closeModal>
        <ProviderAddNew {onSave} />
    </ButtonAddModal>

    {#snippet buttonTiles()}
        <div class="providersList">
            {#each providers.length > PAGE_SIZE_DEFAULT ? providersPaginated : providers as provider (provider.id)}
                <NavButtonTile
                    onclick={() => pid.set(provider.id)}
                    selected={pid.get() === provider.id}
                >
                    {provider.name}
                </NavButtonTile>
            {/each}
        </div>
    {/snippet}
    {#snippet listFooter()}
        {#if providers.length > PAGE_SIZE_DEFAULT}
            <Pagination
                bind:items={providers}
                bind:itemsPaginated={providersPaginated}
                bind:page
                bind:pageSize
            />
        {/if}
    {/snippet}
</NavSub>

<ContentAdmin>
    <div id="federation" aria-label={ta.common.details}>
        {#if provider}
            {#key provider.id}
                <ProviderDetails bind:provider onSave={fetchData} />
            {/key}
        {/if}
    </div>

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
</ContentAdmin>

<style>
    #federation {
        max-width: 54rem;
        margin-inline: 0;
    }

    .providersList {
        max-height: calc(100dvh - 9.5rem);
        margin-top: 0.5rem;
        overflow-y: auto;
    }
</style>
