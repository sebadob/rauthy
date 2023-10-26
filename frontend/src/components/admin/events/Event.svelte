<script>
    import {formatDateFromTs} from "../../../utils/helpers.js";
    import {onMount} from "svelte";

    export let event;
    export let eventColor = () => {
    };
    export let collapsed = true;
    export let wide;

    let cls = 'event';
    let isHover = false;

    $: showDefault = !collapsed && !wide;
    $: showCollapsed = collapsed && !wide && !isHover;
    $: showWide = !collapsed && wide;
    $: borderWidth = showCollapsed ? '.5rem' : '.33rem';

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

    function eventIndex() {
        switch (event.level) {
            case 'test':
                return 'T';
            case 'info':
                return 'I';
            case 'notice':
                return 'N';
            case 'warning':
                return 'W';
            case 'critical':
                return 'C';
            default:
                return 'U';
        }
    }

    function ts() {
        return formatDateFromTs(event.timestamp / 1000);
    }

</script>

<div
        role="contentinfo"
        class={cls}
        class:showCollapsed
        style:border-left={`${borderWidth} solid ${eventColor(event.level)}`}
        on:mouseenter={() => isHover = true}
        on:mouseleave={() => isHover = false}
>
    {#if showWide}
        <div class="row">
            <div class="col-ts">{ts()}</div>

            {#if event.typ === 'Test'}
                <div class="col-typ">{event.typ}</div>
                <div class="col-ip">{event.ip}</div>
                <div class="col-text">{event.text}</div>

            {:else if event.typ === 'InvalidLogins'}
                <div class="col-typ">{`${event.typ}: ${event.data}`}</div>
                <div class="col-ip">{event.ip}</div>

            {:else if event.typ === 'SecretsMigrated'}
                <div class="col-typ">{event.typ}</div>
                <div class="col-ip">{event.ip || ''}</div>

            {:else if event.typ === 'NewRauthyAdmin' || event.typ === 'NewUserRegistered'}
                <div class="col-typ">{event.typ}</div>
                <div class="col-ip">{event.ip || ''}</div>
                <div class="col-text">{@html event.text.replace('@', '<wbr/>@')}</div>

            {:else if event.typ === 'IpBlacklisted'}
                <div class="col-typ">{event.typ}</div>
                <div class="col-ip">{event.ip}</div>
                <div class="col-text">{`Expires: ${formatDateFromTs(event.data)}`}</div>

            {:else if event.typ === 'RauthyStarted'
                || event.typ === 'RauthyHealthy'
                || event.typ === 'RauthyUnhealthy'
            }
                <div class="col-typ">{event.typ}</div>
                <div class="col-ip"></div>
                <div class="col-text">{event.text}</div>

            {:else}
                <div class="col-typ">{event.typ}</div>

            {/if}
        </div>
    {:else if showDefault}
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

        {:else if event.typ === 'SecretsMigrated'}
            {event.ip}

        {:else if event.typ === 'NewRauthyAdmin'
            || event.typ === 'NewUserRegistered'
        }
            <br/>
            {event.ip || ''}
            <br/>
            {@html event.text.replace('@', '<wbr/>@')}

        {:else if event.typ === 'IpBlacklisted'}
            <br/>
            {event.ip}
            <br/>
            {formatDateFromTs(event.data)}

        {:else if event.typ === 'RauthyStarted'
            || event.typ === 'RauthyHealthy'
            || event.typ === 'RauthyUnhealthy'
        }
            <br/>
            {event.text}

        {/if}
    {:else}
        {eventIndex()}
    {/if}
</div>

<style>
    .event, .eventNew {
        width: 100%;
        min-width: 5px;
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

    .showCollapsed {
        padding: 0 .25rem;
    }
</style>
