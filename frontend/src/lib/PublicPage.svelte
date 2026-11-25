<script lang="ts">
    import { setContext, type Snippet } from 'svelte';
    import LangSelector from '#lib/LangSelector.svelte';
    import ThemeSwitch from '#lib/ThemeSwitch.svelte';

    let { children }: { children: Snippet } = $props();

    setContext('rauthy-ui', true);
</script>

<main class="public-page ui-shell ui-canvas">
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
        padding: clamp(0.75rem, 4vw, 2.5rem);
        overflow-y: auto;
    }

    .page-column {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 1.25rem;
        width: 100%;
        margin-block: auto;
    }

    .public-page :global(.public-card) {
        position: relative;
        width: min(100%, var(--public-card-width, 26rem));
        padding: clamp(1.25rem, 6vw, 2.25rem);
        border: 1px solid var(--ui-outline);
        border-radius: var(--card-radius);
        background: var(--ui-surface);
        backdrop-filter: blur(18px) saturate(1.4);
        box-shadow:
            0 1px 0 hsl(var(--bg)) inset,
            0 2px 4px hsl(var(--text) / 0.04),
            0 24px 60px -24px color-mix(in srgb, var(--canvas-a) 45%, hsl(var(--text) / 0.35)),
            0 40px 90px -50px color-mix(in srgb, var(--canvas-b) 50%, transparent);
        overflow-wrap: anywhere;
        animation: cardRise 520ms cubic-bezier(0.2, 0.8, 0.2, 1) both;
    }

    /* the brand colored edge on top of each card */
    .public-page :global(.public-card)::before {
        content: '';
        position: absolute;
        inset-inline: clamp(1.25rem, 6vw, 2.25rem);
        top: -1px;
        height: 2px;
        border-radius: 2px;
        background: linear-gradient(
            90deg,
            transparent,
            var(--canvas-a) 20%,
            var(--canvas-b) 80%,
            transparent
        );
    }

    .public-page :global(.public-card) :global(h1),
    .public-page :global(.public-card) :global(h2) {
        margin-block: 0.25rem 1rem;
        color: var(--ui-text);
        font-size: clamp(1.6rem, 6vw, 2rem);
        line-height: 1.1;
        font-weight: 750;
    }

    .public-page :global(.public-divider) {
        display: flex;
        align-items: center;
        gap: 0.75rem;
        margin-block: 0.5rem;
        color: var(--ui-muted);
        line-height: var(--ui-control-line-height);
        text-align: center;
    }

    .public-page :global(.public-divider)::before,
    .public-page :global(.public-divider)::after {
        content: '';
        flex: 1;
        border-top: 1px dashed var(--ui-outline);
    }

    .preferences {
        display: flex;
        align-items: center;
        gap: 0.25rem;
        padding: 0.25rem;
        border: 1px solid var(--ui-outline);
        border-radius: 999px;
        background: var(--ui-surface);
        backdrop-filter: blur(12px);
        --ui-button-height: 36px;
        --ui-button-min-height: 36px;
        --ui-option-height: 36px;
        --ui-option-padding: 0 0.75rem;
        --field-radius: 999px;
        animation: cardRise 520ms 120ms cubic-bezier(0.2, 0.8, 0.2, 1) both;
    }

    .preferences :global(button.admin-control[role='combobox']),
    .preferences > :global(div:first-child) > :global(button.admin-control) {
        min-width: 36px;
        border-color: transparent;
        border-radius: 999px;
        background: transparent;
        font-family: var(--font-mono);
        font-size: 0.75rem;
        letter-spacing: 0.08em;
    }

    .preferences :global(button.admin-control[role='combobox']:hover),
    .preferences > :global(div:first-child) > :global(button.admin-control:hover) {
        background: var(--ui-hover);
    }

    .preferences > :global(div:first-child) > :global(button .icon) {
        color: var(--ui-text);
    }
</style>
