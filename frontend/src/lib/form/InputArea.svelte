<script lang="ts">
    import { slide } from 'svelte/transition';

    let {
        ref = $bindable(),
        id,
        name,
        value = $bindable(''),
        label = '',

        rows = 3,
        placeholder = '',
        errMsg = '',
        disabled = false,
        fontMono = false,

        maxLength,
        required = false,
        pattern,
        isError = $bindable(false),

        width = 'inherit',

        onBlur,
        onLeft,
        onRight,
        onUp,
        onDown,
    } = $props<{
        ref?: undefined | HTMLTextAreaElement;
        id?: string;
        name?: string;
        value?: string | number;
        label?: string;
        rows?: number;
        placeholder?: string;
        errMsg?: string;
        disabled?: boolean | null | undefined;
        fontMono?: boolean | null | undefined;
        maxLength?: number | null | undefined;
        step?: number;
        required?: boolean;
        pattern?: string;
        isError?: boolean;
        width?: string;

        onBlur?: () => void;
        onLeft?: () => void;
        onRight?: () => void;
        onUp?: () => void;
        onDown?: () => void;
    }>();

    const re = pattern ? new RegExp(pattern, 'gm') : undefined;

    function onblur(event: FocusEvent & { currentTarget: EventTarget & HTMLTextAreaElement }) {
        isValid();
        onBlur?.();
    }

    function oninput(event: Event & { currentTarget: EventTarget & HTMLTextAreaElement }) {
        isValid();
    }

    function oninvalid(event: Event & { currentTarget: EventTarget & HTMLTextAreaElement }) {
        event.preventDefault();
        isError = true;
    }

    function onsubmit(event: Event & { currentTarget: EventTarget & HTMLTextAreaElement }) {
        if (!isValid()) {
            event.preventDefault();
        }
    }

    function onkeydown(ev: KeyboardEvent) {
        isValid();
        switch (ev.code) {
            case 'ArrowUp':
                onUp?.();
                break;
            case 'ArrowDown':
                onDown?.();
                break;
            case 'ArrowLeft':
                onLeft?.();
                break;
            case 'ArrowRight':
                onRight?.();
                break;
        }
    }

    function isValid() {
        if (re) {
            if (value.match(re).length > 2) {
                ref?.setCustomValidity(errMsg);
                isError = true;
            } else {
                ref?.setCustomValidity('');
                isError = false;
            }
        }
        return true;
    }
</script>

<div
    class="container"
    style:width
>
    <textarea
        bind:this={ref}
        {id}
        {name}
        class:font-mono={fontMono}
        title={errMsg}
        aria-label={label || placeholder}
        bind:value
        {rows}
        autocomplete="off"
        {placeholder}
        aria-placeholder={placeholder}
        {disabled}
        aria-disabled={disabled}
        maxlength={maxLength || undefined}
        required={required || undefined}
        aria-required={required || false}
        aria-invalid={isError}
        {onsubmit}
        {oninput}
        {oninvalid}
        {onblur}
        {onkeydown}
    >
    </textarea>
    <div class="label">
        <label
            for={id}
            class="font-label noselect"
            data-required={required}
        >
            {label}
        </label>
        {#if isError}
            <div
                class="error"
                transition:slide={{ duration: 150 }}
            >
                {errMsg}
            </div>
        {/if}
    </div>
</div>

<style>
    textarea {
        width: 100%;
        padding: 0.25rem 0.5rem;
        border: 1px solid hsl(var(--bg-high));
        border-radius: var(--border-radius);
        color: hsl(var(--text));
        background: transparent;
        outline: none;
        resize: none;
    }

    textarea:invalid {
        border: 1px solid hsl(var(--error));
    }

    textarea:focus-visible {
        outline: hsl(var(--accent));
    }

    .container {
        margin: 1rem 0;
    }

    label,
    .error {
        line-height: 1.1rem;
        font-size: 0.9rem;
    }

    label {
        color: hsla(var(--text) / 0.8);
        flex-wrap: wrap;
    }

    .label {
        width: 100%;
        margin-top: -0.8rem;
        padding: 0.5rem;
    }

    .error {
        margin-top: -0.5rem;
        color: hsl(var(--error));
    }
</style>
