<script lang="ts">
    import { onMount } from 'svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import ButtonAddModal from '$lib5/button/ButtonAddModal.svelte';
    import ContentAdmin from '$lib5/ContentAdmin.svelte';
    import NavButtonTile from '$lib5/nav/NavButtonTile.svelte';
    import NavSub from '$lib5/nav/NavSub.svelte';
    import OrderSearchBar from '$lib5/search_bar/OrderSearchBar.svelte';
    import { fetchGet } from '$api/fetch';
    import { useParam } from '$state/param.svelte';
    import type { RoleResponse } from '$api/types/roles.ts';
    import RoleAddNew from '$lib5/admin/roles/RoleAddNew.svelte';
    import RoleDetails from '$lib5/admin/roles/RoleDetails.svelte';
    import { useTrigger } from '$state/callback.svelte';

    let ta = useI18nAdmin();

    let refAddNew: undefined | HTMLButtonElement = $state();
    let tr = useTrigger();
    tr.set('navMain', () => refAddNew?.focus());

    let closeModal: undefined | (() => void) = $state();

    let err = $state('');
    let roles: RoleResponse[] = $state([]);
    let rolesFiltered: RoleResponse[] = $state([]);
    let role: undefined | RoleResponse = $state();
    let rid = useParam('rid');

    const searchOptions = [ta.common.name, 'ID'];
    let searchOption = $state(searchOptions[0]);
    let searchValue = $state('');
    const orderOptions = [ta.common.name, 'ID'];

    onMount(() => {
        fetchRoles();
    });

    $effect(() => {
        role = roles.find(r => r.id === rid.get());
    });

    $effect(() => {
        let search = searchValue.toLowerCase();
        if (!search) {
            rolesFiltered = roles;
        } else if (searchOption === searchOptions[0]) {
            rolesFiltered = roles.filter(r => r.name?.toLowerCase().includes(search));
        } else if (searchOption === searchOptions[1]) {
            rolesFiltered = roles.filter(r => r.id.toLowerCase().includes(search));
        }
    });

    async function fetchRoles() {
        let res = await fetchGet<RoleResponse[]>('/auth/v1/roles');
        if (res.body) {
            roles = res.body;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    function onChangeOrder(option: string, direction: 'up' | 'down') {
        let up = direction === 'up';
        if (option === orderOptions[0]) {
            roles.sort((a, b) =>
                up ? a.name.localeCompare(b.name) : b.name.localeCompare(a.name),
            );
        } else if (option === orderOptions[1]) {
            roles.sort((a, b) => (up ? a.id.localeCompare(b.id) : b.id.localeCompare(a.id)));
        }
    }

    function onSave() {
        fetchRoles();
    }

    async function onAddNew(id: string) {
        closeModal?.();
        await fetchRoles();
        rid.set(id);
    }
</script>

<NavSub
    paddingTop="2.1rem"
    buttonTilesAriaControls="groups"
    width="min(20rem, 100dvw)"
    thresholdNavSub={700}
>
    <ButtonAddModal
        bind:ref={refAddNew}
        level={roles.length === 0 ? 1 : 2}
        bind:closeModal
        alignRight
    >
        <RoleAddNew onSave={onAddNew} {roles} />
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
        <div class="rolesList">
            {#each rolesFiltered as role (role.id)}
                <NavButtonTile onclick={() => rid.set(role.id)} selected={rid.get() === role.id}>
                    {role.name}
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
        {#if role}
            <RoleDetails {role} {roles} {onSave} />
        {/if}
    </div>
</ContentAdmin>

<style>
    .rolesList {
        max-height: calc(100dvh - 9.5rem);
        margin-top: 0.5rem;
        overflow-y: auto;
    }
</style>
