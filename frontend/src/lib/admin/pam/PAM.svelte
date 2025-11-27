<script lang="ts">
    import { useI18n } from '$state/i18n.svelte';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import ContentAdmin from '$lib5/ContentAdmin.svelte';
    import ButtonAddModal from '$lib5/button/ButtonAddModal.svelte';
    import { useTrigger } from '$state/callback.svelte';
    import NavSub from '$lib/nav/NavSub.svelte';
    import SearchBar from '$lib/search_bar/SearchBar.svelte';
    import Tabs from '$lib/tabs/Tabs.svelte';
    import { fetchGet } from '$api/fetch';
    import type { PamGroupResponse, PamHostSimpleResponse, PamUserResponse } from '$api/types/pam';
    import { onMount } from 'svelte';
    import NavButtonTile from '$lib/nav/NavButtonTile.svelte';
    import PAMGroupDetails from '$lib/admin/pam/groups/PAMGroupDetails.svelte';
    import PamHostAdd from '$lib/admin/pam/hosts/PAMHostAdd.svelte';
    import PAMHostDetails from '$lib/admin/pam/hosts/PAMHostDetails.svelte';
    import PAMGroupAdd from '$lib/admin/pam/groups/PAMGroupAdd.svelte';
    import PAMUserAdd from '$lib/admin/pam/users/PAMUserAdd.svelte';
    import UserPicture from '$lib/UserPicture.svelte';
    import PAMUserDetails from '$lib/admin/pam/users/PAMUserDetails.svelte';
    import type { PamGroupsSorted } from '$lib/admin/pam/types';

    let t = useI18n();
    let ta = useI18nAdmin();

    let refAddNew: undefined | HTMLButtonElement = $state();
    let tr = useTrigger();
    tr.set('navMain', () => refAddNew?.focus());

    let closeModal: undefined | (() => void) = $state();
    let searchValue = $state('');

    const tabs = [ta.nav.users, ta.pam.groups, 'Hosts'];
    let selectedNav = $state(tabs[0]);

    let selectedUser: undefined | PamUserResponse = $state();
    let selectedGroup: undefined | PamGroupResponse = $state();
    let selectedHost: undefined | PamHostSimpleResponse = $state();

    let err = $state('');

    let groups: PamGroupResponse[] = $state([]);
    let groupsFiltered: PamGroupResponse[] = $derived.by(() => {
        let s = searchValue.toLowerCase();
        return groups.filter(g => g.name.toLowerCase().includes(s));
    });
    let groupsSorted: PamGroupsSorted = $derived.by(() => {
        let generic: PamGroupResponse[] = [];
        let host: PamGroupResponse[] = [];
        let local: PamGroupResponse[] = [];
        let user: PamGroupResponse[] = [];

        for (let group of groups) {
            switch (group.typ) {
                case 'generic':
                    generic.push(group);
                    break;
                case 'host':
                    host.push(group);
                    break;
                case 'local':
                    local.push(group);
                    break;
                case 'user':
                    user.push(group);
                    break;
            }
        }

        generic.sort((a, b) => a.name.localeCompare(b.name));
        host.sort((a, b) => a.name.localeCompare(b.name));
        local.sort((a, b) => a.name.localeCompare(b.name));
        user.sort((a, b) => a.name.localeCompare(b.name));

        return { generic, host, local, user };
    });
    let hosts: PamHostSimpleResponse[] = $state([]);
    let hostsFiltered: PamHostSimpleResponse[] = $derived.by(() => {
        let s = searchValue.toLowerCase();
        return hosts.filter(h => h.name.toLowerCase().includes(s));
    });
    let users: PamUserResponse[] = $state([]);
    let usersFiltered: PamUserResponse[] = $derived.by(() => {
        let s = searchValue.toLowerCase();
        return users.filter(u => u.name.toLowerCase().includes(s));
    });

    let isUser = $derived(selectedNav === tabs[0]);
    let isGroup = $derived(selectedNav === tabs[1]);
    let isHost = $derived(selectedNav === tabs[2]);

    onMount(() => {
        fetchGroups();
        fetchHosts();
        fetchUsers();
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

    async function fetchUsers() {
        let res = await fetchGet<PamUserResponse[]>('/auth/v1/pam/users');
        if (res.body) {
            users = res.body;
        } else {
            err = res.error?.message || 'ERROR';
        }
    }

    function onCreateGroup(group: PamGroupResponse) {
        closeModal?.();
        fetchGroups();
        selectedGroup = group;
    }

    function onCreateHost(host: PamHostSimpleResponse) {
        closeModal?.();
        fetchHosts();
        selectedHost = host;
    }

    function onCreateUser(user: PamUserResponse) {
        closeModal?.();
        fetchUsers();
        fetchGroups();
        selectedUser = user;
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
            <PAMUserAdd onCreate={onCreateUser} />
        {:else if isGroup}
            <PAMGroupAdd {groups} onCreate={onCreateGroup} />
        {:else if isHost}
            <PamHostAdd {groups} onCreate={onCreateHost} />
        {/if}
    </ButtonAddModal>

    <div class="tabs">
        <Tabs {tabs} bind:selected={selectedNav} center />
    </div>

    <SearchBar bind:value={searchValue} />

    {#if err}
        <div class="err">{err}</div>
    {/if}

    {#snippet buttonTiles()}
        <div class="navList">
            {#if isUser}
                {#if usersFiltered.length === 0}
                    {ta.common.noEntries}
                {:else}
                    {#each usersFiltered as user (user.id)}
                        <NavButtonTile
                            onclick={() => (selectedUser = user)}
                            selected={user.id === selectedUser?.id}
                            pictureLeft
                        >
                            <div class="navBtn">
                                <div class="picture">
                                    <UserPicture
                                        fallbackCharacters={user.name[0]}
                                        size="small"
                                        disableUpload
                                    />
                                </div>
                                <div class="tile">
                                    <div>
                                        {user.name}
                                    </div>
                                    <div class="muted">
                                        {user.email}
                                    </div>
                                </div>
                            </div>
                        </NavButtonTile>
                    {/each}
                {/if}
            {:else if isGroup}
                {#if groupsFiltered.length === 0}
                    {ta.common.noEntries}
                {:else}
                    {#each groupsFiltered as group (group.id)}
                        <NavButtonTile
                            onclick={() => (selectedGroup = group)}
                            selected={group.id === selectedGroup?.id}
                        >
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
                        <NavButtonTile
                            onclick={() => (selectedHost = host)}
                            selected={host.id === selectedHost?.id}
                        >
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
            <PAMUserDetails user={selectedUser} {groupsSorted} />
        {:else if isGroup && selectedGroup}
            <PAMGroupDetails
                group={selectedGroup}
                onDelete={() => {
                    fetchGroups();
                    selectedGroup = undefined;
                }}
            />
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
    .muted {
        margin-bottom: -0.2rem;
        opacity: 0.65;
        font-size: 0.8rem;
    }

    .navBtn {
        display: flex;
        align-items: center;
        gap: 0.5rem;
    }

    .navList {
        max-height: calc(100dvh - 10.25rem);
        margin-top: 0.5rem;
        overflow-y: auto;
    }

    .tabs {
        margin: 1rem 0 0.5rem 0;
    }

    .tile {
        display: flex;
        flex-direction: column;
        align-items: flex-start;
        line-height: 1rem;
        overflow-x: clip;
    }

    .typ {
        color: hsl(var(--accent));
    }
</style>
