<script lang="ts">
    import IconChevronRight from "$icons/IconChevronRight.svelte";
    import {genKey} from "$utils/helpers.ts";
    import {useI18n} from "$state/i18n.svelte.ts";
    import Button from "$lib5/Button.svelte";
    import {slide} from "svelte/transition";

    import type {Snippet} from "svelte";

    let t = useI18n();

    const idSummary = genKey();
    const idDetails = genKey();

    let {
        expand = $bindable(false),
        summary,
        details,
    }: {
        expand?: boolean;
        summary: Snippet;
        details: Snippet;
    } = $props();

    function toggle(ev: Event) {
        ev.preventDefault();
        expand = !expand;
    }

</script>

<div class="container" aria-expanded={expand}>
    <div class="header">
        <Button
                ariaLabel={t.common.expandContent}
                ariaControls={idDetails}
                invisible
                onclick={ev => toggle(ev)}
        >
            <div class="chevron">
                <IconChevronRight/>
            </div>
        </Button>

        <div id={idSummary} aria-label={t.common.summary} class="summary">
            {@render summary()}
        </div>
    </div>

    <div
            id={idDetails}
            class="details"
            aria-label={t.common.details}
            aria-labelledby={idSummary}
            aria-hidden={!expand}
    >
        {#if expand}
            <div transition:slide={{duration: 150}}>
                {@render details()}
            </div>
        {/if}
    </div>
</div>

<style>
    .chevron {
        margin-bottom: -.7rem;
        padding: 0 .2rem;
        color: hsl(var(--action));
        transition: all 150ms ease-in-out;
    }

    .chevron:hover {
        transform: scale(1.05);
        color: hsla(var(--action) / .9);
    }

    .container {
        display: flex;
        flex-direction: column;
        transition: border-left 150ms ease-in-out;
        border-left: 1px solid hsl(var(--bg-high));
    }

    .container[aria-expanded="true"] {
        border-left: 1px solid hsl(var(--accent));
    }

    .container[aria-expanded="true"]:nth-child(even) {
        border-left: 1px solid hsla(var(--accent) / .75);
    }

    .container[aria-expanded="true"] .chevron {
        transform: translate(-2px, 2px);
        rotate: 90deg;
    }

    /* necessary for nested <Expandable>s */
    .container[aria-expanded="false"] .chevron {
        rotate: 0deg;
    }

    .container:nth-child(odd) {
        background: hsla(var(--bg-high), .2);
    }

    .header {
        display: flex;
        align-items: center;
    }

    .summary {
        margin-right: .5rem;
        padding-top: .25rem;
        /*margin: .2rem .5rem 0 0;*/
        display: inline-flex;
        align-items: center;
    }

    .details {
        padding: .5rem;
    }
</style>
