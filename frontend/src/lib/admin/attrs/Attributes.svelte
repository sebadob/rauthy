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
    import type {UserAttrConfigResponse, UserAttrConfigValueResponse} from "$api/types/user_attrs.ts";
    import AttrAddNew from "$lib5/admin/attrs/AttrAddNew.svelte";
    import AttrDetails from "$lib5/admin/attrs/AttrDetails.svelte";

    let ta = useI18nAdmin();

    let closeModal: undefined | (() => void) = $state();

    let err = $state('');
    let attrs: UserAttrConfigValueResponse[] = $state([]);
    let attrsFiltered: UserAttrConfigValueResponse[] = $state([]);
    let attr: undefined | UserAttrConfigValueResponse = $state();
    let an = useParam('an');

    const searchOptions = [ta.common.name];
    let searchOption = $state(searchOptions[0]);
    let searchValue = $state('');
    const orderOptions = [ta.common.name];

    onMount(() => {
        fetchAttrs();
    });

    $effect(() => {
        attr = attrs.find(a => a.name === an.get());
    });

    $effect(() => {
        let search = searchValue.toLowerCase();
        if (!search) {
            attrsFiltered = attrs;
        } else if (searchOption === searchOptions[0]) {
            attrsFiltered = attrs.filter(a => a.name?.toLowerCase().includes(search));
        }
    });

    async function fetchAttrs() {
        let res = await fetchGet<UserAttrConfigResponse>('/auth/v1/users/attr');
        if (res.body) {
            attrs = res.body.values;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    function onChangeOrder(option: string, direction: 'up' | 'down') {
        let up = direction === 'up';
        if (option === orderOptions[0]) {
            attrs.sort((a, b) => up ? a.name.localeCompare(b.name) : b.name.localeCompare(a.name));
        }
    }

    function onSave() {
        fetchAttrs();
    }

    async function onAddNew(name: string) {
        closeModal?.();
        await fetchAttrs();
        an.set(name);
    }

</script>

<NavSub
        paddingTop="2.1rem"
        buttonTilesAriaControls="groups"
        width="min(20rem, 100dvw)"
        thresholdNavSub={700}
>
    <ButtonAddModal level={attrs.length === 0 ? 1 : 2} bind:closeModal alignRight>
        <AttrAddNew onSave={onAddNew} {attrs}/>
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
        <div class="attrsList">
            {#each attrsFiltered as attr (attr.name)}
                <NavButtonTile onclick={() => an.set(attr.name)} selected={an.get() === attr.name}>
                    {attr.name}
                </NavButtonTile>
            {/each}
        </div>
    {/snippet}
</NavSub>

<ContentAdmin>
    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}

    <div id="groups">
        {#if attr}
            <AttrDetails {attr} {attrs} {onSave}/>
        {/if}
    </div>
</ContentAdmin>

<style>
    .attrsList {
        max-height: calc(100dvh - 9.5rem);
        margin-top: .5rem;
        overflow-y: auto;
    }
</style>