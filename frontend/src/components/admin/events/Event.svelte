<script>
    import {formatDateFromTs} from "../../../utils/helpers.js";
    import {onMount} from "svelte";

    export let event;
    export let eventColor = () => {
    };
    export let collapsed = true;
    export let wide;

    let cls = 'event';

    $: showDefault = !collapsed && !wide;
    $: showCollapsed = collapsed && !wide;
    $: showWide = !collapsed && wide;

    onMount(() => {
        let now = new Date().getTime();
        // this will only flash the event if it is not older than 30 seconds
        if (now - 1000 * 30 < event.timestamp) {
            setTimeout(() => {
                cls = 'eventNew';
            }, 100)

            setTimeout(() => {
                cls = 'event';
            }, 2500)
        }
    });

    function ts() {
        return formatDateFromTs(event.timestamp / 1000);
    }

</script>

<div class={cls} style:border-left={`.33rem solid ${eventColor(event.level)}`}>
    {#if showWide}
        <div class="row">
            <div class="col-ts">{ts()}</div>

            {#if event.typ === 'NewRauthyAdmin'}
                <div class="col-typ">{event.typ}</div>
                <div class="col-text">{event.text}</div>

            {:else if event.typ === 'InvalidLogins'}
                <div class="col-typ">{`${event.typ}: ${event.data}`}</div>
                <div class="col-ip">{event.ip}</div>

            {:else}
                <div class="col-typ">{event.typ}</div>
                <div class="col-ip">{event.ip}</div>

            {/if}
        </div>
    {:else}
        {ts()}<br/>

        {event.typ}

        {#if event.typ === 'Test'}
            <br/>
            {#if event.text}
                {event.text}
                <br/>
            {/if}
            {event.ip}

        {:else if event.typ === 'InvalidLogins'}
            {`: ${event.data}`}<br/>
            {event.ip}

        {:else if event.typ === 'NewRauthyAdmin'}
            <br/>
            {event.text}

        {:else if event.typ === 'IpBlacklisted'}
            <br/>
            {event.ip}

        {:else if event.typ === 'PossibleBruteForce'}
            <br/>
            {event.ip}

        {:else if event.typ === 'PossibleDoS'}
            <br/>
            {event.ip}

        {/if}
    {/if}
</div>

<style>
    .event, .eventNew {
        width: 100%;
        padding: 0 1rem;
        margin-bottom: .33rem;
        word-break: break-word;
    }

    .event {
        background: var(--col-bg);
        color: var(--col-text);
        transition: all 1s ease-in-out;
    }

    .eventNew {
        background: var(--col-text);
        color: var(--col-bg);
        transition: all 250ms ease-in-out;
    }

    .col-ts {
        width: 10rem;
    }

    .col-typ {
        width: 9rem;
    }

    .col-ip {
        width: 7.5rem;
    }

    .col-text {
    }

    .row {
        display: inline-flex;
        flex-wrap: wrap;
    }
</style>
