<script>
    import {onMount} from "svelte";
    import {formatDateFromTs} from "../../../utils/helpers.js";

    let latest = 100;
    let es;
    let events = [];

    onMount(() => {
        if (!es || es.closed) {
            es = new EventSource(`/auth/v1/events?latest=${latest}`);

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
                    // console.log(event);
                    events = [event, ...events];
                }
            };
        }

        console.log(es);
    });

    function eventColor(level) {
        switch (level) {
            case 'test': return '#b2b2b2';
            case 'info': return '#388c51';
            case 'notice': return '#3d5d99';
            case 'warning': return '#c29a4f';
            case 'critical': return '#993d49';
        }
    }

</script>

<div id="events">
    <div class="upper">
        <b>Events</b><br/><br/>

        <div class="data">
            {#each events as event (event.id)}
                <div class="event" style:border-left={`.33rem solid ${eventColor(event.level)}`}>
                    {formatDateFromTs(event.timestamp / 1000)}<br/>
                    {event.typ}<br/>
                    {event.ip}<br/>
                </div>
            {/each}
        </div>
    </div>

    <div class="legend">
        <div class="legendEntry" style:border-left=".33rem solid var(--col-bg)"><b>Legend</b></div>
        <div class="legendEntry" style:border-left={`.33rem solid ${eventColor('test')}`}>TEST</div>
        <div class="legendEntry" style:border-left={`.33rem solid ${eventColor('info')}`}>INFO</div>
        <div class="legendEntry" style:border-left={`.33rem solid ${eventColor('notice')}`}>NOTICE</div>
        <div class="legendEntry" style:border-left={`.33rem solid ${eventColor('warning')}`}>WARNING</div>
        <div class="legendEntry" style:border-left={`.33rem solid ${eventColor('critical')}`}>CRITICAL</div>
    </div>
</div>

<style>
    #events {
        height: 100dvh;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
        box-shadow: -2px 0 5px var(--col-gmid);
    }

    .data {
        max-height: calc(100dvh - 10.5rem);
        overflow-y: auto;
        box-shadow: inset 0 0 3px var(--col-gmid);
    }

    .event {
        padding: 0 1rem;
        margin-bottom: .33rem;
    }

    .legendEntry {
        padding: 0 1rem;
    }
</style>
