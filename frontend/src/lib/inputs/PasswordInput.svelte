<script>
    import {createEventDispatcher} from "svelte";
    import IconClipboard from "$lib/icons/IconClipboard.svelte";
    import IconEyeSlash from "$lib/icons/IconEyeSlash.svelte";
    import IconEye from "$lib/icons/IconEye.svelte";
    import {getKey} from "../utils/helpers";
    import {slide} from "svelte/transition";


    /**
     * @typedef {Object} Props
     * @property {any} [bindThis]
     * @property {any} [name]
     * @property {boolean} [disabled]
     * @property {string} [error]
     * @property {boolean} [showCopy]
     * @property {string} [value]
     * @property {string} [width]
     * @property {string} [maxWidth]
     * @property {string} [autocomplete]
     * @property {string} [type]
     * @property {import('svelte').Snippet} [children]
     */

    /** @type {Props & { [key: string]: any }} */
    let {
        bindThis = $bindable(undefined),
        name = getKey(),
        disabled = false,
        error = '',
        showCopy = false,
        value = $bindable(''),
        width = '250px',
        maxWidth = 'inherit',
        autocomplete = 'current-password',
        type = $bindable('password'),
        children,
        ...rest
    } = $props();

    const dispatch = createEventDispatcher();

    function handleKeyPress(ev) {
        if (ev.code === 'Enter') {
            dispatch('enter', true);
        } else {
            dispatch('keypress', true);
        }
    }

    function handleOnBlur() {
        dispatch('blur', true);
    }

    function copy() {
        navigator.clipboard.writeText(value);
    }

    function toggle() {
        if (type === 'password') {
            type = 'text';
        } else {
            type = 'password';
        }
    }

    function onInput(event) {
        value = event.target.value
    }
</script>

<div class="container" style:width="calc({width} + 12px)" style:max-width={maxWidth}>
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
            bind:this={bindThis}
            style:width
            style:padding-right={showCopy ? '45px' : '25px'}
            {type}
            {name}
            {disabled}
            {value}
            {autocomplete}
            {...rest}
            oninput={onInput}
            onkeypress={handleKeyPress}
            onblur={handleOnBlur}
    />

    <div class="rel">
        {#if showCopy}
            <div
                    role="button"
                    tabindex="0"
                    class="btn clip"
                    onclick={copy}
                    onkeypress={copy}
            >
                <IconClipboard/>
            </div>
        {/if}

        <div
                role="button"
                tabindex="0"
                class="btn show"
                onclick={toggle}
                onkeypress={toggle}
        >
            {#if type === 'password'}
                <IconEyeSlash width={22}/>
            {:else}
                <IconEye width={22}/>
            {/if}
        </div>
    </div>

    {#if error}
        <div class="rel">
            <div
                    class="err font-label"
                    style:width={width}
                    transition:slide|global={{ duration: 250 }}
            >
                {error}
            </div>
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
        margin-top: -6px;
        margin-left: 5px;
        padding: 0 10px;
        font-size: .85rem;
        color: hsl(var(--error));
    }

    .labelInner {
        position: absolute;
        top: 0;
        left: 10px;
        border-radius: 5px;
        padding: 0 5px;
        font-size: .8rem;
    }

    .container {
        display: flex;
        flex-direction: column;
        margin-bottom: 3px;
    }

    .rel {
        position: relative;
    }

    .btn {
        position: absolute;
        top: -32px;
        right: 10px;
        margin-left: 100px;
        opacity: 0.85;
        cursor: pointer;
    }

    .clip {
        right: 32px;
        opacity: 0.85;
    }

    .show {
        opacity: 0.85;
    }
</style>
