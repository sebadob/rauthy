<script lang="ts">
    import {onMount} from "svelte";
    import FederationTileAddNew from "../../../components/admin/providers/ProviderTileAddNew.svelte";
    import {fetchPost} from "$api/fetch.ts";
    import type {ProviderResponse} from "$api/types/auth_provider.ts";
    import ContentAdmin from "$lib5/ContentAdmin.svelte";
    import NavSub from "$lib5/nav/NavSub.svelte";
    import {useParam} from "$state/param.svelte.ts";
    import NavButtonTile from "$lib5/nav/NavButtonTile.svelte";
    import ButtonAddModal from "$lib5/button/ButtonAddModal.svelte";
    import ProviderDetails from "$lib5/admin/providers/ProviderDetails.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";

    let ta = useI18nAdmin();

    let pid = useParam('pid');

    let err = $state('');
    let providers: ProviderResponse[] = $state([]);
    let provider = $derived(providers.find(p => p.id === pid.get()));

    onMount(() => {
        fetchData();
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
        fetchData();
    }

</script>

<NavSub width="11rem" buttonTilesAriaControls="federation">
    <FederationTileAddNew onSave={onSave}/>
    <ButtonAddModal>
        TODO
    </ButtonAddModal>

    {#snippet buttonTiles()}
        {#each providers as provider (provider.id)}
            <NavButtonTile onclick={() => pid.set(provider.id)} selected={pid.get() === provider.id}>
                {provider.name}
            </NavButtonTile>
        {/each}
    {/snippet}
</NavSub>

<ContentAdmin>
    <div id="federation" aria-label={ta.common.details}>
        {#if provider}
            <ProviderDetails {provider} onSave={fetchData}/>
        {/if}
    </div>

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
</ContentAdmin>
