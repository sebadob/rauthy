<script>
    import {navIsExpanded, navSelected} from "./navStore.js";
    import {fade} from "svelte/transition";

    export let label = '';
    export let slotCollapsed = true;

    let isExpanded = true;
    let color = 'var(--col-text)';

    let selected = '';
    navSelected.subscribe(s => selected = s);
    navIsExpanded.subscribe(v => isExpanded = v);

    let hover = false;

    $: if (selected) {
        checkSelected();
    }

    $: {
        if (hover) {
            color = 'var(--col-err)';
        } else {
            checkSelected();
        }
    }

    function checkSelected() {
        if (hover) {
            color = 'var(--col-err)';
        } else if (selected === label) {
            color = 'var(--col-ok)';
        } else {
            color = 'var(--col-text)';
        }
    }

    function select() {
        navSelected.set(label);
    }

</script>

{#if isExpanded}
    <div
            role="menuitem"
            tabindex="0"
            class="entry noselect font-label"
            class:selected={selected === label}
            on:click={select}
            on:keypress={select}
            on:mouseenter={() => hover = true}
            on:mouseleave={() => hover = false}
            transition:fade|global={{ duration: 100 }}
    >
        <slot></slot>
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
            on:click={select}
            on:keypress={select}
            on:mouseenter={() => hover = true}
            on:mouseleave={() => hover = false}
            in:fade|global={{ delay: 200, duration: 100 }}
    >
        {#if slotCollapsed}
            <slot></slot>
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
        color: var(--col-text);
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
        color: var(--col-act2);
    }

    .label {
        margin-top: 5px;
        margin-left: 3px;
    }

    .labelCollapsed {
        font-size: .8rem;
    }

    .selected, .selectedCollapsed {
        color: var(--col-act2a);
    }

    .selected:after {
        margin-top: 3px;
        padding-left: 5px;
        content: '◄';
    }

    .selectedCollapsed {
        text-decoration: underline;
        text-underline: var(--col-act2);
    }
</style>
