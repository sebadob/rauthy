<script lang="ts">
    import { untrack, type Snippet } from 'svelte';
    import { page } from '$app/state';
    import { useI18nAdmin } from '#state/i18n_admin.svelte.ts';
    import { genKey } from '#utils/helpers.ts';
    import Button from '#lib/button/Button.svelte';

    let {
        width = 'clamp(17rem, 22vw, 21rem)',
        collapseButtonThreshold = 800,
        thresholdNavSub = 700,
        children,
        buttonTiles,
        buttonTilesAriaControls,
    }: {
        width?: string;
        collapseButtonThreshold?: number;
        thresholdNavSub?: number;
        children: Snippet;
        buttonTiles?: Snippet;
        buttonTilesAriaControls?: string;
    } = $props();

    const id = genKey();
    let ta = useI18nAdmin();

    $inspect(thresholdNavSub).with(() => {
        if (thresholdNavSub > collapseButtonThreshold) {
            console.warn(
                'thresholdNavMain must be lower than or equal to collapseButtonThreshold: ',
                collapseButtonThreshold,
            );
        }
    });

    let innerWidth: undefined | number = $state();
    let shouldCollapse = $derived(!!(innerWidth && innerWidth < thresholdNavSub));

    let collapsed = $state(false);
    let showIcon = $state(false);

    $effect(() => {
        if (innerWidth) {
            collapsed = shouldCollapse;
        }
    });

    // On narrow screens the list covers the details. Whenever the selected entry in the URL
    // changes (picked from the list, or just created in the "Add New" dialog), show the details.
    let lastSearch: undefined | string;
    $effect(() => {
        let search = page.url.search;
        if (lastSearch !== undefined && search !== lastSearch && untrack(() => shouldCollapse)) {
            collapsed = true;
        }
        lastSearch = search;
    });

    function toggle() {
        collapsed = !collapsed;
    }

    function onclick(ev: MouseEvent) {
        if (
            shouldCollapse &&
            ev.target instanceof Element &&
            ev.target.closest('.admin-record-row, a')
        ) {
            collapsed = true;
        }
    }

    function onmouseenter() {
        if (innerWidth && innerWidth < collapseButtonThreshold) {
            showIcon = true;
        }
    }

    function onmouseleave() {
        showIcon = false;
    }
</script>

<svelte:window bind:innerWidth />

<div class="navSub">
    {#snippet btn()}
        <Button
            ariaLabel={ta.common.selectEntry}
            ariaControls={id}
            ariaExpanded={!collapsed}
            invisible
            onclick={toggle}
        >
            <span class="btn"
                ><svg
                    width="20"
                    height="20"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="1.7"
                    ><path
                        stroke-linecap="round"
                        d="M9 6h11M9 12h11M9 18h11M4 6h.01M4 12h.01M4 18h.01"
                    /></svg
                ></span
            >
        </Button>
    {/snippet}

    {#if collapsed || shouldCollapse}
        <div class="btnCollapsed">
            {@render btn()}
        </div>
    {/if}

    <nav
        {id}
        aria-hidden={collapsed}
        data-collapsed={collapsed}
        style:--list-width={width}
        inert={collapsed}
        {onclick}
        {onmouseenter}
        {onmouseleave}
    >
        {#if showIcon && !(collapsed || shouldCollapse)}
            <div class="absolute">
                <div class="iconHover" class:hoverLeft={collapsed}>
                    {@render btn()}
                </div>
            </div>
        {/if}

        <div class="list-content">
            {@render children()}

            {#if buttonTiles}
                <ul aria-controls={buttonTilesAriaControls}>
                    {@render buttonTiles()}
                </ul>
            {/if}
        </div>
    </nav>
</div>

<style>
    nav {
        background: hsl(var(--bg));
        border-left: 1px solid hsla(var(--bg-high) / 0.35);
        border-right: 1px solid hsla(var(--bg-high) / 0.23);
        border-radius: 0 var(--border-radius) var(--border-radius) 0;
        overflow: clip;
        z-index: 1;
        transition: width 150ms;
    }

    nav[data-collapsed='true'] {
        width: 0;
    }

    .list-content {
        height: 100dvh;
        background: hsla(var(--bg-high) / 0.1);
        padding: 1rem 0;
        line-height: 1.6rem;
    }

    @media (width < 700px) {
        nav {
            position: absolute;
            inset: 0 auto 0 0;
            background: var(--ui-surface);
        }
    }

    .btn {
        margin-top: -0.1rem;
        color: hsla(var(--text) / 0.8);
        transition: color 150ms;
    }

    .btn:hover {
        color: hsl(var(--action));
    }

    .btnCollapsed {
        position: absolute;
        width: 0;
        margin-left: 1.5rem;
        margin-top: 0.1rem;
        z-index: 2;
    }

    .iconHover {
        top: -1rem;
        left: 1.5rem;
        position: relative;
    }

    .hoverLeft {
        left: 1rem;
    }

    .navSub {
        position: relative;
        z-index: 1;
    }

    .navSub {
        flex-shrink: 0;
    }
    nav {
        height: 100%;
        border: 0;
        border-right: 1px solid var(--ui-outline);
        border-radius: 0;
        background: transparent;
    }
    nav:not([data-collapsed='true']) {
        width: var(--list-width);
    }
    .list-content {
        height: 100%;
        padding: 28px 16px;
        background: transparent;
        overflow-y: auto;
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
    .navSub :global(ul) {
        margin: 12px 0 0;
        padding: 0;
    }
    .navSub :global(li:is([aria-current='page'], [aria-current='false'])) {
        margin: 4px 0;
        padding: 0;
        border: 0;
        border-radius: var(--border-radius);
        background: transparent;
        list-style: none;
    }
    .navSub :global(li:is([aria-current='page'], [aria-current='false'])) > :global(div) {
        margin: 0;
    }
    .navSub :global(li:is([aria-current='page'], [aria-current='false'])) :global(.admin-control) {
        width: 100%;
        height: auto;
        min-height: 64px;
        padding: 12px;
        justify-content: flex-start;
        border-radius: var(--border-radius);
        text-align: left;
    }
    .navSub
        :global(li:is([aria-current='page'], [aria-current='false']))
        :global(.admin-control)
        > :global(.children) {
        width: 100%;
        justify-content: flex-start;
    }
    .navSub :global(li[aria-current='page']) :global(.admin-control) {
        background: var(--ui-tonal);
    }
    .navSub :global(li[aria-current='page']) :global(.admin-row-title) {
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
    @media (max-width: 800px) {
        .list-content {
            padding-top: 48px;
        }
        nav:not([data-collapsed='true']) {
            width: min(20rem, 100dvw);
            min-width: min(20rem, 100dvw);
            /* below 500px the list floats above the details, so it needs its own surface */
            background: var(--ui-surface);
            box-shadow: 0 8px 24px #00000024;
        }
        .navSub :global(.btnCollapsed) {
            margin-top: 6px;
            margin-left: 8px;
        }
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

    .navSub :global(.btnCollapsed) > :global(.admin-control[data-variant]) {
        width: 40px;
        height: 40px;
        background: var(--ui-container);
        border-radius: var(--border-radius);
    }
    .navSub :global(.btnCollapsed) :global(.btn) {
        display: flex;
        align-items: center;
        margin: 0;
    }
</style>
