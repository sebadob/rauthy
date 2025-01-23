<script lang="ts">
    import type {Snippet} from "svelte";
    import Button from "$lib5/Button.svelte";
    import {genKey} from "../utils/helpers.ts";

    let {
        ref = $bindable(),
        ariaLabel,
        children,
        roleButton = 'button',
        offsetLeft = '0px',
        offsetTop = '0px',
        absolute,
        lazy,
        btnDisabled,
        btnInvisible,
        button,
        close = $bindable(),

        onToggle,
        onLeft,
        onRight,
        onUp,
        onDown,
    }: {
        ref?: undefined | HTMLButtonElement,
        ariaLabel: string;
        children: Snippet,
        roleButton?: string;
        offsetLeft?: string;
        offsetTop?: string;
        absolute?: boolean;
        lazy?: boolean;
        btnDisabled?: boolean;
        btnInvisible?: boolean;
        button: Snippet,
        close?: () => void;

        onToggle?: (newState: 'open' | 'closed') => void;
        onLeft?: () => void,
        onRight?: () => void,
        onUp?: () => void,
        onDown?: () => void,
    } = $props();

    const idButton = genKey();
    const idPopover = genKey();

    let refPopover: undefined | HTMLDivElement = $state();
    let isOpen = $state(false);

    $effect(() => {
        close = closePopover;
    });

    function onclick(ev: Event) {
        ev.stopPropagation();

        if (ref && refPopover) {
            if (absolute) {
                refPopover.style.top = offsetTop;
                refPopover.style.left = offsetLeft;
            } else {
                let rectBtn = ref.getBoundingClientRect();
                refPopover.style.top = `calc(${rectBtn.bottom + window.scrollY}px + ${offsetTop})`;
                refPopover.style.left = `calc(${rectBtn.left + window.scrollX}px + ${offsetLeft})`;
            }
        } else {
            console.warn('button and popover ref missing');
        }
    }

    function closePopover() {
        refPopover?.hidePopover();
    }

    function ontoggle(ev: ToggleEvent) {
        let st = ev.newState as 'open' | 'closed';
        isOpen = 'open' === st;
        onToggle?.(st);
    }
</script>

<div>
    <Button
            bind:ref
            role={roleButton}
            id={idButton}
            ariaControls={idPopover}
            popovertarget={idPopover}
            {onclick}
            invisible={btnInvisible}
            isDisabled={btnDisabled}
            {onLeft}
            {onRight}
            {onUp}
            {onDown}
    >
        {@render button()}
    </Button>

    <div
            bind:this={refPopover}
            id={idPopover}
            aria-label={ariaLabel}
            aria-labelledby={idButton}
            class="popover"
            popover="auto"
            {ontoggle}
    >
        <div class="inner fade-in">
            {#if lazy}
                {#if isOpen}
                    {@render children()}
                {/if}
            {:else}
                {@render children()}
            {/if}
        </div>
    </div>
</div>

<style>
    .popover {
        position: absolute;
        background: hsla(var(--bg) / .98);
        margin: 0;
        padding: 0;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
        box-shadow: 0 0 2px 1px hsl(var(--bg-high));
    }

    .inner {
        color: hsl(var(--text));
    }

    .popover:popover-open {
        animation: var(--animate-zoom);
    }
</style>
