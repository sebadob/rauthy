<script lang="ts">
    import type {Snippet} from "svelte";
    import {genKey} from "$utils/helpers.ts";
    import Button from "$lib5/Button.svelte";
    import IconArrowRightLeft from "$icons/IconArrowRightLeft.svelte";

    let {
        width = 'min(25rem, 100dvw)',
        paddingTop = '7.1rem',
        collapse = $bindable(),
        collapseButtonThreshold = 800,
        thresholdNavSub = 500,
        children,
    }: {
        width?: string;
        paddingTop?: string;
        collapse?: () => void;
        collapseButtonThreshold?: number;
        thresholdNavSub?: number;
        children: Snippet,
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
        collapse = () => collapsed = shouldCollapse;
    });

    $effect(() => {
        if (innerWidth) {
            collapsed = shouldCollapse;
        }
    });

    // $effect(() => {
    //     collapsed = shouldCollapse;
    // })

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
        <span class="btn">
            <IconArrowRightLeft width="1.4rem"/>
        </span>
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
        </div>
    </nav>
</div>

<style>
    nav {
        margin-left: -.5rem;
        background: hsl(var(--bg));
        border-right: 1px solid hsla(var(--bg-high) / .23);
        overflow: clip;
        z-index: 1;
        transition: width 150ms;
    }

    nav[data-collapsed="true"] {
        width: 0;
    }

    nav > div {
        height: 100dvh;
        background: hsla(var(--bg-high) / .07);
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
        color: hsla(var(--text) / .5);
        transition: color 150ms;
    }

    .btn:hover {
        color: hsl(var(--action));
    }

    .btnCollapsed {
        position: fixed;
        width: 0;
        transform: translateX(.8rem);
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
