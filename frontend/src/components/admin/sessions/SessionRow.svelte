<script>
    import {formatDateFromTs} from "../../../utils/helpers.js";
    import CheckIcon from "$lib/CheckIcon.svelte";

    export let session = {};
    export let now = 0;
</script>

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

<style>
    /*.button {*/
    /*    margin: 10px 0 0 25px;*/
    /*}*/

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
        margin-bottom: .5rem;
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
</style>