<script>
    import { run } from 'svelte/legacy';

    import {onMount} from "svelte";
    import IconEyeSlash from "$lib/icons/IconEyeSlash.svelte";
    import IconEye from "$lib/icons/IconEye.svelte";
    import {sleepAwait} from "../utils/helpers.js";
    import IconClipboard from "$lib/icons/IconClipboard.svelte";

    /**
     * @typedef {Object} Props
     * @property {string} [value]
     */

    /** @type {Props} */
    let { value = '' } = $props();

    let hidden = '';
    let text = $state('');
    let txt = $state();
    let showValue = $state(false);

    const typingSpeed = 10;

    onMount(() => {
        for (let i = 0; i < value.length; i++) {
            hidden = hidden + '*';
        }
        text = hidden;
    });


    function copy() {
        navigator.clipboard.writeText(value);
    }

    async function hide() {
        for (let i = 1; i < value.length + 1; i++) {
            text = hidden.slice(0, i) + value.slice(i, hidden.length);
            await sleepAwait(typingSpeed);
        }
    }

    async function show() {
        for (let i = 1; i < value.length + 1; i++) {
            text = value.slice(0, i) + hidden.slice(i, hidden.length);
            await sleepAwait(typingSpeed);
        }
    }

    function toggle() {
        showValue = !showValue;
    }
    run(() => {
        if (showValue) {
            show();
        } else {
            hide();
        }
    });
</script>

<div class="container">
    <div class="val" bind:this={txt}>
        {text}
    </div>

    <div class="btn" onclick={copy} onkeypress={copy}>
        <IconClipboard/>
    </div>

    <div class="btn" onclick={toggle} onkeypress={toggle}>
        {#if showValue}
            <IconEye width={22}/>
        {:else}
            <IconEyeSlash width={22}/>
        {/if}
    </div>
</div>

<style>
    .btn {
        margin-left: 10px;
        opacity: 0.85;
        cursor: pointer;
    }

    .container {
        display: flex;
    }

    .val {
    }
</style>
