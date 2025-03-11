<script lang="ts">
    import {onMount} from "svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import ButtonAddModal from "$lib5/button/ButtonAddModal.svelte";
    import ContentAdmin from "$lib5/ContentAdmin.svelte";
    import NavButtonTile from "$lib5/nav/NavButtonTile.svelte";
    import NavSub from "$lib5/nav/NavSub.svelte";
    import OrderSearchBar from "$lib5/search_bar/OrderSearchBar.svelte";
    import {fetchGet} from "$api/fetch.ts";
    import {useParam} from "$state/param.svelte.ts";
    import type {ScopeResponse} from "$api/types/scopes.ts";
    import type {ClientResponse} from "$api/types/clients.ts";
    import ClientAddNew from "$lib5/admin/clients/ClientAddNew.svelte";
    import ClientDetails from "$lib5/admin/clients/ClientDetails.svelte";

    let ta = useI18nAdmin();

    let closeModal: undefined | (() => void) = $state();

    let err = $state('');
    let clients: ClientResponse[] = $state([]);
    let clientsFiltered: ClientResponse[] = $state([]);
    let client: undefined | ClientResponse = $state();
    let cid = useParam('cid');
    let scopesAll: string[] = $state([]);

    const searchOptions = ['ID'];
    let searchOption = $state(searchOptions[0]);
    let searchValue = $state('');
    const orderOptions = ['ID'];

    onMount(() => {
        fetchClients();
        fetchScopes();
    });

    $effect(() => {
        client = clients.find(c => c.id === cid.get());
    });

    $effect(() => {
        let search = searchValue.toLowerCase();
        if (!search) {
            clientsFiltered = clients;
        } else if (searchOption === searchOptions[0]) {
            clientsFiltered = clients.filter(c => c.id.toLowerCase().includes(search));
        }
    });

    async function fetchClients() {
        let res = await fetchGet<ClientResponse[]>('/auth/v1/clients');
        if (res.body) {
            clients = res.body;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    async function fetchScopes() {
        let res = await fetchGet<ScopeResponse[]>('/auth/v1/scopes');
        if (res.body) {
            scopesAll = res.body.map(s => s.name);
        } else {
            err = res.error?.message || 'Error';
        }
    }

    function onChangeOrder(option: string, direction: 'up' | 'down') {
        let up = direction === 'up';
        if (option === orderOptions[0]) {
            clients.sort((a, b) => up ? a.id.localeCompare(b.id) : b.id.localeCompare(a.id));
        }
    }

    function onSave() {
        fetchClients();
    }

    async function onAddNew(id: string) {
        closeModal?.();
        await fetchClients();
        cid.set(id);
    }

</script>

<NavSub
        paddingTop="2.1rem"
        buttonTilesAriaControls="groups"
        width="min(20rem, 100dvw)"
        thresholdNavSub={700}
>
    <ButtonAddModal level={clients.length === 0 ? 1 : 2} bind:closeModal alignRight>
        <ClientAddNew onSave={onAddNew} {clients}/>
    </ButtonAddModal>
    <OrderSearchBar
            {searchOptions}
            bind:searchOption
            bind:value={searchValue}
            {orderOptions}
            {onChangeOrder}
            searchWidth="min(19.5rem, 100dvw - .5rem)"
    />

    {#snippet buttonTiles()}
        <div style:height=".5rem"></div>
        {#each clientsFiltered as client (client.id)}
            <NavButtonTile onclick={() => cid.set(client.id)} selected={cid.get() === client.id}>
                <div class="tile">
                    {client.id}
                    <div class="muted">
                        {client.name}
                    </div>
                </div>
            </NavButtonTile>
        {/each}
    {/snippet}
</NavSub>

<ContentAdmin>
    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}

    <div id="groups">
        {#if client}
            <ClientDetails {client} {clients} {scopesAll} {onSave}/>
        {/if}
    </div>
</ContentAdmin>

<style>
    .muted {
        height: .8rem;
        opacity: .5;
        font-size: .8rem;
    }

    .tile {
        line-height: 1rem;
        text-align: left;
        transform: translateY(.2rem);
    }
</style>
