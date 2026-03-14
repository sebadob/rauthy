<script lang="ts">
    import { onMount } from 'svelte';
    import { fetchGet } from '$api/fetch';
    import ContentAdmin from '$lib5/ContentAdmin.svelte';
    import NavSub from '$lib5/nav/NavSub.svelte';
    import { useParam } from '$state/param.svelte';
    import NavButtonTile from '$lib5/nav/NavButtonTile.svelte';
    import ButtonAddModal from '$lib5/button/ButtonAddModal.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import { useTrigger } from '$state/callback.svelte';
    import type { KVNamespaceResponse } from '$api/types/kv';
    import KVDetails from '$lib/admin/kv/KVDetails.svelte';
    import Tabs from '$lib/tabs/Tabs.svelte';
    import KVAccess from '$lib/admin/kv/KVAccess.svelte';
    import KVDelete from '$lib/admin/kv/KVDelete.svelte';

    let ta = useI18nAdmin();

    let refAddNew: undefined | HTMLButtonElement = $state();
    let tr = useTrigger();
    tr.set('navMain', () => refAddNew?.focus());

    let closeModal: undefined | (() => void) = $state();

    let ns = useParam('ns');

    let err = $state('');

    // order: data, access, delete
    let tab = $state(ta.kv.tabs[0]);
    let namespaces: KVNamespaceResponse[] = $state([]);

    onMount(() => {
        fetchData();
    });

    async function fetchData() {
        let res = await fetchGet<KVNamespaceResponse[]>('/auth/v1/kv/ns');
        if (res.body) {
            namespaces = res.body;
            if (!ns.get() && res.body.length > 0) {
                ns.set(res.body[0].name);
            }
        } else {
            err = res.error?.message || 'Error';
        }
    }

    function onSave() {
        ns.set(undefined);
        closeModal?.();
        fetchData();
    }
</script>

<NavSub width="min(16rem, 100dvw - .5rem)" buttonTilesAriaControls="kv" paddingTop="2.35rem">
    <ButtonAddModal
        bind:ref={refAddNew}
        level={namespaces.length === 0 ? 1 : 2}
        bind:closeModal
        alignRight
    >
        TODO
    </ButtonAddModal>

    {#snippet buttonTiles()}
        <div class="nsList">
            {#each namespaces as namespace (namespace.name)}
                <NavButtonTile
                    onclick={() => ns.set(namespace.name)}
                    selected={ns.get() === namespace.name}
                >
                    {namespace.name}
                </NavButtonTile>
            {/each}
        </div>
    {/snippet}
</NavSub>

<ContentAdmin>
    <div id="kv" aria-label={ta.common.details}>
        {#if ns.get() !== undefined}
            <div class="tabs">
                <Tabs tabs={ta.kv.tabs} bind:selected={tab} />
            </div>

            {#if tab === ta.kv.tabs[0]}
                <KVDetails {ns} />
            {:else if tab === ta.kv.tabs[1]}
                <KVAccess {ns} />
            {:else if tab === ta.kv.tabs[2]}
                <KVDelete {ns} {onSave} />
            {/if}
        {/if}
    </div>

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}
</ContentAdmin>

<style>
    #kv {
        max-width: 60rem;
    }

    .nsList {
        max-height: calc(100dvh - 9.5rem);
        margin-top: 0.5rem;
        overflow-y: auto;
    }

    .tabs {
        display: flex;
        margin-bottom: 0.5rem;
    }
</style>
