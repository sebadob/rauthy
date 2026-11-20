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
    <div class="admin-entity-symbol">{@render children()}</div>
    <div class="admin-entity-title">
        <h2>{title}</h2>
        <p>{subtitle}</p>
    </div>
    {#if actions}
        <div class="actions">{@render actions()}</div>
    {/if}
</header>

<style>
    .admin-entity-header {
        display: flex;
        align-items: center;
        gap: 18px;
        margin-bottom: 28px;
    }
    .admin-entity-symbol {
        display: grid;
        place-items: center;
        width: 64px;
        height: 64px;
        flex-shrink: 0;
        border-radius: var(--border-radius);
        background: var(--ui-tonal);
        color: var(--ui-primary);
    }
    .admin-entity-symbol :global(.avatar),
    .admin-entity-symbol :global(.avatar) > :global(label) {
        width: 64px !important;
        height: 64px;
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
        min-width: 0;
        flex: 1;
    }
    .actions {
        align-self: start;
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
    }
    @media (max-width: 800px) {
        .admin-entity-header {
            gap: 12px;
        }
        .admin-entity-title :global(h2) {
            font-size: 24px;
            line-height: 30px;
        }
    }
</style>
