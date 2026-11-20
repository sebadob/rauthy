<script lang="ts">
    import { onDestroy, tick, type Snippet } from 'svelte';

    let {
        xOffset = 10,
        yOffset = 10,
        text,
        children,
    }: {
        xOffset?: number;
        yOffset?: number;
        text: string;
        children: Snippet;
    } = $props();

    let anchor: HTMLDivElement;
    let tip: HTMLDivElement;
    let timer: ReturnType<typeof setTimeout> | undefined;
    let hovering = false;
    let focused = false;
    let hoveredAt: { x: number; y: number } | undefined;
    let x = $state(0);
    let y = $state(0);

    onDestroy(() => clearTimeout(timer));

    function position() {
        const rect = anchor.getBoundingClientRect();
        const baseX = hovering && hoveredAt ? hoveredAt.x : rect.left;
        const baseY = hovering && hoveredAt ? hoveredAt.y : rect.bottom;
        const inset = 8;
        x = Math.max(inset, Math.min(baseX + xOffset, window.innerWidth - tip.offsetWidth - inset));
        const below = baseY + yOffset;
        y =
            below + tip.offsetHeight + inset <= window.innerHeight
                ? Math.max(inset, below)
                : Math.max(
                      inset,
                      (hovering && hoveredAt ? hoveredAt.y : rect.top) - tip.offsetHeight - yOffset,
                  );
    }

    function schedule() {
        clearTimeout(timer);
        if (tip.matches(':popover-open')) {
            position();
            return;
        }
        timer = setTimeout(async () => {
            tip.showPopover();
            await tick();
            if (tip.matches(':popover-open')) position();
        }, 400);
    }

    function hide() {
        if (hovering || focused) return;
        clearTimeout(timer);
        tip.hidePopover();
    }

    function pointerEnter(event: PointerEvent) {
        if (event.pointerType === 'touch') return;
        hovering = true;
        pointerMove(event);
        schedule();
    }

    function pointerMove(event: PointerEvent) {
        if (!hovering) return;
        hoveredAt = { x: event.clientX, y: event.clientY };
        if (tip.matches(':popover-open')) position();
    }

    function pointerLeave() {
        hovering = false;
        if (focused && tip.matches(':popover-open')) position();
        hide();
    }

    function focusIn(event: FocusEvent) {
        if (!(event.target instanceof Element) || !event.target.matches(':focus-visible')) return;
        focused = true;
        schedule();
    }

    function focusOut(event: FocusEvent) {
        if (event.relatedTarget instanceof Node && anchor.contains(event.relatedTarget)) return;
        focused = false;
        hide();
    }

    function dismiss() {
        clearTimeout(timer);
        if (tip?.matches(':popover-open')) tip.hidePopover();
    }
</script>

<svelte:window onscroll={dismiss} onresize={dismiss} />

<div
    bind:this={anchor}
    role="presentation"
    onpointerenter={pointerEnter}
    onpointermove={pointerMove}
    onpointerleave={pointerLeave}
    onfocusin={focusIn}
    onfocusout={focusOut}
>
    {@render children()}
    <div
        bind:this={tip}
        class="tooltip"
        role="tooltip"
        popover="manual"
        style:left={`${x}px`}
        style:top={`${y}px`}
    >
        {text}
    </div>
</div>

<style>
    .tooltip {
        position: fixed;
        inset: auto;
        margin: 0;
        max-width: min(22rem, calc(100vw - 1rem));
        padding: 0.33rem 0.5rem;
        background: hsl(var(--bg));
        color: hsl(var(--text-high));
        font-family: inherit;
        font-size: var(--ui-control-font-size, 0.875rem);
        font-weight: 400;
        line-height: var(--ui-control-line-height, 1.25rem);
        letter-spacing: normal;
        text-align: left;
        white-space: normal;
        overflow-wrap: anywhere;
        pointer-events: none;
        border: 1px solid hsl(var(--text) / 0.25);
        border-radius: var(--border-radius);
        box-shadow: 0 2px 8px hsl(var(--text) / 0.12);
    }
</style>
