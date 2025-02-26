<script lang="ts">
    import {onMount} from "svelte";
    import {redirectToLogin} from "$utils/helpers.ts";
    import Button from "$lib5/button/Button.svelte";
    import Pagination from "$lib5/pagination/Pagination.svelte";
    import SessionRow from "./SessionRow.svelte";
    import type {SessionResponse} from "$api/types/session.ts";
    import ContentAdmin from "$lib5/ContentAdmin.svelte";
    import OrderSearchBar from "$lib5/search_bar/OrderSearchBar.svelte";
    import {fetchDelete, fetchGet} from "$api/fetch.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import {useSession} from "$state/session.svelte.ts";
    import PaginationServerSide from "$lib5/pagination/PaginationServerSide.svelte";
    import {PAGE_SIZE_DEFAULT} from "$lib5/pagination/props.ts";

    let ta = useI18nAdmin();

    let err = $state('');
    let sessions: SessionResponse[] = $state([]);
    let sessionsFiltered: SessionResponse[] = $state([]);
    let sessionsPaginated: SessionResponse[] = $state([]);
    let now = $state(Date.now() / 1000);

    let firstFetchHeaders: undefined | Headers = $state();

    let searchOptions = ['User ID', 'Session ID', 'IP'];
    let searchOption = $state(searchOptions[0]);
    let searchValue = $state('');
    let orderOptions = [ta.options.expires, ta.options.lastSeen, 'Session ID', 'User ID', ta.options.state, 'IP'];

    onMount(() => {
        fetchSessions('page_size=' + PAGE_SIZE_DEFAULT);
    });

    $effect(() => {
        let search = searchValue.toLowerCase();
        if (!search) {
            sessionsFiltered = sessions;
        } else if (searchOption === searchOptions[0]) {
            sessionsFiltered = sessions.filter(s => s.user_id?.toLowerCase().includes(search));
        } else if (searchOption === searchOptions[1]) {
            sessionsFiltered = sessions.filter(s => s.id.toLowerCase().includes(search));
        } else if (searchOption === searchOptions[2]) {
            sessionsFiltered = sessions.filter(s => s.remote_ip?.toLowerCase().includes(search));
        }
    });

    async function fetchSessions(urlParams?: string): Promise<[number, Headers]> {
        let url = '/auth/v1/sessions';
        if (urlParams) {
            url += `?${urlParams}`
        }

        let res = await fetchGet<SessionResponse[]>(url);
        if (res.error) {
            err = 'Error fetching sessions: ' + res.error.message;
        } else if (res.body) {
            // HTTP 206 --> Backend is using SSP
            if (res.status === 206) {
                firstFetchHeaders = res.headers;
            } else {
                firstFetchHeaders = undefined;
            }

            sessions = res.body;
            now = Date.now() / 1000;
        }

        return [res.status, res.headers];
    }

    function onChangeOrder(option: string, direction: 'up' | 'down') {
        let up = direction === 'up';
        if (option === orderOptions[0]) {
            sessions.sort((a, b) => up ? a.exp - b.exp : b.exp - a.exp);
        } else if (option === orderOptions[1]) {
            sessions.sort((a, b) => up ? a.last_seen - b.last_seen : b.last_seen - a.last_seen);
        } else if (option === orderOptions[2]) {
            sessions.sort((a, b) => up ? a.id.localeCompare(b.id) : b.id.localeCompare(a.id));
        } else if (option === orderOptions[3]) {
            sessions.sort((a, b) => {
                if (a.user_id && b.user_id) {
                    if (up) {
                        return a.user_id.localeCompare(b.user_id);
                    } else {
                        return b.user_id.localeCompare(a.user_id);
                    }
                } else if (!a.user_id) {
                    return up ? 1 : -1;
                } else {
                    return up ? -1 : 1;
                }
            });
        } else if (option === orderOptions[4]) {
            sessions.sort((a, b) => up ? a.state.localeCompare(b.state) : b.state.localeCompare(a.state));
        } else if (option === orderOptions[5]) {
            sessions.sort((a, b) => {
                if (a.remote_ip && b.remote_ip) {
                    if (up) {
                        return a.remote_ip.localeCompare(b.remote_ip);
                    } else {
                        return b.remote_ip.localeCompare(a.remote_ip);
                    }
                } else if (!a.remote_ip) {
                    return up ? 1 : -1;
                } else {
                    return up ? -1 : 1;
                }
            });
        }
    }

    function onDeleted(sid: string) {
        sessions = sessions.filter(s => s.id !== sid);
        // reload (trigger new login) if the delete session was ours
        if (useSession('admin').get()?.id === sid) {
            window.location.reload();
        }
    }

    async function invalidateSessions() {
        let res = await fetchDelete('/auth/v1/sessions');
        if (res.error) {
            err = res.error.message;
        } else {
            redirectToLogin();
        }
    }
</script>

<ContentAdmin>
    <div class="top">
        <OrderSearchBar
                bind:value={searchValue}
                {searchOptions}
                bind:searchOption
                {orderOptions}
                {onChangeOrder}
                searchWidth="min(25rem, calc(100dvw - 1rem))"
        />
        <div class="btn">
            <Button level={-1} onclick={invalidateSessions}>
                {ta.sessions.invalidateAll}
            </Button>
        </div>
    </div>

    {#if err}
        <div class="err">
            {err}
        </div>
    {/if}

    <div id="sessions">
        {#if firstFetchHeaders}
            {#each sessions as session (session.id)}
                <SessionRow {session} {now} {onDeleted}/>
            {/each}
        {:else}
            {#each sessionsPaginated as session (session.id)}
                <SessionRow {session} {now} {onDeleted}/>
            {/each}
        {/if}
    </div>

    {#if firstFetchHeaders}
        <PaginationServerSide
                sspFetch={fetchSessions}
                itemsLength={sessions.length}
                {firstFetchHeaders}
        />
    {:else}
        <Pagination
                bind:items={sessionsFiltered}
                bind:itemsPaginated={sessionsPaginated}
        />
    {/if}
</ContentAdmin>

<style>
    .btn {
        margin: 0 .25rem;
    }

    .top {
        margin-bottom: .5rem;
        display: flex;
        gap: .5rem 1rem;
        flex-wrap: wrap;
    }
</style>
