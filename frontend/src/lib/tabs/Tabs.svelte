<script lang="ts">
    import Button from '#lib/button/Button.svelte';
    import { untrack } from 'svelte';

    let {
        tabs,
        selected = $bindable(''),
        borderRadius = 'var(--border-radius)',
        center = false,
        width = 'inherit',
        focusFirst = $bindable(),
        onDown,
    }: {
        tabs: string[];
        selected: string;
        borderRadius?: string;
        center?: boolean;
        width?: string;
        focusFirst?: () => void;
        onDown?: () => void;
    } = $props();

    untrack(() => {
        if (tabs.length > 0 && selected === '') {
            selected = tabs[0];
        }
    });

    let ref: undefined | HTMLDivElement = $state();

    focusFirst = focus;

    function focus(idx?: number) {
        let first = ref?.getElementsByTagName('button')[idx || 0];
        if (first) {
            first.focus();
        }
    }

    function onLeft(idx: number) {
        if (idx === 0) {
            focus(tabs.length - 1);
        } else {
            focus(idx - 1);
        }
    }

    function onRight(idx: number) {
        if (idx === tabs.length - 1) {
            focus(0);
        } else {
            focus(idx + 1);
        }
    }
</script>

<div
    bind:this={ref}
    class="tabs"
    class:center
    style:border-radius={borderRadius}
    style:--tabs-width={width}
>
    {#each tabs as tab, i}
        <Button
            ariaCurrent={tab === selected ? 'step' : undefined}
            invisible
            onclick={() => (selected = tab)}
            onLeft={() => onLeft(i)}
            onRight={() => onRight(i)}
        >
            <span class="font-label tab" data-selected={tab === selected}>
                {tab}
            </span>
        </Button>
    {/each}
</div>

<style>
    .tabs {
        width: var(--tabs-width);
        padding: 0.5rem 0.25rem;
        display: flex;
        column-gap: 0.25rem;
        row-gap: 0.75rem;
        align-items: center;
        flex-wrap: wrap;
        line-height: 1rem;
        background: hsl(var(--bg-high));
    }

    .center {
        justify-content: center;
    }

    .tab {
        padding: 0.25rem 0.4rem;
        border-radius: var(--border-radius);
        color: hsl(var(--text));
        background: hsl(var(--bg-high));
        transition: all 150ms ease-in-out;
    }

    .tab:hover {
        color: hsl(var(--action));
    }

    .tab[data-selected='true'] {
        color: hsl(var(--action));
        background: hsl(var(--bg));
    }

    :global(.ui-shell) .tabs {
        width: 100%;
        display: flex;
        gap: 4px;
        padding: 4px;
        margin: var(--ui-tabs-margin, 0 0 28px);
        border: 0;
        border-radius: var(--border-radius);
        background: var(--ui-container);
    }
    :global(.ui-shell) .tabs :global(.admin-control) {
        flex: 1;
        height: 38px;
        min-height: 38px;
        padding: 0 10px;
        border-radius: var(--border-radius);
    }
    :global(.ui-shell) .tabs :global(.admin-control[aria-current='step']) {
        background: var(--ui-surface);
        box-shadow: 0 1px 3px #00000012;
    }
    :global(.ui-shell) .tabs .tab {
        padding: 0;
        background: transparent;
        color: var(--ui-muted);
        font-family: inherit;
        font-size: 12px;
        font-weight: 500;
        line-height: 20px;
        border-radius: 0;
        white-space: nowrap;
    }
    :global(.ui-shell) .tabs .tab[data-selected='true'] {
        color: var(--ui-text);
        background: transparent;
        font-weight: 600;
    }
    @media (max-width: 800px) {
        :global(.ui-shell) .tabs {
            justify-content: flex-start;
            gap: 0.25rem;
        }
        :global(.ui-shell) .tabs :global(.admin-control) {
            flex: 0 0 auto;
        }
    }
    :global(.ui-shell) .tabs {
        width: 100%;
        flex: 1;
        min-width: 0;
    }
    :global(.ui-shell) .tabs :global(.admin-control[data-variant]) {
        min-width: max-content;
        padding: 0 12px;
        border-radius: var(--border-radius);
    }
</style>
