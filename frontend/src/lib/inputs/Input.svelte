<script>
    import {createEventDispatcher, tick} from "svelte";
    import {getKey} from "../utils/helpers";
    import {slide} from "svelte/transition";


    /**
     * @typedef {Object} Props
     * @property {any} [name]
     * @property {boolean} [disabled]
     * @property {string} [error]
     * @property {any} value
     * @property {string} [width]
     * @property {string} [autocomplete]
     * @property {import('svelte').Snippet} [children]
     */

    /** @type {Props & { [key: string]: any }} */
    let {
        name = getKey(),
        disabled = false,
        error = $bindable(),
        value = $bindable(),
        width = '250px',
        autocomplete = 'on',
        children,
        ...rest
    } = $props();

    const dispatch = createEventDispatcher();

    function handleKeyPress(event) {
        dispatch('keypress', event);
        if (event.code === 'Enter') {
            dispatch('enter', event);
        }
    }

    dispatch('blur', true);

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
                style:background={disabled ? 'hsla(var(--bg-high) / .25)' : 'hsl(var(--bg))'}
        >
            <label for={name}>
                {@render children?.()}
            </label>
        </div>
    </div>

    <input
            style:width={width}
            {name}
            {disabled}
            {value}
            {autocomplete}
            {...rest}
            oninput={onInput}
            onkeypress={handleKeyPress}
            onblur={handleOnBlur}
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
    input {
        margin: 12px 5px 5px 5px;
        padding: 5px 10px;
        border: 1px solid hsl(var(--bg-high));
        border-radius: 3px;
        font-size: 1.05rem;
        outline: none;
        box-shadow: 1px 1px 2px hsl(var(--bg-high));
    }

    input:disabled {
        background: hsla(var(--bg-high) / .25);
    }

    input:focus {
        border: 1px solid hsl(var(--accent));
    }

    .label {
        position: relative;
    }

    .err {
        margin-top: -.3rem;
        margin-left: 5px;
        padding: 0 10px;
        font-size: .85rem;
        color: hsl(var(--error));
    }

    .labelInner {
        position: absolute;
        top: 0;
        left: 11px;
        border-radius: 5px;
        padding: 0 5px;
        font-size: .8rem;
    }

    .container {
        display: flex;
        flex-direction: column;
        margin-bottom: 3px;
    }
</style>
