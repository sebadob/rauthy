<script lang="ts">
    import {onMount} from "svelte";
    import {fetchGet} from "$api/fetch.ts";
    import type {ApiKeyResponse, ApiKeysResponse} from "$api/types/api_keys.ts";
    import ContentAdmin from "$lib5/ContentAdmin.svelte";
    import NavButtonTile from "$lib5/nav/NavButtonTile.svelte";
    import NavSub from "$lib5/nav/NavSub.svelte";
    import ButtonAddModal from "$lib5/button/ButtonAddModal.svelte";
    import {useParam} from "$state/param.svelte.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import OrderSearchBar from "$lib5/search_bar/OrderSearchBar.svelte";

    let ta = useI18nAdmin();

    let closeModal: undefined | (() => void) = $state();

    let err = $state('');
    let keys: ApiKeyResponse[] = $state([]);
    let keysFiltered: ApiKeyResponse[] = $state([]);
    let key: undefined | ApiKeyResponse = $state();

    let kid = useParam('kn');

    const orderOptions = ['Name'];
    const searchOptions = ['Name'];
    let searchOption = $state(searchOptions[0]);
    let searchValue = $state('');

    onMount(() => {
        fetchApiKeys();
    });

    $effect(() => {
        key = keys.find(k => k.name === kid.get())
    });

    $effect(() => {
        let search = searchValue.toLowerCase();
        if (!search) {
            keysFiltered = keys;
        } else if (searchOption === searchOptions[0]) {
            keysFiltered = keys.filter(k => k.name?.includes(search));
        }
    });

    async function fetchApiKeys() {
        let res = await fetchGet<ApiKeysResponse>('/auth/v1/api_keys');
        if (res.body) {
            keys = res.body.keys;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    function onChangeOrder(option: string, direction: 'up' | 'down') {
        let up = direction === 'up';
        if (option === orderOptions[0]) {
            keys.sort((a, b) => up ? a.name.localeCompare(b.name) : b.name.localeCompare(a.name));
        }
    }

</script>

{err}

<NavSub paddingTop="2rem" buttonTilesAriaControls="keys">
    <ButtonAddModal level={keys.length === 0 ? 1 : 2} bind:closeModal alignRight>
        TODOw
    </ButtonAddModal>
    <OrderSearchBar
            {searchOptions}
            bind:searchOption
            bind:value={searchValue}
            {orderOptions}
            {onChangeOrder}
    />

    {#snippet buttonTiles()}
        <div style:height=".5rem"></div>
        {#each keysFiltered as key (key.name)}
            <NavButtonTile onclick={() => kid.set(key.name)} selected={kid.get() === key.name}>
                {key.name}
            </NavButtonTile>
        {/each}
    {/snippet}
</NavSub>

<ContentAdmin>
    <div id="keys" aria-label={ta.common.details}>
        {#if key}
            {key.name}
        {/if}
    </div>
</ContentAdmin>
