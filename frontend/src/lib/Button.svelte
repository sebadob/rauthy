<script>
    import {createEventDispatcher} from "svelte";
    import Loading from "./Loading.svelte";
    import {fade} from "svelte/transition";

    export let level = 2;
    export let width = 'inherit';
    export let selected = undefined;
    export let isDisabled = false;
    export let isLoading = false;

    let showText = !isLoading;
    let cls = 'button';
    let loadCol = 'white';

    $: disabled = isDisabled || isLoading;

    $: if (level) {
        switch (level) {
            case 1:
                cls = 'font-label l1';
                break;
            case 2:
                cls = 'font-label l2';
                break;
            case 3:
                cls = 'font-label l3';
                loadCol = 'var(--col-acnt)';
                break;
            default:
                cls = 'font-label l4';
                loadCol = 'var(--col-acnt)';
        }
    }

    $: if (isLoading) {
        setTimeout(() => {
            showText = false;
        }, 120);
    }

    $: if (!isLoading) {
        setTimeout(() => {
            showText = true;
        }, 120);
    }

    const dispatch = createEventDispatcher();

    function handleCLick() {
        if (selected !== undefined) {
            selected = !selected;
        }
        dispatch('click', true);
    }

</script>

<button
        class={cls}
        style:width={width}
        style:box-shadow="{selected ? 'inset 0 0 3px 2px var(--col-glow)' : ''}"
        style:cursor="{isLoading ? 'default' : 'pointer'}"
        on:click={handleCLick}
        on:keypress={handleCLick}
        { disabled }
>
    {#if isLoading}
        <div class="load">
            <Loading background={false} color={loadCol}/>
        </div>
    {:else if showText}
        <div in:fade class="txt">
            <slot></slot>
        </div>
    {/if}
</button>

<style>
    .l1, .l2, .l3, .l4 {
        height: 30px;
        margin: 5px;
        padding: 0 10px;
        font-weight: bold;
        border: 1px solid var(--col-gmid);
        border-radius: 3px;
        box-shadow: 1px 1px 2px var(--col-gmid);
    }

    .l1 {
        color: white;
        background: var(--col-act1);
    }

    .l1:hover {
        background: var(--col-act1a);
    }

    .l2 {
        color: var(--col-ghigh);
        background: var(--col-act2);
    }

    .l2:hover {
        background: var(--col-act2a);
    }

    .l3 {
        color: var(--col-act2a);
        border: 1px solid var(--col-act2a);
        background: var(--col-ghigh);
    }

    .l3:hover {
        background: white;
    }

    .l4 {
        color: var(--col-act2a);
        border: none;
        background: none;
        box-shadow: none;
    }

    .l4:hover {
        box-shadow: 1px 1px 2px var(--col-gmid);
    }

    .load {
        display: flex;
        justify-content: center;
    }

    .txt {
        margin-top: 4px;
    }
</style>
