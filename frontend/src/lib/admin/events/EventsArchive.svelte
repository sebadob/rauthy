<script lang="ts">
    import {EVENT_LEVELS, EVENT_TYPES} from "$utils/constants.ts";
    import Event from "$lib5/admin/events/Event.svelte";
    import type {EventLevel, EventResponse, EventsRequest, EventType} from "$api/types/events.ts";
    import {fetchPost} from "$api/fetch.ts";
    import Options from "$lib5/Options.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import InputDateTimeCombo from "$lib5/form/InputDateTimeCombo.svelte";
    import {fmtDateInput, fmtTimeInput, unixTsFromLocalDateTime} from "$utils/form.ts";
    import OrderSearchBar from "$lib5/search_bar/OrderSearchBar.svelte";
    import ContentAdmin from "$lib5/ContentAdmin.svelte";

    let ta = useI18nAdmin();

    let events: EventResponse[] = $state([]);
    let eventsFiltered: EventResponse[] = $state([]);

    let fromDate = $state(fmtDateInput());
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
            eventsFiltered = events.filter(e => e.typ?.toLowerCase().includes(search)
                || e.text?.toLowerCase().includes(search)
            );
        }
    });

    async function fetchData() {
        let from = unixTsFromLocalDateTime(fromDate, fromTime);
        if (!from) {
            console.error('from ts invalid', from);
            return;
        }
        let until: undefined | number;
        if (untilDate && untilTime) {
            until = unixTsFromLocalDateTime(untilDate, untilTime);
        }

        let payload: EventsRequest = {
            from,
            until,
            level,
            typ: typ !== EVENT_TYPES[0] ? typ : undefined,
        };
        let res = await fetchPost<EventResponse[]>('/auth/v1/events', payload);
        if (res.body) {
            events = res.body;
        } else {
            console.error(res.error);
        }
    }

    function onChangeOrder(option: string, direction: 'up' | 'down') {
        let up = direction === 'up';
        if (option === orderOptions[0]) {
            events.sort((a, b) => up ? a.timestamp - b.timestamp : b.timestamp - a.timestamp);
        } else if (option === orderOptions[1]) {
            events.sort((a, b) => up ? a.level.localeCompare(b.level) : b.level.localeCompare(a.level));
        } else if (option === orderOptions[2]) {
            events.sort((a, b) => up ? a.typ.localeCompare(b.typ) : b.typ.localeCompare(a.typ));
        }
    }
</script>

<ContentAdmin>
    <div id="archive">
        <OrderSearchBar
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
                <Options
                        ariaLabel={ta.events.eventLevel}
                        options={EVENT_LEVELS}
                        bind:value={level}
                        borderless
                />
            </div>
            <div class="typ">
                <Options
                        ariaLabel={ta.events.eventType}
                        options={EVENT_TYPES}
                        bind:value={typ}
                        borderless
                />
            </div>
        </div>

        {#if eventsFiltered.length === 0}
            <div class="row">
                No events found
            </div>
        {:else}
            {#each eventsFiltered as event (event.id)}
                <Event {event} inline/>
            {/each}
        {/if}
    </div>
</ContentAdmin>

<style>
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
</style>
