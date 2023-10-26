<script>
    import {onMount} from "svelte";
    import Event from "./Event.svelte";
    import EventsLegend from "./EventsLegend.svelte";
    import Button from "$lib/Button.svelte";
    import {postTestEvent} from "../../../utils/dataFetching.js";
    import OptionSelect from "$lib/OptionSelect.svelte";
    import {EVENT_LEVELS} from "../../../utils/constants.js";

    export let collapsed = true;
    export let wide;

    let latest = 50;
    let es;
    let events = [];
    let eventsFiltered = [];
    let eventLevel;
    let isHover = false;

    $: widthDefault = !collapsed && !wide || isHover;
    $: widthCollapsed = collapsed && !wide && !isHover;
    $: widthWide = !collapsed && wide;

    $: if (events) {
        switch (eventLevel) {
            case 'Info':
                eventsFiltered = events;
                break;
            case 'Notice':
                eventsFiltered = events.filter(
                    evt => evt.typ === 'Test'
                        || evt.level === 'notice'
                        || evt.level === 'warning'
                        || evt.level === 'critical'
                );
                break;
            case 'Warning':
                eventsFiltered = events.filter(
                    evt => evt.typ === 'Test' || evt.level === 'warning' || evt.level === 'critical'
                );
                break;
            case 'Critical':
                eventsFiltered = events.filter(evt => evt.typ === 'Test' || evt.level === 'critical');
                break;
        }
    }

    $: if (eventLevel) {
        saveEventLevel(eventLevel);
        if (es && es.readyState !== 2) {
            es.close();
        }
        stream();
    }

    onMount(async () => {
        eventLevel = await readSavedEventLevel();
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

    async function readSavedEventLevel() {
        return localStorage.getItem('eventLevel') || 'Info';
    }

    async function saveEventLevel(level) {
        localStorage.setItem('eventLevel', level);
    }

    async function sendTestEvent() {
        await postTestEvent();
    }

    function stream() {
        console.log('opening SSE stream');
        es = new EventSource(`/auth/v1/events?latest=${latest}&level=${eventLevel?.toLowerCase() || 'info'}`);

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
                // keep max 500 events in the UI to not consume endless amounts of memory
                events = [event, ...events.slice(-499)];
            }
        };
    }

</script>

<div
        role="contentinfo"
        id="events"
        class:widthDefault
        class:widthCollapsed
        class:widthWide
        on:mouseenter={() => isHover = true}
        on:mouseleave={() => isHover = false}
>
    <div class="upper">
        <div class="header">
            {#if !widthCollapsed}
                {#if widthWide}
                    <div class="headerWideInner">
                        <b>Events</b>
                        <OptionSelect
                                bind:value={eventLevel}
                                options={EVENT_LEVELS}
                        />
                    </div>
                {:else}
                    <b>Events</b>
                {/if}

                <Button
                        on:click={sendTestEvent}
                        level=3
                >
                    TEST
                </Button>
            {/if}
        </div>
        {#if !widthWide}
            <div class="opts">
                <OptionSelect
                        bind:value={eventLevel}
                        options={EVENT_LEVELS}
                />
            </div>
        {/if}

        <div class={widthWide ? 'dataWide' : widthCollapsed ? 'dataCollapsed' : 'data'}>
            {#each eventsFiltered as event (event.id)}
                <Event bind:event eventColor={eventColor} collapsed={collapsed && !isHover} bind:wide/>
            {/each}
        </div>
    </div>

    {#if !collapsed || isHover}
        <EventsLegend eventColor={eventColor} bind:wide/>
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

    .data, .dataCollapsed, .dataWide {
        overflow-y: auto;
    }

    .data {
        max-height: calc(100dvh - 10rem);
    }

    .dataCollapsed {
        max-height: 100dvh;
    }

    .dataWide {
        max-height: calc(100dvh - 5.1rem);
    }

    .header, .opts {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0 .5rem 0 1.25rem;
    }

    .headerWideInner {
        display: flex;
        align-items: center;
        gap: 2rem;
    }

    .opts {
        padding: 0 1.25rem;
        width: 100%;
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
