<script>
    import {onMount} from "svelte";
    import Event from "./Event.svelte";
    import EventsLegend from "./EventsLegend.svelte";

    export let collapsed = true;
    export let wide;

    let latest = 20;
    let es;
    let events = [];

    $: widthDefault = !collapsed && !wide;
    $: widthCollapsed = collapsed && !wide;
    $: widthWide = !collapsed && wide;

    onMount(() => {
        if (!es || es.closed) {
            es = new EventSource(`/auth/v1/events?latest=${latest}`);

            es.onopen = () => {
                console.log('SSE Events Stream opened');
                events = [];
            };

            es.onerror = () => {
                console.error('SSE Events Stream closed');
            };

            es.onmessage = ev => {
                if (ev.data) {
                    let event = JSON.parse(ev.data);
                    events = [event, ...events];
                }
            };
        }
    });

    function eventColor(level) {
        switch (level) {
            case 'test':
                return '#b2b2b2';
            case 'info':
                return '#388c51';
            case 'notice':
                return '#3d5d99';
            case 'warning':
                return '#c29a4f';
            case 'critical':
                return '#993d49';
        }
    }

</script>

<div
        id="events"
        class:widthDefault
        class:widthCollapsed
        class:widthWide
>
    <div class="upper">
        <b>Events</b><br/><br/>

        <div class="data">
            {#each events as event (event.id)}
                <Event bind:event eventColor={eventColor} bind:collapsed bind:wide />
            {/each}
        </div>
    </div>

    {#if !collapsed}
        <EventsLegend eventColor={eventColor} bind:wide />
    {/if}
</div>

<style>
    #events {
        position: absolute;
        right: 0;
        top: 0;
        height: 100dvh;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        box-shadow: -2px 0 5px var(--col-gmid);
        transition: all 250ms ease-in-out;
    }

    .data {
        max-height: calc(100dvh - 10.5rem);
        overflow-y: auto;
    }

    .upper {
    }

    .widthDefault {
        width: var(--width-events);
    }

    .widthCollapsed {
        width: var(--width-events-collapsed);
    }

    .widthWide {
        width: var(--width-events-wide);
    }
</style>
