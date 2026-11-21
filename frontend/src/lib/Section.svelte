<script lang="ts">
    import type { Snippet } from 'svelte';

    let { title, children, actions }: { title?: string; children: Snippet; actions?: Snippet } =
        $props();
</script>

<section class="section" aria-label={title}>
    {#if title || actions}
        <header>
            {#if title}<h3>{title}</h3>{/if}
            {@render actions?.()}
        </header>
    {/if}
    {@render children()}
</section>

<style>
    .section {
        display: grid;
        align-content: start;
        gap: var(--ui-space);
        min-width: 0;
        padding: var(--ui-section-padding, clamp(1rem, 2.5vw, 1.5rem));
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-surface);
    }

    header {
        display: flex;
        flex-wrap: wrap;
        align-items: center;
        justify-content: space-between;
        gap: 0.5rem;
    }

    h3 {
        margin: 0;
        color: var(--ui-text);
        font-size: 1rem;
        font-weight: 600;
        line-height: 1.5;
    }
</style>
