<script lang="ts">
    import AdminListHeading from '#lib/admin/AdminListHeading.svelte';
    import { onMount } from 'svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import ButtonAddModal from '#lib/button/ButtonAddModal.svelte';
    import ContentAdmin from '#lib/ContentAdmin.svelte';
    import NavButtonTile from '#lib/nav/NavButtonTile.svelte';
    import NavSub from '#lib/nav/NavSub.svelte';
    import OrderSearchBar from '#lib/search_bar/OrderSearchBar.svelte';
    import { fetchGet } from '#api/fetch.ts';
    import { useParam } from '#state/param.svelte.ts';
    import type { UnsavedField } from '#utils/unsaved.ts';
    import type {
        UserAttrConfigResponse,
        UserAttrConfigValueResponse,
    } from '#api/types/user_attrs.ts';
    import AttrAddNew from '#lib/admin/attrs/AttrAddNew.svelte';
    import AttrDetails from '#lib/admin/attrs/AttrDetails.svelte';
    import UnsavedGuard from '#lib/admin/UnsavedGuard.svelte';
    import { page } from '$app/state';

    let ta = useI18nAdmin();

    let closeModal: undefined | (() => void) = $state();

    let err = $state('');
    let attrs: UserAttrConfigValueResponse[] = $state([]);
    let attrsFiltered: UserAttrConfigValueResponse[] = $state([]);
    let attr: undefined | UserAttrConfigValueResponse = $state();
    let an = useParam('an');
    // set by the guard inside AttrDetails' form; switching an attribute unmounts it
    let guard: (run: () => void) => void = $state(run => run());
    let fields: UnsavedField[] = $state([]);
    let reset: () => void = $state(() => {});

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
            attrs.sort((a, b) =>
                up ? a.name.localeCompare(b.name) : b.name.localeCompare(a.name),
            );
        }
    }

    function onSave() {
        fetchAttrs();
    }

    // search, ordering and re-selecting this very attribute keep the form mounted.
    // compare against the current URL: a tile click already updates the `an` param
    // before `goto` resolves, so the `attr` at that point is the target attribute
    function isSameTarget(to: URL): boolean {
        return (
            to.pathname === page.url.pathname &&
            to.searchParams.get('an') === page.url.searchParams.get('an')
        );
    }

    async function onAddNew(name: string) {
        closeModal?.();
        await fetchAttrs();
        guard(() => an.set(name));
    }
</script>

<NavSub
    detail={!!attr}
    onBack={() => guard(() => an.set(undefined))}
    error={err}
    buttonTilesAriaControls="groups"
>
    <AdminListHeading title={ta.nav.attributes} count={attrsFiltered.length} />
    <ButtonAddModal level={attrs.length === 0 ? 1 : 2} bind:closeModal alignRight>
        <AttrAddNew onSave={onAddNew} {attrs} />
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
                <NavButtonTile
                    onclick={() => guard(() => an.set(attr.name))}
                    selected={an.get() === attr.name}
                >
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
            <AttrDetails {attr} {attrs} bind:fields bind:reset {onSave} />
        {/if}
    </div>
</ContentAdmin>

<UnsavedGuard {fields} bind:guard {isSameTarget} onDiscard={reset} />

<style>
    .attrsList {
        max-height: calc(100dvh - 9.5rem);
        margin-top: 0.5rem;
        overflow-y: auto;
    }
</style>
