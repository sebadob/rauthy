<script lang="ts">
    import Button from '#lib/button/Button.svelte';
    import type { Snippet } from 'svelte';

    let {
        badge,
        selected,
        pictureLeft,
        onclick,
        children,
    }: {
        badge?: string;
        selected: boolean;
        pictureLeft?: boolean;
        onclick: () => void;
        children: Snippet;
    } = $props();

    let ref: undefined | HTMLButtonElement = $state();
</script>

<li class="admin-record-row" class:pictureLeft aria-current={selected ? 'page' : 'false'}>
    <div>
        <Button bind:ref invisible {onclick}>
            <div class:withBadge={badge}>
                {#if badge}
                    <span class="badge-label">{@render children()}</span>
                    <span class="record-badge">{badge}</span>
                {:else}
                    {@render children()}
                {/if}
            </div>
        </Button>
    </div>
</li>

<style>
    li {
        border-left: 2px solid transparent;
        border-radius: var(--border-radius);
        margin: 0;
        padding: 0 0.5rem 0.25rem 0.5rem;
        list-style: none;
        transition: all 150ms;
    }

    li div {
        color: hsl(var(--text));
        text-wrap: wrap;
        transition: all 150ms;
    }

    li:nth-of-type(2n + 1) {
        background: hsla(var(--bg-high) / 0.15);
    }

    li[aria-current='page'] {
        border-right: 1px solid hsl(var(--accent));
        background: hsla(var(--bg-high) / 0.8);
    }

    li[aria-current='page'] div {
        color: hsl(var(--text-high));
    }

    li:hover {
        background: hsl(var(--bg-high));
        border-right: 1px solid hsl(var(--action));
    }

    li:hover div {
        color: hsl(var(--action));
    }

    .pictureLeft {
        margin: 0.1rem 0.25rem;
        padding: 0;
        border-radius: 0.65rem var(--border-radius) var(--border-radius) 0.65rem;
    }

    .pictureLeft > div {
        margin-left: -0.1rem;
    }

    .withBadge {
        display: flex;
        justify-content: space-between;
        align-items: center;
        gap: 0.75rem;
        width: 100%;
        min-width: 0;
    }

    .badge-label {
        min-width: 0;
        overflow-wrap: anywhere;
    }

    .record-badge {
        flex: none;
        padding: 0.125rem 0.5rem;
        border-radius: 999px;
        background: var(--ui-container);
        color: var(--ui-muted);
        font-size: 0.6875rem;
        font-weight: 500;
        line-height: 1.25;
    }
</style>
