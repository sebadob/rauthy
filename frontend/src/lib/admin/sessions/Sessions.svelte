<script lang="ts">
    import { onMount, untrack } from 'svelte';
    import { redirectToLogin } from '$utils/helpers';
    import Button from '$lib5/button/Button.svelte';
    import Pagination from '$lib5/pagination/Pagination.svelte';
    import SessionRow from './SessionRow.svelte';
    import type { SessionResponse } from '$api/types/session.ts';
    import ContentAdmin from '$lib5/ContentAdmin.svelte';
    import OrderSearchBar from '$lib5/search_bar/OrderSearchBar.svelte';
    import { fetchDelete, fetchGet } from '$api/fetch';
    import { useI18nAdmin } from '$state/i18n_admin.svelte';
    import { useSession } from '$state/session.svelte';
    import PaginationServerSide from '$lib5/pagination/PaginationServerSide.svelte';
    import { PAGE_SIZE_DEFAULT, type PageSize } from '$lib5/pagination/props';
    import { fetchSearchServer, type SearchParamsIdxSession } from '$utils/search';
    import { useTrigger } from '$state/callback.svelte';

    let ta = useI18nAdmin();

    let refOpts: undefined | HTMLButtonElement = $state();
    let tr = useTrigger();
    tr.set('navMain', () => refOpts?.focus());

    let err = $state('');
    let sessions: SessionResponse[] = $state([]);
    let sessionsFiltered: SessionResponse[] = $state([]);
    let sessionsPaginated: SessionResponse[] = $state([]);
    let now = $state(Date.now() / 1000);

    let useServerSide = $state(false);
    let firstFetchHeaders: undefined | Headers = $state();
    let sspPageSize: PageSize = $state(PAGE_SIZE_DEFAULT);
    let isSearchedServer = $state(false);

    let searchOptions = ['User ID', 'Session ID', 'IP'];
    let searchOption = $state(searchOptions[0]);
    let searchValue = $state('');
    let orderOptions = [
        ta.options.expires,
        ta.options.lastSeen,
        'Session ID',
        'User ID',
        ta.options.state,
        'IP',
    ];

    onMount(() => {
        fetchSessions('page_size=' + sspPageSize);
    });

    $effect(() => {
        let search = searchValue.toLowerCase();
        if (useServerSide) {
            if (search.length < 3) {
                if (isSearchedServer) {
                    fetchSessions('page_size=' + sspPageSize);
                    isSearchedServer = false;
                }
            } else {
                searchServer(search);
            }
        } else {
            if (!search) {
                sessionsFiltered = sessions;
            } else if (searchOption === searchOptions[0]) {
                sessionsFiltered = sessions.filter(s => s.user_id?.toLowerCase().includes(search));
            } else if (searchOption === searchOptions[1]) {
                sessionsFiltered = sessions.filter(s => s.id.toLowerCase().includes(search));
            } else if (searchOption === searchOptions[2]) {
                sessionsFiltered = sessions.filter(s =>
                    s.remote_ip?.toLowerCase().includes(search),
                );
            }
        }
    });

    async function searchServer(q: string) {
        firstFetchHeaders = undefined;
        isSearchedServer = true;

        let idx: SearchParamsIdxSession;
        if (searchOption === searchOptions[0]) {
            idx = 'userid';
        } else if (searchOption === searchOptions[1]) {
            idx = 'sessionid';
        } else {
            idx = 'ip';
        }

        let res = await fetchSearchServer<SessionResponse[]>({
            ty: 'session',
            idx,
            q,
        });
        if (res.body) {
            sessions = res.body;
        } else {
            console.error(res.error);
        }
    }

    async function fetchSessions(urlParams?: string): Promise<[number, Headers]> {
        let url = '/auth/v1/sessions';
        if (urlParams) {
            url += `?${urlParams}`;
        }

        let res = await fetchGet<SessionResponse[]>(url);
        if (res.error) {
            err = 'Error fetching sessions: ' + res.error.message;
        } else if (res.body) {
            // HTTP 206 --> Backend is using SSP
            if (res.status === 206) {
                useServerSide = true;
                firstFetchHeaders = res.headers;
            } else {
                useServerSide = false;
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
            sessions.sort((a, b) => (up ? a.exp - b.exp : b.exp - a.exp));
        } else if (option === orderOptions[1]) {
            sessions.sort((a, b) => (up ? a.last_seen - b.last_seen : b.last_seen - a.last_seen));
        } else if (option === orderOptions[2]) {
            sessions.sort((a, b) => (up ? a.id.localeCompare(b.id) : b.id.localeCompare(a.id)));
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
            sessions.sort((a, b) =>
                up ? a.state.localeCompare(b.state) : b.state.localeCompare(a.state),
            );
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
            bind:ref={refOpts}
            bind:value={searchValue}
            {searchOptions}
            bind:searchOption
            {orderOptions}
            {onChangeOrder}
            searchWidth="min(25rem, calc(100dvw - 1rem))"
            firstDirReverse
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
                <SessionRow {session} {now} {onDeleted} />
            {/each}
        {:else}
            {#each sessionsPaginated as session (session.id)}
                <SessionRow {session} {now} {onDeleted} />
            {/each}
        {/if}
    </div>

    {#if firstFetchHeaders}
        <PaginationServerSide
            bind:pageSize={sspPageSize}
            sspFetch={fetchSessions}
            itemsLength={sessions.length}
            {firstFetchHeaders}
        />
    {:else if useServerSide}
        <Pagination bind:items={sessions} bind:itemsPaginated={sessionsPaginated} />
    {:else}
        <Pagination bind:items={sessionsFiltered} bind:itemsPaginated={sessionsPaginated} />
    {/if}
</ContentAdmin>

<style>
    #sessions {
        max-height: calc(100dvh - 7rem);
        overflow-y: auto;
    }

    .btn {
        margin: 0 0.25rem;
    }

    .top {
        margin-bottom: 0.5rem;
        display: flex;
        gap: 0.5rem 1rem;
        flex-wrap: wrap;
    }
</style>
