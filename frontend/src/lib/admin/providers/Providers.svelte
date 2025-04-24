<script lang="ts">
    import {onMount} from "svelte";
    import {fetchPost} from "$api/fetch.ts";
    import type {ProviderResponse} from "$api/types/auth_provider.ts";
    import ContentAdmin from "$lib5/ContentAdmin.svelte";
    import NavSub from "$lib5/nav/NavSub.svelte";
    import {useParam} from "$state/param.svelte.ts";
    import NavButtonTile from "$lib5/nav/NavButtonTile.svelte";
    import ButtonAddModal from "$lib5/button/ButtonAddModal.svelte";
    import ProviderDetails from "$lib5/admin/providers/ProviderDetails.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import ProviderAddNew from "$lib5/admin/providers/ProviderAddNew.svelte";

    let ta = useI18nAdmin();

    let closeModal: undefined | (() => void) = $state();

    let pid = useParam('pid');

    let err = $state('');
    let providers: ProviderResponse[] = $state([]);
    let provider: undefined | ProviderResponse = $state();

    onMount(() => {
        fetchData();
    });

    $effect(() => {
        provider = providers.find(p => p.id === pid.get())
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

<NavSub width="11rem" buttonTilesAriaControls="federation" paddingTop="6.65rem">
    <ButtonAddModal level={providers.length === 0 ? 1 : 2} bind:closeModal>
        <ProviderAddNew {onSave}/>
    </ButtonAddModal>

    {#snippet buttonTiles()}
        <div class="providersList">
            {#each providers as provider (provider.id)}
                <NavButtonTile onclick={() => pid.set(provider.id)} selected={pid.get() === provider.id}>
                    {provider.name}
                </NavButtonTile>
            {/each}
        </div>
    {/snippet}
</NavSub>

<ContentAdmin>
    <div id="federation" aria-label={ta.common.details}>
        {#if provider}
            <ProviderDetails bind:provider onSave={fetchData}/>
        {/if}
    </div>

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
</ContentAdmin>

<style>
    .providersList {
        max-height: calc(100dvh - 9.5rem);
        margin-top: .5rem;
        overflow-y: auto;
    }
</style>