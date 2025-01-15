<script>
    import {run} from 'svelte/legacy';

    import {onMount, untrack} from "svelte";
    import {postEvents} from "../../../utils/dataFetchingAdmin.js";
    import {formatDateToDateInput, formatUtcTsFromDateInput} from "../../../utils/helpers";
    import OrderSearchBar from "$lib/search/OrderSearchBar.svelte";
    import Input from "$lib/inputs/Input.svelte";
    import {EVENT_LEVELS, EVENT_TYPES} from "../../../utils/constants.js";
    import OptionSelect from "$lib/OptionSelect.svelte";
    import Event from "./Event.svelte";
    import Switch from "$lib/Switch.svelte";
    import {sleepAwait} from "$lib/utils/helpers";

    let err = '';
    let events = $state([]);
    let resEvents = $state([]);

    let from = $state(formatDateToDateInput(new Date(new Date().getTime() - 3600 * 1000)));
    /** @type {string | undefined} */
    let until = $state(undefined);
    let level = $state('Info');
    let filter = $state(false);
    let typ = $state(EVENT_TYPES[0]);

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
        let data = {
            from: formatUtcTsFromDateInput(from),
            level: level.toLowerCase(),
        };
        if (until) {
            data.until = formatUtcTsFromDateInput(until);
        }
        if (filter) {
            data.typ = typ;
        }

        let res = await postEvents(data);
        let body = await res.json();
        if (res.ok) {
            events = body;
        } else {
            err = body.message;
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
        <Input
                type="datetime-local"
                step="60"
                width="17.5rem"
                bind:value={from}
                max="2099-01-01T00:00"
        >
            FROM
        </Input>
        <Input
                type="datetime-local"
                step="60"
                width="17rem"
                bind:value={until}
                max="2099-01-01T00:00"
        >
            UNTIL
        </Input>
    </div>
    <div class="row filterOpts">
        <div style:margin="0 5.5rem 0 .25rem">
            <OptionSelect
                    bind:value={level}
                    options={EVENT_LEVELS}
            />
        </div>
        Filter
        <Switch bind:selected={filter}/>
        {#if filter}
            <OptionSelect
                    bind:value={typ}
                    options={EVENT_TYPES}
            />
        {/if}
    </div>

    {#if resEvents.length === 0}
        <div class="row">
            No events found
        </div>
    {:else}
        {#each resEvents as event, i (event.id)}
            <Event bind:event={resEvents[i]} collapsed={false} wide/>
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

    .row {
        display: flex;
        flex-wrap: wrap;
        align-items: center;
    }
</style>
