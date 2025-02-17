<script lang="ts">
    import {onMount} from "svelte";
    import {redirectToLogin} from "$utils/helpers.ts";
    import Button from "$lib5/button/Button.svelte";
    import Pagination from "$lib5/Pagination.svelte";
    import PaginationServer from "$lib/PaginationServer.svelte";
    import SessionRow from "./SessionRow.svelte";
    import type {SessionResponse} from "$api/types/session.ts";
    import ContentAdmin from "$lib5/ContentAdmin.svelte";
    import OrderSearchBar from "$lib5/search_bar/OrderSearchBar.svelte";
    import {fetchDelete, fetchGet} from "$api/fetch.ts";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import {useSession} from "$state/session.svelte.ts";

    let ta = useI18nAdmin();

    let err = $state('');
    let sessions: SessionResponse[] = $state([]);
    let sessionsFiltered: SessionResponse[] = $state([]);
    let sessionsPaginated: SessionResponse[] = $state([]);
    let now = $state(Date.now() / 1000);

    let useServerSideIdx = $state('');
    let isSearchFiltered = $state(false);

    let sspPageSize = $state(15);
    let sspContinuationToken = $state('');
    let sspPage = $state(1);

    let searchOptions = ['User ID', 'Session ID', 'IP'];
    let searchOption = $state(searchOptions[0]);
    let searchValue = $state('');
    let orderOptions = [ta.options.expires, ta.options.lastSeen, 'Session ID', 'User ID', ta.options.state, 'IP'];

    onMount(() => {
        fetchSessions();
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

    async function fetchSessions(useSsp?: boolean, offset?: number, backwards?: boolean, pageSize?: number) {
        let url = '/auth/v1/sessions';
        if (useSsp === true) {
            url += `page_size=${pageSize || sspPageSize}`;
            if (offset) {
                url += `&offset=${offset}`;
            }
            if (backwards) {
                url += `&backwards=${backwards}`;
                if (sspPage !== 2 && sspContinuationToken) {
                    url += `&continuation_token=${sspContinuationToken}`;
                }
            }
        }
        let res = await fetchGet<SessionResponse[]>(url);
        if (res.error) {
            err = 'Error fetching sessions: ' + res.error.message;
        } else if (res.body) {
            // HTTP 206 --> Backend is using SSP
            if (res.status === 206) {
                // we get a few headers during SSP we can use for the navigation
                let xPageSize = res.headers.get('x-page-size');
                if (!xPageSize) {
                    console.error('Did not receive x-page-size with SSP');
                    return;
                }
                sspPageSize = Number.parseInt(xPageSize);
                sspContinuationToken = res.headers.get('x-continuation-token') || '';
                // sspPageCount = res.headers.get('x-page-count');
                useServerSideIdx = 'session';
            } else {
                useServerSideIdx = '';
            }

            sessions = res.body;
            now = Date.now() / 1000;
        }
    }

    // Callback function for <PaginationServer>
    // Fetches the next page during server side pagination with the given offset and direction.
    async function fetchSessionsSsp(offset: number, backwards: boolean) {
        await fetchSessions(true, offset, backwards);
        if (backwards) {
            sspPage -= 1;
        } else {
            sspPage += 1;
        }
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

    // Callback function for <PaginationServer> to make page size switches work
    async function sspPageSizeChange(pageSize: number) {
        sspContinuationToken = '';
        await fetchSessions(true, 0, false, pageSize);
        sspPage = 1;
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
        {#if useServerSideIdx && !isSearchFiltered}
            {#each sessions as session (session.id)}
                <SessionRow {session} {now} {onDeleted}/>
            {/each}
        {:else}
            {#each sessionsPaginated as session (session.id)}
                <SessionRow {session} {now} {onDeleted}/>
            {/each}
        {/if}
    </div>

    <!--
    Even with server side pagination, we must use it client side if we
    have a filtered search result. Otherwise, switching pages would
    overwrite the filtered data.
    -->
    {#if useServerSideIdx && !isSearchFiltered}
        <PaginationServer
                bind:sspPage
                bind:sspPageSize
                bind:sspContinuationToken
                fetchPageCallback={fetchSessionsSsp}
                sspPageSizeChange={sspPageSizeChange}
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
