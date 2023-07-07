<script>
    import {fade, scale, slide} from "svelte/transition";
    import {navContainerExpanded, navIsExpanded, navWidthCollapsed} from "./navStore.js";
    import IconChevronRight from "../icons/IconChevronRight.svelte";
    import {spring} from "svelte/motion";
    import {sleepAwait} from "../utils/helpers.js";

    export let label = '';
    export let slotCollapsed = true;

    let expandedContainer;
    let isExpanded;
    let isNavExpanded = true;
    let color = 'var(--col-text)';
    let outerWidthCollapsed;

    let hover = false;

    $: if (hover) {
        color = 'var(--col-err)';
    }

    $: isExpanded = expandedContainer === label;

    const rotation = spring(isExpanded ? 90 : 180, {
        stiffness: 0.1,
        damping: 0.3
    });
    $: rotation.set(isExpanded ? 90 : 180);

    navWidthCollapsed.subscribe(w => outerWidthCollapsed = w);
    navIsExpanded.subscribe(v => isNavExpanded = v);
    navContainerExpanded.subscribe(c => expandedContainer = c);

    function toggle() {
        if (expandedContainer === label) {
            navContainerExpanded.set('');
        } else {
            navContainerExpanded.set(label);
        }
    }

    async function toggleHover() {
        if (hover) {
            await sleepAwait(150);
            hover = false;
        } else {
            hover = true;
        }
    }

</script>

{#if isNavExpanded}
    <div
            class="container noselect font-label"
            in:fade|global={{ delay: 100, duration: 100 }}
    >
        <div
                class="labelContainer"
                on:click={toggle}
                on:keypress={toggle}
                on:mouseenter={toggleHover}
                on:mouseleave={toggleHover}
        >
            {#if isNavExpanded}
                <slot name="icon"></slot>
                <span class="label">
          {label}
        </span>
                <span style="rotate: {$rotation}deg" style:margin-top="-5px" style:margin-left="2px">
          <IconChevronRight width={16}/>
        </span>
            {/if}
        </div>

        {#if isExpanded}
            <div
                    class="entries"
                    transition:slide|global={{ duration: 150 }}
            >
                <slot name="body"></slot>
            </div>
        {/if}
    </div>
{:else}
    <div
            class="collapsed noselect font-label"
            on:click={toggle}
            on:keypress={toggle}
            on:mouseenter={toggleHover}
            on:mouseleave={toggleHover}
            in:fade|global={{ delay: 200, duration: 100 }}
    >
        {#if hover}
            <div
                    class="collapsedInnerWrapper"
                    transition:scale|global={{ duration: 150 }}
            >
                <div
                        class="collapsedInner"
                        style:width="{outerWidthCollapsed}px"
                        style:left="calc({outerWidthCollapsed}px / 2 - 8px)"
                >
                    <slot name="body"></slot>
                </div>
            </div>
        {/if}

        <div class="collapsedInnerWrapper">
            <div
                    class="innerChevron"
                    style:left="calc({outerWidthCollapsed}px / 2 - 15px)"
            >
                <IconChevronRight width={12}/>
            </div>
        </div>

        {#if slotCollapsed}
            <slot name="icon"></slot>
        {/if}
        <span class="labelCollapsed">
      {label}
    </span>
    </div>
{/if}

<style>
    .container, .collapsed {
        display: flex;
        color: var(--col-text);
        cursor: pointer;
    }

    .container {
        flex-direction: column;
        margin: 10px 0;
    }

    .collapsed {
        flex-direction: column;
        align-items: center;
        text-align: center;
        word-break: break-word;
        margin: 10px 0;
    }

    .collapsedInner {
        position: absolute;
        top: 0;
        border: 1px solid var(--col-gmid);
        background: var(--col-bg);
        box-shadow: 5px 5px 5px var(--col-gmid);
        z-index: 10;
    }

    .collapsedInnerWrapper {
        position: relative;
    }

    .labelContainer:hover, .collapsed:hover {
        color: var(--col-act2);
    }

    .entries {
        padding-left: 10px;
    }

    .innerChevron {
        position: absolute;
        top: 3px;
    }

    .label {
        margin-left: 3px;
    }

    .labelCollapsed {
        font-size: .8rem;
    }

    .labelContainer {
        display: flex;
        flex-direction: row;
        align-items: center;
    }
</style>
