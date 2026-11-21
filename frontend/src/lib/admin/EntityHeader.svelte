<script lang="ts">
    import type { Snippet } from 'svelte';
    let {
        title,
        subtitle,
        children,
        actions,
    }: { title: string; subtitle: string; children: Snippet; actions?: Snippet } = $props();
</script>
<header class="admin-entity-header">
    <div class="entity-layout">
        <div class="admin-entity-symbol">{@render children()}</div>
        <div class="admin-entity-title">
            <h2>{title}</h2>
            <p>{subtitle}</p>
        </div>
        {#if actions}
            <div class="actions">{@render actions()}</div>
        {/if}
    </div>
</header>

<style>
    .admin-entity-header {
        container: entity-header / inline-size;
        margin-bottom: 28px;
    }
    .entity-layout {
        --symbol-size: 64px;
        display: grid;
        grid-template-columns: auto minmax(0, 1fr) auto;
        grid-template-areas: 'symbol title actions';
        align-items: center;
        column-gap: 18px;
    }
    .entity-layout:not(:has(.actions > *)) {
        grid-template-columns: auto minmax(0, 1fr);
        grid-template-areas: 'symbol title';
    }
    .admin-entity-symbol {
        display: grid;
        place-items: center;
        grid-area: symbol;
        width: var(--symbol-size);
        height: var(--symbol-size);
        border-radius: var(--border-radius);
        background: var(--ui-tonal);
        color: var(--ui-primary);
    }
    .admin-entity-symbol :global(.avatar),
    .admin-entity-symbol :global(.avatar) > :global(label) {
        width: var(--symbol-size) !important;
        height: var(--symbol-size);
        border-radius: var(--border-radius);
        background: transparent !important;
    }
    .admin-entity-symbol :global(.font-large) {
        top: 0;
        font-family: inherit;
        font-size: 22px;
        font-weight: 600;
        color: var(--ui-primary);
    }
    .admin-entity-title {
        grid-area: title;
        min-width: 0;
    }
    .actions {
        grid-area: actions;
        align-self: start;
        min-width: 0;
        display: flex;
        align-items: center;
        justify-content: flex-end;
        flex-wrap: wrap;
        gap: 0.25rem;
    }
    .actions:empty {
        display: none;
    }
    .admin-entity-title :global(h2) {
        margin: 0 0 4px;
        font-size: 28px;
        font-weight: 600;
        letter-spacing: -0.04em;
        line-height: 34px;
        color: var(--ui-text);
        overflow-wrap: anywhere;
    }
    .admin-entity-title :global(p) {
        margin: 0;
        font-size: 14px;
        line-height: 22px;
        overflow-wrap: anywhere;
    }

    @container entity-header (width < 29rem) {
        .entity-layout {
            --symbol-size: 52px;
            column-gap: 12px;
        }
        .admin-entity-title :global(h2) {
            font-size: 24px;
            line-height: 1.25;
        }
    }

    @container entity-header (width < 23rem) {
        .entity-layout {
            --symbol-size: 44px;
            grid-template-columns: auto minmax(0, 1fr);
            grid-template-areas: 'symbol title' 'actions actions';
            row-gap: 8px;
        }
        .admin-entity-title :global(h2) {
            font-size: 22px;
        }
    }

    @container entity-header (width < 14rem) {
        .entity-layout {
            grid-template-columns: minmax(0, 1fr);
            grid-template-areas: 'symbol' 'title' 'actions';
        }
        .entity-layout:not(:has(.actions > *)) {
            grid-template-columns: minmax(0, 1fr);
            grid-template-areas: 'symbol' 'title';
        }
        .admin-entity-title :global(h2) {
            font-size: 20px;
        }
    }
</style>
