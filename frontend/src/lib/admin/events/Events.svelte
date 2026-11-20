<script lang="ts">
    import Event from '#lib/admin/events/Event.svelte';
    import EventsLegend from './EventsLegend.svelte';
    import Button from '#lib/button/Button.svelte';
    import { isBrowser } from '#utils/helpers.ts';
    import { EVENT_LEVELS, type EventLevel, type EventResponse } from '#api/types/events.ts';
    import EventLevelFilter from './EventLevelFilter.svelte';
    import { onDestroy, untrack } from 'svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { fetchPost } from '#api/fetch.ts';
    import { useSession } from '#state/session.svelte.ts';

    // Request the entire recent window so a sparse selection can find matching history.
    const latest = 100;

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
    function storedLevels(): EventLevel[] {
        if (isBrowser()) {
            try {
                const saved = localStorage.getItem('eventLevels');
                if (saved !== null) {
                    const values = saved.split(',');
                    return EVENT_LEVELS.filter(level => values.includes(level));
                }

                // Earlier versions stored a minimum level. Preserve that selection.
                const legacy = EVENT_LEVELS.findIndex(
                    level => level === localStorage.getItem('eventLevel')?.toLowerCase(),
                );
                return EVENT_LEVELS.slice(Math.max(0, legacy));
            } catch {
                // Storage may be unavailable in a restricted browser session.
            }
        }
        return [...EVENT_LEVELS];
    }

    let levels: EventLevel[] = $state(storedLevels());
    let minimumLevel = $derived(EVENT_LEVELS.find(level => levels.includes(level)));
    let eventsFiltered = $derived(events.filter(event => levels.includes(event.level)));
    let testPending = $state(false);
    let testError = $state('');

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
        const level = minimumLevel;
        untrack(() => stream(level));
    });

    $effect(() => {
        try {
            localStorage.setItem('eventLevels', levels.join(','));
        } catch {
            // A storage failure must not stop the live stream.
        }
    });

    async function sendTestEvent() {
        testPending = true;
        testError = '';
        const response = await fetchPost('/auth/v1/events/test');
        testPending = false;
        if (response.error) testError = response.error.message;
    }

    function stream(level: EventLevel | undefined) {
        if (disposed) return;
        clearTimeout(retryTimer);
        es?.close();
        es = undefined;

        if (!level) {
            events = [];
            return;
        }

        es = new EventSource(`/auth/v1/events/stream?latest=${latest}&level=${level}`);

        es.onopen = () => {
            events = [];
        };

        es.onerror = () => {
            // Never reconnect after the optional panel has been closed.
            if (disposed) return;
            clearTimeout(retryTimer);
            retryTimer = setTimeout(() => stream(level), 3000);
        };

        es.onmessage = ev => {
            if (ev.data) {
                let event: EventResponse = JSON.parse(ev.data);
                // keep max 500 events in the UI to not consume endless amounts of memory
                events = [event, ...events.slice(0, 499)];
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
                <EventLevelFilter bind:levels />
            </div>
        </div>

        {#if testError}<p class="err test-error" role="alert">{testError}</p>{/if}

        <div aria-live="polite" aria-relevant="additions" class="data">
            {#each eventsFiltered as event (event.id)}
                <Event {event} />
            {/each}
        </div>
    </div>

    <div class="event-footer">
        <EventsLegend {wide} />
        {#if !isGroupAdmin}
            <Button level={3} isLoading={testPending} onclick={sendTestEvent}>
                {ta.events.sendTest}
            </Button>
        {/if}
    </div>
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

    .event-footer {
        display: flex;
        justify-content: space-between;
        align-items: start;
        border-top: 1px solid var(--ui-outline);
        padding: 0.25rem 0.5rem;
    }

    .header > .flex {
        gap: 8px;
    }

    .test-error {
        padding-inline: var(--ui-space);
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
