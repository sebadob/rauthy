<script>
    import {onDestroy, onMount} from "svelte";
    import {spring} from 'svelte/motion';
    import {fade} from 'svelte/transition';

    /**
     * @typedef {Object} Props
     * @property {number} [xOffset]
     * @property {number} [yOffset]
     * @property {string} [text]
     * @property {string} [html]
     * @property {import('svelte').Snippet} [children]
     */

    /** @type {Props} */
    let {
        xOffset = 10,
        yOffset = 10,
        text = '',
        html = '',
        children
    } = $props();

    let show = $state(false);

    let coords = $state();
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
        role="none"
        onmouseover={handleHover}
        onfocus={handleHover}
        onmouseout={handleHide}
        onblur={handleHide}
        onmousemove={handleMove}
>
    {@render children?.()}
    {#if show}
        <div
                class="tooltip"
                style="top: {`${$coords.y}px`}; left: {`${$coords.x}px`}"
                transition:fade|global={{ delay: 400, duration: 200 }}
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
