<script lang="ts">
    import Event from "$lib5/admin/events/Event.svelte";
    import EventsLegend from "./EventsLegend.svelte";
    import Button from "$lib5/button/Button.svelte";
    import {EVENT_LEVELS} from "$utils/constants.ts";
    import {isBrowser} from "$utils/helpers.ts";
    import type {EventLevel, EventResponse} from "$api/types/events.ts";
    import Options from "$lib5/Options.svelte";
    import {onDestroy} from "svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import {fetchPost} from "$api/fetch.ts";

    const latest = 50;

    let ta = useI18nAdmin();

    let innerWidth: undefined | number = $state();
    let wide = $state(false);

    let es: undefined | EventSource = $state();
    let events: EventResponse[] = $state([]);
    let eventsFiltered: EventResponse[] = $state([]);
    let level: EventLevel = $state(
        isBrowser()
            ? localStorage.getItem('eventLevel')?.toLowerCase() as EventLevel || 'info'
            : 'info'
    );
    let levelBefore = '';

    onDestroy(() => {
        es?.close();
    });

    $effect(() => {
        if (innerWidth) {
            wide = innerWidth > 1680;
        }
    });

    $effect(() => {
        if (level !== levelBefore) {
            levelBefore = level;
            stream();
        }
    });

    $effect(() => {
        switch (level) {
            case 'info':
                eventsFiltered = events;
                break;
            case 'notice':
                eventsFiltered = events.filter(
                    evt => evt.typ === 'Test'
                        || evt.level === 'notice'
                        || evt.level === 'warning'
                        || evt.level === 'critical'
                );
                break;
            case 'warning':
                eventsFiltered = events.filter(
                    evt => evt.typ === 'Test' || evt.level === 'warning' || evt.level === 'critical'
                );
                break;
            case 'critical':
                eventsFiltered = events.filter(evt => evt.typ === 'Test' || evt.level === 'critical');
                break;
        }
    });

    async function sendTestEvent() {
        await fetchPost('/auth/v1/events/test');
    }

    function stream() {
        localStorage.setItem('eventLevel', level)

        if (es && es.readyState !== 2) {
            es.close();
        }

        es = new EventSource(`/auth/v1/events/stream?latest=${latest}&level=${level.toLowerCase()}`);

        es.onopen = () => {
            events = [];
        };

        es.onerror = () => {
            console.error('SSE Events Stream closed');
        };

        es.onmessage = ev => {
            if (ev.data) {
                let event: EventResponse = JSON.parse(ev.data);
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
                        ariaLabel={ta.events.eventLevel}
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
        margin-left: .5rem;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        background: hsla(var(--bg-high) / .1);
        border-radius: var(--border-radius) 0 0 var(--border-radius);
        overflow-y: clip;
        transition: all 150ms ease-in-out;
    }

    .data {
        max-height: calc(100dvh - 7.5rem);
        background: hsla(var(--bg-high) / .1);
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
        min-width: 15rem;
    }

    .wide {
        width: 25rem;
        min-width: 25rem;
    }

    .wide .data {
        max-height: calc(100dvh - 4.5rem);
    }
</style>
