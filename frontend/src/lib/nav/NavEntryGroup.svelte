<script>
    import { run } from 'svelte/legacy';

    import {fade, scale, slide} from "svelte/transition";
    import {navContainerExpanded, navIsExpanded, navWidthCollapsed} from "./navStore.js";
    import IconChevronRight from "$lib/icons/IconChevronRight.svelte";
    import {spring} from "svelte/motion";
    import {sleepAwait} from "../utils/helpers.js";

    /**
     * @typedef {Object} Props
     * @property {string} [label]
     * @property {boolean} [slotCollapsed]
     * @property {import('svelte').Snippet} [icon]
     * @property {import('svelte').Snippet} [body]
     */

    /** @type {Props} */
    let {
        label = '',
        slotCollapsed = true,
        icon,
        body
    } = $props();

    let expandedContainer = $state();
    let isExpanded = $state();
    let isNavExpanded = $state(true);
    let color = $state('var(--col-text)');
    let outerWidthCollapsed = $state();

    let hover = $state(false);

    run(() => {
        if (hover) {
            color = 'var(--col-err)';
        }
    });

    run(() => {
        isExpanded = expandedContainer === label;
    });

    const rotation = spring(isExpanded ? 90 : 180, {
        stiffness: 0.1,
        damping: 0.3
    });
    run(() => {
        rotation.set(isExpanded ? 90 : 180);
    });

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
                onclick={toggle}
                onkeypress={toggle}
                onmouseenter={toggleHover}
                onmouseleave={toggleHover}
        >
            {#if isNavExpanded}
                {@render icon?.()}
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
                {@render body?.()}
            </div>
        {/if}
    </div>
{:else}
    <div
            class="collapsed noselect font-label"
            onclick={toggle}
            onkeypress={toggle}
            onmouseenter={toggleHover}
            onmouseleave={toggleHover}
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
                    {@render body?.()}
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
            {@render icon?.()}
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
