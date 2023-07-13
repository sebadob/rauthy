<script>
    import IconChevronRight from "./icons/IconChevronRight.svelte";
    import {slide} from 'svelte/transition';
    import {spring} from "svelte/motion";

    export let idx = 0;
    export let show = false;
    export let expandedCallback;

    let isHover = false;
    let element;
    let bodyElement;
    let borderLeft = idx % 2 === 0 ? '2px solid var(--col-acnt)' : '2px solid var(--col-acnta)';

    const rotate = spring(0, {
        stiffness: 0.1,
        damping: 0.4
    });

    $: if (show) {
        rotate.set(90);
    } else {
        rotate.set(0);
    }

    $: if (show && bodyElement) {
        setTimeout(() => {
            scrollBody();
        }, 100);
    }

    function toggle() {
        if (!show && expandedCallback) {
            // If we will toggle from not showing to showing, we call the callback
            expandedCallback();
        }
        show = !show;
    }

    function scrollBody() {
        const endPos = element.offsetTop + bodyElement.scrollHeight + 100;
        const maxVisible = window.innerHeight + window.scrollY;
        const diff = maxVisible - endPos;

        if (diff < 0) {
            window.scroll({
                top: endPos,
                behavior: 'smooth'
            });
        }
    }

</script>

<div
        class="container"
        style:border-left={show ? borderLeft : 'none'}
        bind:this={element}
>
    <div class="containerHeader">
        <div
                role="button"
                tabindex="0"
                class="expand"
                on:mouseenter={() => isHover = true}
                on:mouseleave={() => isHover = false}
                on:click={toggle}
                on:keypress={toggle}
        >
            <div style="rotate: {$rotate}deg">
                <IconChevronRight
                        color={isHover ? 'var(--col-act2a)' : 'var(--col-act2)'}
                />
            </div>
        </div>

        <div class="header">
            <slot name="header"></slot>
        </div>
    </div>

    {#if show}
        <div class="body" bind:this={bodyElement} transition:slide|global={{ duration: 200 }}>
            <slot name="body"></slot>
        </div>
    {/if}
</div>

<style>
    .container {
        display: flex;
        flex-direction: column;
        width: 100%;
        max-width: 1200px;
    }

    .containerHeader {
        display: flex;
        align-items: center;
        padding: 5px 7px;
    }

    .expand {
        display: flex;
        align-items: center;
        padding-top: 2px;
        cursor: pointer;
    }

    .header {
        margin: 3px;
        display: flex;
    }

    .body {
        overflow: hidden;
        overflow-y: auto;
    }
</style>
