<script lang="ts">
    import { EVENT_LEVELS, EVENT_TYPES } from '#utils/constants.ts';
    import Event from '#lib/admin/events/Event.svelte';
    import type { EventLevel, EventResponse, EventsRequest, EventType } from '#api/types/events.ts';
    import { fetchPost } from '#api/fetch.ts';
    import Options from '#lib/Options.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import InputDateTimeCombo from '#lib/form/InputDateTimeCombo.svelte';
    import { fmtDateInput, fmtTimeInput, unixTsFromLocalDateTime } from '#utils/form.ts';
    import OrderSearchBar from '#lib/search_bar/OrderSearchBar.svelte';
    import ContentAdmin from '#lib/ContentAdmin.svelte';
    import { useTrigger } from '#state/callback.svelte.ts';

    let ta = useI18nAdmin();

    let refOpts: undefined | HTMLButtonElement = $state();
    let tr = useTrigger();
    tr.set('navMain', () => refOpts?.focus());

    let err = $state('');
    let events: EventResponse[] = $state([]);
    let eventsFiltered: EventResponse[] = $state([]);

    let fromDate = $state(fmtDateInput(new Date(Date.now() - 86400000)));
    let fromTime = $state(fmtTimeInput());
    let untilDate: undefined | string = $state('');
    let untilTime = $state('--:--');
    let level: EventLevel = $state('info');
    let typ: EventType = $state(EVENT_TYPES[0] as EventType);

    let searchOptions = ['IP', 'Content'];
    let searchOption = $state(searchOptions[0]);
    let searchValue = $state('');
    let orderOptions = ['Timestamp', 'Level', 'Type'];

    $effect(() => {
        fetchData();
    });

    $effect(() => {
        let search = searchValue.toLowerCase();
        if (!search) {
            eventsFiltered = events;
        } else if (searchOption === searchOptions[0]) {
            eventsFiltered = events.filter(e => e.ip?.includes(search));
        } else if (searchOption === searchOptions[1]) {
            eventsFiltered = events.filter(
                e =>
                    e.typ?.toLowerCase().includes(search) ||
                    e.typ
                        ?.replace(/([a-z])([A-Z])/g, '$1 $2')
                        .toLowerCase()
                        .includes(search) ||
                    e.text?.toLowerCase().includes(search),
            );
        }
    });

    let latestRequest = 0;

    async function fetchData() {
        const request = ++latestRequest;
        let from = unixTsFromLocalDateTime(fromDate, fromTime);
        // A time field is temporarily incomplete while the user types.
        if (!from) return;
        let until: undefined | number;
        if (untilDate && untilTime) {
            until = unixTsFromLocalDateTime(untilDate, untilTime);
            if (!until) return;
        }

        let payload: EventsRequest = {
            from,
            until,
            level,
            typ: typ !== EVENT_TYPES[0] ? typ : undefined,
        };
        let res = await fetchPost<EventResponse[]>('/auth/v1/events', payload);
        if (request !== latestRequest) return;
        if (res.body) {
            err = '';
            events = res.body;
        } else {
            err = res.error?.message || 'Error';
        }
    }

    function onChangeOrder(option: string, direction: 'up' | 'down') {
        let up = direction === 'up';
        if (option === orderOptions[0]) {
            events.sort((a, b) => (up ? a.timestamp - b.timestamp : b.timestamp - a.timestamp));
        } else if (option === orderOptions[1]) {
            events.sort((a, b) =>
                up ? a.level.localeCompare(b.level) : b.level.localeCompare(a.level),
            );
        } else if (option === orderOptions[2]) {
            events.sort((a, b) => (up ? a.typ.localeCompare(b.typ) : b.typ.localeCompare(a.typ)));
        }
    }
</script>

<ContentAdmin>
    <div id="archive">
        <h2 class="admin-page-title">{ta.nav.events}</h2>
        <div class="admin-event-filters">
            <OrderSearchBar
                bind:ref={refOpts}
                {searchOptions}
                bind:searchOption
                bind:value={searchValue}
                {orderOptions}
                {onChangeOrder}
                firstDirReverse
            />
            <div class="row">
                <InputDateTimeCombo
                    label={ta.common.from}
                    bind:value={fromDate}
                    bind:timeValue={fromTime}
                    withTime
                />
                <InputDateTimeCombo
                    label={ta.common.until}
                    bind:value={untilDate}
                    bind:timeValue={untilTime}
                    withTime
                    withDelete
                />
            </div>
            <div class="filters">
                <div class="level">
                    <span class="admin-field-caption">{ta.events.eventLevel}</span>
                    <Options
                        ariaLabel={ta.events.eventLevel}
                        options={EVENT_LEVELS}
                        bind:value={level}
                        borderless
                    />
                </div>
                <div class="typ">
                    <span class="admin-field-caption">{ta.events.eventType}</span>
                    <Options
                        ariaLabel={ta.events.eventType}
                        options={EVENT_TYPES}
                        bind:value={typ}
                        borderless
                    />
                </div>
            </div>
        </div>
        {#if err}<div class="err">{err}</div>{/if}
        {#if eventsFiltered.length === 0}
            <div class="admin-list-empty">{ta.common.noEntries}</div>
        {:else}
            <div class="eventsList">
                {#each eventsFiltered as event (event.id)}
                    <Event {event} inline />
                {/each}
            </div>
        {/if}
    </div>
</ContentAdmin>

<style>
    .eventsList {
        max-height: calc(100dvh - 11.5rem);
        overflow-y: auto;
    }

    @media (max-width: 39rem) {
        .eventsList {
            max-height: calc(100dvh - 13.5rem);
        }
    }

    @media (max-width: 32rem) {
        .eventsList {
            max-height: calc(100dvh - 17.5rem);
        }
    }

    .filters {
        display: flex;
        align-items: center;
        height: 3rem;
    }

    .level {
        width: 5rem;
    }

    .row {
        display: flex;
        flex-wrap: wrap;
        gap: 0 1rem;
    }

    .typ {
        width: 11rem;
    }

    #archive {
        max-width: 1100px;
    }
    .admin-event-filters {
        display: flex;
        flex-wrap: wrap;
        align-items: flex-end;
        gap: 12px 24px;
        padding: 20px;
        background: var(--ui-container);
        border-radius: var(--border-radius);
        margin-bottom: 24px;
    }
    .admin-event-filters > :global(.admin-list-tools) {
        width: 100%;
        max-width: none;
    }
    .admin-event-filters :global(.row) {
        gap: 16px;
    }
    .admin-event-filters :global(.filters) {
        height: auto;
        gap: 12px;
        flex-wrap: wrap;
    }
    .admin-event-filters :global(.level),
    .admin-event-filters :global(.typ) {
        width: auto;
    }
    .eventsList {
        max-height: none;
        overflow: visible;
        display: flex;
        flex-direction: column;
        gap: 12px;
    }
    @media (max-width: 800px) {
        .admin-event-filters {
            padding: 16px;
        }
        .admin-event-filters :global(.row) {
            width: 100%;
        }
    }

    .admin-event-filters :global(.admin-date-field) {
        margin-bottom: 0;
    }
    .admin-event-filters :global(.inputs-wrapper) > :global(.delete) {
        padding-bottom: 4px;
    }
    .admin-event-filters :global(.filters) {
        align-items: flex-end;
    }
    .admin-event-filters :global(.admin-field-caption) {
        margin-bottom: 6px;
    }
    @media (max-width: 800px) {
        .admin-event-filters :global(.admin-date-field) {
            width: 160px;
        }
        .admin-event-filters :global(.admin-time-field) {
            width: 104px;
        }
    }
</style>
