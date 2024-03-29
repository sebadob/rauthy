<script>
    import {createEventDispatcher, tick} from "svelte";
    import {getKey} from "../utils/helpers.js";
    import {slide} from "svelte/transition";

    export let rows = 10;
    export let cols = 10;
    export let name = getKey();
    export let disabled = false;
    export let error = '';
    export let value = '' || null;
    export let width = '32rem';
    export let fixed = true;

    const dispatch = createEventDispatcher();

    async function onInput(event) {
        await tick();
        value = event.target.value
        dispatch('input', value);
    }

    function handleOnFocus() {
        dispatch('focus', true);
    }

    function onBlur() {
        dispatch('blur', true);
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

    <textarea
            style:resize={fixed ? 'none' : ''}
            style:width={width}
            {name}
            {disabled}
            {value}
            {rows}
            {cols}
            {...$$restProps}
            on:input={onInput}
            on:focus={handleOnFocus}
            on:blur={onBlur}
    />

    {#if error}
        <div
                class="err font-label"
                style:width={width}
                transition:slide|global={{ duration: 250 }}
        >
            {error}
        </div>
    {/if}
</div>

<style>
    textarea {
        padding: 10px;
        border: 1px solid var(--col-gmid);
        border-radius: 5px;
        outline: none;
    }

    textarea:focus {
        resize: none;
        border: 1px solid var(--col-acnt);
    }

    .label {
        position: relative;
    }

    .err {
        margin-top: -1px;
        padding: 0 10px;
        font-size: .85em;
        color: var(--col-err);
    }

    .labelInner {
        position: absolute;
        top: -13px;
        left: 5px;
        border-radius: 5px;
        padding: 0 5px;
        font-size: .85em;
        background: var(--col-bg);
    }

    .container {
        display: flex;
        flex-direction: column;
        margin: 1.33rem .5rem .66rem .5rem;
    }
</style>