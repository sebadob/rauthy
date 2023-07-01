<script>
    import {fade, scale} from 'svelte/transition';
    import {onMount} from "svelte";
    import {sleepAwait} from "./utils/helpers.js";

    export let offset = 0;
    export let background = false;
    export let global = false;
    export let color = 'var(--col-acnt)';

    let run = false;
    let show1 = true;
    let show2 = false;
    let show3 = false;

    onMount(async () => {
        run = true;

        await sleepAwait(300);
        show2 = true;

        await sleepAwait(300);
        show3 = true;
    });

</script>

<div
        class="container"
        class:global
        class:local={!global}
        class:background
        transition:fade|global="{{ duration: 100 }}"
>
    {#if run}
        <div
                style="margin-top: {offset}px; display: flex;"
                transition:fade="{{ duration: 100 }}"
        >
            {#if show1}
                <div
                        class="circle"
                        style:background={color}
                        transition:scale|global="{{ delay: 600, duration: 1000 }}"
                        on:introend="{() => show1 = false}"
                        on:outroend="{() => show1 = true}"
                ></div>
            {/if}

            {#if show2}
                <div
                        class="circle"
                        style:background={color}
                        transition:scale|global="{{ delay: 600, duration: 1000 }}"
                        on:introend="{() => show2 = false}"
                        on:outroend="{() => show2 = true}"
                ></div>
            {/if}

            {#if show3}
                <div
                        class="circle"
                        style:background={color}
                        transition:scale|global="{{ delay: 600, duration: 1000 }}"
                        on:introend="{() => show3 = false}"
                        on:outroend="{() => show3 = true}"
                ></div>
            {/if}
        </div>
    {/if}
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
        min-width: 70px;
        max-width: 70px;
    }

    .background {
        background: rgba(127, 127, 127, .5);
    }

    .circle {
        margin: 3px;
        width: 15px;
        aspect-ratio: 1;
        border-radius: 50%;
    }
</style>
