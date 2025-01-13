<script>
    import {run} from 'svelte/legacy';

    import {navIsExpanded, navSelected, navWidthCollapsed, navWidthExpanded} from "./navStore.js";
    import IconBurger from "$lib/icons/IconBurger.svelte";
    import IconClose from "$lib/icons/IconStop.svelte";
    import {fade} from "svelte/transition";
    import {tweened} from "svelte/motion";
    import {cubicOut} from "svelte/easing";
    import {onMount} from "svelte";
    import AppVersion from "../../components/AppVersion.svelte";


    /**
     * @typedef {Object} Props
     * @property {string} [selected]
     * @property {number} [widthExpanded]
     * @property {number} [widthCollapsed]
     * @property {boolean} [isExpanded]
     * @property {import('svelte').Snippet} [logo]
     * @property {import('svelte').Snippet} [entries]
     */

    /** @type {Props} */
    let {
        selected = $bindable(''),
        widthExpanded = 185,
        widthCollapsed = 60,
        isExpanded = $bindable(true),
        logo,
        entries
    } = $props();

    const width = tweened(isExpanded ? widthExpanded : widthCollapsed, {
        duration: 200,
        easing: cubicOut
    });

    navIsExpanded.subscribe(e => isExpanded = e);
    navSelected.subscribe(s => {
        if (s) {
            selected = s;
        }
    });

    run(() => {
        if (isExpanded) {
            navIsExpanded.set(true);
            width.set(widthExpanded);
        } else {
            navIsExpanded.set(false);
            setTimeout(() => {
                width.set(widthCollapsed);
            }, 200);
        }
    });

    run(() => {
        if (selected) {
            navSelected.set(selected);
        }
    });

    onMount(() => {
        navWidthExpanded.set(widthExpanded);
        navWidthCollapsed.set(widthCollapsed);
        navSelected.set(selected);
    });

    function toggle() {
        isExpanded = !isExpanded;
    }

</script>

<nav
        class="nav"
        style:min-width="{$width}px"
        style:width="{$width}px"
        style:padding={isExpanded ? '20px 10px' : '10px 3px'}
>
    <div>
        {#if isExpanded}
            <div
                    role="button"
                    tabindex="0"
                    class="close"
                    style:left="calc({$width}px - 22px)"
                    in:fade={{ delay: 500, duration: 200 }}
                    out:fade={{ duration: 100 }}
                    onclick={toggle}
                    onkeypress={toggle}
            >
                <IconClose/>
            </div>

            <div class="logo" in:fade={{ delay: 250, duration: 100 }} out:fade={{ duration: 20 }}>
                {@render logo?.()}
            </div>
        {:else}
            <div
                    role="button"
                    tabindex="0"
                    class="burger"
                    style:left="3px"
                    in:fade={{ delay: 500, duration: 200 }}
                    out:fade={{ duration: 100 }}
                    onclick={toggle}
                    onkeypress={toggle}
            >
                <IconBurger width={24}/>
            </div>
            <div style:height="10px"></div>
            <div class="logo" in:fade={{ delay: 250, duration: 100 }} out:fade={{ duration: 20 }}>
                {@render logo?.()}
            </div>
        {/if}

        <div class="menu">
            <div class="links">
                {@render entries?.()}
            </div>
        </div>
    </div>

    {#if isExpanded}
        <div
                class="version"
                in:fade={{ delay: 500, duration: 200 }}
                out:fade={{ duration: 100 }}
        >
            <AppVersion/>
        </div>
    {/if}

</nav>

<style>
    .burger:hover, .close:hover {
        cursor: pointer;
        color: var(--col-act2);
    }

    .burger {
        position: absolute;
        top: 0;
        left: 0;
    }

    .close {
        position: absolute;
        top: 0;
    }

    .menu {
        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }

    .logo {
    }

    .links {
        flex: 1;
    }

    .nav {
        height: 100vh;
        border-right: 1px solid var(--col-gmid);
        box-shadow: 1px 0 5px var(--col-gmid);
        display: flex;
        flex-direction: column;
        justify-content: space-between;
    }

    .version {
        margin: 0 0 -15px -5px;
    }
</style>
