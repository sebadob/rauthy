<script lang="ts">
    import Button from '$lib5/button/Button.svelte';

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

    if (tabs.length > 0 && selected === '') {
        selected = tabs[0];
    }

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
    style:width
>
    {#each tabs as tab, i}
        <Button
            ariaCurrent={tab === selected ? 'step' : undefined}
            invisible
            onclick={() => (selected = tab)}
            onLeft={() => onLeft(i)}
            onRight={() => onRight(i)}
        >
            <span
                class="font-label tab"
                data-selected={tab === selected}
            >
                {tab}
            </span>
        </Button>
    {/each}
</div>

<style>
    .tabs {
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
</style>
