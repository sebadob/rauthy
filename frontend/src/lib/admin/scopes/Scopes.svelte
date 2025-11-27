<script lang="ts">
    import { onMount } from 'svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import OrderSearchBar from '$lib5/search_bar/OrderSearchBar.svelte';
    import { fetchGet } from '$api/fetch';
    import type { UserAttrConfigResponse, UserAttrConfigValueResponse } from '$api/types/user_attrs.ts';
    import type { ScopeResponse } from '$api/types/scopes.ts';
    import ContentAdmin from '$lib5/ContentAdmin.svelte';
    import NavButtonTile from '$lib5/nav/NavButtonTile.svelte';
    import ButtonAddModal from '$lib5/button/ButtonAddModal.svelte';
    import NavSub from '$lib5/nav/NavSub.svelte';
    import { useParam } from '$state/param.svelte';
    import ScopeDetails from '$lib5/admin/scopes/ScopeDetails.svelte';
    import ScopeAddNew from './ScopeAddNew.svelte';
    import { isDefaultScope } from '$utils/helpers';
    import { useTrigger } from '$state/callback.svelte';

    let ta = useI18nAdmin();

    let refAddNew: undefined | HTMLButtonElement = $state();
    let tr = useTrigger();
    tr.set('navMain', () => refAddNew?.focus());

    let closeModal: undefined | (() => void) = $state();

    let err = $state('');

    let sid = useParam('sid');
    let attrs: UserAttrConfigValueResponse[] = $state([]);
    let scopes: ScopeResponse[] = $state([]);
    let scopesFiltered: ScopeResponse[] = $state([]);
    let scope: undefined | ScopeResponse = $state();

    const searchOptions = [ta.common.name, 'ID'];
    let searchOption = $state(searchOptions[0]);
    let searchValue = $state('');
    const orderOptions = [ta.common.name, 'ID'];

    onMount(() => {
        fetchScopes();
        fetchAttr();
    });

    $effect(() => {
        scope = scopes.find(s => s.id === sid.get());
    });

    $effect(() => {
        let search = searchValue.toLowerCase();
        if (!search) {
            scopesFiltered = scopes;
        } else if (searchOption === searchOptions[0]) {
            scopesFiltered = scopes.filter(s => s.name?.toLowerCase().includes(search));
        } else if (searchOption === searchOptions[1]) {
            scopesFiltered = scopes.filter(s => s.id.toLowerCase().includes(search));
        }
    });

    async function fetchAttr() {
        let res = await fetchGet<UserAttrConfigResponse>('/auth/v1/users/attr');
        if (res.body) {
            attrs = res.body.values;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    async function fetchScopes() {
        let res = await fetchGet<ScopeResponse[]>('/auth/v1/scopes');
        if (res.body) {
            scopes = res.body;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    function onChangeOrder(option: string, direction: 'up' | 'down') {
        let up = direction === 'up';
        if (option === orderOptions[0]) {
            scopes.sort((a, b) => (up ? a.name.localeCompare(b.name) : b.name.localeCompare(a.name)));
        } else if (option === orderOptions[1]) {
            scopes.sort((a, b) => (up ? a.id.localeCompare(b.id) : b.id.localeCompare(a.id)));
        }
    }

    function onSave() {
        fetchScopes();
    }

    async function onAddNew(id: string) {
        closeModal?.();
        await fetchScopes();
        sid.set(id);
    }
</script>

<NavSub
    paddingTop="2.1rem"
    buttonTilesAriaControls="scopes"
    width="min(20rem, 100dvw)"
    thresholdNavSub={700}
>
    <ButtonAddModal
        bind:ref={refAddNew}
        level={scopes.length === 0 ? 1 : 2}
        bind:closeModal
        alignRight
    >
        <ScopeAddNew
            onSave={onAddNew}
            {scopes}
        />
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
        <div class="scopesList">
            {#each scopesFiltered as scope (scope.id)}
                <NavButtonTile
                    onclick={() => sid.set(scope.id)}
                    selected={sid.get() === scope.id}
                >
                    {scope.name}
                    {#if isDefaultScope(scope.name)}
                        <span class="default">
                            <i>default</i>
                        </span>
                    {/if}
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

    <div id="scopes">
        {#if scope}
            <ScopeDetails
                {attrs}
                {scope}
                {scopes}
                {onSave}
            />
        {/if}
    </div>
</ContentAdmin>

<style>
    .default {
        font-size: 0.8rem;
        color: hsla(var(--text) / 0.5);
    }

    .scopesList {
        max-height: calc(100dvh - 9.5rem);
        margin-top: 0.5rem;
        overflow-y: auto;
    }
</style>
