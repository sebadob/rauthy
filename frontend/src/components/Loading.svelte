<script>
    import {fade, scale} from 'svelte/transition';

    /**
     * @typedef {Object} Props
     * @property {number} [offset]
     * @property {boolean} [background]
     * @property {boolean} [local]
     */

    /** @type {Props} */
    let { offset = 0, background = true, local = false } = $props();

    let show1 = $state(true);
    let show2 = $state(false);
    let show3 = $state(false);

    setTimeout(() => {
        show2 = true;
    }, 300);

    setTimeout(() => {
        show3 = true;
    }, 600);
</script>

<div
        class="container"
        class:global={!local}
        class:local
        class:background
        transition:fade|global="{{ duration: 100 }}"
>
    <div
            class="inner"
            style="margin-top: {`${offset}px`}"
            transition:fade="{{ duration: 100 }}"
    >
        {#if show1}
            <div
                    class="circle"
                    transition:scale|global="{{ delay: 600, duration: 1000 }}"
                    onintroend={() => show1 = false}
                    onoutroend={() => show1 = true}
            ></div>
        {/if}

        {#if show2}
            <div
                    class="circle"
                    transition:scale|global="{{ delay: 600, duration: 1000 }}"
                    onintroend={() => show2 = false}
                    onoutroend={() => show2 = true}
            ></div>
        {/if}

        {#if show3}
            <div
                    class="circle"
                    transition:scale|global="{{ delay: 600, duration: 1000 }}"
                    onintroend={() => show3 = false}
                    onoutroend={() => show3 = true}
            ></div>
        {/if}
    </div>
</div>

<style>
    .container {
        display: flex;
        flex: 1;
        justify-content: center;
        align-items: center;
        z-index: 1;
    }

    .global {
        position: absolute;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
    }

    .local {
    }

    .background {
        background: rgba(0, 0, 0, .7);
    }

    .inner {
        display: flex;
        flex: 1;
        align-items: center;
        justify-content: center;
    }

    .circle {
        margin: 3px;
        width: 15px;
        aspect-ratio: 1;
        border-radius: 50%;
        background: rgba(200, 200, 200, .9);
    }
</style>
