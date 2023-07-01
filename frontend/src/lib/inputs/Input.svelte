<script>
    import {createEventDispatcher, tick} from "svelte";
    import {getKey} from "../utils/helpers.js";
    import {slide} from "svelte/transition";

    export let name = getKey();
    export let disabled = false;
    export let error = '';
    export let value;
    export let width = '250px';

    export let autocomplete = 'on';

    const dispatch = createEventDispatcher();

    function handleKeyPress(event) {
        dispatch('keypress', event);
    }

    function handleOnBlur() {
        dispatch('blur', true);
    }

    async function onInput(event) {
        await tick();
        value = event.target.value
        dispatch('input', true);
    }

</script>

<div class="container" style:width="calc({width} + 12px)">
    <div class="label">
        <div
                class="labelInner font-label noselect"
                style:background={disabled ? 'var(--col-gmid)' : 'var(--col-bg)'}
        >
            <label for={name}>
                <slot></slot>
            </label>
        </div>
    </div>

    <input
            style:width={width}
            {name}
            {disabled}
            {value}
            {autocomplete}
            {...$$restProps}
            on:input={onInput}
            on:keypress={handleKeyPress}
            on:blur={handleOnBlur}
    />

    {#if error}
        <div
                class="err font-label"
                style:width={width}
                transition:slide={{ duration: 250 }}
        >
            {error}
        </div>
    {/if}
</div>

<style>
    input {
        margin: 12px 5px 5px 5px;
        padding: 5px 10px;
        background: var(--col-bg);
        border: 1px solid var(--col-glow);
        border-radius: 7px;
        color: var(--col-text);
        font-size: 1.05em;
        outline: none;
        box-shadow: 1px 1px 2px var(--col-gmid);
    }

    input:disabled {
        background: var(--col-gmid);
    }

    input:disabled:hover {
        background: var(--col-gmid);
    }

    input:hover {
        /*background: var(--col-bg);*/
    }

    input:focus {
        /*background: var(--col-bg);*/
        border: 1px solid var(--col-acnt);
    }

    .label {
        position: relative;
    }

    .err {
        margin-top: -6px;
        margin-left: 5px;
        padding: 0 10px;
        font-size: .85em;
        color: var(--col-err);
    }

    .labelInner {
        position: absolute;
        top: 0;
        left: 11px;
        border-radius: 5px;
        padding: 0 5px;
        font-size: .8em;
    }

    .container {
        display: flex;
        flex-direction: column;
        margin-bottom: 3px;
    }
</style>
