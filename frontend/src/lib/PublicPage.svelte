<script lang="ts">
    import { setContext, type Snippet } from 'svelte';
    import LangSelector from '#lib/LangSelector.svelte';
    import ThemeSwitch from '#lib/ThemeSwitch.svelte';

    let { children }: { children: Snippet } = $props();

    setContext('rauthy-ui', true);
</script>

<main class="public-page ui-shell">
    <div class="page-column">
        {@render children()}

        <div class="preferences">
            <ThemeSwitch />
            <LangSelector uppercase />
        </div>
    </div>
</main>

<style>
    .public-page {
        display: flex;
        flex-direction: column;
        width: 100%;
        min-width: 0;
        min-height: 100%;
        padding: clamp(0.75rem, 4vw, 2rem);
        background: var(--ui-container);
        overflow-y: auto;
    }

    .page-column {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: var(--ui-space);
        width: 100%;
        margin-block: auto;
    }

    .public-page :global(.public-card) {
        width: min(100%, var(--public-card-width, 26rem));
        padding: clamp(1rem, 5vw, 1.75rem);
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-surface);
        overflow-wrap: anywhere;
    }

    .public-page :global(.public-card) :global(h1),
    .public-page :global(.public-card) :global(h2) {
        color: var(--ui-text);
        font-size: 1.375rem;
        line-height: 1.3;
        font-weight: 650;
    }

    .public-page :global(.public-divider) {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        margin-block: 0.25rem;
        color: var(--ui-muted);
        font-size: var(--ui-control-font-size);
        line-height: var(--ui-control-line-height);
        text-align: center;
    }

    .public-page :global(.public-divider)::before,
    .public-page :global(.public-divider)::after {
        content: '';
        flex: 1;
        border-top: 1px solid var(--ui-outline);
    }

    .preferences {
        display: flex;
        align-items: center;
        gap: 0.375rem;
        --ui-button-height: 40px;
        --ui-button-min-height: 40px;
        --ui-option-height: 38px;
        --ui-option-padding: 0 0.75rem;
    }

    .preferences :global(button.admin-control[role='combobox']),
    .preferences > :global(div:first-child) > :global(button.admin-control) {
        min-width: 40px;
        border-color: var(--ui-outline);
        background: var(--ui-surface);
    }

    .preferences :global(button.admin-control[role='combobox']:hover),
    .preferences > :global(div:first-child) > :global(button.admin-control:hover) {
        background: var(--ui-hover);
    }

    .preferences > :global(div:first-child) > :global(button .icon) {
        color: var(--ui-text);
    }
</style>
