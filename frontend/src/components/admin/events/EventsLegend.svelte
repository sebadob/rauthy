<script>
    export let eventColor = () => {
    };
</script>

<div class="legend">
    <div class="legendEntry" style:border-left=".33rem solid var(--col-bg)"><b>Legend</b></div>
    <div class="legendEntry" style:border-left={`.33rem solid ${eventColor('test')}`}>TEST</div>
    <div class="legendEntry" style:border-left={`.33rem solid ${eventColor('info')}`}>INFO</div>
    <div class="legendEntry" style:border-left={`.33rem solid ${eventColor('notice')}`}>NOTICE</div>
    <div class="legendEntry" style:border-left={`.33rem solid ${eventColor('warning')}`}>WARNING</div>
    <div class="legendEntry" style:border-left={`.33rem solid ${eventColor('critical')}`}>CRITICAL</div>
</div>

<style>
    .legend {
        /*box-shadow: 0 -1px 2px var(--col-gmid);*/
    }

    .legendEntry {
        padding: 0 1rem;
    }
</style>
