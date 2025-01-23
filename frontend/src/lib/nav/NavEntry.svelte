<script>
    import {run} from 'svelte/legacy';

    import {navIsExpanded, navSelected} from "./navStore.js";
    import {fade} from "svelte/transition";

    /**
     * @typedef {Object} Props
     * @property {string} [label]
     * @property {boolean} [slotCollapsed]
     * @property {import('svelte').Snippet} [children]
     */

    /** @type {Props} */
    let {label = '', slotCollapsed = true, children} = $props();

    let isExpanded = $state(true);
    let color = $state('hsl(var(--text))');

    let selected = $state('');
    navSelected.subscribe(s => selected = s);
    navIsExpanded.subscribe(v => isExpanded = v);

    let hover = $state(false);


    function checkSelected() {
        if (hover) {
            color = 'hsl(var(--error))';
        } else if (selected === label) {
            color = 'hsl(var(--action))';
        } else {
            color = 'hsl(var(--text))';
        }
    }

    function select() {
        navSelected.set(label);
    }

    run(() => {
        if (selected) {
            checkSelected();
        }
    });
    run(() => {
        if (hover) {
            color = 'hsl(var(--error))';
        } else {
            checkSelected();
        }
    });
</script>

{#if isExpanded}
    <div
            role="menuitem"
            tabindex="0"
            class="entry noselect font-label"
            class:selected={selected === label}
            onclick={select}
            onkeypress={select}
            onmouseenter={() => hover = true}
            onmouseleave={() => hover = false}
            transition:fade|global={{ duration: 100 }}
    >
        {@render children?.()}
        <span class="label">
      {label}
    </span>
    </div>
{:else}
    <div
            role="menuitem"
            tabindex="0"
            class="entryCollapsed noselect font-label"
            class:selectedCollapsed={selected === label}
            onclick={select}
            onkeypress={select}
            onmouseenter={() => hover = true}
            onmouseleave={() => hover = false}
            in:fade|global={{ delay: 200, duration: 100 }}
    >
        {#if slotCollapsed}
            {@render children?.()}
        {/if}
        <span class="labelCollapsed">
      {label}
    </span>
    </div>
{/if}

<style>
    .entry, .entryCollapsed {
        display: flex;
        align-items: center;
        color: hsl(var(--text));
        cursor: pointer;
    }

    .entry {
        margin: 10px 0;
    }

    .entryCollapsed {
        flex-direction: column;
        text-align: center;
        margin: 10px 0;
    }

    .entry:hover, .entryCollapsed:hover {
        color: hsl(var(--action));
    }

    .label {
        margin-top: 5px;
        margin-left: 3px;
    }

    .labelCollapsed {
        font-size: .8rem;
    }

    .selected, .selectedCollapsed {
        color: hsl(var(--action));
    }

    .selected:after {
        margin-top: 3px;
        padding-left: 5px;
        content: '◄';
    }

    .selectedCollapsed {
        text-decoration: underline;
        text-underline: hsl(var(--action));
    }
</style>
