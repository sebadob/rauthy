<script lang="ts">
    import Event from "$lib5/admin/events/Event.svelte";
    import EventsLegend from "./EventsLegend.svelte";
    import Button from "$lib5/Button.svelte";
    import {postTestEvent} from "$utils/dataFetching.js";
    import {EVENT_LEVELS} from "$utils/constants";
    import {isBrowser} from "$utils/helpers.ts";
    import type {EventResponse} from "$api/types/events.ts";
    import Options from "$lib5/Options.svelte";
    import {onDestroy} from "svelte";

    const latest = 50;

    let innerWidth: undefined | number = $state();
    let wide = $state(false);

    let es: undefined | EventSource = $state();
    let events: EventResponse[] = $state([]);
    let eventsFiltered: EventResponse[] = $state([]);
    let level: string = $state(isBrowser() ? localStorage.getItem('eventLevel') || 'Info' : 'Info');
    let levelBefore = '';

    onDestroy(() => {
        es?.close();
    });

    $effect(() => {
        if (innerWidth) {
            if (innerWidth > 1440) {
                wide = true;
            } else {
                wide = false;
            }
        }
    });

    $effect(() => {
        if (level !== levelBefore) {
            levelBefore = level;
            stream();
        }
    });

    $effect(() => {
        if (events) {
            switch (level) {
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
    });

    async function sendTestEvent() {
        await postTestEvent();
    }

    function stream() {
        localStorage.setItem('eventLevel', level)

        if (es?.readyState !== 2) {
            es?.close();
        }

        console.log('opening SSE stream');
        es = new EventSource(`/auth/v1/events/stream?latest=${latest}&level=${level.toLowerCase()}`);

        es.onopen = () => {
            // console.log('SSE Events Stream opened');
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

<svelte:window bind:innerWidth/>

<div
        role="none"
        id="events"
        class:wide
        class:narrow={!wide}
>
    <div class="upper">
        <div class="header">
            <div class="flex gap-10">
                <b>Events</b>
                <Options
                        ariaLabel="Event Level"
                        options={EVENT_LEVELS}
                        bind:value={level}
                        borderless
                />
            </div>

            <Button level={3} onclick={sendTestEvent}>
                Test
            </Button>
        </div>

        <div class="data">
            {#each eventsFiltered as event (event.id)}
                <Event {event}/>
            {/each}
        </div>
    </div>

    <EventsLegend {wide}/>
</div>

<style>
    #events {
        height: 100dvh;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        background: hsla(var(--bg-high) / .2);
        box-shadow: -2px 0 5px hsl(var(--bg-high));
        transition: all 150ms ease-in-out;
    }

    .data {
        max-height: calc(100dvh - 7.5rem);
        overflow-y: auto;
    }

    .header {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 0 .5rem 0 calc(2px + .5rem);
    }

    .narrow {
        width: 15rem;
    }

    .wide {
        width: 25rem;
    }

    .wide .data {
        max-height: calc(100dvh - 4.5rem);
    }
</style>
