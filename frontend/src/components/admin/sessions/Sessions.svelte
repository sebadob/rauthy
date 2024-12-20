<script>
    import {onMount} from "svelte";
    import {
        deleteAllSessions,
        getSessions,
        getSessionsSsp,
        getUsers,
        getUsersSsp
    } from "../../../utils/dataFetchingAdmin.js";
    import CheckIcon from "$lib/CheckIcon.svelte";
    import {formatDateFromTs, redirectToLogin} from "../../../utils/helpers.js";
    import Loading from "$lib/Loading.svelte";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import Button from "$lib/Button.svelte";
    import Pagination from "$lib/Pagination.svelte";
    import PaginationServer from "$lib/PaginationServer.svelte";
    import SessionRow from "./SessionRow.svelte";

    let err = $state('');
    let sessions = $state([]);
    let resSessions = $state([]);
    let resSessionsPaginated = $state([]);
    let deletingSessions = false;
    // let deletingSessions = false;
    let now = $state(Date.now() / 1000);

    let useServerSideIdx = $state('');
    let isSearchFiltered = $state(false);

    let sspPageSize = $state(15);
    let sspContinuationToken = $state('');
    let sspPage = $state(1);

    let searchOptions = [
        {
            label: 'User ID',
            callback: (item, search) => item.user_id?.toLowerCase().includes(search.toLowerCase()),
        },
        {
            label: 'Session ID',
            callback: (item, search) => item.id.toLowerCase().includes(search.toLowerCase()),
        },
        {
            label: 'IP',
            callback: (item, search) => item.remote_ip?.toLowerCase().includes(search.toLowerCase()),
        },
    ];
    let orderOptions = [
        {label: 'Expires', callback: (a, b) => a.exp - b.exp},
        {label: 'Last Seen', callback: (a, b) => a.last_seen - b.last_seen},
        {label: 'Session ID', callback: (a, b) => a.id.localeCompare(b.id)},
        {label: 'User ID', callback: (a, b) => a.user_id?.localeCompare(b.user_id)},
        {label: 'State', callback: (a, b) => a.state.localeCompare(b.state)},
        {label: 'IP', callback: (a, b) => a.remote_ip?.localeCompare(b.remote_ip)},
    ];

    onMount(() => {
        fetchSessions();
    });

    async function fetchSessions(useSsp, offset, backwards, pageSize) {
        let res;
        if (useSsp === true) {
            if (backwards && sspPage === 2) {
                // In this case, just do a normal fetch of the very first entry to have a clean start again
                res = await getSessionsSsp(pageSize || sspPageSize, offset, undefined, false);
            } else {
                res = await getSessionsSsp(pageSize || sspPageSize, offset, sspContinuationToken, backwards);
            }
        } else {
            res = await getSessions();
        }
        if (!res.ok) {
            err = 'Error fetching sessions: ' + res.body.message;
        } else {
            const isSsp = res.status === 206;
            if (isSsp) {
                // we get a few headers during SSP we can use for the navigation
                sspPageSize = Number.parseInt(res.headers.get('x-page-size'), 10);
                sspContinuationToken = res.headers.get('x-continuation-token');
                // sspPageCount = res.headers.get('x-page-count');
                useServerSideIdx = 'session';
            } else {
                useServerSideIdx = '';
            }

            let s = await res.json();
            sessions = [...s];
            resSessions = [...s];

            now = Date.now() / 1000;
        }
    }

    // Callback function for <PaginationServer>
    // Fetches the next page during server side pagination with the given offset and direction.
    async function fetchSessionsSsp(offset, backwards) {
        await fetchSessions(true, offset, backwards);
        if (backwards) {
            sspPage -= 1;
        } else {
            sspPage += 1;
        }
    }

    // Callback function for <PaginationServer> to make page size switches work
    async function sspPageSizeChange(pageSize) {
        sspContinuationToken = '';
        await fetchSessions(true, 0, false, pageSize);
        sspPage = 1;
    }

    async function invalidateSessions() {
        deletingSessions = true;
        let res = await deleteAllSessions();
        if (res.ok) {
            redirectToLogin();
        } else {
            let body = await res.json();
            err = body.message;
        }

        now = new Date();
        deletingSessions = false;
    }
</script>

<div class="content">
    <div class="row">
        <OrderSearchBar
                items={sessions}
                bind:resItems={resSessions}
                searchOptions={searchOptions}
                orderOptions={orderOptions}
                firstDirReverse
                bind:useServerSideIdx
                bind:isSearchFiltered
        />
        <div class="button" style:margin-top="-10px">
            <Button on:click={invalidateSessions} level={3}>
                Invalidate All Sessions
            </Button>
        </div>
    </div>

    {err}

    <div id="sessions">
        {#if useServerSideIdx && !isSearchFiltered}
            {#each sessions as session, i (session.id)}
                <div>
                    <SessionRow bind:session={sessions[i]} bind:now/>
                </div>
            {/each}
        {:else}
            {#each resSessionsPaginated as session, i (session.id)}
                <div>
                    <SessionRow bind:session={resSessionsPaginated[i]} bind:now/>
                </div>
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
        <Pagination bind:items={resSessions} bind:resItems={resSessionsPaginated}/>
    {/if}
</div>

<style>
    #sessions div:nth-of-type(2n + 1) {
        background: linear-gradient(90deg, var(--col-ghigh) 35rem, var(--col-bg) 50rem);
    }

    .button {
        margin: 10px 0 0 25px;
    }

    .row {
        display: flex;
        flex-wrap: wrap;
        align-items: center;
    }
</style>
