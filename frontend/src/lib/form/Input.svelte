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
        'data-bwignore': bitwardenIgnore,
        placeholder = '',
        description,
        formatBadge,
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
        'data-bwignore'?: boolean;
        placeholder?: string;
        description?: string;
        formatBadge?: string;
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
    const helpId = genKey();
    const badgeId = genKey();

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
    <div aria-live="assertive" class="label admin-field-label">
        <label for={id} class="font-label noselect" data-required={required}>
            {label}
        </label>
    </div>
    <div class="control" class:with-badge={formatBadge}>
        <input
            class="ui-text-input"
            bind:this={ref}
            type={typ}
            {id}
            {name}
            title={errMsg}
            aria-label={label || placeholder}
            aria-describedby={[description && helpId, formatBadge && badgeId]
                .filter(Boolean)
                .join(' ') || undefined}
            bind:value
            {autocomplete}
            data-bwignore={bitwardenIgnore ? true : undefined}
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
        {#if formatBadge}<span class="ui-format-badge" id={badgeId}>{formatBadge}</span>{/if}
    </div>
    {#if description}<p class="ui-field-hint" id={helpId}>{description}</p>{/if}
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
    .control {
        display: contents;
    }

    .with-badge {
        display: grid;
        position: relative;
    }

    :global(.ui-shell) .with-badge input.ui-text-input {
        padding-inline-end: 7.5rem;
    }

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

    :global(.ui-shell) .admin-input-field:not(:global(.branding-preview *)) {
        max-width: 100%;
        margin-bottom: var(--ui-field-margin, var(--ui-space));
    }
    :global(.ui-shell) input.invalid:not(:global(.branding-preview *)) {
        border-color: hsl(var(--error));
    }
</style>
