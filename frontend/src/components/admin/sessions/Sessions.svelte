<script>
    import {onMount} from "svelte";
    import {deleteAllSessions, getSessions} from "../../../utils/dataFetchingAdmin.js";
    import CheckIcon from "$lib/CheckIcon.svelte";
    import {formatDateFromTs, redirectToLogin} from "../../../utils/helpers.js";
    import Loading from "$lib/Loading.svelte";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import Button from "$lib/Button.svelte";

    let err = '';
    let sessions = [];
    let resSessions = [];
    let isLoading = true;
    let deletingSessions = false;
    let now = Date.now() / 1000;

    let searchOptions = [
        {
            label: ' User ID',
            callback: (item, search) => item.user_id?.toLowerCase().includes(search.toLowerCase()),
        },
        {
            label: 'Session ID',
            callback: (item, search) => item.id.toLowerCase().includes(search.toLowerCase()),
        },
    ];
    let orderOptions = [
        {label: 'Expires', callback: (a, b) => a.exp - b.exp},
        {label: 'Last Seen', callback: (a, b) => a.last_seen - b.last_seen},
        {label: 'Session ID', callback: (a, b) => a.id.localeCompare(b.id)},
        {label: 'User ID', callback: (a, b) => a.user_id?.localeCompare(b.user_id)},
        {label: 'State', callback: (a, b) => a.state.localeCompare(b.state)},
    ];

    onMount(async () => {
        fetchData();
    });

    async function fetchData() {
        let res = await getSessions();
        let body = await res.json();
        if (res.ok) {
            sessions = body;
        } else {
            err = body.message;
        }

        isLoading = false;
        now = Date.now() / 1000;
    }

    function onSave() {
        fetchData();
    }

    async function invalidateSessions() {
        deletingSessions = true;
        let res = await deleteAllSessions();
        let body = await res.json();
        if (res.ok) {
            redirectToLogin();
        } else {
            err = body.message;
        }

        now = new Date();
        deletingSessions = true;
    }
</script>

{#if isLoading}
    <Loading/>
{:else}
    <div class="content">
        <div class="row">
            <OrderSearchBar
                    items={sessions}
                    bind:resItems={resSessions}
                    searchOptions={searchOptions}
                    orderOptions={orderOptions}
                    firstDirReverse
            />
            <div class="button">
                <Button width={200} on:click={invalidateSessions} level={3}>
                    Invalidate All Sessions
                </Button>
            </div>
        </div>

        <!--  header-->
        <div class="row">
            <div class="c1">
                <b>
                    Session ID
                </b>
            </div>

            <div class="c2">
                <b>
                    User ID
                </b>
            </div>

            <div class="c3">
                <b>
                    State
                </b>
            </div>

            <div class="c4">
                <b>
                    MFA
                </b>
            </div>

            <div class="c5">
                <b>
                    Expires
                </b>
            </div>

            <div class="c6">
                <b>
                    Last Seen
                </b>
            </div>
        </div>

        {#each resSessions as session}
            <div class={session.exp > now ? 'entryRow' : 'entryRow expired'}>
                <div class="c1 font-mono">
                    {session.id}
                </div>

                <div class="c2 font-mono">
                    {session.user_id}
                </div>

                <div class="c3">
                    {session.state}
                </div>

                <div class="c4">
                    <CheckIcon check={session.is_mfa}/>
                </div>

                <div class="c5">
                    {formatDateFromTs(session.exp)}
                </div>

                <div class="c6">
                    {formatDateFromTs(session.last_seen)}
                </div>
            </div>
        {/each}
    </div>
{/if}

<style>
    .button {
        margin: 10px 0 0 25px;
    }

    .c1 {
        width: 325px;
        margin-left: 5px;
    }

    .c2 {
        width: 240px;
    }

    .c3 {
        width: 90px;
    }

    .c4 {
        width: 45px;
    }

    .c5 {
        width: 160px;
    }

    .c6 {
        width: 160px;
    }

    .expired {
        background: var(--col-gmid);
    }

    .row, .entryRow {
        display: flex;
        justify-content: space-between;
        width: 1023px;
        border-bottom: 1px solid var(--col-gmid);
    }

    .entryRow:hover {
        background: var(--col-acnt);
        color: white
    }
</style>
