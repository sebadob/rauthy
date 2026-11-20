<script lang="ts">
    import { slide } from 'svelte/transition';
    import { useI18n } from '#state/i18n.svelte.ts';
    import type { FullAutoFill } from 'svelte/elements';
    import { genKey } from '#utils/helpers.ts';

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
        touched = true;
        isValid();
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

<div class="admin-input-field" style:width>
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

    :global(.admin-shell) .admin-input-field:not(:global(.branding-preview *)) {
        max-width: 100%;
        margin-bottom: 16px;
    }
    :global(.admin-shell) .label:not(:global(.branding-preview *)) {
        padding: 0;
        margin: 0 0 6px;
        line-height: 18px;
    }
    :global(.admin-shell)
        input:where(
            :not([type='checkbox']):not([type='hidden']):not([type='file']):not([type='range'])
        ):not(:global(.branding-preview *)) {
        box-sizing: border-box;
        width: 100%;
        min-width: 0;
        min-height: 44px;
        margin: 0;
        padding: 11px 14px;
        border: 1px solid var(--ui-outline);
        border-radius: var(--border-radius);
        background: var(--ui-surface);
        color: var(--ui-text);
        font: inherit;
        font-size: 14px;
        line-height: 20px;
        box-shadow: none;
    }
    :global(.admin-shell) input:not(:global(.branding-preview *))::placeholder {
        color: hsl(var(--text) / 0.65);
    }
    :global(.admin-shell) input:focus:not(:global(.branding-preview *)) {
        outline: 2px solid var(--ui-primary);
        outline-offset: -1px;
    }
    :global(.admin-shell) input.invalid:not(:global(.branding-preview *)) {
        border-color: hsl(var(--error));
    }
</style>
