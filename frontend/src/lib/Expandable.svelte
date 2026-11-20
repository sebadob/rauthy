<script lang="ts">
    import IconChevronRight from '#icons/IconChevronRight.svelte';
    import { genKey } from '#utils/helpers.ts';
    import { useI18n } from '#state/i18n.svelte.ts';
    import Button from '#lib/button/Button.svelte';
    import { slide } from 'svelte/transition';

    import type { Snippet } from 'svelte';

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

<div class="container admin-expandable" aria-expanded={expand}>
    <div class="header">
        <Button
            ariaLabel={t.common.expandContent}
            ariaControls={idDetails}
            ariaExpanded={expand}
            invisible
            onclick={ev => toggle(ev)}
        >
            <div class="chevron">
                <IconChevronRight />
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
            <div transition:slide={{ duration: 150 }}>
                {@render details()}
            </div>
        {/if}
    </div>
</div>

<style>
    .chevron {
        margin-bottom: -0.7rem;
        padding: 0 0.2rem;
        color: hsl(var(--action));
        transition: all 150ms ease-in-out;
    }

    .chevron:hover {
        transform: scale(1.05);
        color: hsla(var(--action) / 0.9);
    }

    .container {
        display: flex;
        flex-direction: column;
        transition: border-left 150ms ease-in-out;
        border-left: 1px solid hsl(var(--bg-high));
    }

    .container[aria-expanded='true'] {
        border-left: 1px solid hsl(var(--accent));
    }

    .container[aria-expanded='true']:nth-child(even) {
        border-left: 1px solid hsla(var(--accent) / 0.75);
    }

    .container[aria-expanded='true'] .chevron {
        transform: translate(-2px, 2px);
        rotate: 90deg;
    }

    /* necessary for nested <Expandable>s */
    .container[aria-expanded='false'] .chevron {
        rotate: 0deg;
    }

    .container:nth-child(odd) {
        background: hsla(var(--bg-high) / 0.2);
    }

    .header {
        display: flex;
        align-items: center;
    }

    .summary {
        margin-right: 0.5rem;
        padding-top: 0.25rem;
        display: inline-flex;
        align-items: center;
    }

    .details {
        transition: padding 150ms ease-in-out;
    }

    .container[aria-expanded='true'] .details {
        padding: 0.5rem;
    }

    :global(.admin-shell) .admin-expandable {
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-surface);
        overflow: hidden;
    }
    :global(.admin-shell) .admin-expandable > :global(.header) {
        padding: 14px 16px;
        gap: 12px;
    }
    :global(.admin-shell) .admin-expandable :global(.chevron) {
        margin: 0;
        padding: 0;
        display: flex;
        align-items: center;
    }
    :global(.admin-shell) .admin-expandable[aria-expanded='true'] {
        border-color: hsl(var(--action) / 0.4);
    }
    :global(.admin-shell) .admin-expandable[aria-expanded='true'] :global(.chevron) {
        transform: none;
        rotate: 90deg;
    }
    :global(.admin-shell) .admin-expandable :global(.summary) {
        flex: 1;
        min-width: 0;
        padding: 0;
        margin: 0;
        display: block;
    }
    :global(.admin-shell) .admin-expandable[aria-expanded='true'] > :global(.details) {
        padding: 20px 24px;
        border-top: 1px solid var(--ui-outline);
    }

    :global(.admin-shell)
        .admin-expandable[aria-expanded='true']
        > :global(.header)
        > :global(.admin-control)
        :global(.chevron) {
        rotate: 90deg;
    }
    :global(.admin-shell)
        .admin-expandable[aria-expanded='false']
        > :global(.header)
        > :global(.admin-control)
        :global(.chevron) {
        rotate: 0deg;
    }
</style>
