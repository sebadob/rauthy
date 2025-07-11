<script lang="ts">
    import {slide} from "svelte/transition";
    import {useI18n} from "$state/i18n.svelte";
    import type {FullAutoFill} from "svelte/elements";
    import {genKey} from "$utils/helpers";

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
        ref?: undefined | HTMLInputElement,
        typ?: string,
        id?: string,
        name?: string,
        value?: string | number,
        label?: string,
        autocomplete?: FullAutoFill | null | undefined,
        placeholder?: string,
        disabled?: boolean | null | undefined,
        maxLength?: number | null | undefined,
        min?: string,
        max?: string,
        step?: number,
        required?: boolean,
        pattern?: string,
        errMsg?: string,
        isError?: boolean,
        width?: string,

        onBlur?: () => void,
        onEnter?: () => void,
        onInput?: () => void,
        onLeft?: () => void,
        onRight?: () => void,
        onUp?: () => void,
        onDown?: () => void,
        onSubmit?: (event: Event & { currentTarget: EventTarget & HTMLInputElement }) => void,
    } = $props();

    let t = useI18n();

    $effect(() => {
        // this unused assignment is used only to trigger a re-validation on any input change
        let v = value;
        isValid();
    });

    function onblur(event: FocusEvent & { currentTarget: EventTarget & HTMLInputElement }) {
        isError = false;
        // the animation frame triggers some screen readers to re-read errors if they still exist
        requestAnimationFrame(() => {
            isValid();
            onBlur?.();
        });
    }

    function oninput(event: Event & { currentTarget: EventTarget & HTMLInputElement }) {
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

<div style:width={width}>
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

            maxlength={maxLength || undefined}
            min={min || undefined}
            max={max || undefined}
            {step}
            required={required || undefined}
            aria-required={required || false}
            aria-invalid={isError}
            pattern={pattern || undefined}

            {oninput}
            {oninvalid}
            {onblur}
            {onkeydown}
            onsubmit={onSubmit}
    />
    <div aria-live="assertive" class="label">
        <label for={id} class="font-label noselect" data-required={required}>
            {label}
        </label>
        {#if isError}
            <div
                    aria-relevant="all"
                    class="error"
                    class:errWithLabel={!!label}
                    transition:slide={{duration: 150}}
            >
                {errMsg || t.common.invalidInput}
            </div>
        {/if}
    </div>
</div>

<style>
    label, .error {
        line-height: 1.1rem;
        font-size: .9rem;
    }

    label {
        flex-wrap: wrap;
    }

    .label {
        width: 100%;
        margin-top: -1.1rem;
        padding: .5rem;
    }

    .error {
        color: hsl(var(--error));
    }

    .errWithLabel {
        margin-top: -.5rem;
    }
</style>
