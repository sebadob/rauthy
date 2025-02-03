<script>
    import {eventColor} from "$utils/helpers";

    let {wide = $bindable()} = $props();
</script>

<div>
    <div
            class={wide ? ' legendEntryWide' : 'legendEntry'}
            style:border-left=".33rem solid var(--col-bg)"
    >
        <b>Legend</b>
    </div>
    <div class={wide ? 'legendWide' : 'legend'}>
        <div
                class={wide ? ' legendEntryWide' : 'legendEntry'}
                style:border-left={`.33rem solid ${eventColor('test')}`}
        >
            Test
        </div>
        <div
                class={wide ? ' legendEntryWide' : 'legendEntry'}
                style:border-left={`.33rem solid ${eventColor('info')}`}
        >
            Info
        </div>
        <div
                class={wide ? ' legendEntryWide' : 'legendEntry'}
                style:border-left={`.33rem solid ${eventColor('notice')}`}
        >
            Notice
        </div>
        <div
                class={wide ? ' legendEntryWide' : 'legendEntry'}
                style:border-left={`.33rem solid ${eventColor('warning')}`}
        >
            Warning
        </div>
        <div
                class={wide ? ' legendEntryWide' : 'legendEntry'}
                style:border-left={`.33rem solid ${eventColor('critical')}`}
        >
            Critical
        </div>
    </div>
</div>

<style>
    .legend, .legendWide {
        width: 100%;
        line-height: 1rem;
        overflow: clip;
    }

    .legendWide {
        display: flex;
        justify-content: space-between;
    }

    .legendEntry {
        padding: 0 1rem;
    }

    .legendEntryWide {
        padding: 0 .2rem;
    }
</style>
