<script>
    import {formatDateFromTs} from "../../../utils/helpers.js";
    import {onMount} from "svelte";

    export let event;
    export let eventColor = () => {
    };
    let cls = 'event';

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

</script>

<div class={cls} style:border-left={`.33rem solid ${eventColor(event.level)}`}>
    {formatDateFromTs(event.timestamp / 1000)}<br/>

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
</div>

<style>
    .event, .eventNew {
        padding: 0 1rem;
        margin-bottom: .33rem;
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
</style>
