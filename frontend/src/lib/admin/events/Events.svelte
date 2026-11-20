<script lang="ts">
    import Event from '#lib/admin/events/Event.svelte';
    import EventsLegend from './EventsLegend.svelte';
    import Button from '#lib/button/Button.svelte';
    import { isBrowser } from '#utils/helpers.ts';
    import type { EventLevel, EventResponse } from '#api/types/events.ts';
    import EventLevelFilter from './EventLevelFilter.svelte';
    import { onDestroy } from 'svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { fetchPost } from '#api/fetch.ts';
    import { useTrigger } from '#state/callback.svelte.ts';
    import { useSession } from '#state/session.svelte.ts';

    const latest = 50;

    let ta = useI18nAdmin();
    let session = useSession('admin');

    // The "Test" event verifies the notification pipeline, which is a full-admin concern
    // (fixing it needs config changes a group admin has no access to), so hide it for them.
    // The principal acts as a group admin when it reached the admin UI without the
    // full `rauthy_admin` role.
    let isGroupAdmin = $derived(!session.isAdmin());

    let innerWidth: undefined | number = $state();
    let wide = $state(false);

    let es: undefined | EventSource = $state();
    let events: EventResponse[] = $state([]);
    let eventsFiltered: EventResponse[] = $state([]);
    function storedLevel(): EventLevel {
        if (isBrowser()) {
            try {
                const value = localStorage.getItem('eventLevel')?.toLowerCase();
                if (value === 'notice' || value === 'warning' || value === 'critical') {
                    return value;
                }
            } catch {
                // Storage may be unavailable in a restricted browser session.
            }
        }
        return 'info';
    }
    let level: EventLevel = $state(storedLevel());
    let levelBefore = '';

    let disposed = false;
    let retryTimer: ReturnType<typeof setTimeout> | undefined;
    onDestroy(() => {
        disposed = true;
        clearTimeout(retryTimer);
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
                    evt =>
                        evt.typ === 'Test' ||
                        evt.level === 'notice' ||
                        evt.level === 'warning' ||
                        evt.level === 'critical',
                );
                break;
            case 'warning':
                eventsFiltered = events.filter(
                    evt =>
                        evt.typ === 'Test' || evt.level === 'warning' || evt.level === 'critical',
                );
                break;
            case 'critical':
                eventsFiltered = events.filter(
                    evt => evt.typ === 'Test' || evt.level === 'critical',
                );
                break;
        }
    });

    async function sendTestEvent() {
        await fetchPost('/auth/v1/events/test');
    }

    function stream() {
        if (disposed) return;
        clearTimeout(retryTimer);
        try {
            localStorage.setItem('eventLevel', level);
        } catch {
            // A storage failure must not stop the live stream.
        }

        if (es && es.readyState !== 2) {
            es.close();
        }

        es = new EventSource(
            `/auth/v1/events/stream?latest=${latest}&level=${level.toLowerCase()}`,
        );

        es.onopen = () => {
            events = [];
        };

        es.onerror = () => {
            // Never reconnect after the optional panel has been closed.
            if (disposed) return;
            clearTimeout(retryTimer);
            retryTimer = setTimeout(stream, 3000);
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

<svelte:window bind:innerWidth />

<div id="events">
    <div class="upper">
        <div class="header">
            <div class="flex gap-10">
                <b>{ta.nav.events}</b>
                <EventLevelFilter bind:level />
            </div>

            {#if !isGroupAdmin}
                <Button ariaLabel="Test Event" level={3} onclick={sendTestEvent}>Test</Button>
            {/if}
        </div>

        <div aria-live="polite" aria-relevant="additions" class="data">
            {#each eventsFiltered as event (event.id)}
                <Event {event} />
            {/each}
        </div>
    </div>

    <EventsLegend {wide} />
</div>

<style>
    #events {
        width: 20rem;
        height: 100%;
        display: flex;
        flex-direction: column;
        border-left: 1px solid var(--ui-outline);
        background: var(--ui-surface);
    }

    .upper {
        display: flex;
        flex-direction: column;
        flex: 1;
        min-height: 0;
    }

    .header {
        display: flex;
        flex-wrap: wrap;
        justify-content: space-between;
        align-items: center;
        gap: 8px;
        min-height: 72px;
        padding: 16px;
        border-bottom: 1px solid var(--ui-outline);
    }

    .header > .flex {
        gap: 8px;
    }

    .data {
        flex: 1;
        min-height: 0;
        overflow-y: auto;
        display: flex;
        flex-direction: column;
        gap: 12px;
        padding: 16px;
        background: hsla(var(--bg-high) / 0.1);
    }

    .data :global(.event) {
        margin: 0;
        padding: 12px;
        border: 1px solid var(--ui-outline);
        border-left: 3px solid var(--event-color);
        border-radius: var(--border-radius);
        font-size: 12px;
        line-height: 20px;
    }
</style>
