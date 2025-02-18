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
    import type {Group} from "$api/types/groups.ts";

    let ta = useI18nAdmin();

    let closeModal: undefined | (() => void) = $state();

    let err = $state('');
    let groups: Group[] = $state([]);
    let groupsFiltered: Group[] = $state([]);
    let group: undefined | Group = $state();
    let gid = useParam('gid');

    const searchOptions = [ta.common.name, 'ID'];
    let searchOption = $state(searchOptions[0]);
    let searchValue = $state('');
    const orderOptions = [ta.common.name, 'ID'];
    // let searchOptions = [
    //     {
    //         label: 'Name',
    //         callback: (item, search) => item.name.toLowerCase().includes(search.toLowerCase()),
    //     },
    //     {
    //         label: 'ID',
    //         callback: (item, search) => item.id.toLowerCase().includes(search.toLowerCase()),
    //     },
    // ];
    // let orderOptions = [
    //     {
    //         label: 'Name',
    //         callback: (a, b) => a.name.localeCompare(b.name),
    //     },
    //     {
    //         label: 'ID',
    //         callback: (a, b) => a.id.localeCompare(b.id),
    //     },
    // ];

    onMount(() => {
        fetchGroups();
    });

    $effect(() => {
        group = groups.find(g => g.id === gid.get());
    });

    $effect(() => {
        let search = searchValue.toLowerCase();
        if (!search) {
            groupsFiltered = groups;
        } else if (searchOption === searchOptions[0]) {
            groupsFiltered = groups.filter(g => g.name?.toLowerCase().includes(search));
        } else if (searchOption === searchOptions[1]) {
            groupsFiltered = groups.filter(g => g.id.toLowerCase().includes(search));
        }
    });

    async function fetchGroups() {
        let res = await fetchGet<Group[]>('/auth/v1/groups');
        if (res.body) {
            groups = res.body;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    function onChangeOrder(option: string, direction: 'up' | 'down') {
        let up = direction === 'up';
        if (option === orderOptions[0]) {
            groups.sort((a, b) => up ? a.name.localeCompare(b.name) : b.name.localeCompare(a.name));
        } else if (option === orderOptions[1]) {
            groups.sort((a, b) => up ? a.id.localeCompare(b.id) : b.id.localeCompare(a.id));
        }
    }

    function onSave() {
        fetchGroups();
    }

    async function onAddNew(id: string) {
        closeModal?.();
        await fetchGroups();
        gid.set(id);
    }

</script>

<NavSub
        paddingTop="2.1rem"
        buttonTilesAriaControls="groups"
        width="min(20rem, 100dvw)"
        thresholdNavSub={700}
>
    <ButtonAddModal level={groups.length === 0 ? 1 : 2} bind:closeModal alignRight>
        TODO
        <!--        <ScopeAddNew onSave={onAddNew} {scopes}/>-->
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
        {#each groupsFiltered as group (group.id)}
            <NavButtonTile onclick={() => gid.set(group.id)} selected={gid.get() === group.id}>
                {group.name}
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
        {#if group}
            TODO
            <!--            <ScopeDetails {attrs} {scope} {scopes} {onSave}/>-->
        {/if}
    </div>
</ContentAdmin>
