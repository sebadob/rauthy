<script lang="ts">
    import {formatUtcTsFromDateInput} from "$utils/helpers";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import {EVENT_LEVELS, EVENT_TYPES} from "$utils/constants";
    import Event from "$lib5/admin/events/Event.svelte";
    import Switch from "$lib5/Switch.svelte";
    import type {EventLevel, EventResponse, EventsRequest, EventType} from "$api/types/events.ts";
    import {fetchPost} from "$api/fetch.ts";
    import Options from "$lib5/Options.svelte";
    import {useI18nAdmin} from "$state/i18n_admin.svelte.ts";
    import InputDateTimeCombo from "$lib5/form/InputDateTimeCombo.svelte";
    import {fmtDateInput, fmtTimeInput, unixTsFromLocalDateTime} from "$utils/form.ts";

    let ta = useI18nAdmin();

    let events: EventResponse[] = $state([]);
    let resEvents: EventResponse[] = $state([]);

    let fromDate = $state(fmtDateInput());
    let fromTime = $state(fmtTimeInput());
    let untilDate: undefined | string = $state('');
    let untilTime = $state('--:--');
    // let untilTime = $state(fmtTimeInput());
    let level: EventLevel = $state('info');
    // let filter = $state(false);
    let typ: EventType = $state(EVENT_TYPES[0] as EventType);

    let searchOptions = [
        {
            label: 'IP',
            callback: (item, search) => item.ip?.includes(search),
        },
        {
            label: 'Content',
            callback: (item, search) => item.text?.toLowerCase()?.includes(search.toLowerCase()),
        },
    ];
    let orderOptions = [
        {label: 'Timestamp', callback: (a, b) => a.timestamp - b.timestamp},
        {label: 'Level', callback: (a, b) => a.level.localeCompare(b.level)},
        {label: 'Type', callback: (a, b) => a.typ.localeCompare(b.typ)},
    ];

    $effect(() => {
        fetchData();
    });

    async function fetchData() {
        let from = unixTsFromLocalDateTime(fromDate, fromTime);
        if (!from) {
            console.error('from ts invalid format', from);
            return;
        }
        let until: undefined | number;
        if (untilDate && untilTime) {
            let u = unixTsFromLocalDateTime(untilDate, untilTime);
            if (!u) {
                console.error('until ts invalid format', from);
                return;
            }
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

    function onSave() {
        fetchData();
    }
</script>

<div class="content">
    <div class="row">
        <OrderSearchBar
                items={events}
                bind:resItems={resEvents}
                searchOptions={searchOptions}
                orderOptions={orderOptions}
                firstDirReverse
        />
    </div>
    <div class="row filter">
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
    <div class="row filterOpts">
        <div class="level">
            <Options
                    ariaLabel={ta.events.eventLevel}
                    options={EVENT_LEVELS}
                    bind:value={level}
                    borderless
            />
        </div>
        <!--        <Switch ariaLabel={ta.common.filter} bind:checked={filter}>-->
        <!--            {ta.common.filter}-->
        <!--        </Switch>-->
        <!--{#if filter}-->
        <Options
                ariaLabel={ta.events.eventType}
                options={EVENT_TYPES}
                bind:value={typ}
                borderless
        />
        <!--{/if}-->
    </div>

    {#if resEvents.length === 0}
        <div class="row">
            No events found
        </div>
    {:else}
        {#each resEvents as event (event.id)}
            <Event {event} inline/>
        {/each}
    {/if}
</div>

<style>
    .filter, .filterOpts {
        margin: -.5rem 0 1rem -.5rem;
    }

    .filterOpts {
        gap: 1rem;
    }

    .level {
        width: 8rem;
    }

    .row {
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        gap: 1rem;
    }
</style>
