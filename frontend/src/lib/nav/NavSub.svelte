<script lang="ts">
    import { tick, untrack, type Snippet } from 'svelte';
    import { MediaQuery } from 'svelte/reactivity';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { genKey } from '#utils/helpers.ts';
    import Button from '#lib/button/Button.svelte';
    import IconArrowLeft from '#icons/IconArrowLeft.svelte';

    let {
        detail,
        onBack,
        error,
        children,
        buttonTiles,
        listFooter,
        buttonTilesAriaControls,
    }: {
        detail: boolean;
        onBack: () => void;
        error?: string;
        children: Snippet;
        buttonTiles?: Snippet;
        listFooter?: Snippet;
        buttonTilesAriaControls?: string;
    } = $props();

    const id = genKey();
    const ta = useI18nAdmin();
    const mobile = new MediaQuery('(width < 800px)', false);

    let nav: HTMLElement | undefined = $state();
    let backButton: HTMLButtonElement | undefined = $state();
    let previousDetail = untrack(() => detail);

    $effect(() => {
        const isDetail = detail;
        const changed = previousDetail !== isDetail;
        previousDetail = isDetail;

        if (changed && mobile.current) {
            void tick().then(() => {
                const target = detail ? backButton : nav;
                if (!document.activeElement || document.activeElement === document.body) {
                    target?.focus({ preventScroll: true });
                }
            });
        }
    });
</script>

<div class="navSub" data-detail={detail}>
    {#if detail}
        <div class="back-bar">
            <Button bind:ref={backButton} ariaControls={id} invisible onclick={onBack}>
                <IconArrowLeft width="20px" />{ta.common.back}
            </Button>
        </div>
    {/if}
    <nav {id} bind:this={nav} tabindex="-1">
        <div class="list-content">
            {@render children()}
            {#if buttonTiles}
                <ul aria-controls={buttonTilesAriaControls}>
                    {@render buttonTiles()}
                </ul>
            {/if}
            {#if listFooter}
                <div class="list-footer">{@render listFooter()}</div>
            {/if}
            {#if error}
                <p class="mobile-error err">{error}</p>
            {/if}
        </div>
    </nav>
</div>

<style>
    .navSub {
        flex-shrink: 0;
        min-height: 0;
    }

    nav {
        height: 100%;
        width: clamp(17rem, 22vw, 21rem);
        border-right: 1px solid var(--ui-outline);
        overflow: clip;
    }

    .list-content {
        height: 100%;
        padding: 28px 16px;
        overflow-y: auto;
    }

    .list-footer:not(:empty) {
        margin-top: 0.5rem;
    }

    .back-bar,
    .mobile-error {
        display: none;
    }

    .navSub :global(.topBtns) {
        display: flex;
        align-items: center;
        gap: 4px;
        margin: 0 0 20px;
    }
    .navSub :global(.admin-add-button) {
        text-align: left;
        margin: 0 0 20px;
    }
    .navSub :global(.topBtns) :global(.admin-add-button) {
        order: -1;
        margin: 0 auto 0 0;
    }
    .list-content > :global(ul) {
        margin: 12px 0 0;
        padding: 0;
    }
    .navSub :global(li.admin-record-row) {
        margin: 4px 0;
        padding: 0;
        border: 0;
        border-radius: var(--border-radius);
        background: transparent;
        list-style: none;
    }
    .navSub :global(li.admin-record-row) > :global(div) {
        margin: 0;
    }
    .navSub :global(li.admin-record-row) :global(.admin-control) {
        width: 100%;
        height: auto;
        min-height: 64px;
        padding: 12px;
        justify-content: flex-start;
        border-radius: var(--border-radius);
        text-align: left;
    }
    .navSub :global(li.admin-record-row) :global(.admin-control) > :global(.children) {
        width: 100%;
        justify-content: flex-start;
    }
    .navSub :global(li.admin-record-row[aria-current='page']) :global(.admin-control) {
        background: var(--ui-tonal);
    }
    .navSub :global(li.admin-record-row[aria-current='page']) :global(.admin-row-title) {
        color: var(--ui-text);
    }
    .navSub :global(.tile) {
        min-width: 0;
        transform: none;
        line-height: 20px;
        overflow-wrap: anywhere;
    }
    .navSub :global(.admin-row-title) {
        font-size: 14px;
        font-weight: 600;
    }
    .navSub :global(.muted) {
        height: auto;
        margin: 2px 0 0;
        opacity: 1;
        font-size: 12px;
        font-weight: 400;
        line-height: 18px;
        color: var(--ui-muted);
    }
    .navSub :global(.avatar) {
        border-radius: var(--border-radius);
    }

    .navSub :global(.avatar:not(:has(img))) {
        display: grid;
        place-items: center;
        width: 36px;
        height: 36px;
        flex-shrink: 0;
        --avatar-background: var(--ui-container);
        border: 0;
        color: var(--ui-primary);
    }
    .navSub :global(.avatar) :global(.font-small) {
        font-family: inherit;
        font-size: 12px;
        font-weight: 600;
        color: var(--ui-primary);
        top: 0;
    }
    nav > :global(div:has(.usersList)) {
        display: flex;
        flex-direction: column;
    }
    .navSub :global(ul:has(> .usersList)) {
        display: flex;
        flex-direction: column;
        flex: 1;
        min-height: 0;
    }
    .navSub :global(.usersList) {
        flex: 1;
        min-height: 0;
        max-height: none;
    }
    .navSub :global(a) {
        display: block;
        padding: 10px 12px;
        margin: 2px 0;
        border-radius: var(--border-radius);
        color: var(--ui-muted);
        font-size: 14px;
        font-weight: 500;
        line-height: 20px;
        text-decoration: none;
    }
    .navSub :global(a:hover) {
        background: var(--ui-hover);
    }
    .navSub :global(a[aria-current='page']) {
        background: var(--ui-tonal);
        color: var(--ui-primary);
        text-decoration: none;
    }
    .navSub :global(li.admin-record-row) {
        margin: 8px 0;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: transparent;
    }
    .navSub :global(li.admin-record-row) :global(.admin-control) {
        --ui-button-height: auto;
        --ui-button-min-height: 3.25rem;
        --ui-button-padding: 0.375rem 0.625rem;
    }
    .navSub :global(li.admin-record-row[aria-current='page']) {
        border-color: hsl(var(--action) / 0.35);
    }
    .navSub :global(li.admin-record-row) :global(.tile) {
        font-size: 14px;
        font-weight: 500;
    }

    @media (width < 800px) {
        .navSub[data-detail='false'] {
            flex: 1;
        }

        nav {
            width: 100%;
            border-right: 0;
        }

        .list-content {
            padding: 20px 16px;
        }

        .navSub[data-detail='true'] nav {
            display: none;
        }

        .back-bar {
            display: flex;
            padding: 12px 16px 8px;
        }

        .back-bar :global(.admin-control) {
            --ui-button-height: 44px;
            --ui-button-padding: 0 12px;
            background: var(--ui-container);
        }

        .back-bar :global(.admin-control:hover) {
            background: var(--ui-hover);
        }

        .mobile-error {
            display: block;
            overflow-wrap: anywhere;
        }
    }
</style>
