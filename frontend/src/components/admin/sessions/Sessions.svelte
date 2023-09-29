<script>
    import {onMount} from "svelte";
    import {deleteAllSessions, getSessions} from "../../../utils/dataFetchingAdmin.js";
    import CheckIcon from "$lib/CheckIcon.svelte";
    import {formatDateFromTs, redirectToLogin} from "../../../utils/helpers.js";
    import Loading from "$lib/Loading.svelte";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import Button from "$lib/Button.svelte";
    import Pagination from "$lib/Pagination.svelte";

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
            <div class="button" style:margin-top="-10px">
                <Button on:click={invalidateSessions} level={3}>
                    Invalidate All Sessions
                </Button>
            </div>
        </div>

        {#each resSessions as session}
            <div class={session.exp > now ? 'entryRow' : 'entryRow expired'}>
                <div class="row1">
                    <div class="col-sid flex font-mono">
                        <div class="label" style:margin-right=".75rem">SID:</div>
                        {session.id}
                    </div>

                    <div class="col-exp flex">
                        <div class="label">EXP:</div>
                        {formatDateFromTs(session.exp)}
                    </div>

                    <div class="col-seen flex">
                        <div class="label">SEEN:</div>
                        {formatDateFromTs(session.last_seen)}
                    </div>
                </div>

                <div class="row2">
                    <div class="col-uid flex font-mono">
                        <div class="label">USER:</div>
                        {session.user_id}
                    </div>

                    <div class="col-state flex">
                        <div class="label">STATE:</div>
                        {session.state}
                    </div>

                    <div class="col-ip flex">
                        <div class="label">IP:</div>
                        {session.remote_ip}
                    </div>

                    <div class="col-mfa flex">
                        <div class="label">MFA:</div>
                        <CheckIcon check={session.is_mfa}/>
                    </div>
                </div>
            </div>
        {/each}
    </div>
{/if}

<style>
    .button {
        margin: 10px 0 0 25px;
    }

    .col-sid {
        width: 23rem;
    }

    .col-uid {
        width: 23rem;
    }

    .col-state {
        width: 12.25rem;
    }

    .col-mfa {
        width: 4.5rem;
    }

    .col-exp {
        width: 12.25rem;
    }

    .col-seen {
        width: 13rem;
    }

    .col-ip {
        width: 9.75rem;
    }

    .expired {
        background: var(--col-gmid);
    }

    .flex {
        display: flex;
        align-items: center;
    }

    .label {
        margin-right: .25rem;
        font-weight: bold;
        font-size: .9rem;
    }

    .entryRow {
        max-width: 50rem;
        margin-bottom: 1rem;
    }

    .entryRow:hover {
        background: var(--col-acnt);
        color: white
    }

    .row1, .row2 {
        display: inline-flex;
        flex-wrap: wrap;
        flex: 1;
    }

    .row {
        display: flex;
        flex-wrap: wrap;
        align-items: center;
    }
</style>
