<script>
    import {navIsExpanded, navSelected, navWidthCollapsed, navWidthExpanded} from "./navStore.js";
    import IconBurger from "$lib/icons/IconBurger.svelte";
    import IconClose from "$lib/icons/IconStop.svelte";
    import {fade} from "svelte/transition";
    import {tweened} from "svelte/motion";
    import {cubicOut} from "svelte/easing";
    import {onMount} from "svelte";
    import AppVersion from "../../components/AppVersion.svelte";

    export let selected = '';
    export let widthExpanded = 180;
    export let widthCollapsed = 60;

    export let isExpanded = true;

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

    $: if (isExpanded) {
        navIsExpanded.set(true);
        width.set(widthExpanded);
    } else {
        navIsExpanded.set(false);
        setTimeout(() => {
            width.set(widthCollapsed);
        }, 200);
    }

    $: if (selected) {
        navSelected.set(selected);
    }

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
                    on:click={toggle}
                    on:keypress={toggle}
            >
                <IconClose/>
            </div>

            <div class="logo" in:fade={{ delay: 250, duration: 100 }} out:fade={{ duration: 20 }}>
                <slot name="logo"></slot>
            </div>
        {:else}
            <div
                    role="button"
                    tabindex="0"
                    class="burger"
                    style:left="3px"
                    in:fade={{ delay: 500, duration: 200 }}
                    out:fade={{ duration: 100 }}
                    on:click={toggle}
                    on:keypress={toggle}
            >
                <IconBurger width={24}/>
            </div>
            <div style:height="10px"></div>
            <div class="logo" in:fade={{ delay: 250, duration: 100 }} out:fade={{ duration: 20 }}>
                <slot name="logo"></slot>
            </div>
        {/if}

        <div class="menu">
            <div class="links">
                <slot name="entries"></slot>
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
