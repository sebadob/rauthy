<script lang="ts">
    import { EVENT_TYPES } from '#utils/constants.ts';
    import Event from '#lib/admin/events/Event.svelte';
    import {
        EVENT_LEVELS,
        type EventLevel,
        type EventResponse,
        type EventsRequest,
        type EventType,
    } from '#api/types/events.ts';
    import { fetchPost } from '#api/fetch.ts';
    import Options from '#lib/Options.svelte';
    import EventLevelFilter from './EventLevelFilter.svelte';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import InputDateTimeCombo from '#lib/form/InputDateTimeCombo.svelte';
    import { fmtDateInput, fmtTimeInput, unixTsFromLocalDateTime } from '#utils/form.ts';
    import OrderSearchBar from '#lib/search_bar/OrderSearchBar.svelte';
    import ContentAdmin from '#lib/ContentAdmin.svelte';
    import Section from '#lib/Section.svelte';

    let ta = useI18nAdmin();

    let err = $state('');
    let events: EventResponse[] = $state([]);

    let fromDate = $state(fmtDateInput(new Date(Date.now() - 86400000)));
    let fromTime = $state(fmtTimeInput());
    let untilDate: undefined | string = $state('');
    let untilTime = $state('--:--');
    let levels: EventLevel[] = $state([...EVENT_LEVELS]);
    let minimumLevel = $derived(EVENT_LEVELS.find(level => levels.includes(level)));
    let typ: EventType = $state(EVENT_TYPES[0] as EventType);

    let searchOptions = ['IP', ta.events.searchContent];
    let searchOption = $state(searchOptions[0]);
    let searchValue = $state('');
    let orderOptions = [ta.events.timestamp, ta.events.level, ta.events.type];

    $effect(() => {
        fetchData();
    });

    let eventsFiltered = $derived.by(() => {
        const selected = events.filter(event => levels.includes(event.level));
        const search = searchValue.toLowerCase();
        if (!search) {
            return selected;
        }
        if (searchOption === searchOptions[0]) {
            return selected.filter(e => e.ip?.includes(search));
        }
        return selected.filter(
            e =>
                e.typ?.toLowerCase().includes(search) ||
                e.typ
                    ?.replace(/([a-z])([A-Z])/g, '$1 $2')
                    .toLowerCase()
                    .includes(search) ||
                e.text?.toLowerCase().includes(search),
        );
    });

    let latestRequest = 0;

    async function fetchData() {
        const request = ++latestRequest;
        const level = minimumLevel;
        if (!level) {
            events = [];
            err = '';
            return;
        }
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

<ContentAdmin wide>
    <div id="archive">
        <h2 class="admin-page-title">{ta.nav.events}</h2>
        <div class="filters-panel">
            <Section>
                <OrderSearchBar
                    {searchOptions}
                    bind:searchOption
                    bind:value={searchValue}
                    {orderOptions}
                    {onChangeOrder}
                    firstDirReverse
                    inline
                    searchWidth="100%"
                />
                <div class="row">
                    <InputDateTimeCombo
                        label={ta.common.from}
                        labelTime={ta.common.time}
                        timeName="from_time"
                        bind:value={fromDate}
                        bind:timeValue={fromTime}
                        withTime
                    />
                    <InputDateTimeCombo
                        label={ta.common.until}
                        labelTime={ta.common.time}
                        timeName="until_time"
                        bind:value={untilDate}
                        bind:timeValue={untilTime}
                        withTime
                        withDelete
                    />
                </div>
                <div class="filters">
                    <div class="level">
                        <span class="admin-field-label">{ta.events.eventLevel}</span>
                        <EventLevelFilter bind:levels field />
                    </div>
                    <div class="admin-field-select">
                        <span class="admin-field-label">{ta.events.eventType}</span>
                        <Options
                            ariaLabel={ta.events.eventType}
                            options={EVENT_TYPES}
                            bind:value={typ}
                            anchored
                            placement="below"
                        />
                    </div>
                </div>
            </Section>
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
    .filters-panel {
        margin-bottom: var(--ui-space);
        container-type: inline-size;
    }

    .row,
    .filters {
        display: grid;
        gap: var(--ui-space);
        grid-template-columns: minmax(0, 1fr);
        align-items: start;
    }

    .filters {
        grid-template-columns: repeat(2, minmax(0, 1fr));
    }

    @container (width >= 40rem) {
        .row {
            grid-template-columns: repeat(2, minmax(0, 1fr));
        }
        .filters {
            grid-template-columns: repeat(2, minmax(0, 14rem));
        }
    }

    .eventsList {
        display: grid;
        gap: 0.75rem;
        container: archive / inline-size;
    }
</style>
