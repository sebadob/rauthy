<script>
    import {onDestroy, onMount} from "svelte";
    import {spring} from 'svelte/motion';
    import {fade} from 'svelte/transition';

    export let xOffset = 10;
    export let yOffset = 10;
    export let text;
    export let html;

    let show = false;

    let coords;
    let timer;

    onMount(() => {
        coords = spring({x: window?.innerWidth / 2, y: window.innerHeight / 2}, {
            stiffness: 0.1,
            damping: 0.6
        });
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

    function handleMove(event) {
        // coords.set({ x: event.layerX + xOffset, y: event.layerY + yOffset });
        coords.set({x: event.clientX + xOffset, y: event.clientY + yOffset});
    }
</script>

<div
        on:mouseover={handleHover}
        on:focus={handleHover}
        on:mouseout={handleHide}
        on:blur={handleHide}
        on:mousemove={handleMove}
>
    <slot></slot>
    {#if show}
        <div
                class="tooltip"
                style="top: {`${$coords.y}px`}; left: {`${$coords.x}px`}"
                transition:fade={{ delay: 400, duration: 200 }}
        >
            {#if text}
                {text}
            {:else if html}
                {@html html}
            {/if}
        </div>
    {/if}
</div>

<style>
    .tooltip {
        position: absolute;
        padding: 5px 7px;
        font-weight: bold;
        background: var(--col-acnt);
        color: white;
        z-index: 1;
        border-radius: 3px;
    }
</style>
