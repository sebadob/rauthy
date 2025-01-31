<script lang="ts">
    import {onDestroy} from "svelte";
    import {fade} from 'svelte/transition';
    import type {Snippet} from 'svelte'
    import {Spring} from "svelte/motion";

    let {
        xOffset = 10,
        yOffset = 10,
        text,
        children,
    } = $props<{
        /** The x offset of the tooltip */
        xOffset?: number,
        /** The y offset of the tooltip */
        yOffset?: number,
        /** The tooltip text content */
        text: string,
        children: Snippet,
    }>();

    let title = $state(text);
    let show = $state(false);
    let timer = $state<number | undefined>();

    const top = new Spring(0, {
        stiffness: 0.1,
        damping: 0.6
    });
    const left = new Spring(0, {
        stiffness: 0.1,
        damping: 0.6
    });

    $effect(() => {
        // we want to set title to empty string, if we have JS available
        // this way, the component works with the nice JS tooltip and has a fallback for HTML only
        title = '';
    });

    onDestroy(() => {
        clearTimeout(timer);
    });

    function handleHover() {
        clearTimeout(timer)
        show = true;
    }

    function handleHide() {
        timer = setTimeout(() => {
            show = false;
        }, 100);
    }

    function handleMove(event: MouseEvent) {
        top.set(event.clientY + yOffset);
        left.set(event.clientX + xOffset);
    }
</script>

<div
        role="none"
        onmouseover={handleHover}
        onfocus={handleHover}
        onmouseout={handleHide}
        onblur={handleHide}
        onmousemove={handleMove}
        title={title}
>
    {@render children()}
    {#if show}
        <div
                class="tooltip"
                style="top: {`${top.current}px`}; left: {`${left.current}px`}"
                transition:fade={{ delay: 400, duration: 200 }}
        >
            {text}
        </div>
    {/if}
</div>

<style>
    .tooltip {
        position: absolute;
        padding: .33rem .5rem;
        background: hsl(var(--bg));
        color: hsl(var(--text));
        z-index: 999;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
        box-shadow: 1px 1px 2px hsl(var(--bg-high));
    }
</style>
