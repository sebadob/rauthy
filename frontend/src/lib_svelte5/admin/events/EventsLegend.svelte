<script lang="ts">
    import {eventColor} from "$utils/helpers.ts";
    import {useI18n} from "$state/i18n.svelte.ts";

    let {wide}: { wide: boolean } = $props();

    let t = useI18n();
</script>

<fieldset>
    <legend class="font-label">{t.common.legend}</legend>
    <ul class:wide>
        <li style:border-color={eventColor('info')}>Info</li>
        <li style:border-color={eventColor('notice')}>Notice</li>
        <li style:border-color={eventColor('warning')}>Warning</li>
        <li style:border-color={eventColor('critical')}>Critical</li>
    </ul>
</fieldset>

<style>
    fieldset {
        line-height: 1rem;
        border: none;
    }

    legend {
        font-size: .9rem;
        color: hsla(var(--text) / .5);
        margin-left: calc(2px + .5rem);
    }

    ul {
        padding-right: .5rem;
        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }

    li {
        padding-left: .5rem;
        list-style: none;
        border-left: 2px solid transparent;
        color: hsla(var(--text) / .7);
    }

    .wide {
        flex-direction: row;
    }
</style>
