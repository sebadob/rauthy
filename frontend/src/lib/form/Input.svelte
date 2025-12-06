<script lang="ts">
    import { slide } from 'svelte/transition';
    import { useI18n } from '$state/i18n.svelte';
    import type { FullAutoFill } from 'svelte/elements';
    import { genKey } from '$utils/helpers';

    let {
        ref = $bindable(),
        typ = 'text',
        id = genKey(),
        name,
        value = $bindable(),
        label = '',

        autocomplete = 'on',
        placeholder = '',
        disabled = false,

        minLength,
        maxLength,
        min,
        max,
        step = 1,

        required = false,
        pattern,
        errMsg,
        isError = $bindable(false),

        width = 'inherit',

        onBlur,
        onEnter,
        onInput,
        onLeft,
        onRight,
        onUp,
        onDown,
        onSubmit,
    }: {
        ref?: undefined | HTMLInputElement;
        typ?: string;
        id?: string;
        name?: string;
        value?: string | number | null;
        label?: string;
        autocomplete?: FullAutoFill | null | undefined;
        placeholder?: string;
        disabled?: boolean | null | undefined;
        minLength?: number | null | undefined;
        maxLength?: number | null | undefined;
        min?: string;
        max?: string;
        step?: number;
        required?: boolean;
        pattern?: string;
        errMsg?: string;
        isError?: boolean;
        width?: string;

        onBlur?: () => void;
        onEnter?: () => void;
        onInput?: () => void;
        onLeft?: () => void;
        onRight?: () => void;
        onUp?: () => void;
        onDown?: () => void;
        onSubmit?: (event: Event & { currentTarget: EventTarget & HTMLInputElement }) => void;
    } = $props();

    let t = useI18n();

    let touched = $state(false);

    $effect(() => {
        // this unused assignment is used only to trigger a re-validation on any input change
        let v = value;
        if (touched) {
            isValid();
        }
    });

    function onblur(event: FocusEvent & { currentTarget: EventTarget & HTMLInputElement }) {
        touched = true;
        isError = false;
        // the animation frame triggers some screen readers to re-read errors if they still exist
        requestAnimationFrame(() => {
            isValid();
            onBlur?.();
        });
    }

    function oninput(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
        if (touched) {
            isValid();
        }
        onInput?.();
    }

    function oninvalid(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
        event.preventDefault();
        isError = true;
    }

    function onkeydown(ev: KeyboardEvent) {
        isValid();
        switch (ev.code) {
            case 'Enter':
                if (isValid()) {
                    onEnter?.();
                }
                break;
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
        let validity = ref?.validity;
        if (validity) {
            isError = !validity.valid;
            return validity.valid;
        }
        isError = false;
        return true;
    }
</script>

<div style:width>
    <div aria-live="assertive" class="label">
        <label for={id} class="font-label noselect" data-required={required}>
            {label}
        </label>
    </div>
    <input
        bind:this={ref}
        type={typ}
        {id}
        {name}
        title={errMsg}
        aria-label={label || placeholder}
        bind:value
        {autocomplete}
        {placeholder}
        aria-placeholder={placeholder}
        {disabled}
        aria-disabled={disabled}
        minlength={minLength || undefined}
        maxlength={maxLength || undefined}
        min={min || undefined}
        max={max || undefined}
        {step}
        required={required || undefined}
        aria-required={required || false}
        aria-invalid={isError}
        pattern={pattern || undefined}
        class:invalid={isError}
        {oninput}
        {oninvalid}
        {onblur}
        {onkeydown}
        onsubmit={onSubmit}
    />
    {#if isError}
        <div
            aria-relevant="all"
            class="error"
            class:errWithLabel={!!label}
            transition:slide={{ duration: 150 }}
        >
            {errMsg || t.common.invalidInput}
        </div>
    {/if}
</div>

<style>
    label {
        font-size: 0.9em;
        flex-wrap: wrap;
    }

    .label {
        width: 100%;
        margin-bottom: -0.3rem;
        padding-left: 0.1rem;
        padding-top: 0.1rem;
    }

    .error {
        color: hsl(var(--error));
        margin-top: -0.4rem;
        padding-left: 0.1rem;
        font-size: 0.8rem;
    }
</style>
