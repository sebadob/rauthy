<script lang="ts">
    import {useI18n} from "$state/i18n.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte";
    import ContentAdmin from "$lib5/ContentAdmin.svelte";
    import ButtonAddModal from "$lib5/button/ButtonAddModal.svelte";
    import {useTrigger} from "$state/callback.svelte";
    import NavSub from "$lib/nav/NavSub.svelte";
    import SearchBar from "$lib/search_bar/SearchBar.svelte";
    import Tabs from "$lib/tabs/Tabs.svelte";
    import {fetchGet} from "$api/fetch";
    import type {PamGroupResponse, PamHostSimpleResponse} from "$api/types/pam";
    import {onMount} from "svelte";
    import NavButtonTile from "$lib/nav/NavButtonTile.svelte";
    import PAMGroupDetails from "$lib/admin/pam/groups/PAMGroupDetails.svelte";
    import PAMAddHost from "$lib/admin/pam/hosts/PAMAddHost.svelte";
    import PAMHostDetails from "$lib/admin/pam/hosts/PAMHostDetails.svelte";


    let t = useI18n();
    let ta = useI18nAdmin();

    let refAddNew: undefined | HTMLButtonElement = $state();
    let tr = useTrigger();
    tr.set('navMain', () => refAddNew?.focus());

    let closeModal: undefined | (() => void) = $state();
    let searchValue = $state('');

    const tabs = ['Users', 'Groups', 'Hosts'];
    let selectedNav = $state(tabs[0]);

    let selectedUser: undefined | PamGroupResponse = $state();
    let selectedGroup: undefined | PamGroupResponse = $state();
    let selectedHost: undefined | PamHostSimpleResponse = $state();

    let err = $state('');

    let groups: PamGroupResponse[] = $state([]);
    let groupsFiltered: PamGroupResponse[] = $derived.by(() => {
        let s = searchValue.toLowerCase();
        return groups.filter(g => g.name.toLowerCase().includes(s));
    });
    let hosts: PamHostSimpleResponse[] = $state([]);
    let hostsFiltered: PamHostSimpleResponse[] = $derived.by(() => {
        let s = searchValue.toLowerCase();
        return hosts.filter(g => g.name.toLowerCase().includes(s));
    });

    let isUser = $derived(selectedNav === tabs[0]);
    let isGroup = $derived(selectedNav === tabs[1]);
    let isHost = $derived(selectedNav === tabs[2]);

    onMount(() => {
        fetchGroups();
        fetchHosts();
    });

    $effect(() => {
        if (isUser || isGroup || isHost) {
            searchValue = '';
        }
    });

    async function fetchGroups() {
        let res = await fetchGet<PamGroupResponse[]>('/auth/v1/pam/groups');
        if (res.body) {
            groups = res.body;
        } else {
            err = res.error?.message || 'ERROR';
        }
    }

    async function fetchHosts() {
        let res = await fetchGet<PamHostSimpleResponse[]>('/auth/v1/pam/hosts');
        if (res.body) {
            hosts = res.body;
        } else {
            err = res.error?.message || 'ERROR';
        }
    }

    function onCreateHost(host: PamHostSimpleResponse) {
        closeModal?.();
        fetchHosts();
        selectedHost = host;
    }
</script>

<NavSub
        paddingTop="2.1rem"
        buttonTilesAriaControls="scopes"
        width="min(20rem, 100dvw)"
        thresholdNavSub={700}
>
    <ButtonAddModal bind:ref={refAddNew} level={2} bind:closeModal alignRight>
        {#if isUser}
            TODO
        {:else if isGroup}
            TODO
        {:else if isHost}
            <PAMAddHost {groups} onCreate={onCreateHost}/>
        {/if}
    </ButtonAddModal>

    <div class="tabs">
        <Tabs {tabs} bind:selected={selectedNav} center/>
    </div>

    <SearchBar bind:value={searchValue}/>

    {#if err}
        <div class="err">{err}</div>
    {/if}

    {#snippet buttonTiles()}
        <div class="navList">
            {#if isUser}
                USERS TODO
            {:else if isGroup}
                {#if groupsFiltered.length === 0}
                    {ta.common.noEntries}
                {:else}
                    {#each groupsFiltered as group (group.id)}
                        <NavButtonTile onclick={() => selectedGroup = group} selected={group.id === selectedGroup?.id}>
                            <span class="typ font-mono">
                                {group.typ[0].toUpperCase()}
                            </span>
                            {group.name}
                        </NavButtonTile>
                    {/each}
                {/if}
            {:else if isHost}
                {#if hostsFiltered.length === 0}
                    {ta.common.noEntries}
                {:else}
                    {#each hostsFiltered as host (host.id)}
                        <NavButtonTile onclick={() => selectedHost = host} selected={host.id === selectedHost?.id}>
                            {host.name}
                        </NavButtonTile>
                    {/each}
                {/if}
            {/if}
        </div>
    {/snippet}
</NavSub>

<ContentAdmin>
    <div id="pam">
        {#if isUser && selectedUser}
            USERS TODO
        {:else if isGroup && selectedGroup}
            <PAMGroupDetails group={selectedGroup}/>
        {:else if isHost && selectedHost}
            <PAMHostDetails
                    hostSimple={selectedHost}
                    {groups}
                    onDelete={() => {
                        fetchHosts();
                        selectedHost = undefined;
                    }}
            />
        {/if}
    </div>
</ContentAdmin>

<style>
    .navList {
        max-height: calc(100dvh - 9.5rem);
        margin-top: .5rem;
        overflow-y: auto;
    }

    .tabs {
        margin: 1rem 0 .5rem 0;
    }

    .typ {
        color: hsl(var(--accent));
    }
</style>
