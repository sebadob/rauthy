<script>
    import {fade} from 'svelte/transition';
    import {onMount} from "svelte";

    export let center = false;
    export let columns;

    let show = false;
    let flexDir;
    let innerWidth;
    $: if (columns) {
        if (innerWidth >= columns) {
            flexDir = 'row';
        } else {
            flexDir = 'column';
        }
    }

    onMount(() => {
        show = true;
        return () => show = false;
    });

</script>

<svelte:window bind:innerWidth/>

<div class="wrapper" style:align-items={center ? 'center' : 'flex-start'}>
    {#if show}
        <div
                transition:fade|global={{ duration: 100 }}
                class="inner"
                style:align-items={center ? 'center' : 'flex-start'}
                style:flex-direction={flexDir}
        >
            <slot></slot>
        </div>
    {/if}
</div>

<style>
    .inner {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
        overflow-y: auto;
    }

    .wrapper {
        display: flex;
        flex-direction: row;
    }
</style>
