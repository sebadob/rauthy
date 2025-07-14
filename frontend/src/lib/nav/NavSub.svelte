<script lang="ts">
    import type {Snippet} from "svelte";
    import {genKey} from "$utils/helpers";
    import Button from "$lib5/button/Button.svelte";
    import {useTrigger} from "$state/callback.svelte";

    let {
        width = 'min(25rem, 100dvw)',
        paddingTop = '4.5rem',
        collapseButtonThreshold = 800,
        thresholdNavSub = 500,
        children,
        buttonTiles,
        buttonTilesAriaControls,
    }: {
        width?: string;
        paddingTop?: string;
        collapseButtonThreshold?: number;
        thresholdNavSub?: number;
        children: Snippet,
        buttonTiles?: Snippet,
        buttonTilesAriaControls?: string,
    } = $props();

    const id = genKey();

    $inspect(thresholdNavSub).with(() => {
        if (thresholdNavSub > collapseButtonThreshold) {
            console.warn(
                'thresholdNavMain must be lower than or equal to collapseButtonThreshold: ',
                collapseButtonThreshold
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

    function toggle() {
        collapsed = !collapsed;
    }

    function onclick() {
        if (shouldCollapse) {
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

<svelte:window bind:innerWidth></svelte:window>

<div class="navSub">
    {#snippet btn()}
        <Button ariaControls={id} invisible onclick={toggle}>
            <svg
                    aria-expanded={!collapsed}
                    class="btn"
                    viewBox="0 0 24 24"
                    stroke="currentColor"
                    stroke-width={2}
                    width="1.5rem"
                    opacity={0.9}
            >
                <path
                        stroke-linecap="round"
                        stroke-linejoin="round"
                        d="M7.5 21 3 16.5m0 0L7.5 12M3 16.5h13.5m0-13.5L21 7.5m0 0L16.5 12M21 7.5H7.5"
                />
            </svg>
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
            style:width={collapsed ? 0 : width}
            style:min-width={collapsed ? 0 : width}
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

        <div style:padding-top={paddingTop}>
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
        /*background: hsl(var(--bg));*/
        border-left: 1px solid hsla(var(--bg-high) / .35);
        border-right: 1px solid hsla(var(--bg-high) / .23);
        border-radius: 0 var(--border-radius) var(--border-radius) 0;
        overflow: clip;
        z-index: 1;
        transition: width 150ms;
    }

    nav[data-collapsed="true"] {
        width: 0;
    }

    nav > div {
        height: 100dvh;
        background: hsla(var(--bg-high) / .1);
        padding: 1rem 0;
        line-height: 1.6rem;
    }

    @media (max-width: 500px) {
        nav {
            position: fixed;
            top: 0;
            left: 0;
        }
    }

    .btn {
        margin-left: -.2rem;
        margin-top: -.1rem;
        color: hsla(var(--text) / .5);
        transition: color 150ms;
    }

    .btn:hover {
        color: hsl(var(--action));
    }

    .btnCollapsed {
        position: fixed;
        width: 0;
        margin-left: 1.5rem;
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
        z-index: 100;
    }
</style>
