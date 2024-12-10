<script>
    /**
     * @typedef {Object} Props
     * @property {boolean} [eventsCollapsed]
     * @property {boolean} [eventsWide]
     * @property {import('svelte').Snippet} [children]
     */

    /** @type {Props} */
    let { eventsCollapsed = true, eventsWide = false, children } = $props();
</script>

<div
        class="content"
        class:mr={!eventsCollapsed && !eventsWide}
        class:mrCollapsed={eventsCollapsed && !eventsWide}
        class:mrWide={!eventsCollapsed && eventsWide}
>
    {@render children?.()}
</div>

<style>
    .content {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        height: 100vh;
        flex: 1;
        overflow-y: auto;
        transition: all 250ms ease-in-out;
    }

    .mr {
        margin-right: var(--width-events);
    }

    .mrCollapsed {
        margin-right: var(--width-events-collapsed);
    }

    .mrWide {
        margin-right: var(--width-events-wide);
    }
</style>
